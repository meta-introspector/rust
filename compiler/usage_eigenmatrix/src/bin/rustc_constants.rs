use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() -> Result<()> {
    println!("🔢 Numerical and String Constants in All Rustc Code");
    
    // Load all usage data
    let all_usages = load_all_usages()?;
    println!("📊 Analyzing {} usage entries", all_usages.len());
    
    // Extract constants
    let string_constants = find_string_constants(&all_usages);
    let numeric_constants = find_numeric_constants(&all_usages);
    
    println!("\n📝 Top 20 String Constants:");
    for (i, (constant, count)) in string_constants.iter().take(20).enumerate() {
        println!("{}. \"{}\" - {} times", i+1, constant, count);
    }
    
    println!("\n🔢 Top 20 Numeric Constants:");
    for (i, (constant, count)) in numeric_constants.iter().take(20).enumerate() {
        println!("{}. {} - {} times", i+1, constant, count);
    }
    
    Ok(())
}

fn load_all_usages() -> Result<Vec<Value>> {
    let mut usages = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if filename.ends_with(".json") {
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

fn find_string_constants(usages: &[Value]) -> Vec<(String, usize)> {
    let mut constants = HashMap::new();
    let string_regex = Regex::new(r#""([^"\\]*(\\.[^"\\]*)*)""#).unwrap();
    
    for usage in usages {
        let usage_text = usage["usage"].as_str().unwrap_or("");
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Extract string literals from usage text
        for cap in string_regex.captures_iter(usage_text) {
            if let Some(string_content) = cap.get(1) {
                let content = string_content.as_str();
                if !content.is_empty() && content.len() < 50 { // Reasonable length
                    *constants.entry(content.to_string()).or_insert(0) += 1;
                }
            }
        }
        
        // Extract from DefIds that might contain string constants
        if def_id.contains("\"") {
            for cap in string_regex.captures_iter(def_id) {
                if let Some(string_content) = cap.get(1) {
                    let content = string_content.as_str();
                    if !content.is_empty() && content.len() < 50 {
                        *constants.entry(content.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    let mut sorted: Vec<_> = constants.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_numeric_constants(usages: &[Value]) -> Vec<(String, usize)> {
    let mut constants = HashMap::new();
    let number_regex = Regex::new(r"\b(\d+(?:\.\d+)?)\b").unwrap();
    
    for usage in usages {
        let usage_text = usage["usage"].as_str().unwrap_or("");
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Extract numbers from usage text
        for cap in number_regex.captures_iter(usage_text) {
            if let Some(number) = cap.get(1) {
                let num_str = number.as_str();
                // Filter out DefId numbers and focus on actual constants
                if !usage_text.contains(&format!("DefId({}:", num_str)) {
                    *constants.entry(num_str.to_string()).or_insert(0) += 1;
                }
            }
        }
        
        // Extract from DefIds
        for cap in number_regex.captures_iter(def_id) {
            if let Some(number) = cap.get(1) {
                let num_str = number.as_str();
                if !def_id.contains(&format!("DefId({}:", num_str)) {
                    *constants.entry(num_str.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    let mut sorted: Vec<_> = constants.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}
