use std::collections::HashMap;
use std::fs;
use serde_json::Value;

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
    
    // Create synthetic resonance data from known Rust ecosystem patterns
    let ecosystem_nodes = vec![
        ("syn", 5874, 928.0),
        ("proc_macro2", 522, 89.0), 
        ("quote", 424, 110.0),
        ("serde", 304, 152.0),
        ("tokio", 1060, 427.0),
        ("regex", 47, 27.0),
        ("libc", 11337, 377.0),
        ("futures_util", 958, 309.0),
        ("anyhow", 98, 40.0),
        ("clap_builder", 1508, 136.0),
    ];
    
    // Find resonances for each target node
    for &target in target_nodes {
        let mut all_nodes = Vec::new();
        
        for entry in entries.flatten() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(crate_name) = entry.file_name().to_str() {
                        let usage_count = extract_usage_count(&json);
                        let complexity = extract_complexity(&json);
                        
                        all_nodes.push(ResonanceNode {
                            name: crate_name.replace("_manifest.json", ""),
                            complexity,
                            usage_count,
                            resonance_score: 0.0,
                        });
                    }
                }
            }
        }
        
        // Find resonances for each target node
        for &target in target_nodes {
            let mut resonant_nodes = Vec::new();
            
            for node in &all_nodes {
                let score = calculate_resonance(target, &node.name, 100, node.usage_count);
                if score > 10.0 {  // Threshold for resonance
                    resonant_nodes.push(ResonanceNode {
                        name: node.name.clone(),
                        complexity: node.complexity,
                        usage_count: node.usage_count,
                        resonance_score: score,
                    });
                }
            }
            
            resonant_nodes.sort_by(|a, b| b.resonance_score.partial_cmp(&a.resonance_score).unwrap());
            resonant_nodes.truncate(5);  // Top 5 resonances
            resonances.insert(target.to_string(), resonant_nodes);
        }
    }
    
    resonances
}

fn extract_usage_count(json: &Value) -> usize {
    // Extract from debug output pattern
    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            if key.contains("total_usages") || key.contains("usages") {
                return value.as_u64().unwrap_or(0) as usize;
            }
        }
    }
    0
}

fn extract_complexity(json: &Value) -> f64 {
    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            if key.contains("complexity") {
                return value.as_f64().unwrap_or(0.0);
            }
        }
    }
    1.0
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
