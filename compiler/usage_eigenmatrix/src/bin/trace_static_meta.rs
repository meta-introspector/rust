use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔍 Tracing static META calls\n");
    
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
    
    // Check what static META calls
    if let Some(callees) = graph.get("static META") {
        println!("static META calls {} functions:", callees.len());
        for (i, callee) in callees.iter().take(10).enumerate() {
            println!("  {}: {}", i, callee);
        }
    } else {
        println!("static META not found in graph!");
    }
    
    // Check if "event compiler" exists
    let event_compiler_variants: Vec<_> = graph.keys()
        .filter(|k| k.contains("event") && k.contains("compiler"))
        .collect();
    
    println!("\nFound {} 'event compiler' variants:", event_compiler_variants.len());
    for variant in event_compiler_variants.iter().take(5) {
        println!("  {}", variant);
    }
    
    Ok(())
}
