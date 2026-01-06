use std::collections::{HashMap, HashSet};
use serde_json::Value;

fn main() {
    println!("🔢 Prime Arrow Sieve: Filtering Out Labeling Systems");
    
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    // Prime patterns (generators for sieving)
    let terminal_primes = vec!["TRACE", "DEBUG", "WARN", "META", "EVENT", "SPAN", "_"];
    let constructor_primes = vec!["new", "create", "make", "build"];
    
    let mut all_arrows = HashMap::new();
    let mut sieved_out = HashSet::new();
    
    // Collect all arrows
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
                                *all_arrows.entry(arrow).or_insert(0) += usage_count;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sieve using prime patterns
    for (arrow, _count) in &all_arrows {
        let parts: Vec<&str> = arrow.split("::").collect();
        if parts.len() == 2 {
            let symbol = parts[1];
            
            // Sieve out terminal primes
            if terminal_primes.iter().any(|&prime| symbol == prime || symbol.contains(prime)) {
                sieved_out.insert(arrow.clone());
            }
            
            // Sieve out constructor primes  
            if constructor_primes.iter().any(|&prime| symbol == prime) {
                sieved_out.insert(arrow.clone());
            }
            
            // Sieve out tracing-related (from harmonic analysis)
            if arrow.contains("tracing") || arrow.contains("log") || arrow.contains("span") {
                sieved_out.insert(arrow.clone());
            }
        }
    }
    
    // Filter to transformation arrows only
    let mut transformation_arrows: Vec<_> = all_arrows.iter()
        .filter(|(arrow, _)| !sieved_out.contains(*arrow))
        .collect();
    
    transformation_arrows.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n📊 Sieve Results:");
    println!("  Total arrows: {}", all_arrows.len());
    println!("  Sieved out (labeling): {}", sieved_out.len());
    println!("  Remaining (transformations): {}", transformation_arrows.len());
    println!("  Sieve efficiency: {:.1}%", (sieved_out.len() as f64 / all_arrows.len() as f64) * 100.0);
    
    println!("\n🏹 Top 10 Transformation Arrows (Post-Sieve):");
    for (i, (arrow, count)) in transformation_arrows.iter().take(10).enumerate() {
        println!("  T{}: {} (usage: {})", i+1, arrow, count);
    }
    
    println!("\n🔢 Prime Sieve Patterns Used:");
    println!("  Terminal primes: {:?}", terminal_primes);
    println!("  Constructor primes: {:?}", constructor_primes);
    println!("  Harmonic filters: tracing, log, span");
    
    // Analyze remaining arrows for bijection potential
    let mut potential_bijections = 0;
    for (arrow, count) in transformation_arrows.iter().take(20) {
        if arrow.contains("rustc_") && *count > &5 {
            potential_bijections += 1;
        }
    }
    
    println!("\n🎯 Bijection Analysis:");
    println!("  Potential bijection arrows: {}", potential_bijections);
    println!("  Ready for domain/range analysis: ✅");
    
    if transformation_arrows.len() > 0 {
        println!("\n✅ Sieve successful! Pure transformation arrows isolated.");
        println!("🔄 Ready to analyze true compiler stage bijections.");
    }
}
