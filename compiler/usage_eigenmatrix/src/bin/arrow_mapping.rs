use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("🏹 Second Largest Vector Arrow Mapping: rustc_proc_macro::Tag");
    
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    // Find domain and range for rustc_proc_macro::Tag
    let mut domain_symbols = Vec::new();
    let mut range_symbols = Vec::new();
    let mut arrow_mappings = HashMap::new();
    
    // Scan for what Tag uses (Tag as user, not used)
    for entry in std::fs::read_dir(mycelial_path).unwrap() {
        let path = entry.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.contains("rustc") && name.ends_with(".json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if let Some(usages) = data["usages"].as_array() {
                            for usage in usages {
                                let symbol = usage["symbol"].as_str().unwrap_or("unknown");
                                let user_crate = usage["user_crate"].as_str().unwrap_or("unknown");
                                let used_crate = usage["used_crate"].as_str();
                                
                                // Tag as domain (what Tag uses to define itself)
                                if user_crate == "rustc_proc_macro" && symbol == "Tag" {
                                    domain_symbols.push(format!("{}::{}", user_crate, symbol));
                                    if let Some(used) = used_crate {
                                        range_symbols.push(used.to_string());
                                        arrow_mappings.insert(
                                            format!("{}::{}", user_crate, symbol),
                                            used.to_string()
                                        );
                                    }
                                }
                                
                                // Tag as range (what uses Tag)
                                if symbol == "Tag" && used_crate == Some("rustc_proc_macro") {
                                    range_symbols.push(format!("rustc_proc_macro::{}", symbol));
                                    domain_symbols.push(user_crate.to_string());
                                    arrow_mappings.insert(
                                        user_crate.to_string(),
                                        format!("rustc_proc_macro::{}", symbol)
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Deduplicate and sort
    domain_symbols.sort();
    domain_symbols.dedup();
    range_symbols.sort();
    range_symbols.dedup();
    
    println!("\n📥 Domain (Input Types):");
    for (i, symbol) in domain_symbols.iter().enumerate() {
        println!("  D{}: {}", i+1, symbol);
    }
    
    println!("\n📤 Range (Output Types):");
    for (i, symbol) in range_symbols.iter().enumerate() {
        println!("  R{}: {}", i+1, symbol);
    }
    
    println!("\n🏹 Arrow Mappings (Domain → Range):");
    for (domain, range) in &arrow_mappings {
        println!("  {} → {}", domain, range);
    }
    
    println!("\n🧮 Arrow Properties:");
    println!("  |Domain|: {}", domain_symbols.len());
    println!("  |Range|: {}", range_symbols.len());
    println!("  |Mappings|: {}", arrow_mappings.len());
    
    // Check if it's a bijection
    let is_injective = arrow_mappings.len() == domain_symbols.len();
    let is_surjective = range_symbols.iter().all(|r| arrow_mappings.values().any(|v| v == r));
    let is_bijection = is_injective && is_surjective;
    
    println!("  Injective: {}", is_injective);
    println!("  Surjective: {}", is_surjective);
    println!("  Bijection: {}", is_bijection);
    
    if is_bijection {
        println!("\n✅ rustc_proc_macro::Tag forms a bijection!");
        println!("🔄 Inverse mapping exists: Range → Domain");
    } else {
        println!("\n⚠️  Not a bijection - need to analyze transformation structure");
    }
}
