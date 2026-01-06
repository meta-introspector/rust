use std::collections::{HashMap, HashSet};
use std::fs;
use std::process::Command;

fn main() {
    println!("🔥 Hot Function Analysis - Profile vs Static Usage");
    println!("================================================");
    
    // Step 1: Get all static symbols
    let mut static_symbols = HashSet::new();
    
    let find_output = Command::new("find")
        .args(&["../../test_usage_data", "-name", "*.json", "-not", "-name", "*manifest*"])
        .output()
        .expect("Failed to find usage files");
    
    let files = String::from_utf8_lossy(&find_output.stdout);
    
    for file_path in files.lines() {
        if let Ok(content) = fs::read_to_string(file_path.trim()) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
                    for usage in usages {
                        if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                            static_symbols.insert(symbol.to_string());
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Static symbols found: {}", static_symbols.len());
    
    // Step 2: Analyze profile data for hot functions
    let profile_output = Command::new("find")
        .args(&["../../", "-name", "*profdata"])
        .output()
        .expect("Failed to find profile files");
    
    let profile_files = String::from_utf8_lossy(&profile_output.stdout);
    let files: Vec<&str> = profile_files.lines().collect();
    
    let mut hot_functions = HashMap::new();
    let mut profile_functions = HashSet::new();
    let mut matched_functions = Vec::new();
    
    println!("🔧 Analyzing {} profile files...", files.len());
    
    for file_path in files.iter().take(5) {
        let profile_base = file_path.trim_end_matches(".mm_profdata");
        let crate_name = file_path.split('/').last()
            .and_then(|f| f.split('-').next())
            .unwrap_or("unknown");
        
        let summarize_output = Command::new("../measureme/target/release/summarize")
            .args(&["summarize", profile_base])
            .current_dir("../../rust")
            .output();
        
        if let Ok(result) = summarize_output {
            let summary_text = String::from_utf8_lossy(&result.stdout);
            
            for line in summary_text.lines() {
                // Look for timing data and function names
                if line.contains("ms") || line.contains("μs") || line.contains("ns") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    
                    // Extract timing and function name
                    for (i, part) in parts.iter().enumerate() {
                        if part.contains("::") {
                            let func_name = part.split("::").last().unwrap_or(part);
                            profile_functions.insert(func_name.to_string());
                            
                            // Try to extract timing from previous parts
                            if i > 0 {
                                if let Some(timing_part) = parts.get(i-1) {
                                    if timing_part.contains("ms") || timing_part.contains("μs") {
                                        let timing_str = timing_part.replace("ms", "").replace("μs", "").replace("ns", "");
                                        if let Ok(timing) = timing_str.parse::<f64>() {
                                            *hot_functions.entry(func_name.to_string()).or_insert(0.0) += timing;
                                        }
                                    }
                                }
                            }
                            
                            // Check if this function is in our static symbols
                            if static_symbols.contains(func_name) {
                                matched_functions.push((crate_name.to_string(), func_name.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Step 3: Generate report
    println!("\n🎯 CORRELATION ANALYSIS:");
    println!("- Static symbols: {}", static_symbols.len());
    println!("- Profile functions: {}", profile_functions.len());
    println!("- Matched functions: {}", matched_functions.len());
    
    let match_rate = if static_symbols.len() > 0 {
        (matched_functions.len() as f64 / static_symbols.len() as f64) * 100.0
    } else {
        0.0
    };
    println!("- Match rate: {:.1}%", match_rate);
    
    // Sort hot functions by timing
    let mut sorted_hot: Vec<_> = hot_functions.iter().collect();
    sorted_hot.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    println!("\n🔥 HOTTEST FUNCTIONS (with timing data):");
    for (func, timing) in sorted_hot.iter().take(10) {
        let in_static = if static_symbols.contains(*func) { "✅" } else { "❌" };
        println!("  {} {:.2}ms - {} {}", in_static, timing, func, 
                if static_symbols.contains(*func) { "(FOUND in static)" } else { "(not in static)" });
    }
    
    if matched_functions.len() > 0 {
        println!("\n📋 MATCHED FUNCTIONS (in both profile and static):");
        for (crate_name, func_name) in matched_functions.iter().take(10) {
            let timing = hot_functions.get(func_name).unwrap_or(&0.0);
            println!("  - {} in {} ({:.2}ms)", func_name, crate_name, timing);
        }
    }
    
    println!("\n📈 SUMMARY:");
    println!("- Found: {} functions in both profile and static data", matched_functions.len());
    println!("- Not found: {} static symbols not seen in profiles", static_symbols.len() - matched_functions.len());
    println!("- Hot functions with timing: {}", hot_functions.len());
}
