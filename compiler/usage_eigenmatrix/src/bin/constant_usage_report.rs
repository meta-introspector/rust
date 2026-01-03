use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("📊 Constant Usage Report for Our Collector");
    
    // Load our collector's usage data
    let our_usages = load_our_usages()?;
    
    // Analyze constant patterns
    let constants = find_constants(&our_usages);
    let literals = find_literals(&our_usages);
    let static_refs = find_static_references(&our_usages);
    
    println!("\n🔢 Constants Used:");
    for (constant, count) in constants.iter().take(10) {
        println!("  {} - {} times", constant, count);
    }
    
    println!("\n📝 Literals Used:");
    for (literal, count) in literals.iter().take(10) {
        println!("  {} - {} times", literal, count);
    }
    
    println!("\n🏗️ Static References:");
    for (static_ref, count) in static_refs.iter().take(10) {
        println!("  {} - {} times", static_ref, count);
    }
    
    // Find most used constant patterns
    println!("\n💡 Most Common Constant Patterns:");
    analyze_constant_patterns(&our_usages);
    
    Ok(())
}

fn load_our_usages() -> Result<Vec<Value>> {
    let mut usages = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if filename.starts_with("working_usage_collector") && filename.ends_with(".json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usage_array) = data["usages"].as_array() {
                for usage in usage_array {
                    usages.push(usage.clone());
                }
            }
        }
    }
    
    Ok(usages)
}

fn find_constants(usages: &[Value]) -> Vec<(String, usize)> {
    let mut constants = HashMap::new();
    
    for usage in usages {
        let usage_text = usage["usage"].as_str().unwrap_or("");
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Look for constant patterns
        if def_id.contains("::new") || def_id.contains("::default") {
            *constants.entry(def_id.to_string()).or_insert(0) += 1;
        }
        
        if usage_text.contains("USES") {
            // Extract what's being used
            if let Some(uses_part) = usage_text.split("USES").nth(1) {
                if let Some(constant) = uses_part.split("(").next() {
                    *constants.entry(constant.trim().to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    let mut sorted: Vec<_> = constants.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_literals(usages: &[Value]) -> Vec<(String, usize)> {
    let mut literals = HashMap::new();
    
    for usage in usages {
        let usage_text = usage["usage"].as_str().unwrap_or("");
        
        // Look for string literals, numbers, etc.
        if usage_text.contains("\"") {
            // Extract string literals
            let parts: Vec<&str> = usage_text.split("\"").collect();
            for i in (1..parts.len()).step_by(2) {
                if !parts[i].is_empty() {
                    *literals.entry(format!("\"{}\"", parts[i])).or_insert(0) += 1;
                }
            }
        }
    }
    
    let mut sorted: Vec<_> = literals.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_static_references(usages: &[Value]) -> Vec<(String, usize)> {
    let mut statics = HashMap::new();
    
    for usage in usages {
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Look for static/const references
        if def_id.contains("static") || def_id.contains("CONST") || def_id.to_uppercase() == def_id {
            *statics.entry(def_id.to_string()).or_insert(0) += 1;
        }
    }
    
    let mut sorted: Vec<_> = statics.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn analyze_constant_patterns(usages: &[Value]) {
    let mut patterns = HashMap::new();
    
    for usage in usages {
        let usage_type = usage["usage_type"].as_str().unwrap_or("");
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        if usage_type == "MethodCall" && def_id.contains("::new") {
            patterns.entry("Constructor calls".to_string()).or_insert(Vec::new()).push(def_id);
        }
        
        if def_id.contains("std::") {
            patterns.entry("Standard library".to_string()).or_insert(Vec::new()).push(def_id);
        }
        
        if def_id.contains("core::") {
            patterns.entry("Core library".to_string()).or_insert(Vec::new()).push(def_id);
        }
    }
    
    for (pattern, items) in patterns {
        println!("  {} - {} items", pattern, items.len());
    }
}
