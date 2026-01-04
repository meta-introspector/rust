use std::collections::HashMap;
use serde_json::Value;
use std::fs;

fn main() {
    let symbol_map: HashMap<String, Value> = 
        serde_json::from_str(&fs::read_to_string("symbol_map.json").unwrap()).unwrap();
    
    println!("🎯 Generating test cases for unresolved dependencies...\n");
    
    if let Some(driver_main) = symbol_map.get("rustc_driver_impl::lib::main") {
        if let Some(deps) = driver_main["dependencies"].as_array() {
            let mut test_cases = Vec::new();
            
            for dep in deps {
                let dep_str = dep.as_str().unwrap();
                
                // Check if dependency exists in symbol map
                if !symbol_map.contains_key(dep_str) {
                    // Try to find similar matches
                    let matches: Vec<_> = symbol_map.keys()
                        .filter(|k| k.contains(&extract_function_name(dep_str)))
                        .take(3)
                        .collect();
                    
                    if matches.is_empty() {
                        test_cases.push((dep_str, Vec::new()));
                    } else {
                        test_cases.push((dep_str, matches.into_iter().map(|s| s.clone()).collect()));
                    }
                }
            }
            
            // Generate test files for each unresolved dependency
            for (i, (dep, matches)) in test_cases.iter().enumerate() {
                let test_content = generate_test_case(dep, matches);
                let filename = format!("test_unresolved_{}.rs", i + 1);
                fs::write(&filename, test_content).unwrap();
                println!("📝 Generated {}: {}", filename, dep);
                if !matches.is_empty() {
                    println!("   Similar matches found: {}", matches.len());
                    for m in matches.iter().take(2) {
                        println!("     - {}", m);
                    }
                }
            }
            
            println!("\n✅ Generated {} test case files", test_cases.len());
        }
    }
}

fn extract_function_name(dep: &str) -> String {
    dep.split("::").last().unwrap_or(dep).to_string()
}

fn generate_test_case(dep: &str, matches: &[String]) -> String {
    let parts: Vec<&str> = dep.split("::").collect();
    let function_name = parts.last().unwrap_or(&"unknown");
    
    let mut content = format!("// Test case for unresolved dependency: {}\n", dep);
    
    if !matches.is_empty() {
        content.push_str("// Similar matches found in symbol database:\n");
        for m in matches.iter().take(3) {
            content.push_str(&format!("// - {}\n", m));
        }
    }
    
    content.push_str("\n");
    
    // Generate minimal test function
    if parts.len() >= 2 {
        let module_path = parts[..parts.len()-1].join("::");
        content.push_str(&format!("// Expected: use {};\n", module_path));
    }
    
    content.push_str(&format!("fn test_{}() {{\n", function_name.replace("::", "_")));
    content.push_str(&format!("    // Call: {};\n", dep));
    content.push_str("    println!(\"Testing dependency resolution\");\n");
    content.push_str("}\n");
    
    content
}
