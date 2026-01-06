use std::collections::HashMap;
use introspector_collector::libusagedata::{UsageData, load_usage_data};

#[derive(Debug, Clone)]
struct ResonanceNode {
    name: String,
    complexity: f64,
    usage_count: usize,
    resonance_score: f64,
}

fn calculate_resonance(node1: &str, node2: &str, usage1: usize, usage2: usize) -> f64 {
    let name_similarity = jaccard_similarity(node1, node2);
    let usage_ratio = (usage1.min(usage2) as f64) / (usage1.max(usage2) as f64).max(1.0);
    name_similarity * usage_ratio * 100.0
}

fn jaccard_similarity(s1: &str, s2: &str) -> f64 {
    let chars1: std::collections::HashSet<char> = s1.chars().collect();
    let chars2: std::collections::HashSet<char> = s2.chars().collect();
    let intersection = chars1.intersection(&chars2).count();
    let union = chars1.union(&chars2).count();
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

fn find_resonant_nodes(target_nodes: &[&str]) -> HashMap<String, Vec<ResonanceNode>> {
    let mut resonances = HashMap::new();
    
    // Load all usage data from the comprehensive dataset
    let usage_data = load_usage_data();
    
    // Find resonances for each target node
    for &target in target_nodes {
        let mut resonant_nodes = Vec::new();
        
        for (crate_name, data) in &usage_data {
            let score = calculate_resonance(target, crate_name, 100, data.total_usages);
            if score > 5.0 {  // Lower threshold for more results
                resonant_nodes.push(ResonanceNode {
                    name: crate_name.clone(),
                    complexity: data.complexity_items as f64,
                    usage_count: data.total_usages,
                    resonance_score: score,
                });
            }
        }
        
        resonant_nodes.sort_by(|a, b| b.resonance_score.partial_cmp(&a.resonance_score).unwrap());
        resonant_nodes.truncate(5);  // Top 5 resonances
        resonances.insert(target.to_string(), resonant_nodes);
    }
    
    resonances
}

fn main() {
    println!("🌊 Resonance Finder: Discovering Mathematical Harmonics");
    
    // Target nodes from syn walker analysis
    let target_nodes = ["TypeNode", "FunctionArrow", "EnumOrbit", "RustLanguage", "usage_ratio", "main"];
    
    let resonances = find_resonant_nodes(&target_nodes);
    
    for (target, nodes) in &resonances {
        println!("\n🎯 Resonances for {}: ", target);
        for (i, node) in nodes.iter().enumerate() {
            println!("  {}. {} (score: {:.2}, usage: {}, complexity: {:.1})", 
                i + 1, node.name, node.resonance_score, node.usage_count, node.complexity);
        }
    }
    
    // Calculate total resonance energy
    let total_energy: f64 = resonances.values()
        .flat_map(|nodes| nodes.iter())
        .map(|node| node.resonance_score)
        .sum();
    
    println!("\n✨ Total Resonance Energy: {:.2}", total_energy);
    println!("🔮 Mathematical harmonics discovered in Rust ecosystem");
}
