use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🕸️ Rustc Driver Dependency Graph Analysis");
    
    let usage_data = load_usage_data()?;
    let driver_connections = find_driver_connections(&usage_data);
    let central_nodes = find_most_connected_nodes(&usage_data);
    
    println!("\n🎯 Rustc Driver Direct Connections ({} nodes):", driver_connections.len());
    for (node, weight) in driver_connections.iter().take(15) {
        println!("  {} -> {} (weight: {})", "rustc_driver", node, weight);
    }
    
    println!("\n🌟 Most Central Nodes in Rust Ecosystem:");
    for (i, (node, connections)) in central_nodes.iter().take(10).enumerate() {
        println!("{}. {} - {} connections", i+1, node, connections);
    }
    
    generate_dot_graph(&usage_data)?;
    generate_network_stats(&usage_data);
    
    Ok(())
}

fn load_usage_data() -> Result<Vec<(String, String, usize)>> {
    let mut edges = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let user_id = usage_obj["user_def_id"].as_str().unwrap_or("unknown");
                    let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                    let count = usage_obj["usage_count"].as_u64().unwrap_or(1) as usize;
                    
                    edges.push((user_id.to_string(), used_id.to_string(), count));
                }
            }
        }
    }
    
    Ok(edges)
}

fn find_driver_connections(edges: &[(String, String, usize)]) -> Vec<(String, usize)> {
    let mut connections = HashMap::new();
    
    for (user, used, weight) in edges {
        if user.contains("rustc_driver") || user.contains("driver") {
            *connections.entry(used.clone()).or_insert(0) += weight;
        }
        if used.contains("rustc_driver") || used.contains("driver") {
            *connections.entry(user.clone()).or_insert(0) += weight;
        }
    }
    
    let mut sorted: Vec<_> = connections.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_most_connected_nodes(edges: &[(String, String, usize)]) -> Vec<(String, usize)> {
    let mut node_connections = HashMap::new();
    
    for (user, used, weight) in edges {
        *node_connections.entry(user.clone()).or_insert(0) += weight;
        *node_connections.entry(used.clone()).or_insert(0) += weight;
    }
    
    let mut sorted: Vec<_> = node_connections.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn generate_dot_graph(edges: &[(String, String, usize)]) -> Result<()> {
    let mut dot_content = String::from("digraph rustc_ecosystem {\n");
    dot_content.push_str("  rankdir=LR;\n");
    dot_content.push_str("  node [shape=box, style=filled];\n");
    
    // Highlight rustc_driver
    dot_content.push_str("  \"rustc_driver\" [fillcolor=red, fontcolor=white];\n");
    
    // Add top connections only (to keep graph readable)
    let mut added_edges = std::collections::HashSet::new();
    for (user, used, weight) in edges.iter().take(100) {
        if *weight > 10 { // Only show significant connections
            let edge = format!("\"{}\" -> \"{}\"", 
                simplify_node_name(user), 
                simplify_node_name(used)
            );
            
            if !added_edges.contains(&edge) {
                dot_content.push_str(&format!("  {} [label=\"{}\"];\n", edge, weight));
                added_edges.insert(edge);
            }
        }
    }
    
    dot_content.push_str("}\n");
    
    fs::write("rustc_ecosystem_graph.dot", dot_content)?;
    println!("\n📊 Generated DOT graph: rustc_ecosystem_graph.dot");
    println!("   Use: dot -Tpng rustc_ecosystem_graph.dot -o graph.png");
    
    Ok(())
}

fn simplify_node_name(name: &str) -> String {
    if name.contains("rustc_") {
        name.split("::").next().unwrap_or(name).to_string()
    } else if name.contains("DefId") {
        "core_function".to_string()
    } else {
        name.chars().take(20).collect()
    }
}

fn generate_network_stats(edges: &[(String, String, usize)]) {
    let total_edges = edges.len();
    let unique_nodes: std::collections::HashSet<_> = edges.iter()
        .flat_map(|(u, v, _)| vec![u, v])
        .collect();
    
    println!("\n📈 Network Statistics:");
    println!("  Total edges: {}", total_edges);
    println!("  Unique nodes: {}", unique_nodes.len());
    println!("  Average connections per node: {:.2}", total_edges as f64 / unique_nodes.len() as f64);
    
    // Find rustc_driver centrality
    let driver_edges = edges.iter()
        .filter(|(u, v, _)| u.contains("rustc_driver") || v.contains("rustc_driver"))
        .count();
    
    println!("  Rustc driver centrality: {:.2}%", (driver_edges as f64 / total_edges as f64) * 100.0);
}
