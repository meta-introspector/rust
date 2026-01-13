use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🔍 RUST AST FREQUENCY ANALYSIS");
    println!("===============================");

    let eigenmatrix_content =
        fs::read_to_string("rust_eigenmatrix.json").expect("Failed to read eigenmatrix file");

    let eigenmatrix: Value =
        serde_json::from_str(&eigenmatrix_content).expect("Failed to parse eigenmatrix JSON");

    // Get features and eigenvalues
    let features = eigenmatrix["features"].as_array().unwrap();
    let eigenvalues = eigenmatrix["eigenvalues"].as_array().unwrap();

    // Create frequency map from eigenvalues (which represent feature weights)
    let mut frequency_map: Vec<(String, f64)> = features
        .iter()
        .zip(eigenvalues.iter())
        .map(|(feature, eigenval)| {
            (feature.as_str().unwrap().to_string(), eigenval.as_f64().unwrap().abs())
        })
        .collect();

    // Sort by frequency (eigenvalue magnitude)
    frequency_map.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n📊 TOP 20 MOST FREQUENT AST PATTERNS:");
    println!("=====================================");
    for (i, (feature, freq)) in frequency_map.iter().take(20).enumerate() {
        println!("{:2}. {:60} | Weight: {:.2e}", i + 1, feature, freq);
    }

    // Analyze by depth
    let mut depth_analysis: HashMap<usize, Vec<(String, f64)>> = HashMap::new();
    for (feature, freq) in &frequency_map {
        let depth = feature.matches('.').count();
        depth_analysis.entry(depth).or_insert_with(Vec::new).push((feature.clone(), *freq));
    }

    println!("\n🌳 FREQUENCY BY AST DEPTH:");
    println!("==========================");
    for depth in 0..=6 {
        if let Some(patterns) = depth_analysis.get(&depth) {
            let total_weight: f64 = patterns.iter().map(|(_, w)| w).sum();
            println!(
                "Depth {}: {} patterns, Total weight: {:.2e}",
                depth,
                patterns.len(),
                total_weight
            );

            // Show top 3 at this depth
            let mut sorted_patterns = patterns.clone();
            sorted_patterns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            for (i, (pattern, weight)) in sorted_patterns.iter().take(3).enumerate() {
                println!("  {}. {} (weight: {:.2e})", i + 1, pattern, weight);
            }
            println!();
        }
    }

    // Analyze common prefixes
    let mut prefix_weights: HashMap<String, f64> = HashMap::new();
    for (feature, freq) in &frequency_map {
        let parts: Vec<&str> = feature.split('.').collect();
        for i in 1..=parts.len() {
            let prefix = parts[..i].join(".");
            *prefix_weights.entry(prefix).or_insert(0.0) += freq;
        }
    }

    let mut sorted_prefixes: Vec<(String, f64)> = prefix_weights.into_iter().collect();
    sorted_prefixes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("🎯 TOP 15 STRUCTURAL PATTERNS:");
    println!("===============================");
    for (i, (prefix, weight)) in sorted_prefixes.iter().take(15).enumerate() {
        println!("{:2}. {:50} | Weight: {:.2e}", i + 1, prefix, weight);
    }
}
