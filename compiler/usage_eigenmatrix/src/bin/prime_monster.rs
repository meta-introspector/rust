use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("🔢 Prime Assignment: First 25 Primes (2-97) to Monster Arrows");
    
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    let mut all_arrows = HashMap::new();
    
    // Collect all transformation arrows (post-sieve)
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
                                
                                // Skip labeling primes (already sieved)
                                if symbol == "TRACE" || symbol == "DEBUG" || symbol == "WARN" || 
                                   symbol == "new" || symbol == "create" || symbol.contains("tracing") ||
                                   symbol == "META" || symbol == "EVENT" || symbol == "SPAN" || symbol == "_" {
                                    continue;
                                }
                                
                                let arrow = format!("{}::{}", user_crate, symbol);
                                *all_arrows.entry(arrow).or_insert(0) += usage_count;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Rank arrows by usage (descending)
    let mut ranked_arrows: Vec<_> = all_arrows.iter().collect();
    ranked_arrows.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 Prime Assignment to Monster Arrows:");
    println!("  Assigning primes 2-97 to top 25 transformation arrows");
    
    for (i, (arrow, count)) in ranked_arrows.iter().take(25).enumerate() {
        let prime = primes[i];
        println!("  Prime {}: {} (usage: {})", prime, arrow, count);
    }
    
    // Calculate prime-weighted complexity
    let mut total_prime_weight = 0u64;
    let mut total_usage = 0u64;
    
    for (i, (_, count)) in ranked_arrows.iter().take(25).enumerate() {
        let prime = primes[i] as u64;
        total_prime_weight += prime * *count;
        total_usage += *count;
    }
    
    println!("\n📊 Prime Monster Statistics:");
    println!("  Total arrows analyzed: {}", all_arrows.len());
    println!("  Top 25 prime arrows: {}", ranked_arrows.len().min(25));
    println!("  Prime range: 2-97");
    println!("  Total usage (top 25): {}", total_usage);
    println!("  Prime-weighted complexity: {}", total_prime_weight);
    println!("  Average prime weight: {:.1}", total_prime_weight as f64 / total_usage as f64);
    
    // Find the "monster prime" (highest weighted)
    if let Some((monster_arrow, monster_usage)) = ranked_arrows.first() {
        let monster_prime = primes[0]; // Prime 2 for highest usage
        let monster_weight = monster_prime as u64 * *monster_usage;
        
        println!("\n👹 Monster Prime Discovery:");
        println!("  Monster Arrow: {}", monster_arrow);
        println!("  Monster Prime: {}", monster_prime);
        println!("  Monster Usage: {}", monster_usage);
        println!("  Monster Weight: {} (prime × usage)", monster_weight);
        println!("  Monster Dominance: {:.1}%", (**monster_usage as f64 / total_usage as f64) * 100.0);
    }
    
    // Check for prime gaps in the monster
    println!("\n🔍 Prime Gap Analysis:");
    let mut significant_gaps = 0;
    for i in 1..25.min(ranked_arrows.len()) {
        let current_usage = *ranked_arrows[i].1;
        let prev_usage = *ranked_arrows[i-1].1;
        let gap_ratio = prev_usage as f64 / current_usage as f64;
        
        if gap_ratio > 2.0 {
            println!("  Gap at Prime {}: {}x drop ({} → {})", 
                    primes[i], gap_ratio, prev_usage, current_usage);
            significant_gaps += 1;
        }
    }
    
    println!("\n✅ Prime Monster Construction Complete!");
    println!("🔢 First 25 primes assigned to transformation arrows");
    println!("👹 Monster ready for bijection proof generation");
    println!("📈 Significant gaps found: {}", significant_gaps);
}
