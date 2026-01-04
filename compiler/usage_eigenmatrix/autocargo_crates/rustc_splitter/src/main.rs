use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("ast_patterns.json")?;
    let patterns: HashMap<String, u64> = serde_json::from_str(&data)?;
    
    // Build symbol dependency graph
    let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    let mut reverse_deps: HashMap<String, HashSet<String>> = HashMap::new();
    
    for (pattern, _count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        
        for i in 0..parts.len()-1 {
            let from = parts[i].to_string();
            let to = parts[i+1].to_string();
            
            dependencies.entry(from.clone())
                .or_insert_with(HashSet::new)
                .insert(to.clone());
            
            reverse_deps.entry(to)
                .or_insert_with(HashSet::new)
                .insert(from);
        }
    }
    
    println!("🔗 Dependency Graph: {} symbols, {} edges", 
        dependencies.len(), 
        dependencies.values().map(|s| s.len()).sum::<usize>());
    
    // Find rustc main entry points
    let main_symbols: Vec<String> = dependencies.keys()
        .filter(|s| s.contains("main") || s.contains("rustc_driver"))
        .cloned()
        .collect();
    
    println!("\n🎯 Main Entry Points: {:?}", main_symbols);
    
    // Cluster analysis using connectivity
    let mut clusters: Vec<HashSet<String>> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    
    // Start from main symbols and build clusters
    for main_symbol in &main_symbols {
        if !visited.contains(main_symbol) {
            let mut cluster = HashSet::new();
            dfs_cluster(main_symbol, &dependencies, &mut visited, &mut cluster);
            if cluster.len() > 1 {
                clusters.push(cluster);
            }
        }
    }
    
    // Find remaining unvisited symbols and cluster them
    for symbol in dependencies.keys() {
        if !visited.contains(symbol) {
            let mut cluster = HashSet::new();
            dfs_cluster(symbol, &dependencies, &mut visited, &mut cluster);
            if cluster.len() > 5 { // Only keep significant clusters
                clusters.push(cluster);
            }
        }
    }
    
    clusters.sort_by(|a, b| b.len().cmp(&a.len()));
    
    println!("\n🧩 Functional Clusters Found: {}", clusters.len());
    for (i, cluster) in clusters.iter().take(10).enumerate() {
        let sample: Vec<_> = cluster.iter().take(5).collect();
        println!("  Cluster {}: {} symbols - {:?}", i+1, cluster.len(), sample);
    }
    
    // Analyze coupling between clusters
    let mut coupling_matrix: HashMap<(usize, usize), u64> = HashMap::new();
    
    for (i, cluster_a) in clusters.iter().enumerate() {
        for (j, cluster_b) in clusters.iter().enumerate() {
            if i != j {
                let mut coupling_strength = 0u64;
                
                for symbol_a in cluster_a {
                    if let Some(deps) = dependencies.get(symbol_a) {
                        for dep in deps {
                            if cluster_b.contains(dep) {
                                coupling_strength += 1;
                            }
                        }
                    }
                }
                
                if coupling_strength > 0 {
                    coupling_matrix.insert((i, j), coupling_strength);
                }
            }
        }
    }
    
    println!("\n🔗 Cluster Coupling Analysis:");
    let mut coupling_pairs: Vec<_> = coupling_matrix.iter().collect();
    coupling_pairs.sort_by(|a, b| b.1.cmp(a.1));
    
    for ((i, j), strength) in coupling_pairs.iter().take(10) {
        println!("  Cluster {} → Cluster {}: {} connections", i+1, j+1, strength);
    }
    
    // Identify loosely coupled functional units
    let mut loose_units: Vec<Vec<usize>> = Vec::new();
    let mut cluster_visited: HashSet<usize> = HashSet::new();
    
    for i in 0..clusters.len() {
        if !cluster_visited.contains(&i) {
            let mut unit = vec![i];
            cluster_visited.insert(i);
            
            // Find tightly coupled clusters
            for j in (i+1)..clusters.len() {
                if !cluster_visited.contains(&j) {
                    let coupling_ij = coupling_matrix.get(&(i, j)).unwrap_or(&0);
                    let coupling_ji = coupling_matrix.get(&(j, i)).unwrap_or(&0);
                    let total_coupling = coupling_ij + coupling_ji;
                    
                    // If coupling is high relative to cluster sizes, merge
                    let cluster_i_size = clusters[i].len() as u64;
                    let cluster_j_size = clusters[j].len() as u64;
                    let coupling_ratio = total_coupling as f64 / (cluster_i_size + cluster_j_size) as f64;
                    
                    if coupling_ratio > 0.1 { // High coupling threshold
                        unit.push(j);
                        cluster_visited.insert(j);
                    }
                }
            }
            
            loose_units.push(unit);
        }
    }
    
    println!("\n🎯 Loosely Coupled Functional Units: {}", loose_units.len());
    for (i, unit) in loose_units.iter().enumerate() {
        let total_symbols: usize = unit.iter().map(|&idx| clusters[idx].len()).sum();
        println!("  Unit {}: {} clusters, {} total symbols", i+1, unit.len(), total_symbols);
        
        // Show representative symbols from each cluster in the unit
        for &cluster_idx in unit {
            let sample: Vec<_> = clusters[cluster_idx].iter().take(3).collect();
            println!("    Cluster {}: {:?}", cluster_idx+1, sample);
        }
    }
    
    // Generate split recommendations
    println!("\n📋 Rustc Split Recommendations:");
    for (i, unit) in loose_units.iter().enumerate() {
        let total_symbols: usize = unit.iter().map(|&idx| clusters[idx].len()).sum();
        
        if total_symbols > 20 {
            let unit_name = generate_unit_name(&clusters, unit);
            println!("  📦 Unit {}: '{}' ({} symbols)", i+1, unit_name, total_symbols);
            println!("    - Can be extracted as independent module");
            println!("    - Minimal external dependencies");
            
            // Check for main entry points
            let has_main = unit.iter().any(|&idx| {
                clusters[idx].iter().any(|s| s.contains("main") || s.contains("driver"))
            });
            
            if has_main {
                println!("    - ⭐ Contains main entry point - core module");
            }
        }
    }
    
    Ok(())
}

fn dfs_cluster(
    symbol: &str,
    dependencies: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    cluster: &mut HashSet<String>
) {
    if visited.contains(symbol) {
        return;
    }
    
    visited.insert(symbol.to_string());
    cluster.insert(symbol.to_string());
    
    if let Some(deps) = dependencies.get(symbol) {
        for dep in deps {
            dfs_cluster(dep, dependencies, visited, cluster);
        }
    }
}

fn generate_unit_name(clusters: &[HashSet<String>], unit: &[usize]) -> String {
    let mut all_symbols: Vec<String> = Vec::new();
    
    for &idx in unit {
        all_symbols.extend(clusters[idx].iter().cloned());
    }
    
    // Find most common theme
    let themes = vec![
        ("driver", "Driver"),
        ("main", "Main"),
        ("tcx", "TypeContext"),
        ("emit", "Diagnostics"),
        ("call", "Execution"),
        ("collect", "Collection"),
        ("map", "Transform"),
        ("span", "SourceInfo"),
        ("ty", "Types"),
        ("def_id", "Definitions"),
    ];
    
    for (pattern, name) in themes {
        let count = all_symbols.iter().filter(|s| s.contains(pattern)).count();
        if count > all_symbols.len() / 4 {
            return name.to_string();
        }
    }
    
    "Generic".to_string()
}
