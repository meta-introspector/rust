use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("🔢 Finding Prime 3: Accessor Pattern Analysis");
    
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    let mut accessor_count = 0u64;
    let mut converter_count = 0u64;
    let mut iterator_count = 0u64;
    let mut other_count = 0u64;
    let mut total_count = 0u64;
    
    // Analyze transformation arrows for patterns
    for entry in std::fs::read_dir(mycelial_path).unwrap() {
        let path = entry.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.contains("rustc") && name.ends_with(".json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if let Some(usages) = data["usages"].as_array() {
                            for usage in usages {
                                let symbol = usage["symbol"].as_str().unwrap_or("unknown");
                                let usage_count = usage["usage_count"].as_u64().unwrap_or(0);
                                
                                // Skip labeling primes
                                if symbol == "TRACE" || symbol == "DEBUG" || symbol == "WARN" || 
                                   symbol == "new" || symbol == "create" || symbol.contains("tracing") {
                                    continue;
                                }
                                
                                total_count += usage_count;
                                
                                // Categorize by pattern
                                if symbol == "into" || symbol == "from" || symbol == "as_ref" || symbol == "as_mut" {
                                    converter_count += usage_count;
                                } else if symbol.contains("iter") || symbol == "next" || symbol == "collect" {
                                    iterator_count += usage_count;
                                } else if symbol.starts_with("get_") || symbol.ends_with("_mut") || 
                                         symbol == "len" || symbol == "is_empty" || symbol == "fields" {
                                    accessor_count += usage_count;
                                } else {
                                    other_count += usage_count;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("\n🔍 Pattern Analysis Results:");
    println!("  Accessor patterns: {} ({:.1}%)", accessor_count, (accessor_count as f64 / total_count as f64) * 100.0);
    println!("  Converter patterns: {} ({:.1}%)", converter_count, (converter_count as f64 / total_count as f64) * 100.0);
    println!("  Iterator patterns: {} ({:.1}%)", iterator_count, (iterator_count as f64 / total_count as f64) * 100.0);
    println!("  Other patterns: {} ({:.1}%)", other_count, (other_count as f64 / total_count as f64) * 100.0);
    println!("  Total analyzed: {}", total_count);
    
    // Find Prime 3
    let max_pattern = if accessor_count > converter_count && accessor_count > iterator_count {
        ("accessor", accessor_count)
    } else if converter_count > iterator_count {
        ("converter", converter_count)
    } else {
        ("iterator", iterator_count)
    };
    
    let prime3_ratio = max_pattern.1 as f64 / total_count as f64;
    
    println!("\n🎯 Prime 3 Discovery:");
    println!("  Pattern: {}", max_pattern.0);
    println!("  Usage: {}", max_pattern.1);
    println!("  Ratio: {:.1}%", prime3_ratio * 100.0);
    
    if prime3_ratio > 0.15 {
        println!("\n✅ Prime 3 Found: {} (~{:.0}%)", max_pattern.0, prime3_ratio * 100.0);
        println!("🔢 Prime Sequence:");
        println!("  Prime 1: Terminal objects (Tag-like)");
        println!("  Prime 2: Constructors (new-like) ~50%");
        println!("  Prime 3: {} ~{:.0}%", max_pattern.0, prime3_ratio * 100.0);
    } else {
        println!("\n⚠️  No dominant Prime 3 pattern (largest: {:.1}%)", prime3_ratio * 100.0);
    }
}
