use std::fs;
use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("🏹 Type Arrow Data Histogram");
    
    // Load existing JSON files from mycelial data
    let data_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    let mut total_usage = 0u64;
    let mut kind_counts = HashMap::new();
    
    // Read all JSON files in the directory
    if let Ok(entries) = fs::read_dir(data_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            // Handle both array format and object with usages field
                            let usages = if let Some(array) = json.as_array() {
                                array
                            } else if let Some(usages_array) = json.get("usages").and_then(|v| v.as_array()) {
                                usages_array
                            } else {
                                continue;
                            };
                            
                            for item in usages {
                                if let Some(usage_count) = item.get("usage_count").and_then(|v| v.as_u64()) {
                                    total_usage += usage_count;
                                    
                                    if let Some(kind) = item.get("kind").and_then(|v| v.as_str()) {
                                        *kind_counts.entry(kind.to_string()).or_insert(0) += usage_count;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Total usage: {}", total_usage);
    println!("Found {} different kinds", kind_counts.len());
    
    // Sort by usage count (most common first)
    let mut sorted_kinds: Vec<_> = kind_counts.iter().collect();
    sorted_kinds.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\nKind Distribution (sorted by usage count):");
    println!("Kind                     | Count      | Frequency");
    println!("-------------------------|------------|----------");
    
    for (kind, count) in sorted_kinds {
        let frequency = (*count as f64 / total_usage as f64) * 100.0;
        println!("{:<24} | {:>10} | {:>7.3}%", kind, count, frequency);
    }
}
