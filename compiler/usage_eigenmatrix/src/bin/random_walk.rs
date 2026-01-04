use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use rand::Rng;

fn main() -> Result<()> {
    println!("🎲 Random Walk: Graph Processing Graph as Input\n");
    
    // Load the graph
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
    
    let nodes: Vec<String> = graph.keys().cloned().collect();
    println!("📊 Graph: {} nodes loaded", nodes.len());
    
    // Random walk: each step picks a random processor and random input
    let mut rng = rand::thread_rng();
    
    println!("\n🔄 Random Walk Execution:");
    println!("Format: Processor(Input) → Output");
    
    for step in 1..=15 {
        // Pick random processor (function from graph)
        let processor_idx = rng.gen_range(0..nodes.len());
        let processor = &nodes[processor_idx];
        
        // Pick random input (another node from graph)  
        let input_idx = rng.gen_range(0..nodes.len());
        let input = &nodes[input_idx];
        
        // Process: if processor has callees, pick one as output
        let output = if let Some(callees) = graph.get(processor) {
            if !callees.is_empty() {
                let output_idx = rng.gen_range(0..callees.len());
                &callees[output_idx]
            } else {
                "∅"
            }
        } else {
            "∅"
        };
        
        println!("Step {}: {}({}) → {}", 
                 step,
                 clean_name(processor),
                 clean_name(input), 
                 clean_name(output));
    }
    
    println!("\n🎯 Self-Processing Demonstration:");
    println!("The graph is literally processing itself as input!");
    println!("Each step: Graph Node processes Graph Node → Graph Node");
    
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
