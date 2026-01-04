use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() -> Result<()> {
    println!("📊 Enhanced Literals Report from Usage Data");
    
    let mut string_literals = HashMap::new();
    let mut numeric_literals = HashMap::new();
    let mut boolean_literals = HashMap::new();
    let usage_dir = "../../usage_data";
    
    let string_regex = Regex::new(r#""([^"\\]*(\\.[^"\\]*)*)""#)?;
    let number_regex = Regex::new(r"\b(\d+(?:\.\d+)?)\b")?;
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let used_def_id = usage_obj["used_def_id"].as_str().unwrap_or("");
                    let usage_text = usage_obj["usage"].as_str().unwrap_or("");
                    
                    // Direct literal values in def_id
                    if used_def_id == "true" || used_def_id == "false" {
                        *boolean_literals.entry(used_def_id.to_string()).or_insert(0) += 1;
                    }
                    
                    // String literals in def_id (quoted strings)
                    if used_def_id.starts_with("\"") && used_def_id.ends_with("\"") && used_def_id.len() > 2 {
                        let content = &used_def_id[1..used_def_id.len()-1];
                        if content.len() < 100 { // Reasonable length filter
                            *string_literals.entry(content.to_string()).or_insert(0) += 1;
                        }
                    }
                    
                    // Numeric literals in def_id
                    if used_def_id.chars().all(|c| c.is_ascii_digit() || c == '.') && 
                       !used_def_id.is_empty() && used_def_id != "." {
                        *numeric_literals.entry(used_def_id.to_string()).or_insert(0) += 1;
                    }
                    
                    // Extract from usage text
                    for cap in string_regex.captures_iter(usage_text) {
                        if let Some(string_content) = cap.get(1) {
                            let content = string_content.as_str();
                            if !content.is_empty() && content.len() < 100 {
                                *string_literals.entry(content.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                    
                    for cap in number_regex.captures_iter(usage_text) {
                        if let Some(number) = cap.get(1) {
                            let num_str = number.as_str();
                            // Filter out DefId numbers
                            if !usage_text.contains(&format!("DefId({}:", num_str)) &&
                               !usage_text.contains(&format!("[{}]", num_str)) {
                                *numeric_literals.entry(num_str.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort all categories
    let mut sorted_strings: Vec<_> = string_literals.into_iter().collect();
    sorted_strings.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut sorted_numbers: Vec<_> = numeric_literals.into_iter().collect();
    sorted_numbers.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut sorted_booleans: Vec<_> = boolean_literals.into_iter().collect();
    sorted_booleans.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n📝 Top 15 String Literals:");
    for (i, (literal, count)) in sorted_strings.iter().take(15).enumerate() {
        println!("{}. \"{}\" - {} occurrences", i+1, literal, count);
    }
    
    println!("\n🔢 Top 15 Numeric Literals:");
    for (i, (literal, count)) in sorted_numbers.iter().take(15).enumerate() {
        println!("{}. {} - {} occurrences", i+1, literal, count);
    }
    
    println!("\n✅ Boolean Literals:");
    for (literal, count) in sorted_booleans.iter() {
        println!("  {} - {} occurrences", literal, count);
    }
    
    println!("\n📈 Summary:");
    println!("  String literals: {}", sorted_strings.len());
    println!("  Numeric literals: {}", sorted_numbers.len());
    println!("  Boolean literals: {}", sorted_booleans.len());
    
    Ok(())
}
