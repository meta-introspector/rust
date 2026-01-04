use anyhow::Result;
use serde_json::Value;
use std::collections::{HashMap, VecDeque, HashSet};
use std::fs;

fn main() -> Result<()> {
    println!("🎯 Main Processing BFS Leaf Nodes\n");
    
    // Load graph
    let mut graph = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage in usages {
                    let caller = usage["user_def_id"].as_str().unwrap_or("unknown").to_string();
                    let callee = usage["used_def_id"].as_str().unwrap_or("unknown").to_string();
                    
                    graph.entry(caller).or_insert_with(Vec::new).push(callee);
                }
            }
        }
    }
    
    // Find main (highest eigenvalue)
    let eigenmatrix_data = fs::read_to_string("usage_eigenmatrix.json")?;
    let data: Value = serde_json::from_str(&eigenmatrix_data)?;
    
    let main_node = if let Some(core_def_ids) = data["core_def_ids"].as_array() {
        if let Some(first) = core_def_ids.first() {
            if let Some(node_data) = first.as_array() {
                node_data[0].as_str().unwrap_or("static META").to_string()
            } else {
                "static META".to_string()
            }
        } else {
            "static META".to_string()
        }
    } else {
        "static META".to_string()
    };
    
    println!("🚀 Main node: {}", clean_name(&main_node));
    
    // BFS to find leaf nodes
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut bfs_order = Vec::new();
    
    queue.push_back(main_node.clone());
    visited.insert(main_node.clone());
    
    while let Some(current) = queue.pop_front() {
        bfs_order.push(current.clone());
        
        if let Some(callees) = graph.get(&current) {
            for callee in callees.iter().take(2) {
                if !visited.contains(callee) {
                    visited.insert(callee.clone());
                    queue.push_back(callee.clone());
                }
            }
        }
        
        if bfs_order.len() >= 50 { break; }
    }
    
    // Find leaf nodes (nodes with no callees)
    let mut leaf_nodes = Vec::new();
    for node in &bfs_order {
        if let Some(callees) = graph.get(node) {
            if callees.is_empty() {
                leaf_nodes.push(node.clone());
            }
        } else {
            leaf_nodes.push(node.clone());
        }
    }
    
    println!("🌿 Found {} leaf nodes from BFS", leaf_nodes.len());
    
    // Main processes each leaf node
    println!("\n🔄 Main Processing Leaf Nodes:");
    println!("Format: Main(LeafInput) → Output\n");
    
    for (i, leaf) in leaf_nodes.iter().take(10).enumerate() {
        let output = if let Some(main_callees) = graph.get(&main_node) {
            if !main_callees.is_empty() {
                &main_callees[i % main_callees.len()]
            } else {
                "∅"
            }
        } else {
            "∅"
        };
        
        println!("Step {}: {}({}) → {}", 
                 i + 1,
                 clean_name(&main_node),
                 clean_name(leaf),
                 clean_name(output));
    }
    
    println!("\n🎯 Execution Complete!");
    println!("Main has processed {} leaf nodes from the BFS traversal", leaf_nodes.len().min(10));
    
    Ok(())
}

fn clean_name(name: &str) -> String {
    if name == "∅" { return "∅".to_string(); }
    
    if name.contains("DefId") {
        if let Some(start) = name.find("~ ") {
            if let Some(end) = name[start+2..].find(")") {
                return name[start+2..start+2+end].chars().take(15).collect();
            }
        }
    }
    name.chars().take(12).collect()
}
