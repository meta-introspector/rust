use std::collections::{HashMap, HashSet};
use serde_json::Value;

fn main() {
    println!("🔢 Finding Prime 3: Next Fundamental Pattern");
    
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    // Already sieved out: Prime 1 (Tag-like) and Prime 2 (new-like)
    let sieved_primes = vec!["TRACE", "DEBUG", "WARN", "META", "EVENT", "SPAN", "_", "new", "create", "make", "build"];
    
    let mut transformation_arrows = HashMap::new();
    let mut sieved_out = HashSet::new();
    
    // Collect transformation arrows (post Prime 1&2 sieve)
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
                                let usage_count = usage["usage_count"].as_u64().unwrap_or(0);
                                
                                let arrow = format!("{}::{}", user_crate, symbol);
                                
                                // Skip already sieved primes
                                let parts: Vec<&str> = arrow.split("::").collect();
                                if parts.len() == 2 {
                                    let sym = parts[1];
                                    if sieved_primes.iter().any(|&prime| sym == prime || sym.contains(prime)) {
                                        continue;
                                    }
                                    if arrow.contains("tracing") || arrow.contains("log") || arrow.contains("span") {
                                        continue;
                                    }
                                }
                                
                                *transformation_arrows.entry(arrow).or_insert(0) += usage_count;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Rank transformation arrows
    let mut ranked: Vec<_> = transformation_arrows.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🏹 Top 15 Transformation Arrows (Post Prime 1&2):");
    for (i, (arrow, count)) in ranked.iter().take(15).enumerate() {
        println!("  {}: {} (usage: {})", i+1, arrow, count);
    }
    
    // Analyze for Prime 3 pattern
    let mut pattern_analysis = HashMap::new();
    
    for (arrow, count) in ranked.iter().take(50) {
        let parts: Vec<&str> = arrow.split("::").collect();
        if parts.len() == 2 {
            let symbol = parts[1];
            
            // Look for common patterns in top arrows
            if symbol.len() <= 10 {
                let pattern_type = if symbol.chars().all(|c| c.is_ascii_lowercase()) {
                    "accessor"
                } else if symbol == "into" || symbol == "from" || symbol == "as_ref" {
                    "converter"
                } else if symbol.ends_with("_mut") || symbol.starts_with("get_") {
                    "accessor"
                } else if symbol.contains("iter") || symbol.contains("next") {
                    "iterator"
                } else {
                    "other"
                };
                
                *pattern_analysis.entry(pattern_type).or_insert(0) += *count;
            }
        }
    }
    
    let mut pattern_ranked: Vec<_> = pattern_analysis.iter().collect();
    pattern_ranked.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🔍 Pattern Analysis for Prime 3:");
    for (pattern, count) in &pattern_ranked {
        println!("  {}: {} total usage", pattern, count);
    }
    
    // Find Prime 3 candidate
    if let Some((prime3_pattern, prime3_count)) = pattern_ranked.first() {
        let total_remaining: u64 = transformation_arrows.values().sum();
        let prime3_ratio = **prime3_count as f64 / total_remaining as f64;
        
        println!("\n🎯 Prime 3 Candidate:");
        println!("  Pattern: {}", prime3_pattern);
        println!("  Usage: {}", prime3_count);
        println!("  Ratio: {:.1}%", prime3_ratio * 100.0);
        
        if prime3_ratio > 0.2 {
            println!("\n✅ Prime 3 Found: {} (~{:.0}%)", prime3_pattern, prime3_ratio * 100.0);
            println!("🔢 Prime Sequence: Tag(terminal) → new(constructor) → {}({})", prime3_pattern, prime3_ratio);
        } else {
            println!("\n⚠️  No dominant Prime 3 pattern found");
            println!("🔍 May need deeper analysis or different categorization");
        }
    }
    
    println!("\n📊 Prime Discovery Progress:");
    println!("  Prime 1: Terminal objects (~26% of original)");
    println!("  Prime 2: Constructors (~50% total infrastructure)");
    println!("  Prime 3: {} (~{:.0}% of transformations)", 
             pattern_ranked.first().map(|(p, _)| **p).unwrap_or("unknown"),
             pattern_ranked.first().map(|(_, c)| **c as f64 / transformation_arrows.values().sum::<u64>() as f64 * 100.0).unwrap_or(0.0));
}
