use std::fs;
use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("🔄 Enum → String Orbit Report");
    
    let data_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    let mut enum_variants = HashMap::new();
    let mut total_enum_usage = 0u64;
    let mut file_count = 0;
    
    if let Ok(entries) = fs::read_dir(data_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    file_count += 1;
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            let usages = if let Some(array) = json.as_array() {
                                array
                            } else if let Some(usages_array) = json.get("usages").and_then(|v| v.as_array()) {
                                usages_array
                            } else {
                                continue;
                            };
                            
                            for item in usages {
                                if let Some(kind) = item.get("kind").and_then(|v| v.as_str()) {
                                    if kind == "enum_variant_usage" {
                                        if let Some(symbol) = item.get("symbol").and_then(|v| v.as_str()) {
                                            if let Some(usage_count) = item.get("usage_count").and_then(|v| v.as_u64()) {
                                                *enum_variants.entry(symbol.to_string()).or_insert(0) += usage_count;
                                                total_enum_usage += usage_count;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Total enum variant usage: {}", total_enum_usage);
    println!("Unique enum variants: {}", enum_variants.len());
    println!("Total files processed: {}", file_count);
    
    let mut sorted_variants: Vec<_> = enum_variants.iter().collect();
    sorted_variants.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\nAll Enum Variants:");
    println!("Variant                  | Count | Frequency");
    println!("-------------------------|-------|----------");
    
    for (variant, count) in &sorted_variants {
        let frequency = (**count as f64 / total_enum_usage as f64) * 100.0;
        println!("{:<24} | {:>5} | {:>7.3}%", variant, count, frequency);
    }
}
