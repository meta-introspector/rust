use std::collections::HashMap;
use serde_json::Value;
use std::fs;
use std::io::{Write, Read};
use flate2::read::GzDecoder;

fn main() {
    let compressed_data = fs::read("symbol_map.json.gz").unwrap();
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut symbol_map_content = String::new();
    decoder.read_to_string(&mut symbol_map_content).unwrap();
    let symbol_map: HashMap<String, Value> = 
        serde_json::from_str(&symbol_map_content).unwrap();
    
    println!("🎯 Starting complete rustc analysis...");
    let mut output = std::fs::File::create("rustc_complete_analysis.txt").unwrap();
    println!("📁 Created output file: rustc_complete_analysis.txt");
    
    writeln!(output, "🎯 Complete rustc::main::main Resolution with Full Recursive Analysis\n").unwrap();
    output.flush().unwrap();
    
    let mut function_calls = HashMap::new();
    let mut total_functions = 0;
    let mut visited = std::collections::HashSet::new();
    
    println!("🔍 Tracing rustc::main::main...");
    trace_function_recursive(&symbol_map, "rustc::main::main", 0, &mut function_calls, &mut total_functions, &mut visited, &mut output);
    
    println!("🔍 Tracing rustc_driver_impl::lib::main...");
    writeln!(output, "\n{}", "=".repeat(80)).unwrap();
    trace_function_recursive(&symbol_map, "rustc_driver_impl::lib::main", 0, &mut function_calls, &mut total_functions, &mut visited, &mut output);
    
    println!("📊 Writing final summary...");
    writeln!(output, "\n📊 Complete Function Call Analysis:").unwrap();
    writeln!(output, "  📈 Total Functions Called: {}", total_functions).unwrap();
    writeln!(output, "  🔄 Unique Functions: {}", function_calls.len()).unwrap();
    
    writeln!(output, "\n🔥 Most Called Functions:").unwrap();
    let mut sorted_calls: Vec<_> = function_calls.iter().collect();
    sorted_calls.sort_by(|a, b| b.1.cmp(a.1));
    
    for (func, count) in sorted_calls.iter().take(20) {
        writeln!(output, "  {}x {}", count, func).unwrap();
    }
    
    if sorted_calls.len() > 20 {
        writeln!(output, "  ... and {} more functions", sorted_calls.len() - 20).unwrap();
    }
    
    output.flush().unwrap();
    println!("✅ Analysis complete! Saved to rustc_complete_analysis.txt");
    println!("📊 Total Functions: {}, Unique: {}", total_functions, function_calls.len());
}

fn trace_function_recursive(
    symbol_map: &HashMap<String, Value>, 
    function_name: &str, 
    depth: usize, 
    function_calls: &mut HashMap<String, usize>,
    total_functions: &mut usize,
    visited: &mut std::collections::HashSet<String>,
    output: &mut std::fs::File
) {
    let indent = "  ".repeat(depth);
    
    // Count the call
    *function_calls.entry(function_name.to_string()).or_insert(0) += 1;
    *total_functions += 1;
    
    // If already visited, just count and return - don't recurse again
    if visited.contains(function_name) {
        writeln!(output, "{}🔄 {} (already analyzed - counted)", indent, function_name).unwrap();
        return;
    }
    visited.insert(function_name.to_string());
    
    if let Some(symbol) = symbol_map.get(function_name) {
        writeln!(output, "{}📍 {}: {}", indent, function_name, symbol["symbol_type"].as_str().unwrap_or("unknown")).unwrap();
        
        if let Some(deps) = symbol["dependencies"].as_array() {
            let mut resolved_functions = Vec::new();
            
            for dep in deps {
                let dep_str = dep.as_str().unwrap();
                
                if is_variable(dep_str) {
                    continue;
                }
                
                if let Some(resolved) = resolve_dependency(symbol_map, dep_str) {
                    resolved_functions.push(resolved);
                }
            }
            
            if !resolved_functions.is_empty() {
                writeln!(output, "{}🔗 {} function calls", indent, resolved_functions.len()).unwrap();
                
                for resolved in &resolved_functions {
                    trace_function_recursive(symbol_map, &resolved, depth + 1, function_calls, total_functions, visited, output);
                }
            }
        }
    }
}

fn is_variable(dep: &str) -> bool {
    let name = dep.split("::").last().unwrap_or(dep);
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
    if symbol_map.contains_key(dep) {
        return Some(dep.to_string());
    }
    
    let function_name = dep.split("::").last().unwrap_or(dep);
    
    // Look for use statements
    for (key, _) in symbol_map {
        if key.starts_with("use_") && key.contains(&format!("_{}_", function_name)) {
            return Some(key.clone());
        }
    }
    
    // Pattern match
    let matches: Vec<_> = symbol_map.keys()
        .filter(|k| k.ends_with(&format!("::{}", function_name)))
        .collect();
    
    if matches.len() == 1 {
        return Some(matches[0].clone());
    }
    
    // Fuzzy match
    let matches: Vec<_> = symbol_map.keys()
        .filter(|k| k.contains(function_name))
        .take(1)
        .collect();
    
    if matches.len() == 1 {
        return Some(matches[0].clone());
    }
    
    None
}
