use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::fs;

fn trace_dependencies(symbol_map: &HashMap<String, Value>, symbol_key: &str, depth: usize) -> Vec<String> {
    let indent = "  ".repeat(depth);
    let mut all_deps = Vec::new();
    
    if let Some(symbol) = symbol_map.get(symbol_key) {
        println!("{}📍 {}: {}", indent, symbol_key, symbol["symbol_type"].as_str().unwrap());
        println!("{}   📁 {}", indent, symbol["source_file"].as_str().unwrap());
        
        if let Some(deps) = symbol["dependencies"].as_array() {
            if !deps.is_empty() {
                println!("{}🔗 Dependencies ({}):", indent, deps.len());
                
                for dep in deps {
                    let dep_str = dep.as_str().unwrap();
                    all_deps.push(dep_str.to_string());
                    
                    // Try to resolve and recurse
                    let resolved_key = resolve_dependency(symbol_map, dep_str);
                    if let Some(key) = resolved_key {
                        println!("{}  ✅ {} → {}", indent, dep_str, key);
                        if depth < 2 { // Limit recursion depth
                            let sub_deps = trace_dependencies(symbol_map, &key, depth + 1);
                            all_deps.extend(sub_deps);
                        }
                    } else {
                        println!("{}  ❌ {}: not found", indent, dep_str);
                    }
                }
            }
        }
    }
    
    all_deps
}

fn resolve_dependency(symbol_map: &HashMap<String, Value>, dep: &str) -> Option<String> {
    // Try exact match first
    if symbol_map.contains_key(dep) {
        return Some(dep.to_string());
    }
    
    // Try qualified matches
    let matches: Vec<_> = symbol_map.keys()
        .filter(|k| k.ends_with(&format!("::{}", dep)))
        .collect();
    
    if matches.len() == 1 {
        Some(matches[0].clone())
    } else {
        None
    }
}

fn main() {
    let symbol_map: HashMap<String, Value> = 
        serde_json::from_str(&fs::read_to_string("symbol_map.json").unwrap()).unwrap();
    
    println!("🎯 Full rustc::main::main Resolution Tree\n");
    
    let all_deps = trace_dependencies(&symbol_map, "rustc::main::main", 0);
    
    println!("\n📊 Summary:");
    println!("  📈 Total dependencies traced: {}", all_deps.len());
    
    // Show key resolved dependencies
    println!("\n🔑 Key Resolutions:");
    if let Some(driver_main) = resolve_dependency(&symbol_map, "main") {
        if driver_main.contains("rustc_driver") {
            println!("  ✅ rustc_driver::main → {}", driver_main);
        }
    }
}
