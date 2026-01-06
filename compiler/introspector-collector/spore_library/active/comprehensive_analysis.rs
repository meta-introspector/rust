use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;

#[derive(Debug)]
struct ObjectUsage {
    symbol: String,
    total_usage: usize,
    used_fields: HashSet<String>,
    crates: HashSet<String>,
    usage_types: HashSet<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Comprehensive Usage Analysis - Deduplication & Field Coverage");
    println!("================================================================");
    
    let mut all_objects: HashMap<String, ObjectUsage> = HashMap::new();
    let mut file_count = 0;
    let mut total_usages = 0;
    
    // Process all usage files
    if let Ok(entries) = fs::read_dir("../../test_usage_data") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") 
                && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                
                file_count += 1;
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        let crate_name = data.get("crate")
                            .and_then(|c| c.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                            
                        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                            for usage in usages {
                                total_usages += 1;
                                
                                if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                    let usage_count = usage.get("usage_count")
                                        .and_then(|c| c.as_u64())
                                        .unwrap_or(1) as usize;
                                    
                                    let usage_type = usage.get("usage_type")
                                        .and_then(|t| t.as_str())
                                        .unwrap_or("unknown")
                                        .to_string();
                                    
                                    let entry = all_objects.entry(symbol.to_string()).or_insert_with(|| {
                                        ObjectUsage {
                                            symbol: symbol.to_string(),
                                            total_usage: 0,
                                            used_fields: HashSet::new(),
                                            crates: HashSet::new(),
                                            usage_types: HashSet::new(),
                                        }
                                    });
                                    
                                    entry.total_usage += usage_count;
                                    entry.crates.insert(crate_name.clone());
                                    entry.usage_types.insert(usage_type);
                                    
                                    // Extract field usage from the usage object
                                    for (key, _) in usage.as_object().unwrap_or(&serde_json::Map::new()) {
                                        if key != "symbol" && key != "usage_count" {
                                            entry.used_fields.insert(key.clone());
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
    
    println!("📊 Processing complete:");
    println!("   Files processed: {}", file_count);
    println!("   Total usage entries: {}", total_usages);
    println!("   Unique symbols: {}", all_objects.len());
    
    // Sort by usage frequency
    let mut sorted_objects: Vec<_> = all_objects.values().collect();
    sorted_objects.sort_by(|a, b| b.total_usage.cmp(&a.total_usage));
    
    // Analyze field coverage
    let mut all_possible_fields = HashSet::new();
    for obj in &sorted_objects {
        for field in &obj.used_fields {
            all_possible_fields.insert(field.clone());
        }
    }
    
    println!("\n🔍 FIELD COVERAGE ANALYSIS:");
    println!("All possible fields found: {:?}", all_possible_fields);
    
    // Find objects with partial field usage
    let mut partial_usage_objects = Vec::new();
    for obj in &sorted_objects {
        let coverage = (obj.used_fields.len() as f64 / all_possible_fields.len() as f64) * 100.0;
        if coverage > 20.0 && coverage < 90.0 && obj.total_usage > 5 {
            partial_usage_objects.push((obj, coverage));
        }
    }
    
    println!("\n🎯 TOP USED SYMBOLS (Deduplicated):");
    for (i, obj) in sorted_objects.iter().take(20).enumerate() {
        println!("  {:2}. {:6} {} (in {} crates, {} types)", 
                 i + 1, obj.total_usage, obj.symbol, obj.crates.len(), obj.usage_types.len());
    }
    
    println!("\n🔧 PARTIALLY USED OBJECTS (Candidates for more JSON fields):");
    partial_usage_objects.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    for (obj, coverage) in partial_usage_objects.iter().take(15) {
        let missing_fields: Vec<_> = all_possible_fields.difference(&obj.used_fields).collect();
        println!("\n  📋 {} (usage: {}, coverage: {:.1}%)", obj.symbol, obj.total_usage, coverage);
        println!("     Used fields: {:?}", obj.used_fields);
        println!("     Missing fields: {:?}", missing_fields);
        println!("     Crates: {:?}", obj.crates);
        println!("     Usage types: {:?}", obj.usage_types);
    }
    
    // Generate recommendations
    println!("\n💡 RECOMMENDATIONS FOR JSON DUMP ENHANCEMENT:");
    println!("============================================");
    
    let high_usage_partial: Vec<_> = partial_usage_objects.iter()
        .filter(|(obj, _)| obj.total_usage > 20)
        .take(10)
        .collect();
    
    for (obj, coverage) in &high_usage_partial {
        let missing_fields: Vec<_> = all_possible_fields.difference(&obj.used_fields).collect();
        println!("\n🎯 HIGH PRIORITY: {}", obj.symbol);
        println!("   Usage: {} times across {} crates", obj.total_usage, obj.crates.len());
        println!("   Current coverage: {:.1}%", coverage);
        println!("   Add these fields to JSON dump: {:?}", missing_fields);
    }
    
    // Summary statistics
    let avg_usage = sorted_objects.iter().map(|o| o.total_usage).sum::<usize>() as f64 / sorted_objects.len() as f64;
    let high_usage_count = sorted_objects.iter().filter(|o| o.total_usage > 10).count();
    let multi_crate_count = sorted_objects.iter().filter(|o| o.crates.len() > 1).count();
    
    println!("\n📈 SUMMARY STATISTICS:");
    println!("   Average usage per symbol: {:.1}", avg_usage);
    println!("   High usage symbols (>10): {}", high_usage_count);
    println!("   Multi-crate symbols: {}", multi_crate_count);
    println!("   Partial coverage candidates: {}", partial_usage_objects.len());
    
    Ok(())
}
