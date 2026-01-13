use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("🔗 RUST AST RELATIONSHIP ANALYSIS");
    println!("==================================");

    let eigenmatrix_content =
        fs::read_to_string("rust_eigenmatrix.json").expect("Failed to read eigenmatrix file");

    let eigenmatrix: Value =
        serde_json::from_str(&eigenmatrix_content).expect("Failed to parse eigenmatrix JSON");

    let features = eigenmatrix["features"].as_array().unwrap();
    let eigenvalues = eigenmatrix["eigenvalues"].as_array().unwrap();

    // Create feature-weight map
    let mut feature_weights: HashMap<String, f64> = features
        .iter()
        .zip(eigenvalues.iter())
        .map(|(f, e)| (f.as_str().unwrap().to_string(), e.as_f64().unwrap().abs()))
        .collect();

    // Find parent-child relationships (arrows)
    let mut arrows: Vec<(String, String, f64)> = Vec::new();
    let mut nodes: HashSet<String> = HashSet::new();

    for feature in features {
        let feature_str = feature.as_str().unwrap();
        nodes.insert(feature_str.to_string());

        // Find parent by removing last component
        if let Some(last_dot) = feature_str.rfind('.') {
            let parent = &feature_str[..last_dot];
            if nodes.contains(parent) || features.iter().any(|f| f.as_str().unwrap() == parent) {
                let weight = feature_weights.get(feature_str).unwrap_or(&0.0);
                arrows.push((parent.to_string(), feature_str.to_string(), *weight));
            }
        }
    }

    // Sort arrows by weight
    arrows.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    println!("\n🏹 TOP 20 AST RELATIONSHIPS (Parent → Child):");
    println!("==============================================");
    for (i, (parent, child, weight)) in arrows.iter().take(20).enumerate() {
        let child_name = child.split('.').last().unwrap_or(child);
        println!(
            "{:2}. {} → {} | Weight: {:.2e}",
            i + 1,
            truncate_path(parent, 40),
            child_name,
            weight
        );
    }

    // Analyze relationship patterns
    let mut relationship_types: HashMap<String, Vec<(String, String, f64)>> = HashMap::new();

    for (parent, child, weight) in &arrows {
        let child_type = child.split('.').last().unwrap_or("unknown");
        relationship_types.entry(child_type.to_string()).or_insert_with(Vec::new).push((
            parent.clone(),
            child.clone(),
            *weight,
        ));
    }

    println!("\n🎯 RELATIONSHIP TYPES BY FREQUENCY:");
    println!("===================================");
    let mut sorted_types: Vec<(String, Vec<(String, String, f64)>)> =
        relationship_types.into_iter().collect();
    sorted_types.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    for (rel_type, relationships) in sorted_types.iter().take(10) {
        let total_weight: f64 = relationships.iter().map(|(_, _, w)| w).sum();
        println!(
            "{:15} | Count: {:2} | Total Weight: {:.2e}",
            rel_type,
            relationships.len(),
            total_weight
        );

        // Show top 3 examples
        let mut sorted_rels = relationships.clone();
        sorted_rels.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        for (i, (parent, child, weight)) in sorted_rels.iter().take(3).enumerate() {
            println!(
                "  {}. {} → {} ({:.1e})",
                i + 1,
                truncate_path(parent, 30),
                child.split('.').last().unwrap_or("?"),
                weight
            );
        }
        println!();
    }

    // Find interesting patterns
    println!("🔍 INTERESTING PATTERNS:");
    println!("========================");

    // Most connected nodes (highest out-degree)
    let mut out_degrees: HashMap<String, usize> = HashMap::new();
    for (parent, _, _) in &arrows {
        *out_degrees.entry(parent.clone()).or_insert(0) += 1;
    }

    let mut sorted_degrees: Vec<(String, usize)> = out_degrees.into_iter().collect();
    sorted_degrees.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n📊 Most Connected Nodes (Highest Fan-out):");
    for (i, (node, degree)) in sorted_degrees.iter().take(8).enumerate() {
        println!("{:2}. {} | Children: {}", i + 1, truncate_path(node, 50), degree);
    }

    // Deep paths (longest chains)
    let max_depth =
        features.iter().map(|f| f.as_str().unwrap().matches('.').count()).max().unwrap_or(0);

    println!("\n🌊 Deepest AST Paths (Depth {}):", max_depth);
    let deep_paths: Vec<&str> = features
        .iter()
        .map(|f| f.as_str().unwrap())
        .filter(|f| f.matches('.').count() == max_depth)
        .collect();

    for (i, path) in deep_paths.iter().take(5).enumerate() {
        let weight = feature_weights.get(*path).unwrap_or(&0.0);
        println!("{:2}. {} | Weight: {:.1e}", i + 1, path, weight);
    }
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - max_len + 3..])
    }
}
