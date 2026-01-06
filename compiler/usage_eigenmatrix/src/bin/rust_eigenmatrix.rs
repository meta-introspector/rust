use std::collections::HashMap;
use serde_json::Value;

fn main() {
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    println!("🧮 Rust Compiler Eigenmatrix Analysis");
    
    // Load all enum usage data
    let mut usage_vectors = HashMap::new();
    let mut complexity_scores: HashMap<String, u64> = HashMap::new();
    
    // Scan all rustc enum files
    for entry in std::fs::read_dir(mycelial_path).unwrap() {
        let path = entry.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.contains("rustc") && name.contains("enum") && name.ends_with(".json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if let Some(usages) = data["usages"].as_array() {
                            for usage in usages {
                                let symbol = usage["symbol"].as_str().unwrap_or("unknown");
                                let count = usage["usage_count"].as_u64().unwrap_or(0);
                                let crate_name = usage["user_crate"].as_str().unwrap_or("unknown");
                                
                                let key = format!("{}::{}", crate_name, symbol);
                                *usage_vectors.entry(key).or_insert(0) += count;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Find largest usage vectors
    let mut sorted_usage: Vec<_> = usage_vectors.iter().collect();
    sorted_usage.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n📊 Top 10 Largest Usage Vectors:");
    for (i, (symbol, count)) in sorted_usage.iter().take(10).enumerate() {
        println!("  {}: {} (usage: {})", i+1, symbol, count);
    }
    
    // Calculate eigenmatrix components
    let total_usage: u64 = usage_vectors.values().sum();
    let num_symbols = usage_vectors.len();
    
    println!("\n🔢 Eigenmatrix Properties:");
    println!("  Total symbols: {}", num_symbols);
    println!("  Total usage: {}", total_usage);
    println!("  Mean usage: {:.2}", total_usage as f64 / num_symbols as f64);
    
    // Primary components (top usage patterns)
    let top_10_usage: u64 = sorted_usage.iter().take(10).map(|(_, count)| **count).sum();
    let concentration = (top_10_usage as f64 / total_usage as f64) * 100.0;
    
    println!("  Top 10 concentration: {:.1}%", concentration);
    
    // Find the single largest usage vector
    if let Some((largest_symbol, largest_count)) = sorted_usage.first() {
        println!("\n🎯 Largest Usage Vector:");
        println!("  Symbol: {}", largest_symbol);
        println!("  Usage: {}", largest_count);
        println!("  Dominance: {:.2}%", (**largest_count as f64 / total_usage as f64) * 100.0);
    }
    
    // Eigenmatrix analysis
    println!("\n🧬 Rust Compiler Eigenmatrix:");
    println!("  Rank: {}", num_symbols.min(10)); // Effective rank
    println!("  Sparsity: {:.1}%", (1.0 - (usage_vectors.values().filter(|&&v| v > 0).count() as f64 / num_symbols as f64)) * 100.0);
    
    // Primary components by crate
    let mut crate_usage = HashMap::new();
    for (symbol, count) in &usage_vectors {
        if let Some(crate_name) = symbol.split("::").next() {
            *crate_usage.entry(crate_name.to_string()).or_insert(0) += *count;
        }
    }
    
    let mut sorted_crates: Vec<_> = crate_usage.iter().collect();
    sorted_crates.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🏗️ Primary Components (by crate):");
    for (i, (crate_name, usage)) in sorted_crates.iter().take(5).enumerate() {
        let percentage = (**usage as f64 / total_usage as f64) * 100.0;
        println!("  PC{}: {} ({:.1}%)", i+1, crate_name, percentage);
    }
}
