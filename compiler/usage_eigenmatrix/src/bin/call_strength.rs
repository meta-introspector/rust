use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🎯 Emojiect Call Strength & Tree Analysis\n");
    
    // Load usage data to get call frequencies
    let mut call_counts = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage in usages {
                    let caller = usage["user_def_id"].as_str().unwrap_or("unknown");
                    let callee = usage["used_def_id"].as_str().unwrap_or("unknown");
                    let count = usage["usage_count"].as_u64().unwrap_or(1);
                    
                    let key = format!("{} → {}", clean_name(caller), clean_name(callee));
                    *call_counts.entry(key).or_insert(0) += count;
                }
            }
        }
    }
    
    // Sort by call strength
    let mut sorted_calls: Vec<_> = call_counts.into_iter().collect();
    sorted_calls.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("📊 Strongest Call Relationships (Top 20):");
    println!("Strength | Caller → Callee");
    println!("---------|------------------");
    
    for (call, count) in sorted_calls.iter().take(20) {
        println!("{:8} | {}", count, call);
    }
    
    println!("\n🌳 Tree Structure Analysis:");
    
    // Find nodes with most outgoing calls
    let mut caller_counts = HashMap::new();
    for (call, count) in &sorted_calls {
        if let Some(arrow_pos) = call.find(" → ") {
            let caller = call[..arrow_pos].to_string();
            *caller_counts.entry(caller).or_insert(0) += count;
        }
    }
    
    let mut sorted_callers: Vec<_> = caller_counts.into_iter().collect();
    sorted_callers.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("Top Callers (Tree Roots):");
    for (caller, total_calls) in sorted_callers.iter().take(10) {
        println!("  {} (total: {})", caller, total_calls);
    }
    
    // Find most called nodes (tree leaves/convergence points)
    let mut callee_counts = HashMap::new();
    for (call, count) in &sorted_calls {
        if let Some(arrow_pos) = call.find(" → ") {
            let callee = call[arrow_pos + " → ".len()..].to_string();
            *callee_counts.entry(callee).or_insert(0) += count;
        }
    }
    
    let mut sorted_callees: Vec<_> = callee_counts.into_iter().collect();
    sorted_callees.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\nMost Called (Convergence Points):");
    for (callee, total_calls) in sorted_callees.iter().take(10) {
        println!("  {} (called: {} times)", callee, total_calls);
    }
    
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
