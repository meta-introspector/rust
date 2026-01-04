use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("📊 Enhanced Constant Usage Analysis");
    
    // Load all usage data from the expanded collection
    let all_usages = load_all_usages()?;
    println!("📈 Analyzing {} usage entries from expanded dataset", all_usages.len());
    
    // Analyze different types of constants
    let core_constants = find_core_constants(&all_usages);
    let literal_constants = find_literal_constants(&all_usages);
    let static_constants = find_static_constants(&all_usages);
    let enum_constants = find_enum_constants(&all_usages);
    
    println!("\n🎯 Top 15 Core Constants (Option, Result, etc.):");
    for (i, (constant, count)) in core_constants.iter().take(15).enumerate() {
        println!("{}. {} - {} times", i+1, constant, count);
    }
    
    println!("\n📝 Top 15 Literal Constants:");
    for (i, (literal, count)) in literal_constants.iter().take(15).enumerate() {
        println!("{}. {} - {} times", i+1, literal, count);
    }
    
    println!("\n🏗️ Top 15 Static Constants:");
    for (i, (static_ref, count)) in static_constants.iter().take(15).enumerate() {
        println!("{}. {} - {} times", i+1, static_ref, count);
    }
    
    println!("\n🔢 Top 15 Enum Constants:");
    for (i, (enum_const, count)) in enum_constants.iter().take(15).enumerate() {
        println!("{}. {} - {} times", i+1, enum_const, count);
    }
    
    // Analyze constant usage patterns
    println!("\n💡 Constant Usage Patterns:");
    analyze_usage_patterns(&all_usages);
    
    Ok(())
}

fn load_all_usages() -> Result<Vec<Value>> {
    let mut usages = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        // Load all JSON files from our expanded collection
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

fn find_core_constants(usages: &[Value]) -> Vec<(String, usize)> {
    let mut constants = HashMap::new();
    
    for usage in usages {
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Core Rust constants like Option::None, Option::Some, Result::Ok, etc.
        if def_id.contains("::option::Option::") ||
           def_id.contains("::result::Result::") ||
           def_id.contains("::bool::") ||
           def_id == "false" || def_id == "true" ||
           def_id == "_" {
            *constants.entry(def_id.to_string()).or_insert(0) += 1;
        }
        
        // Format-related constants
        if def_id.contains("::fmt::") && (def_id.contains("new_") || def_id.contains("Display")) {
            *constants.entry(def_id.to_string()).or_insert(0) += 1;
        }
    }
    
    let mut sorted: Vec<_> = constants.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_literal_constants(usages: &[Value]) -> Vec<(String, usize)> {
    let mut literals = HashMap::new();
    
    for usage in usages {
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Look for literal values
        if def_id.starts_with("\"") && def_id.ends_with("\"") {
            *literals.entry(def_id.to_string()).or_insert(0) += 1;
        }
        
        // Numeric literals
        if def_id.chars().all(|c| c.is_ascii_digit() || c == '.') && !def_id.is_empty() {
            *literals.entry(def_id.to_string()).or_insert(0) += 1;
        }
    }
    
    let mut sorted: Vec<_> = literals.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_static_constants(usages: &[Value]) -> Vec<(String, usize)> {
    let mut statics = HashMap::new();
    
    for usage in usages {
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Static references (usually ALL_CAPS or contain "static")
        if (def_id.chars().all(|c| c.is_ascii_uppercase() || c == '_') && def_id.len() > 1) ||
           def_id.contains("static ") ||
           def_id == "META" || def_id == "__CALLSITE" {
            *statics.entry(def_id.to_string()).or_insert(0) += 1;
        }
    }
    
    let mut sorted: Vec<_> = statics.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_enum_constants(usages: &[Value]) -> Vec<(String, usize)> {
    let mut enums = HashMap::new();
    
    for usage in usages {
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Enum variants (contain :: and end with a capitalized name)
        if def_id.contains("::") {
            let parts: Vec<&str> = def_id.split("::").collect();
            if let Some(last_part) = parts.last() {
                if last_part.chars().next().map_or(false, |c| c.is_ascii_uppercase()) &&
                   !last_part.contains("(") && !last_part.contains("{") {
                    *enums.entry(def_id.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    let mut sorted: Vec<_> = enums.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn analyze_usage_patterns(usages: &[Value]) {
    let mut crate_usage = HashMap::new();
    let mut method_patterns = HashMap::new();
    let mut trait_patterns = HashMap::new();
    
    for usage in usages {
        let def_id = usage["used_def_id"].as_str().unwrap_or("");
        
        // Analyze by crate
        if def_id.contains("core[") {
            *crate_usage.entry("core".to_string()).or_insert(0) += 1;
        } else if def_id.contains("std[") {
            *crate_usage.entry("std".to_string()).or_insert(0) += 1;
        } else if def_id.contains("rustc_") {
            *crate_usage.entry("rustc_*".to_string()).or_insert(0) += 1;
        }
        
        // Analyze method patterns
        if def_id.contains("::new") {
            *method_patterns.entry("Constructor (::new)".to_string()).or_insert(0) += 1;
        } else if def_id.contains("::into_iter") {
            *method_patterns.entry("Iterator (::into_iter)".to_string()).or_insert(0) += 1;
        } else if def_id.contains("::next") {
            *method_patterns.entry("Iterator (::next)".to_string()).or_insert(0) += 1;
        } else if def_id.contains("::eq") || def_id.contains("::le") {
            *method_patterns.entry("Comparison traits".to_string()).or_insert(0) += 1;
        }
        
        // Analyze trait patterns
        if def_id.contains("PartialEq") || def_id.contains("PartialOrd") {
            *trait_patterns.entry("Comparison traits".to_string()).or_insert(0) += 1;
        } else if def_id.contains("Iterator") || def_id.contains("IntoIterator") {
            *trait_patterns.entry("Iterator traits".to_string()).or_insert(0) += 1;
        } else if def_id.contains("Display") || def_id.contains("Debug") {
            *trait_patterns.entry("Formatting traits".to_string()).or_insert(0) += 1;
        }
    }
    
    println!("\n📦 Usage by Crate:");
    let mut sorted_crates: Vec<_> = crate_usage.into_iter().collect();
    sorted_crates.sort_by(|a, b| b.1.cmp(&a.1));
    for (crate_name, count) in sorted_crates.iter().take(5) {
        println!("  {} - {} usages", crate_name, count);
    }
    
    println!("\n🔧 Method Patterns:");
    let mut sorted_methods: Vec<_> = method_patterns.into_iter().collect();
    sorted_methods.sort_by(|a, b| b.1.cmp(&a.1));
    for (pattern, count) in sorted_methods.iter().take(5) {
        println!("  {} - {} usages", pattern, count);
    }
    
    println!("\n🎭 Trait Patterns:");
    let mut sorted_traits: Vec<_> = trait_patterns.into_iter().collect();
    sorted_traits.sort_by(|a, b| b.1.cmp(&a.1));
    for (pattern, count) in sorted_traits.iter().take(5) {
        println!("  {} - {} usages", pattern, count);
    }
}
