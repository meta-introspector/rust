use anyhow::Result;
use serde_json::Value;
use std::collections::{HashMap, VecDeque, HashSet};
use std::fs;

fn main() -> Result<()> {
    println!("🎯 Simple BFS from Main Routine\n");
    
    // Load the eigenmatrix data
    let eigenmatrix_data = fs::read_to_string("usage_eigenmatrix.json")?;
    let data: Value = serde_json::from_str(&eigenmatrix_data)?;
    
    // Get the top nodes (highest eigenvalues)
    let mut nodes = Vec::new();
    if let Some(core_def_ids) = data["core_def_ids"].as_array() {
        for node_array in core_def_ids {
            if let Some(node_data) = node_array.as_array() {
                if node_data.len() >= 2 {
                    let name = node_data[0].as_str().unwrap_or("unknown").to_string();
                    let usage_count = node_data[1].as_u64().unwrap_or(0);
                    nodes.push((name, usage_count));
                }
            }
        }
    }
    
    // Assign emojis to nodes
    let emojis = vec!["👑", "⚡", "🔍", "🔘", "🔢", "📝", "🔄", "💾", "⚖️", "🎨", "⚠️", "🔧", "🪄", "📦", "🎮", "❓"];
    let mut node_emojis = HashMap::new();
    
    for (i, (name, _)) in nodes.iter().enumerate() {
        let emoji = emojis.get(i).unwrap_or(&"❓");
        node_emojis.insert(name.clone(), emoji.to_string());
    }
    
    // Build adjacency list from usage data
    let mut graph = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let usage_data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = usage_data["usages"].as_array() {
                for usage in usages {
                    let caller = usage["user_def_id"].as_str().unwrap_or("unknown");
                    let callee = usage["used_def_id"].as_str().unwrap_or("unknown");
                    
                    graph.entry(caller.to_string())
                        .or_insert_with(Vec::new)
                        .push(callee.to_string());
                }
            }
        }
    }
    
    // Start BFS from the main routine (highest eigenvalue node)
    let start_node = nodes.first().map(|(name, _)| name.clone()).unwrap_or("false".to_string());
    
    println!("Starting BFS from: {}\n", clean_name(&start_node));
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut order = 0;
    
    queue.push_back(start_node.clone());
    visited.insert(start_node.clone());
    
    println!("Order | Emoji | Name                          | Calls");
    println!("------|-------|-------------------------------|-------");
    
    while let Some(current) = queue.pop_front() {
        let default_emoji = "❓".to_string();
        let emoji = node_emojis.get(&current).unwrap_or(&default_emoji);
        let clean_current = clean_name(&current);
        
        // Show what this node calls
        let mut calls_display = String::new();
        if let Some(neighbors) = graph.get(&current) {
            for (i, neighbor) in neighbors.iter().take(3).enumerate() {
                let default_neighbor_emoji = "❓".to_string();
                let neighbor_emoji = node_emojis.get(neighbor).unwrap_or(&default_neighbor_emoji);
                if i > 0 { calls_display.push_str(" "); }
                calls_display.push_str(neighbor_emoji);
            }
        }
        
        println!("{:5} | {:5} | {:29} | {}", order, emoji, 
                 clean_current.chars().take(29).collect::<String>(), calls_display);
        order += 1;
        
        // Add neighbors to queue
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors.iter() { // No limit - show all
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
        
        // No order limit - show everything
    }
    
    println!("\n🎭 BFS Complete - Showing compilation order from main routine");
    println!("👑 = static META (highest eigenvalue - the ultimate truth)");
    println!("⚡ = false (second highest - computational power)");
    println!("🔍 = core::fmt functions (discovery/formatting)");
    println!("🔘 = iterator patterns (structure recognition)");
    println!("Each emoji represents a computational 'emojiect' in Rust's bootstrap process!");
    
    Ok(())
}

fn clean_name(name: &str) -> String {
    if name.contains("DefId") {
        if let Some(start) = name.find("~ ") {
            if let Some(end) = name[start+2..].find(")") {
                return name[start+2..start+2+end].chars().take(40).collect();
            }
        }
    }
    name.chars().take(30).collect()
}
