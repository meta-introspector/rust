use std::ffi::{CStr, CString};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;
use libc::{dlsym, RTLD_NEXT};
use serde_json::json;

// Global state for capturing parser data
static PARSER_LOG: Mutex<Option<std::fs::File>> = Mutex::new(None);
static mut CALL_DEPTH: u32 = 0;

fn init_logging() {
    let mut log = PARSER_LOG.lock().unwrap();
    if log.is_none() {
        if let Ok(file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("live_parser_capture.jsonl") 
        {
            *log = Some(file);
            println!("Parser capture initialized: live_parser_capture.jsonl");
        }
    }
}

fn log_parser_event(event_type: &str, function: &str, data: serde_json::Value) {
    init_logging();
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let entry = json!({
        "timestamp": timestamp,
        "event": event_type,
        "function": function,
        "depth": unsafe { CALL_DEPTH },
        "thread": format!("{:?}", std::thread::current().id()),
        "data": data
    });
    
    if let Ok(mut log) = PARSER_LOG.lock() {
        if let Some(ref mut file) = *log {
            let _ = writeln!(file, "{}", entry);
            let _ = file.flush();
        }
    }
}

// Intercept key rustc parser functions
#[no_mangle]
pub unsafe extern "C" fn rustc_parse_crate_from_file(
    path: *const c_char,
    sess: *const c_void
) -> *const c_void {
    static mut ORIGINAL: Option<unsafe extern "C" fn(*const c_char, *const c_void) -> *const c_void> = None;
    
    if ORIGINAL.is_none() {
        let symbol = CString::new("rustc_parse_crate_from_file").unwrap();
        let original = dlsym(RTLD_NEXT, symbol.as_ptr());
        if !original.is_null() {
            ORIGINAL = Some(std::mem::transmute(original));
        }
    }
    
    CALL_DEPTH += 1;
    
    let path_str = if !path.is_null() {
        CStr::from_ptr(path).to_string_lossy().to_string()
    } else {
        "null".to_string()
    };
    
    log_parser_event("enter", "rustc_parse_crate_from_file", json!({
        "file_path": path_str,
        "session_ptr": format!("{:p}", sess)
    }));
    
    let result = if let Some(original) = ORIGINAL {
        original(path, sess)
    } else {
        std::ptr::null()
    };
    
    log_parser_event("exit", "rustc_parse_crate_from_file", json!({
        "result_ptr": format!("{:p}", result),
        "success": !result.is_null()
    }));
    
    CALL_DEPTH -= 1;
    result
}

#[no_mangle]
pub unsafe extern "C" fn rustc_parse_token_stream(
    input: *const c_char,
    len: usize
) -> *const c_void {
    static mut ORIGINAL: Option<unsafe extern "C" fn(*const c_char, usize) -> *const c_void> = None;
    
    if ORIGINAL.is_none() {
        let symbol = CString::new("rustc_parse_token_stream").unwrap();
        let original = dlsym(RTLD_NEXT, symbol.as_ptr());
        if !original.is_null() {
            ORIGINAL = Some(std::mem::transmute(original));
        }
    }
    
    CALL_DEPTH += 1;
    
    // Capture the actual source code being parsed!
    let source_snippet = if !input.is_null() && len > 0 && len < 1000 {
        let slice = std::slice::from_raw_parts(input as *const u8, len.min(500));
        String::from_utf8_lossy(slice).to_string()
    } else {
        format!("len={}", len)
    };
    
    log_parser_event("enter", "rustc_parse_token_stream", json!({
        "source_length": len,
        "source_preview": source_snippet,
        "input_ptr": format!("{:p}", input)
    }));
    
    let result = if let Some(original) = ORIGINAL {
        original(input, len)
    } else {
        std::ptr::null()
    };
    
    log_parser_event("exit", "rustc_parse_token_stream", json!({
        "token_stream_ptr": format!("{:p}", result),
        "tokens_created": !result.is_null()
    }));
    
    CALL_DEPTH -= 1;
    result
}

#[no_mangle]
pub unsafe extern "C" fn rustc_parse_expr(
    parser: *const c_void
) -> *const c_void {
    static mut ORIGINAL: Option<unsafe extern "C" fn(*const c_void) -> *const c_void> = None;
    
    if ORIGINAL.is_none() {
        let symbol = CString::new("rustc_parse_expr").unwrap();
        let original = dlsym(RTLD_NEXT, symbol.as_ptr());
        if !original.is_null() {
            ORIGINAL = Some(std::mem::transmute(original));
        }
    }
    
    CALL_DEPTH += 1;
    
    log_parser_event("enter", "rustc_parse_expr", json!({
        "parser_ptr": format!("{:p}", parser)
    }));
    
    let result = if let Some(original) = ORIGINAL {
        original(parser)
    } else {
        std::ptr::null()
    };
    
    log_parser_event("exit", "rustc_parse_expr", json!({
        "expr_ptr": format!("{:p}", result),
        "expr_parsed": !result.is_null()
    }));
    
    CALL_DEPTH -= 1;
    result
}

// Intercept AST node creation
#[no_mangle]
pub unsafe extern "C" fn rustc_ast_node_new(
    node_type: u32,
    data: *const c_void,
    span_start: u32,
    span_end: u32
) -> *const c_void {
    static mut ORIGINAL: Option<unsafe extern "C" fn(u32, *const c_void, u32, u32) -> *const c_void> = None;
    
    if ORIGINAL.is_none() {
        let symbol = CString::new("rustc_ast_node_new").unwrap();
        let original = dlsym(RTLD_NEXT, symbol.as_ptr());
        if !original.is_null() {
            ORIGINAL = Some(std::mem::transmute(original));
        }
    }
    
    // This is GOLD - we're capturing AST node creation in real-time!
    log_parser_event("ast_node", "rustc_ast_node_new", json!({
        "node_type": node_type,
        "span": {
            "start": span_start,
            "end": span_end,
            "length": span_end - span_start
        },
        "data_ptr": format!("{:p}", data)
    }));
    
    let result = if let Some(original) = ORIGINAL {
        original(node_type, data, span_start, span_end)
    } else {
        std::ptr::null()
    };
    
    result
}

// Constructor - called when library loads
#[ctor::ctor]
fn init_parser_interception() {
    println!("🧟 Zombie Parser Interceptor loaded!");
    println!("📝 Capturing live rustc parsing to: live_parser_capture.jsonl");
    
    // Initialize logging
    init_logging();
    
    // Log startup
    log_parser_event("system", "interceptor_init", json!({
        "message": "Parser interception active",
        "pid": std::process::id(),
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }));
}

// Destructor - called when library unloads
#[ctor::dtor]
fn cleanup_parser_interception() {
    log_parser_event("system", "interceptor_cleanup", json!({
        "message": "Parser interception shutting down"
    }));
    
    println!("🧟 Zombie Parser Interceptor unloaded");
    println!("📊 Check live_parser_capture.jsonl for captured data");
}
