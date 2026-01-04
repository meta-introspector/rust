use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("ast_patterns.json")?;
    let patterns: HashMap<String, u64> = serde_json::from_str(&data)?;
    
    // Extract call targets - what comes after "call::"
    let mut call_targets: HashMap<String, u64> = HashMap::new();
    let mut variable_calls: HashMap<String, u64> = HashMap::new();
    
    for (pattern, count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        
        // Find "call" and see what follows
        for i in 0..parts.len() {
            if parts[i] == "call" && i + 1 < parts.len() {
                let target = parts[i + 1];
                *call_targets.entry(target.to_string()).or_insert(0) += count;
                
                // Categorize call types
                match target {
                    "tcx" | "self" | "dcx" => {
                        *variable_calls.entry(format!("variable::{}", target)).or_insert(0) += count;
                    }
                    "Some" | "Ok" | "Err" | "None" => {
                        *variable_calls.entry(format!("constructor::{}", target)).or_insert(0) += count;
                    }
                    _ => {
                        *variable_calls.entry(format!("function::{}", target)).or_insert(0) += count;
                    }
                }
            }
        }
    }
    
    // Sort by frequency
    let mut sorted_targets: Vec<_> = call_targets.iter().collect();
    sorted_targets.sort_by(|a, b| b.1.cmp(a.1));
    
    let mut sorted_categories: Vec<_> = variable_calls.iter().collect();
    sorted_categories.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("🎯 Most Called Targets:");
    for (target, count) in sorted_targets.iter().take(20) {
        println!("  call::{} → {} times", target, count);
    }
    
    println!("\n📊 Call Categories:");
    for (category, count) in sorted_categories.iter().take(15) {
        println!("  {} → {} times", category, count);
    }
    
    // Find method calls on variables
    let mut method_calls: HashMap<String, u64> = HashMap::new();
    for (pattern, count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        
        // Look for variable::method patterns
        for i in 0..parts.len()-1 {
            let var = parts[i];
            let method = parts[i + 1];
            
            if ["tcx", "self", "dcx", "span", "def_id", "ty", "err"].contains(&var) {
                let method_pattern = format!("{}::{}", var, method);
                *method_calls.entry(method_pattern).or_insert(0) += count;
            }
        }
    }
    
    let mut sorted_methods: Vec<_> = method_calls.iter().collect();
    sorted_methods.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🔧 Variable Method Calls:");
    for (method_call, count) in sorted_methods.iter().take(15) {
        println!("  {} → {} times", method_call, count);
    }
    
    Ok(())
}
