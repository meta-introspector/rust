use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("ast_patterns.json")?;
    let patterns: HashMap<String, u64> = serde_json::from_str(&data)?;
    
    // Build adjacency matrix for symbol transitions
    let mut symbol_set: Vec<String> = Vec::new();
    let mut symbol_index: HashMap<String, usize> = HashMap::new();
    let mut transitions: HashMap<(usize, usize), u64> = HashMap::new();
    
    // Collect all unique symbols
    for pattern in patterns.keys() {
        for symbol in pattern.split("::") {
            if !symbol_index.contains_key(symbol) {
                symbol_index.insert(symbol.to_string(), symbol_set.len());
                symbol_set.push(symbol.to_string());
            }
        }
    }
    
    println!("🔗 Building Transition Matrix: {} symbols", symbol_set.len());
    
    // Build transition matrix from patterns
    for (pattern, count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        
        for i in 0..parts.len()-1 {
            let from_idx = symbol_index[parts[i]];
            let to_idx = symbol_index[parts[i+1]];
            *transitions.entry((from_idx, to_idx)).or_insert(0) += count;
        }
    }
    
    println!("📊 Transition Matrix: {} non-zero entries", transitions.len());
    
    // Find automorphism cycles (data → types → data)
    let mut automorphism_cycles: Vec<(Vec<String>, u64)> = Vec::new();
    
    for (pattern, count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        
        // Look for cycles where we return to similar symbol types
        if parts.len() >= 3 {
            let start_type = classify_symbol(parts[0]);
            let end_type = classify_symbol(parts[parts.len()-1]);
            
            if start_type == end_type && start_type != "other" {
                let cycle: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
                automorphism_cycles.push((cycle, *count));
            }
        }
    }
    
    automorphism_cycles.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🔄 Automorphism Cycles (data→types→data):");
    for (cycle, count) in automorphism_cycles.iter().take(10) {
        let start_type = classify_symbol(&cycle[0]);
        let path: Vec<String> = cycle.iter()
            .map(|s| classify_symbol(s).to_string())
            .collect();
        println!("  {} → {} ({}x)", cycle.join("::"), path.join("→"), count);
    }
    
    // Find eigenvalue clusters - symbols with similar connectivity patterns
    let mut connectivity_patterns: HashMap<String, Vec<(String, u64)>> = HashMap::new();
    
    for ((from_idx, to_idx), weight) in &transitions {
        let from_symbol = &symbol_set[*from_idx];
        let to_symbol = &symbol_set[*to_idx];
        
        connectivity_patterns.entry(from_symbol.clone())
            .or_insert_with(Vec::new)
            .push((to_symbol.clone(), *weight));
    }
    
    // Cluster symbols by connectivity signature
    let mut clusters: HashMap<String, Vec<String>> = HashMap::new();
    
    for (symbol, connections) in &connectivity_patterns {
        if connections.len() >= 3 {
            // Create signature based on connection types
            let mut signature: Vec<String> = connections.iter()
                .map(|(target, _)| classify_symbol(target).to_string())
                .collect();
            signature.sort();
            signature.dedup();
            
            let sig_key = signature.join("-");
            clusters.entry(sig_key)
                .or_insert_with(Vec::new)
                .push(symbol.clone());
        }
    }
    
    println!("\n🎯 Eigenvalue Clusters (similar connectivity):");
    for (signature, symbols) in clusters.iter() {
        if symbols.len() > 2 {
            println!("  {} → {} symbols: {:?}", 
                signature, symbols.len(), &symbols[..3.min(symbols.len())]);
        }
    }
    
    // Build sparse matrix representation
    let matrix_size = symbol_set.len();
    let density = transitions.len() as f64 / (matrix_size * matrix_size) as f64;
    
    println!("\n📐 Sparse Matrix Properties:");
    println!("  Size: {}×{}", matrix_size, matrix_size);
    println!("  Non-zero entries: {}", transitions.len());
    println!("  Density: {:.4}%", density * 100.0);
    
    // Find strongly connected components (fixed point regions)
    let mut scc_components: Vec<Vec<String>> = Vec::new();
    let mut visited: Vec<bool> = vec![false; symbol_set.len()];
    
    for i in 0..symbol_set.len() {
        if !visited[i] {
            let mut component = Vec::new();
            dfs_component(i, &transitions, &symbol_set, &mut visited, &mut component);
            if component.len() > 1 {
                scc_components.push(component);
            }
        }
    }
    
    println!("\n🔗 Strongly Connected Components (Fixed Point Regions):");
    for (i, component) in scc_components.iter().take(5).enumerate() {
        println!("  Component {}: {} symbols: {:?}", 
            i+1, component.len(), &component[..3.min(component.len())]);
    }
    
    Ok(())
}

fn classify_symbol(symbol: &str) -> &'static str {
    match symbol {
        "tcx" | "self" | "dcx" => "context",
        "call" | "emit" | "emit_err" => "action", 
        "Some" | "Ok" | "Err" | "None" => "constructor",
        "span" | "def_id" | "ty" => "data",
        "map" | "collect" | "filter" => "transform",
        _ => "other"
    }
}

fn dfs_component(
    node: usize, 
    transitions: &HashMap<(usize, usize), u64>,
    symbol_set: &[String],
    visited: &mut [bool],
    component: &mut Vec<String>
) {
    visited[node] = true;
    component.push(symbol_set[node].clone());
    
    // Find all connected nodes
    for ((from, to), _) in transitions {
        if *from == node && !visited[*to] {
            dfs_component(*to, transitions, symbol_set, visited, component);
        }
    }
}
