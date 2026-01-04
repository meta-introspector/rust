use std::collections::HashMap;
use serde_json::Value;
use std::fs;

fn main() {
    let symbol_map: HashMap<String, Value> = 
        serde_json::from_str(&fs::read_to_string("symbol_map.json").unwrap()).unwrap();
    
    println!("🎯 Complete rustc::main::main Resolution Chain\n");
    
    // Step 1: rustc::main::main
    if let Some(rustc_main) = symbol_map.get("rustc::main::main") {
        println!("1️⃣  rustc::main::main");
        println!("    📍 Type: {}", rustc_main["symbol_type"].as_str().unwrap());
        println!("    📁 File: {}", rustc_main["source_file"].as_str().unwrap());
        println!("    🔧 Function: Sets up jemalloc and calls rustc_driver::main()");
        
        // Find the rustc_driver::main call
        if let Some(deps) = rustc_main["dependencies"].as_array() {
            for dep in deps {
                if dep.as_str().unwrap() == "rustc_driver::main" {
                    println!("    ⬇️  Calls: rustc_driver::main");
                    break;
                }
            }
        }
    }
    
    println!();
    
    // Step 2: rustc_driver_impl::lib::main (the real implementation)
    if let Some(driver_main) = symbol_map.get("rustc_driver_impl::lib::main") {
        println!("2️⃣  rustc_driver_impl::lib::main");
        println!("    📍 Type: {}", driver_main["symbol_type"].as_str().unwrap());
        println!("    📁 File: {}", driver_main["source_file"].as_str().unwrap());
        println!("    🔧 Function: Real compiler main with logger setup, hooks, and exit handling");
        
        if let Some(deps) = driver_main["dependencies"].as_array() {
            println!("    🔗 Key Dependencies ({}):", deps.len());
            let key_deps: Vec<_> = deps.iter()
                .map(|d| d.as_str().unwrap())
                .filter(|d| !d.contains("attr_") && !d.contains("jemalloc") && 
                           !d.contains("usize") && !d.contains("c_void"))
                .take(10)
                .collect();
            
            for dep in key_deps {
                if let Some(resolved) = resolve_dependency(&symbol_map, dep) {
                    let dep_symbol = &symbol_map[&resolved];
                    println!("      ✅ {} → {}: {}", 
                        dep, resolved, dep_symbol["symbol_type"].as_str().unwrap());
                } else {
                    println!("      ❌ {}: not found", dep);
                }
            }
        }
    }
    
    println!("\n📋 Resolution Summary:");
    println!("  🎯 Entry Point: rustc::main::main (jemalloc setup wrapper)");
    println!("  🚀 Real Compiler: rustc_driver_impl::lib::main (actual implementation)");
    println!("  🔗 Connection: rustc_driver::main() → rustc_driver_impl::lib::main");
    println!("  📊 Total Chain: 2 main functions + dependencies");
}

fn resolve_dependency(symbol_map: &HashMap<String, Value>, dep: &str) -> Option<String> {
    if symbol_map.contains_key(dep) {
        return Some(dep.to_string());
    }
    
    let matches: Vec<_> = symbol_map.keys()
        .filter(|k| k.ends_with(&format!("::{}", dep)))
        .collect();
    
    if matches.len() == 1 {
        Some(matches[0].clone())
    } else {
        None
    }
}
