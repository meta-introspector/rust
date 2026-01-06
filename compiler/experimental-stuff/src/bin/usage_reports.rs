use std::collections::HashMap;
use std::fs;
use analyzeme::ProfilingData;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Generating Usage Reports - Static vs Runtime");
    
    // Load static usage data from test_usage_data directory
    let mut static_usage: HashMap<String, usize> = HashMap::new();
    
    // Read all usage JSON files
    if let Ok(entries) = fs::read_dir("../../test_usage_data") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") 
                && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                            for usage in usages {
                                if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                    if let Some(count) = usage.get("usage_count").and_then(|c| c.as_u64()) {
                                        *static_usage.entry(symbol.to_string()).or_insert(0) += count as usize;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Load runtime profile data
    let mut runtime_usage: HashMap<String, usize> = HashMap::new();
    
    // Read all .mm_profdata files in ../../compilation_trace directory
    if let Ok(entries) = fs::read_dir("../../compilation_trace") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mm_profdata") {
                if let Ok(profiling_data) = ProfilingData::new(&path) {
                    let results = profiling_data.perform_analysis();
                    for query in &results.query_data {
                        let function_name = query.label.to_string();
                        *runtime_usage.entry(function_name).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    // Sort and write static usage report
    let mut static_sorted: Vec<_> = static_usage.into_iter().collect();
    static_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut static_report = format!("STATIC USAGE ANALYSIS - {} symbols\n", static_sorted.len());
    static_report.push_str("=".repeat(50).as_str());
    static_report.push('\n');
    
    for (i, (symbol, count)) in static_sorted.iter().enumerate() {
        static_report.push_str(&format!("{:4}. {:6} {}\n", i + 1, count, symbol));
    }
    
    fs::write("static.txt", static_report)?;
    
    // Sort and write runtime usage report
    let mut runtime_sorted: Vec<_> = runtime_usage.into_iter().collect();
    runtime_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut runtime_report = format!("RUNTIME USAGE ANALYSIS - {} functions\n", runtime_sorted.len());
    runtime_report.push_str("=".repeat(50).as_str());
    runtime_report.push('\n');
    
    for (i, (function, count)) in runtime_sorted.iter().enumerate() {
        runtime_report.push_str(&format!("{:4}. {:6} {}\n", i + 1, count, function));
    }
    
    fs::write("runtime.txt", runtime_report)?;
    
    println!("✅ Reports generated:");
    println!("   📄 static.txt  - {} symbols by usage frequency", static_sorted.len());
    println!("   📄 runtime.txt - {} functions by call frequency", runtime_sorted.len());
    
    Ok(())
}
