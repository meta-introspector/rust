use std::collections::HashMap;
use serde_json::Value;
use std::fs;

fn main() {
    let symbol_map: HashMap<String, Value> = 
        serde_json::from_str(&fs::read_to_string("symbol_map.json").unwrap()).unwrap();
    
    println!("🎯 Resolving function dependencies (ignoring variables)...\n");
    
    if let Some(driver_main) = symbol_map.get("rustc_driver_impl::lib::main") {
        if let Some(deps) = driver_main["dependencies"].as_array() {
            let mut resolved = 0;
            let mut total = 0;
            let mut ignored_vars = 0;
            
            for dep in deps {
                let dep_str = dep.as_str().unwrap();
                
                // Skip variables (internal state)
                if is_variable(dep_str) {
                    println!("🔸 {} (variable - ignored)", dep_str);
                    ignored_vars += 1;
                    continue;
                }
                
                total += 1;
                
                if let Some(resolved_symbol) = resolve_dependency(&symbol_map, dep_str) {
                    println!("✅ {} → {}", dep_str, resolved_symbol);
                    resolved += 1;
                } else {
                    println!("❌ {}: not found", dep_str);
                }
            }
            
            println!("\n📊 Final Resolution Summary:");
            println!("  ✅ Functions Resolved: {}/{} ({:.1}%)", resolved, total, (resolved as f64 / total as f64) * 100.0);
            println!("  ❌ Functions Unresolved: {}", total - resolved);
            println!("  🔸 Variables Ignored: {}", ignored_vars);
            println!("  📈 Effective Resolution Rate: {:.1}%", (resolved as f64 / total as f64) * 100.0);
        }
    }
}

fn is_variable(dep: &str) -> bool {
    // Variables are typically lowercase and don't contain :: or end with function-like patterns
    let name = dep.split("::").last().unwrap_or(dep);
    
    // Common variable patterns
    matches!(name, 
        "early_dcx" | "start_time" | "start_rss" | "end_rss" | 
        "callbacks" | "exit_code" | "format"
    ) || (
        name.chars().all(|c| c.is_lowercase() || c == '_') && 
        !name.ends_with("_handler") && 
        !name.ends_with("_hook") &&
        !dep.contains("::")
    )
}

fn resolve_dependency(symbol_map: &HashMap<String, Value>, dep: &str) -> Option<String> {
    // 1. Direct match
    if symbol_map.contains_key(dep) {
        return Some(dep.to_string());
    }
    
    // 2. Extract function name and search in use statements
    let function_name = dep.split("::").last().unwrap_or(dep);
    
    // Look for use statements that import this function
    for (key, _) in symbol_map {
        if key.starts_with("use_") && key.contains(&format!("_{}_", function_name)) {
            return Some(key.clone());
        }
    }
    
    // 3. Pattern match for similar functions
    let matches: Vec<_> = symbol_map.keys()
        .filter(|k| k.ends_with(&format!("::{}", function_name)))
        .collect();
    
    if matches.len() == 1 {
        return Some(matches[0].clone());
    }
    
    // 4. Fuzzy match for partial names
    let matches: Vec<_> = symbol_map.keys()
        .filter(|k| k.contains(function_name))
        .take(1)
        .collect();
    
    if matches.len() == 1 {
        return Some(matches[0].clone());
    }
    
    None
}
