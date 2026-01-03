use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("📊 Top Literals Report from Real Usage Data");
    
    let mut literals = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let usage_text = usage_obj["usage"].as_str().unwrap_or("");
                    let usage_type = usage_obj["usage_type"].as_str().unwrap_or("");
                    
                    // Look for literal patterns
                    if usage_type == "Literal" || usage_text.contains("LITERAL_VALUE") {
                        let used_def_id = usage_obj["used_def_id"].as_str().unwrap_or("");
                        *literals.entry(used_def_id.to_string()).or_insert(0) += 1;
                    }
                    
                    // Also check for common literal patterns in usage text
                    if usage_text.contains("Int(") || usage_text.contains("Bool(") || 
                       usage_text.contains("Str(") || usage_text.contains("Float(") {
                        // Extract the literal part
                        if let Some(start) = usage_text.find("(") {
                            if let Some(end) = usage_text[start..].find(")") {
                                let literal = &usage_text[start+1..start+end];
                                *literals.entry(literal.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    let mut sorted: Vec<_> = literals.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🔥 Top 20 Literals Found in Real Rustc Code:");
    for (i, (literal, count)) in sorted.iter().take(20).enumerate() {
        println!("{}. {} - {} occurrences", i+1, literal, count);
    }
    
    println!("\n📈 Total unique literals: {}", sorted.len());
    
    Ok(())
}
