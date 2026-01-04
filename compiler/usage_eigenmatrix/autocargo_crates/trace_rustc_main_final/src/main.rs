use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::fs;

fn main() {
    let symbol_map: HashMap<String, Value> = 
        serde_json::from_str(&fs::read_to_string("symbol_map.json").unwrap()).unwrap();
    
    println!("🎯 Tracing rustc::main::main dependencies...\n");
    
    let mut resolved = HashSet::new();
    let mut unresolved = Vec::new();
    
    if let Some(main_symbol) = symbol_map.get("rustc::main::main") {
        println!("📍 rustc::main::main: {}", main_symbol["symbol_type"].as_str().unwrap());
        println!("   📁 {}", main_symbol["source_file"].as_str().unwrap());
        
        if let Some(deps) = main_symbol["dependencies"].as_array() {
            println!("\n🔗 Dependencies ({}):", deps.len());
            
            for dep in deps {
                let dep_str = dep.as_str().unwrap();
                
                // Try exact match first
                if let Some(dep_symbol) = symbol_map.get(dep_str) {
                    println!("  ✅ {}: {} ({})", 
                        dep_str,
                        dep_symbol["symbol_type"].as_str().unwrap(),
                        dep_symbol["crate_name"].as_str().unwrap()
                    );
                    resolved.insert(dep_str);
                } else {
                    // Try qualified matches
                    let matches: Vec<_> = symbol_map.keys()
                        .filter(|k| k.contains(&format!("::{}", dep_str)))
                        .collect();
                    
                    if matches.len() == 1 {
                        let key = matches[0];
                        let dep_symbol = &symbol_map[key];
                        println!("  ✅ {} → {}: {} ({})", 
                            dep_str, key,
                            dep_symbol["symbol_type"].as_str().unwrap(),
                            dep_symbol["crate_name"].as_str().unwrap()
                        );
                        resolved.insert(dep_str);
                    } else if matches.len() > 1 {
                        println!("  ⚠️  {} → {} matches:", dep_str, matches.len());
                        for m in matches.iter().take(3) {
                            let dep_symbol = &symbol_map[*m];
                            println!("      - {}: {} ({})", 
                                m,
                                dep_symbol["symbol_type"].as_str().unwrap(),
                                dep_symbol["crate_name"].as_str().unwrap()
                            );
                        }
                        resolved.insert(dep_str);
                    } else {
                        println!("  ❌ {}: not found", dep_str);
                        unresolved.push(dep_str);
                    }
                }
            }
        }
    }
    
    println!("\n📊 Resolution Summary:");
    println!("  ✅ Resolved: {}", resolved.len());
    println!("  ❌ Unresolved: {}", unresolved.len());
    println!("  📈 Success Rate: {:.1}%", 
        (resolved.len() as f64 / (resolved.len() + unresolved.len()) as f64) * 100.0);
}
