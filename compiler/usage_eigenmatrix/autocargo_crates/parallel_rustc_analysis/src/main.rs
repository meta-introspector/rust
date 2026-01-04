use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::fs;
use std::io::{Write, Read};
use flate2::read::GzDecoder;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    println!("🚀 Starting parallel rustc analysis with {} CPUs", rayon::current_num_threads());
    
    let compressed_data = fs::read("symbol_map.json.gz").unwrap();
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut symbol_map_content = String::new();
    decoder.read_to_string(&mut symbol_map_content).unwrap();
    let symbol_map: HashMap<String, Value> = 
        serde_json::from_str(&symbol_map_content).unwrap();
    
    println!("📊 Loaded {} symbols", symbol_map.len());
    
    let mut output = std::fs::File::create("rustc_parallel_analysis.txt").unwrap();
    writeln!(output, "🚀 Parallel rustc Analysis\n").unwrap();
    
    // Collect all functions to analyze in parallel
    let targets = vec![
        "rustc::main::main",
        "rustc_driver_impl::lib::main",
        "rustc_driver::main",
    ];
    
    let function_calls = Arc::new(Mutex::new(HashMap::new()));
    let visited = Arc::new(Mutex::new(HashSet::new()));
    
    // Process targets in parallel
    targets.par_iter().for_each(|target| {
        println!("🔍 Analyzing {} on thread {:?}", target, std::thread::current().id());
        analyze_function_parallel(&symbol_map, target, &function_calls, &visited);
    });
    
    // Write results
    let final_calls = function_calls.lock().unwrap();
    let final_visited = visited.lock().unwrap();
    
    writeln!(output, "📊 Analysis Results:").unwrap();
    writeln!(output, "  🔄 Unique Functions: {}", final_visited.len()).unwrap();
    writeln!(output, "  📈 Total Calls: {}", final_calls.values().sum::<usize>()).unwrap();
    
    // Top functions
    let mut sorted: Vec<_> = final_calls.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    writeln!(output, "\n🔥 Top 50 Functions:").unwrap();
    for (func, count) in sorted.iter().take(50) {
        writeln!(output, "  {}x {}", count, func).unwrap();
    }
    
    println!("✅ Parallel analysis complete! {} functions analyzed", final_visited.len());
}

fn analyze_function_parallel(
    symbol_map: &HashMap<String, Value>,
    function_name: &str,
    function_calls: &Arc<Mutex<HashMap<String, usize>>>,
    visited: &Arc<Mutex<HashSet<String>>>
) {
    // Check if already processed
    {
        let mut v = visited.lock().unwrap();
        if v.contains(function_name) {
            return;
        }
        v.insert(function_name.to_string());
    }
    
    // Count the call
    {
        let mut calls = function_calls.lock().unwrap();
        *calls.entry(function_name.to_string()).or_insert(0) += 1;
    }
    
    if let Some(symbol) = symbol_map.get(function_name) {
        if let Some(deps) = symbol["dependencies"].as_array() {
            let resolved_functions: Vec<String> = deps.par_iter()
                .filter_map(|dep| {
                    let dep_str = dep.as_str()?;
                    if is_variable(dep_str) {
                        None
                    } else {
                        resolve_dependency(symbol_map, dep_str)
                    }
                })
                .collect();
            
            // Process dependencies in parallel
            resolved_functions.par_iter().for_each(|resolved| {
                analyze_function_parallel(symbol_map, resolved, function_calls, visited);
            });
        }
    }
}

fn is_variable(dep: &str) -> bool {
    dep.starts_with("std::") && 
    (dep.contains("::new") || dep.contains("::default") || dep.contains("::clone"))
}

fn resolve_dependency(symbol_map: &HashMap<String, Value>, dep: &str) -> Option<String> {
    if symbol_map.contains_key(dep) {
        return Some(dep.to_string());
    }
    
    // Try common patterns
    let patterns = [
        format!("{}::main", dep),
        format!("{}::lib::main", dep),
        format!("{}::run", dep),
    ];
    
    for pattern in &patterns {
        if symbol_map.contains_key(pattern) {
            return Some(pattern.clone());
        }
    }
    
    None
}
