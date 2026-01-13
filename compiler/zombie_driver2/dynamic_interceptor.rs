use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TraceEntry {
    pub symbol: String,
    pub timestamp: u64,
    pub args: Vec<u64>,
    pub return_value: Option<u64>,
    pub call_depth: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallGraph {
    pub nodes: HashMap<String, CallNode>,
    pub edges: Vec<CallEdge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallNode {
    pub name: String,
    pub call_count: u64,
    pub total_time: f64,
    pub self_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallEdge {
    pub from: String,
    pub to: String,
    pub count: u64,
}

pub struct RustcInterceptor {
    symbols: Vec<String>,
    trace_log: Vec<TraceEntry>,
    output_dir: PathBuf,
}

impl RustcInterceptor {
    pub fn new(target_functions: Vec<String>) -> Self {
        Self {
            symbols: target_functions,
            trace_log: Vec::new(),
            output_dir: PathBuf::from("./interception_data"),
        }
    }

    pub fn setup_interception(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;
        
        // Build the LD_PRELOAD library
        self.build_preload_lib()?;
        
        // Generate function hooks
        self.generate_hooks()?;
        
        Ok(())
    }

    fn build_preload_lib(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Building LD_PRELOAD interception library...");
        
        // Create Cargo.toml for the interceptor
        let interceptor_toml = r#"
[package]
name = "rustc_interceptor"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
libc = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ctor = "0.2"
"#;
        
        fs::write("interceptor/Cargo.toml", interceptor_toml)?;
        
        // Copy the interceptor source
        let interceptor_src = fs::read_to_string("ld_preload_interceptor.rs")?;
        fs::create_dir_all("interceptor/src")?;
        fs::write("interceptor/src/lib.rs", interceptor_src)?;
        
        // Build the library
        let status = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir("interceptor")
            .status()?;
        
        if !status.success() {
            return Err("Failed to build interceptor library".into());
        }
        
        println!("Interceptor library built successfully");
        Ok(())
    }

    fn generate_hooks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut hook_code = String::new();
        
        for symbol in &self.symbols {
            hook_code.push_str(&format!(r#"
#[no_mangle]
pub unsafe extern "C" fn {}(args: *const c_void) -> *const c_void {{
    static mut ORIGINAL: Option<unsafe extern "C" fn(*const c_void) -> *const c_void> = None;
    
    if ORIGINAL.is_none() {{
        let symbol = CString::new("{}").unwrap();
        let original = dlsym(RTLD_NEXT, symbol.as_ptr());
        if !original.is_null() {{
            ORIGINAL = Some(std::mem::transmute(original));
        }}
    }}
    
    log_call("{}", vec![args as u64], None);
    
    if let Some(original) = ORIGINAL {{
        let result = original(args);
        log_call("{}", vec![], Some(result as u64));
        result
    }} else {{
        std::ptr::null()
    }}
}}
"#, symbol, symbol, symbol, symbol));
        }
        
        fs::write(self.output_dir.join("generated_hooks.rs"), hook_code)?;
        Ok(())
    }

    pub fn run_with_interception(&self, source_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib_path = "interceptor/target/release/librustc_interceptor.so";
        
        println!("Running rustc with interception on: {}", source_file);
        
        let status = Command::new("rustc")
            .arg(source_file)
            .env("LD_PRELOAD", lib_path)
            .env("RUST_LOG", "debug")
            .status()?;
        
        if !status.success() {
            println!("Warning: rustc exited with non-zero status, but interception may have worked");
        }
        
        // Collect the interception log
        if let Ok(log_content) = fs::read_to_string("interception_log.json") {
            fs::write(self.output_dir.join("interception_results.json"), log_content)?;
            println!("Interception results saved to interception_data/");
        }
        
        Ok(())
    }

    pub fn analyze_call_patterns(&self) -> Result<CallGraph, Box<dyn std::error::Error>> {
        let log_file = self.output_dir.join("interception_results.json");
        let log_content = fs::read_to_string(log_file)?;
        let entries: Vec<TraceEntry> = serde_json::from_str(&log_content)?;
        
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();
        let mut call_stack = Vec::new();
        
        for entry in entries {
            // Update node statistics
            let node = nodes.entry(entry.symbol.clone()).or_insert(CallNode {
                name: entry.symbol.clone(),
                call_count: 0,
                total_time: 0.0,
                self_time: 0.0,
            });
            node.call_count += 1;
            
            // Track call relationships
            if let Some(caller) = call_stack.last() {
                edges.push(CallEdge {
                    from: caller.clone(),
                    to: entry.symbol.clone(),
                    count: 1,
                });
            }
            
            // Manage call stack
            if entry.return_value.is_none() {
                // Function entry
                call_stack.push(entry.symbol);
            } else {
                // Function exit
                call_stack.pop();
            }
        }
        
        Ok(CallGraph { nodes, edges })
    }

    pub fn generate_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        let call_graph = self.analyze_call_patterns()?;
        
        let mut report = String::new();
        report.push_str("# Rustc Parser Interception Report\n\n");
        
        report.push_str("## Function Call Statistics\n\n");
        let mut sorted_nodes: Vec<_> = call_graph.nodes.values().collect();
        sorted_nodes.sort_by(|a, b| b.call_count.cmp(&a.call_count));
        
        for node in sorted_nodes.iter().take(20) {
            report.push_str(&format!(
                "- **{}**: {} calls\n",
                node.name, node.call_count
            ));
        }
        
        report.push_str("\n## Call Relationships\n\n");
        for edge in call_graph.edges.iter().take(10) {
            report.push_str(&format!(
                "- {} → {} ({} times)\n",
                edge.from, edge.to, edge.count
            ));
        }
        
        fs::write(self.output_dir.join("interception_report.md"), report)?;
        
        // Save call graph as JSON
        let graph_json = serde_json::to_string_pretty(&call_graph)?;
        fs::write(self.output_dir.join("call_graph.json"), graph_json)?;
        
        println!("Report generated: interception_data/interception_report.md");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load target functions from previous analysis
    let targets_content = fs::read_to_string("parser_target_functions.json")
        .unwrap_or_else(|_| "[]".to_string());
    let targets: Vec<serde_json::Value> = serde_json::from_str(&targets_content)?;
    
    let target_functions: Vec<String> = targets
        .iter()
        .filter_map(|t| t["name"].as_str().map(|s| s.to_string()))
        .collect();
    
    if target_functions.is_empty() {
        println!("No target functions found. Run advanced_perf_recorder first.");
        return Ok(());
    }
    
    println!("Setting up interception for {} functions", target_functions.len());
    
    let interceptor = RustcInterceptor::new(target_functions);
    
    // Setup interception
    interceptor.setup_interception()?;
    
    // Create test file if needed
    if !std::path::Path::new("test.rs").exists() {
        fs::write("test.rs", r#"
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    for i in 0..10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
}
"#)?;
    }
    
    // Run with interception
    interceptor.run_with_interception("test.rs")?;
    
    // Generate analysis report
    interceptor.generate_report()?;
    
    println!("Interception complete! Check interception_data/ for results.");
    
    Ok(())
}
