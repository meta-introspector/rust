use anyhow::Result;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<()> {
    println!("🔄 Finding Loops in the Call Graph\n");
    
    let mut graph = HashMap::new();
    let usage_dir = "../../usage_data";
    
    // Build call graph
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
    
    println!("📊 Graph loaded: {} nodes", graph.len());
    
    // Find self-loops (A calls A)
    println!("\n🔄 Self-Loops (A → A):");
    let mut self_loops = 0;
    for (node, callees) in &graph {
        if callees.contains(node) {
            println!("  {} calls itself", clean_name(node));
            self_loops += 1;
            if self_loops >= 10 { break; }
        }
    }
    
    // Find 2-cycles (A calls B, B calls A)
    println!("\n🔄 2-Cycles (A → B → A):");
    let mut two_cycles = 0;
    for (node_a, callees_a) in &graph {
        for node_b in callees_a {
            if let Some(callees_b) = graph.get(node_b) {
                if callees_b.contains(node_a) && node_a != node_b {
                    println!("  {} ↔ {}", clean_name(node_a), clean_name(node_b));
                    two_cycles += 1;
                    if two_cycles >= 10 { break; }
                }
            }
        }
        if two_cycles >= 10 { break; }
    }
    
    // Find 3-cycles (A → B → C → A)
    println!("\n🔄 3-Cycles (A → B → C → A):");
    let mut three_cycles = 0;
    for (node_a, callees_a) in &graph {
        for node_b in callees_a {
            if let Some(callees_b) = graph.get(node_b) {
                for node_c in callees_b {
                    if let Some(callees_c) = graph.get(node_c) {
                        if callees_c.contains(node_a) && node_a != node_b && node_b != node_c && node_a != node_c {
                            println!("  {} → {} → {} → {}", 
                                     clean_name(node_a), clean_name(node_b), 
                                     clean_name(node_c), clean_name(node_a));
                            three_cycles += 1;
                            if three_cycles >= 5 { break; }
                        }
                    }
                }
                if three_cycles >= 5 { break; }
            }
        }
        if three_cycles >= 5 { break; }
    }
    
    println!("\n📊 Loop Summary:");
    println!("  Self-loops found: {}", self_loops);
    println!("  2-cycles found: {}", two_cycles);
    println!("  3-cycles found: {}", three_cycles);
    
    Ok(())
}

fn clean_name(name: &str) -> String {
    if name.contains("DefId") {
        if let Some(start) = name.find("~ ") {
            if let Some(end) = name[start+2..].find(")") {
                return name[start+2..start+2+end].chars().take(25).collect();
            }
        }
    }
    name.chars().take(20).collect()
}
