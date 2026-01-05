use std::collections::HashMap;
use std::fs;
use std::process::Command;
use serde_json::Value;

fn main() {
    println!("🔍 UsageMeter - Connecting Runtime Compilation Traces to Usage Index");
    println!("================================================================");
    
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
    
    // Step 3: Find and analyze rustc profile data
    println!("\n🔧 Analyzing rustc compilation traces...");
    
    let output = Command::new("find")
        .args(&["../../", "-name", "*profdata"])
        .output();
    
    match output {
        Ok(result) => {
            let profile_files = String::from_utf8_lossy(&result.stdout);
            let files: Vec<&str> = profile_files.lines().collect();
            
            println!("📋 Found {} profile files:", files.len());
            
            let mut trace_analysis = Vec::new();
            
            for (i, file_path) in files.iter().enumerate().take(5) { // Analyze first 5 files
                println!("  {}. {}", i + 1, file_path);
                
                // Extract crate name from path
                let crate_name = file_path.split('/').last()
                    .and_then(|f| f.split('-').next())
                    .unwrap_or("unknown");
                
                // Use measureme to analyze the profile
                let profile_base = file_path.trim_end_matches(".mm_profdata");
                let summarize_output = Command::new("../measureme/target/release/summarize")
                    .args(&["summarize", profile_base])
                    .current_dir("../../rust")
                    .output();
                
                match summarize_output {
                    Ok(summary_result) => {
                        let summary_text = String::from_utf8_lossy(&summary_result.stdout);
                        
                        // Look for function names in the summary that match our index
                        let mut matched_functions = Vec::new();
                        
                        for line in summary_text.lines() {
                            for (indexed_func, (orig_crate, module_name, symbol)) in &function_index {
                                if line.contains(symbol) || line.contains(&format!("{}::", orig_crate)) {
                                    matched_functions.push((indexed_func.clone(), orig_crate, module_name, symbol));
                                }
                            }
                        }
                        
                        let match_count = matched_functions.len();
                        let has_matches = !matched_functions.is_empty();
                        
                        trace_analysis.push((crate_name.to_string(), match_count, matched_functions));
                        
                        if has_matches {
                            println!("    🎯 Found {} function matches in {}", match_count, crate_name);
                        }
                    }
                    Err(e) => {
                        println!("    ❌ Error analyzing {}: {}", crate_name, e);
                    }
                }
            }
            
            // Step 4: Generate UsageMeter report
            let mut report = String::new();
            report.push_str("# UsageMeter Report\n\n");
            report.push_str("## Runtime Compilation Traces → Usage Index Correlation\n\n");
            report.push_str(&format!("- **Total indexed functions**: {}\n", function_index.len()));
            report.push_str(&format!("- **Profile files analyzed**: {}\n", files.len().min(5)));
            report.push_str("- **Tracing method**: rustc -Zself-profile\n");
            report.push_str("- **Analysis tool**: measureme\n\n");
            
            report.push_str("## Trace Analysis Results\n\n");
            
            let mut total_matches = 0;
            for (crate_name, match_count, matches) in &trace_analysis {
                total_matches += match_count;
                report.push_str(&format!("### {}\n", crate_name));
                report.push_str(&format!("- **Function matches**: {}\n", match_count));
                
                if !matches.is_empty() {
                    report.push_str("- **Matched functions**:\n");
                    for (func, orig_crate, module_name, symbol) in matches.iter().take(5) {
                        report.push_str(&format!("  - `{}` ({}::{})\n", symbol, orig_crate, module_name));
                    }
                    if matches.len() > 5 {
                        report.push_str(&format!("  - ... and {} more\n", matches.len() - 5));
                    }
                }
                report.push_str("\n");
            }
            
            report.push_str(&format!("## Summary\n\n"));
            report.push_str(&format!("- **Total function correlations**: {}\n", total_matches));
            report.push_str(&format!("- **Correlation rate**: {:.1}%\n", 
                (total_matches as f64 / function_index.len() as f64) * 100.0));
            
            report.push_str("\n## Next Steps\n\n");
            report.push_str("1. **Performance Hotspots**: Identify most frequently called functions in traces\n");
            report.push_str("2. **Usage Patterns**: Correlate runtime frequency with static usage counts\n");
            report.push_str("3. **Optimization Targets**: Find high-usage, high-runtime-cost functions\n");
            report.push_str("4. **Compilation Bottlenecks**: Analyze slow compilation phases\n");
            
            if let Err(e) = fs::write("usagemeter_report.md", &report) {
                eprintln!("❌ Error writing report: {}", e);
            } else {
                println!("\n📊 UsageMeter report saved: usagemeter_report.md");
            }
            
            println!("\n🎯 USAGEMETER RESULTS:");
            println!("- Indexed functions: {}", function_index.len());
            println!("- Profile files: {}", files.len());
            println!("- Function correlations: {}", total_matches);
            println!("- Correlation rate: {:.1}%", (total_matches as f64 / function_index.len() as f64) * 100.0);
        }
        Err(e) => {
            println!("❌ Error finding profile files: {}", e);
        }
    }
}
