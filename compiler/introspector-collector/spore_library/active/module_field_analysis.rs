use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;

#[derive(Debug)]
struct FieldUsage {
    used_by_modules: HashSet<String>,
    not_used_by_modules: HashSet<String>,
    total_usage: usize,
}

#[derive(Debug)]
struct ObjectAnalysis {
    symbol: String,
    total_usage: usize,
    field_usage: HashMap<String, FieldUsage>,
    all_modules: HashSet<String>,
    usage_types: HashSet<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Module-Based Field Usage Analysis");
    println!("===================================");
    
    let mut all_objects: HashMap<String, ObjectAnalysis> = HashMap::new();
    let mut all_modules_global = HashSet::new();
    let mut file_count = 0;
    
    // Process all usage files
    if let Ok(entries) = fs::read_dir("../../test_usage_data") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") 
                && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                
                file_count += 1;
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        let module_name = data.get("module")
                            .and_then(|m| m.as_str())
                            .or_else(|| data.get("crate").and_then(|c| c.as_str()))
                            .unwrap_or("unknown")
                            .to_string();
                        
                        all_modules_global.insert(module_name.clone());
                            
                        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                            for usage in usages {
                                if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                    let usage_count = usage.get("usage_count")
                                        .and_then(|c| c.as_u64())
                                        .unwrap_or(1) as usize;
                                    
                                    let usage_type = usage.get("usage_type")
                                        .and_then(|t| t.as_str())
                                        .unwrap_or("unknown")
                                        .to_string();
                                    
                                    let entry = all_objects.entry(symbol.to_string()).or_insert_with(|| {
                                        ObjectAnalysis {
                                            symbol: symbol.to_string(),
                                            total_usage: 0,
                                            field_usage: HashMap::new(),
                                            all_modules: HashSet::new(),
                                            usage_types: HashSet::new(),
                                        }
                                    });
                                    
                                    entry.total_usage += usage_count;
                                    entry.all_modules.insert(module_name.clone());
                                    entry.usage_types.insert(usage_type);
                                    
                                    // Track field usage by module
                                    for (field_name, field_value) in usage.as_object().unwrap_or(&serde_json::Map::new()) {
                                        if field_name != "symbol" && field_name != "usage_count" {
                                            let field_entry = entry.field_usage.entry(field_name.clone()).or_insert_with(|| {
                                                FieldUsage {
                                                    used_by_modules: HashSet::new(),
                                                    not_used_by_modules: HashSet::new(),
                                                    total_usage: 0,
                                                }
                                            });
                                            
                                            if !field_value.is_null() && field_value != &Value::String("".to_string()) {
                                                field_entry.used_by_modules.insert(module_name.clone());
                                                field_entry.total_usage += usage_count;
                                            } else {
                                                field_entry.not_used_by_modules.insert(module_name.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fill in missing modules for each field
    for obj in all_objects.values_mut() {
        for field_usage in obj.field_usage.values_mut() {
            for module in &all_modules_global {
                if !field_usage.used_by_modules.contains(module) && !field_usage.not_used_by_modules.contains(module) {
                    field_usage.not_used_by_modules.insert(module.clone());
                }
            }
        }
    }
    
    println!("📊 Processing complete:");
    println!("   Files processed: {}", file_count);
    println!("   Total modules: {}", all_modules_global.len());
    println!("   Unique symbols: {}", all_objects.len());
    
    // Sort by usage frequency
    let mut sorted_objects: Vec<_> = all_objects.values().collect();
    sorted_objects.sort_by(|a, b| b.total_usage.cmp(&a.total_usage));
    
    println!("\n🏗️  TOP SYMBOLS WITH FIELD USAGE BY MODULE:");
    for obj in sorted_objects.iter().take(15) {
        println!("\n📋 {} (total usage: {}, in {} modules)", 
                 obj.symbol, obj.total_usage, obj.all_modules.len());
        
        for (field_name, field_usage) in &obj.field_usage {
            let usage_rate = (field_usage.used_by_modules.len() as f64 / all_modules_global.len() as f64) * 100.0;
            println!("  └─ {}: used by {} modules ({:.1}%), not used by {} modules", 
                     field_name, 
                     field_usage.used_by_modules.len(), 
                     usage_rate,
                     field_usage.not_used_by_modules.len());
            
            if field_usage.used_by_modules.len() <= 3 && !field_usage.used_by_modules.is_empty() {
                println!("     Used by: {:?}", field_usage.used_by_modules);
            }
            if field_usage.not_used_by_modules.len() <= 3 && !field_usage.not_used_by_modules.is_empty() {
                println!("     Not used by: {:?}", field_usage.not_used_by_modules);
            }
        }
    }
    
    // Find fields with low usage across modules
    println!("\n🎯 UNDERUTILIZED FIELDS (used by <50% of modules):");
    for obj in sorted_objects.iter().take(10) {
        let mut underutilized_fields = Vec::new();
        for (field_name, field_usage) in &obj.field_usage {
            let usage_rate = (field_usage.used_by_modules.len() as f64 / all_modules_global.len() as f64) * 100.0;
            if usage_rate < 50.0 && field_usage.total_usage > 0 {
                underutilized_fields.push((field_name, usage_rate, &field_usage.used_by_modules));
            }
        }
        
        if !underutilized_fields.is_empty() {
            println!("\n  📋 {}:", obj.symbol);
            for (field, rate, modules) in underutilized_fields {
                println!("    └─ {}: {:.1}% usage, only in {:?}", field, rate, modules);
            }
        }
    }
    
    println!("\n📈 MODULE SUMMARY:");
    println!("   All modules: {:?}", all_modules_global);
    
    Ok(())
}
