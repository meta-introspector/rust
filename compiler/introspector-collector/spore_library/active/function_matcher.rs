use std::collections::HashSet;
use std::fs;
use std::process::Command;
use serde_json::Value;

fn main() {
    println!("🔍 Function Matcher - Self-Profile Functions vs Usage JSON");
    println!("========================================================");
    
    // Step 1: Extract all symbols from usage JSON
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
    
    let mut usage_symbols = HashSet::new();
    
    // Extract symbols from file_details
    if let Some(file_details) = usage_data.get("file_details").and_then(|f| f.as_object()) {
        for (_, file_data) in file_details {
            if let Some(usages) = file_data.as_array() {
                for usage in usages {
                    if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                        usage_symbols.insert(symbol.to_string());
                    }
                }
            }
        }
    }
    
    println!("📊 Found {} unique symbols in usage data", usage_symbols.len());
    
    // Step 2: Get functions from self-profile data
    let profile_files = Command::new("find")
        .args(&["../../", "-name", "*profdata"])
        .output()
        .expect("Failed to find profile files");
    
    let profile_output = String::from_utf8_lossy(&profile_files.stdout);
    let files: Vec<&str> = profile_output.lines().collect();
    
    if files.is_empty() {
        println!("❌ No profile files found");
        return;
    }
    
    println!("🔧 Analyzing {} profile files...", files.len());
    
    let mut total_matches = 0;
    let mut total_functions = 0;
    let mut matched_functions = Vec::new();
    
    for file_path in files.iter().take(3) { // Analyze first 3 files
        let profile_base = file_path.trim_end_matches(".mm_profdata");
        let crate_name = file_path.split('/').last()
            .and_then(|f| f.split('-').next())
            .unwrap_or("unknown");
        
        println!("  📋 Analyzing {}", crate_name);
        
        let summarize_output = Command::new("../measureme/target/release/summarize")
            .args(&["summarize", profile_base])
            .current_dir("../../rust")
            .output();
        
        match summarize_output {
            Ok(result) => {
                let summary_text = String::from_utf8_lossy(&result.stdout);
                
                // Extract function names from summary
                for line in summary_text.lines() {
                    // Look for function-like patterns
                    if line.contains("::") || line.contains("fn ") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        for part in parts {
                            if part.contains("::") {
                                let func_name = part.split("::").last().unwrap_or(part);
                                total_functions += 1;
                                
                                if usage_symbols.contains(func_name) {
                                    total_matches += 1;
                                    matched_functions.push((crate_name.to_string(), func_name.to_string()));
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("    ❌ Error: {}", e);
            }
        }
    }
    
    println!("\n🎯 FUNCTION MATCHING RESULTS:");
    println!("- Usage symbols: {}", usage_symbols.len());
    println!("- Profile functions: {}", total_functions);
    println!("- Matches found: {}", total_matches);
    println!("- Match rate: {:.1}%", (total_matches as f64 / total_functions as f64) * 100.0);
    
    if !matched_functions.is_empty() {
        println!("\n📋 Matched Functions:");
        for (crate_name, func_name) in matched_functions.iter().take(10) {
            println!("  - {} in {}", func_name, crate_name);
        }
        if matched_functions.len() > 10 {
            println!("  ... and {} more", matched_functions.len() - 10);
        }
    }
}
