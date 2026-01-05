use std::collections::HashMap;
use std::fs;
use std::process::Command;
use serde_json::Value;

fn main() {
    println!("🔍 Function Tracer - Connecting Runtime to Usage Index");
    println!("====================================================");
    
    // Step 1: Load our comprehensive usage report
    let usage_report = match fs::read_to_string("comprehensive_usage_report.json") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading usage report: {}", e);
            return;
        }
    };
    
    let usage_data: Value = match serde_json::from_str(&usage_report) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Error parsing usage report: {}", e);
            return;
        }
    };
    
    // Step 2: Extract function names from usage data
    let mut function_index = HashMap::new();
    
    if let Some(crates) = usage_data.get("crates").and_then(|c| c.as_object()) {
        for (crate_name, crate_data) in crates {
            if let Some(modules) = crate_data.get("modules").and_then(|m| m.as_object()) {
                for (module_name, module_data) in modules {
                    if let Some(usages) = module_data.get("usages").and_then(|u| u.as_array()) {
                        for usage in usages {
                            if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                let key = format!("{}::{}", crate_name, symbol);
                                function_index.insert(key, (crate_name.clone(), module_name.clone(), symbol.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Indexed {} functions from usage data", function_index.len());
    
    // Step 3: Simulate runtime function tracing (using strace as example)
    println!("\n🔧 Tracing compilation with strace...");
    
    let output = Command::new("strace")
        .args(&["-e", "trace=execve", "-o", "/tmp/trace.log", "cargo", "build", "--bin", "monster_rustc_simple"])
        .current_dir("../usage_eigenmatrix")
        .output();
    
    match output {
        Ok(result) => {
            println!("✅ Trace completed with exit code: {}", result.status.code().unwrap_or(-1));
            
            // Read trace log
            if let Ok(trace_content) = fs::read_to_string("/tmp/trace.log") {
                println!("\n📋 Trace Analysis:");
                
                let mut matched_functions = Vec::new();
                
                // Look for function names in trace that match our index
                for line in trace_content.lines() {
                    for (indexed_func, (crate_name, module_name, symbol)) in &function_index {
                        if line.contains(symbol) {
                            matched_functions.push((indexed_func.clone(), crate_name, module_name, symbol));
                        }
                    }
                }
                
                if matched_functions.is_empty() {
                    println!("🔍 No direct function matches found in trace");
                    println!("💡 This is expected - strace shows system calls, not Rust functions");
                } else {
                    println!("🎯 Found {} function matches:", matched_functions.len());
                    for (func, crate_name, module_name, symbol) in matched_functions.iter().take(10) {
                        println!("  - {} ({}::{})", symbol, crate_name, module_name);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Trace failed: {}", e);
        }
    }
    
    // Step 4: Create function mapping report
    let mut report = String::new();
    report.push_str("# Function Tracer Report\n\n");
    report.push_str("## Runtime Tracing → Usage Index Mapping\n\n");
    report.push_str(&format!("- **Total indexed functions**: {}\n", function_index.len()));
    report.push_str("- **Tracing method**: strace (system calls)\n");
    report.push_str("- **Next steps**: Use rustc compiler tracing for actual function calls\n\n");
    
    report.push_str("## Top Functions by Crate\n\n");
    let mut crate_counts = HashMap::new();
    for (_, (crate_name, _, _)) in &function_index {
        *crate_counts.entry(crate_name).or_insert(0) += 1;
    }
    
    let mut sorted_crates: Vec<_> = crate_counts.iter().collect();
    sorted_crates.sort_by(|a, b| b.1.cmp(a.1));
    
    for (crate_name, count) in sorted_crates.iter().take(10) {
        report.push_str(&format!("- **{}**: {} functions\n", crate_name, count));
    }
    
    report.push_str("\n## Integration Strategy\n\n");
    report.push_str("1. **Compiler Tracing**: Use rustc's built-in tracing for function calls\n");
    report.push_str("2. **Symbol Matching**: Map traced functions to usage index\n");
    report.push_str("3. **Performance Analysis**: Correlate runtime performance with usage patterns\n");
    report.push_str("4. **Optimization**: Identify hot paths in compilation\n");
    
    if let Err(e) = fs::write("function_tracer_report.md", &report) {
        eprintln!("❌ Error writing report: {}", e);
    } else {
        println!("\n📊 Function tracer report saved: function_tracer_report.md");
    }
    
    println!("\n🎯 NEXT STEPS:");
    println!("1. Use RUSTC_LOG=debug for compiler function tracing");
    println!("2. Parse rustc debug output for function names");
    println!("3. Match traced functions against our usage index");
    println!("4. Create performance correlation analysis");
}
