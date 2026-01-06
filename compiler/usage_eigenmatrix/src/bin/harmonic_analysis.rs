use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("🎵 Harmonic Analysis: Generalizing Labeling System Arrows");
    
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    // Pattern 1: Terminal objects (many → one)
    let mut terminal_patterns = HashMap::new();
    
    // Pattern 2: Constructor patterns (one → many)
    let mut constructor_patterns = HashMap::new();
    
    // Scan for harmonic patterns
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
                                let usage_count = usage["usage_count"].as_u64().unwrap_or(0);
                                
                                // Look for Tag-like patterns (terminal objects)
                                if symbol.len() <= 5 && symbol.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
                                    let key = format!("{}::{}", user_crate, symbol);
                                    *terminal_patterns.entry(key).or_insert(0) += usage_count;
                                }
                                
                                // Look for new-like patterns (constructors)
                                if symbol == "new" || symbol == "create" || symbol == "make" || symbol == "build" {
                                    if let Some(used) = used_crate {
                                        let key = format!("{} → {}", symbol, used);
                                        *constructor_patterns.entry(key).or_insert(0) += usage_count;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Find harmonic resonances
    let mut terminal_ranked: Vec<_> = terminal_patterns.iter().collect();
    terminal_ranked.sort_by(|a, b| b.1.cmp(a.1));
    
    let mut constructor_ranked: Vec<_> = constructor_patterns.iter().collect();
    constructor_ranked.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 Terminal Object Harmonics (Tag-like patterns):");
    for (i, (pattern, count)) in terminal_ranked.iter().take(10).enumerate() {
        println!("  T{}: {} (usage: {})", i+1, pattern, count);
    }
    
    println!("\n🏗️ Constructor Harmonics (new-like patterns):");
    for (i, (pattern, count)) in constructor_ranked.iter().take(10).enumerate() {
        println!("  C{}: {} (usage: {})", i+1, pattern, count);
    }
    
    // Analyze harmonic frequencies
    let terminal_total: u64 = terminal_patterns.values().sum();
    let constructor_total: u64 = constructor_patterns.values().sum();
    
    println!("\n🎵 Harmonic Analysis:");
    println!("  Terminal frequency: {} total usages", terminal_total);
    println!("  Constructor frequency: {} total usages", constructor_total);
    println!("  Harmonic ratio: {:.2}", constructor_total as f64 / terminal_total as f64);
    
    // Find resonant pairs (constructor → terminal)
    println!("\n🔄 Resonant Pairs (Constructor → Terminal):");
    let mut resonances = 0;
    for (constructor, c_count) in constructor_ranked.iter().take(5) {
        for (terminal, t_count) in terminal_ranked.iter().take(5) {
            let resonance_strength = (**c_count * **t_count) as f64 / (terminal_total + constructor_total) as f64;
            if resonance_strength > 0.01 {
                println!("  {} ↔ {} (resonance: {:.3})", constructor, terminal, resonance_strength);
                resonances += 1;
            }
        }
    }
    
    println!("\n🎼 Generalized Arrow Pattern:");
    println!("  Pattern 1: Terminal(many → one) - Classification/Labeling");
    println!("  Pattern 2: Constructor(one → many) - Creation/Allocation");
    println!("  Harmonic: Constructor → Terminal (Creation → Classification)");
    println!("  Resonances found: {}", resonances);
    
    if resonances > 0 {
        println!("\n✅ Harmonic structure detected in labeling systems!");
        println!("🎵 These form the categorical infrastructure for true transformations");
    }
}
