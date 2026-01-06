use std::collections::{HashMap, HashSet};
use std::fs;
use std::process::Command;
use analyzeme::ProfilingData;

fn main() {
    println!("🔥 Direct Profile Reader - Runtime Functions vs Static Usage");
    println!("==========================================================");
    
    // Step 1: Get static symbols
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
    
    println!("📊 Static symbols: {}", static_symbols.len());
    
    // Step 2: Read profile data directly
    let profile_output = Command::new("find")
        .args(&["../../", "-name", "*profdata"])
        .output()
        .expect("Failed to find profile files");
    
    let profile_files = String::from_utf8_lossy(&profile_output.stdout);
    let files: Vec<&str> = profile_files.lines().collect();
    
    let mut runtime_functions = HashSet::new();
    let mut matched_functions = Vec::new();
    
    println!("🔧 Reading {} profile files directly...", files.len());
    
    for file_path in files.iter().take(3) {
        let profile_base = file_path.trim_end_matches(".mm_profdata");
        let crate_name = file_path.split('/').last()
            .and_then(|f| f.split('-').next())
            .unwrap_or("unknown");
        
        println!("  📋 Reading {}", crate_name);
        
        match ProfilingData::new(&std::path::PathBuf::from(profile_base)) {
            Ok(data) => {
                let results = data.perform_analysis();
                
                println!("    Found {} query entries", results.query_data.len());
                
                for query in &results.query_data {
                    let label = &query.label;
                    runtime_functions.insert(label.clone());
                    
                    // Extract function name from label (labels often contain module::function format)
                    if let Some(func_name) = label.split("::").last() {
                        if static_symbols.contains(func_name) {
                            matched_functions.push((crate_name.to_string(), func_name.to_string(), query.self_time));
                        }
                    }
                    
                    // Also check full label
                    if static_symbols.contains(label) {
                        matched_functions.push((crate_name.to_string(), label.clone(), query.self_time));
                    }
                }
            }
            Err(e) => {
                println!("    ❌ Error reading profile: {}", e);
            }
        }
    }
    
    println!("\n🎯 DIRECT PROFILE ANALYSIS:");
    println!("- Static symbols: {}", static_symbols.len());
    println!("- Runtime functions: {}", runtime_functions.len());
    println!("- Matched functions: {}", matched_functions.len());
    
    let match_rate = if static_symbols.len() > 0 {
        (matched_functions.len() as f64 / static_symbols.len() as f64) * 100.0
    } else {
        0.0
    };
    println!("- Match rate: {:.1}%", match_rate);
    
    if !runtime_functions.is_empty() {
        println!("\n🔥 TOP RUNTIME FUNCTIONS:");
        for (i, func) in runtime_functions.iter().take(10).enumerate() {
            println!("  {}. {}", i + 1, func);
        }
    }
    
    if !matched_functions.is_empty() {
        println!("\n✅ MATCHED FUNCTIONS (in both runtime and static):");
        matched_functions.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by timing
        for (crate_name, func_name, timing) in matched_functions.iter().take(10) {
            println!("  - {} in {} ({:.2?})", func_name, crate_name, timing);
        }
    }
    
    println!("\n📈 SUMMARY:");
    println!("- Found: {} functions in both profile and static data", matched_functions.len());
    println!("- Runtime only: {} functions not in static data", runtime_functions.len() - matched_functions.len());
    println!("- Static only: {} symbols not seen in runtime", static_symbols.len() - matched_functions.len());
}
