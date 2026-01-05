use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone)]
struct MultiStepChain {
    steps: Vec<String>,
    frequency: u32,
    depth: usize,
}

fn main() {
    println!("🌐 Discovering Rust's Emergent Data Model Through Usage Graphs");
    println!("==============================================================");
    
    let mut chains: HashMap<String, u32> = HashMap::new();
    let mut processed = 0;
    
    // Extract multi-step chains
    let usage_dir = "../../usage_data";
    if let Ok(entries) = fs::read_dir(usage_dir) {
        for entry in entries.flatten().take(500) {
            if let Some(name) = entry.file_name().to_str() {
                if name.contains("rustc") && name.ends_with(".json") {
                    extract_multi_step_chains(&entry.path().display().to_string(), &mut chains);
                    processed += 1;
                }
            }
        }
    }
    
    // Convert to structured chains
    let mut multi_chains: Vec<MultiStepChain> = chains.iter()
        .filter_map(|(pattern, count)| {
            let steps: Vec<String> = pattern.split(" -> ").map(|s| s.to_string()).collect();
            if steps.len() >= 2 {
                Some(MultiStepChain {
                    steps: steps.clone(),
                    frequency: *count,
                    depth: steps.len(),
                })
            } else {
                None
            }
        })
        .collect();
    
    multi_chains.sort_by(|a, b| b.frequency.cmp(&a.frequency));
    
    println!("\n🔗 EMERGENT RUST DATA MODEL:");
    println!("============================");
    
    // Group by depth
    for depth in 2..=5 {
        let depth_chains: Vec<_> = multi_chains.iter()
            .filter(|c| c.depth == depth)
            .take(3)
            .collect();
        
        if !depth_chains.is_empty() {
            println!("\n📊 {}-Step Patterns:", depth);
            for (i, chain) in depth_chains.iter().enumerate() {
                let path = chain.steps.join(" → ");
                println!("  {}. {} (freq: {})", i + 1, path, chain.frequency);
            }
        }
    }
    
    // Analyze the graph structure
    analyze_graph_structure(&multi_chains);
    
    // Generate the data model
    generate_rust_data_model(&multi_chains);
}

fn extract_multi_step_chains(filepath: &str, chains: &mut HashMap<String, u32>) {
    if let Ok(content) = fs::read_to_string(filepath) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(usages) = json["usages"].as_array() {
                let mut sequence = Vec::new();
                
                for usage in usages.iter().take(10) { // Sample to avoid noise
                    if let Some(usage_str) = usage["usage"].as_str() {
                        if let Some(method) = extract_method_name(usage_str) {
                            sequence.push(method);
                        }
                    }
                }
                
                // Generate chains of different lengths
                for window_size in 2..=4 {
                    for window in sequence.windows(window_size) {
                        if window.iter().all(|s| !s.is_empty()) {
                            let chain = window.join(" -> ");
                            *chains.entry(chain).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
}

fn extract_method_name(usage_str: &str) -> Option<String> {
    let key_methods = [
        "DefId", "hir", "TyCtxt", "predicates_of", "associated_item", 
        "type_of", "param_env", "generics_of", "def_kind", "def_path_str"
    ];
    
    for method in &key_methods {
        if usage_str.contains(method) {
            return Some(method.to_string());
        }
    }
    None
}

fn analyze_graph_structure(chains: &[MultiStepChain]) {
    println!("\n🌐 GRAPH STRUCTURE ANALYSIS:");
    println!("===========================");
    
    let mut node_frequency: HashMap<String, u32> = HashMap::new();
    let mut edge_frequency: HashMap<String, u32> = HashMap::new();
    
    for chain in chains {
        // Count nodes
        for step in &chain.steps {
            *node_frequency.entry(step.clone()).or_insert(0) += chain.frequency;
        }
        
        // Count edges
        for window in chain.steps.windows(2) {
            let edge = format!("{} -> {}", window[0], window[1]);
            *edge_frequency.entry(edge).or_insert(0) += chain.frequency;
        }
    }
    
    // Find central nodes (hubs)
    let mut sorted_nodes: Vec<_> = node_frequency.iter().collect();
    sorted_nodes.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 Central Nodes (Data Model Hubs):");
    for (i, (node, freq)) in sorted_nodes.iter().take(5).enumerate() {
        println!("  {}. {} (appears in {} chains)", i + 1, node, freq);
    }
    
    // Find critical edges (data flow patterns)
    let mut sorted_edges: Vec<_> = edge_frequency.iter().collect();
    sorted_edges.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🔄 Critical Data Flows:");
    for (i, (edge, freq)) in sorted_edges.iter().take(5).enumerate() {
        println!("  {}. {} (freq: {})", i + 1, edge, freq);
    }
}

fn generate_rust_data_model(chains: &[MultiStepChain]) {
    println!("\n🏗️  EMERGENT RUST DATA MODEL:");
    println!("=============================");
    
    println!("```");
    println!("DefId (Identity Layer)");
    println!("  ↓");
    println!("HIR (Syntax Layer)");
    println!("  ↓");
    println!("TyCtxt (Analysis Layer)");
    println!("  ↓");
    println!("predicates_of (Type Layer)");
    println!("  ↓");
    println!("associated_item (Trait Layer)");
    println!("```");
    
    println!("\nThis reveals Rust's **5-layer data architecture**:");
    println!("1. **Identity**: DefId - unique identifiers");
    println!("2. **Syntax**: HIR - structural representation");  
    println!("3. **Analysis**: TyCtxt - compiler context");
    println!("4. **Types**: predicates_of - type relationships");
    println!("5. **Traits**: associated_item - behavioral contracts");
    
    println!("\n🔍 The usage patterns show the **natural information flow**:");
    println!("- Start with identity (DefId)");
    println!("- Navigate structure (HIR)");
    println!("- Enter analysis context (TyCtxt)");
    println!("- Examine type relationships (predicates)");
    println!("- Resolve behavioral contracts (traits)");
    
    // Save the model
    let model = format!(
        "# Emergent Rust Data Model\n\n\
        ## 5-Layer Architecture\n\
        1. Identity Layer: DefId\n\
        2. Syntax Layer: HIR\n\
        3. Analysis Layer: TyCtxt\n\
        4. Type Layer: predicates_of\n\
        5. Trait Layer: associated_item\n\n\
        ## Usage Flow Patterns\n"
    );
    
    fs::write("rust_data_model.md", model).unwrap();
    println!("\n📁 Data model saved to rust_data_model.md");
}
