use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;
use libc::{dlsym, RTLD_NEXT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct InterceptionLog {
    function_name: String,
    timestamp: u64,
    args: Vec<u64>,
    return_value: Option<u64>,
    thread_id: u64,
}

static INTERCEPT_LOG: Mutex<Vec<InterceptionLog>> = Mutex::new(Vec::new());

// Function pointer types for common rustc functions
type ParseFn = unsafe extern "C" fn(*const c_void) -> *const c_void;
type TokenizeFn = unsafe extern "C" fn(*const c_char) -> *const c_void;

// Original function pointers
static mut ORIGINAL_PARSE: Option<ParseFn> = None;
static mut ORIGINAL_TOKENIZE: Option<TokenizeFn> = None;

fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

fn get_thread_id() -> u64 {
    unsafe { libc::pthread_self() as u64 }
}

fn log_call(function_name: &str, args: Vec<u64>, return_value: Option<u64>) {
    let entry = InterceptionLog {
        function_name: function_name.to_string(),
        timestamp: get_timestamp(),
        args,
        return_value,
        thread_id: get_thread_id(),
    };
    
    if let Ok(mut log) = INTERCEPT_LOG.lock() {
        log.push(entry);
        
        // Flush to disk periodically
        if log.len() % 1000 == 0 {
            flush_log(&log);
        }
    }
}

fn flush_log(log: &[InterceptionLog]) {
    if let Ok(json) = serde_json::to_string_pretty(log) {
        let _ = fs::write("interception_log.json", json);
    }
}

// Intercepted functions
#[no_mangle]
pub unsafe extern "C" fn rustc_parse_something(input: *const c_void) -> *const c_void {
    // Get original function if not already loaded
    if ORIGINAL_PARSE.is_none() {
        let symbol = CString::new("rustc_parse_something").unwrap();
        let original = dlsym(RTLD_NEXT, symbol.as_ptr());
        if !original.is_null() {
            ORIGINAL_PARSE = Some(std::mem::transmute(original));
        }
    }
    
    let args = vec![input as u64];
    log_call("rustc_parse_something", args, None);
    
    // Call original function
    if let Some(original) = ORIGINAL_PARSE {
        let result = original(input);
        log_call("rustc_parse_something", vec![], Some(result as u64));
        result
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn rustc_tokenize(input: *const c_char) -> *const c_void {
    if ORIGINAL_TOKENIZE.is_none() {
        let symbol = CString::new("rustc_tokenize").unwrap();
        let original = dlsym(RTLD_NEXT, symbol.as_ptr());
        if !original.is_null() {
            ORIGINAL_TOKENIZE = Some(std::mem::transmute(original));
        }
    }
    
    let args = vec![input as u64];
    log_call("rustc_tokenize", args, None);
    
    if let Some(original) = ORIGINAL_TOKENIZE {
        let result = original(input);
        log_call("rustc_tokenize", vec![], Some(result as u64));
        result
    } else {
        std::ptr::null()
    }
}

// Constructor to initialize interception
#[ctor::ctor]
fn init_interception() {
    println!("Rust interception library loaded!");
    
    // Set up signal handler for clean shutdown
    unsafe {
        libc::signal(libc::SIGTERM, cleanup_handler as usize);
        libc::signal(libc::SIGINT, cleanup_handler as usize);
    }
}

extern "C" fn cleanup_handler(_: i32) {
    if let Ok(log) = INTERCEPT_LOG.lock() {
        flush_log(&log);
    }
    println!("Interception log flushed on exit");
}

// Destructor to clean up
#[ctor::dtor]
fn cleanup_interception() {
    if let Ok(log) = INTERCEPT_LOG.lock() {
        flush_log(&log);
    }
}
