use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 AST/HIR Usage Analysis - working_usage_collector vs Others");
    println!("=============================================================");
    
    let mut collector_symbols = HashSet::new();
    let mut other_symbols = HashMap::new(); // module -> symbols
    let mut all_usage_types = HashSet::new();
    let mut all_node_types = HashSet::new();
    
    // Process all usage files
    if let Ok(entries) = fs::read_dir("../../test_usage_data") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") 
                && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        let module_name = data.get("module")
                            .and_then(|m| m.as_str())
                            .or_else(|| data.get("crate").and_then(|c| c.as_str()))
                            .unwrap_or("unknown")
                            .to_string();
                        
                        let is_collector = module_name == "working_usage_collector";
                        
                        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                            for usage in usages {
                                if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                    let usage_type = usage.get("usage_type")
                                        .and_then(|t| t.as_str())
                                        .unwrap_or("unknown");
                                    let node_type = usage.get("node_type")
                                        .and_then(|t| t.as_str())
                                        .unwrap_or("unknown");
                                    
                                    all_usage_types.insert(usage_type.to_string());
                                    all_node_types.insert(node_type.to_string());
                                    
                                    if is_collector {
                                        collector_symbols.insert(symbol.to_string());
                                    } else {
                                        other_symbols.entry(module_name.clone())
                                            .or_insert_with(HashSet::new)
                                            .insert(symbol.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Find symbols used by others but not collector
    let mut all_other_symbols = HashSet::new();
    for symbols in other_symbols.values() {
        all_other_symbols.extend(symbols.iter().cloned());
    }
    
    let missing_in_collector: Vec<_> = all_other_symbols.difference(&collector_symbols).collect();
    let collector_only: Vec<_> = collector_symbols.difference(&all_other_symbols).collect();
    let common_symbols: Vec<_> = collector_symbols.intersection(&all_other_symbols).collect();
    
    println!("📊 Analysis Results:");
    println!("   working_usage_collector symbols: {}", collector_symbols.len());
    println!("   Other modules symbols: {}", all_other_symbols.len());
    println!("   Common symbols: {}", common_symbols.len());
    println!("   Missing in collector: {}", missing_in_collector.len());
    println!("   Collector-only symbols: {}", collector_only.len());
    
    println!("\n🎯 SYMBOLS MISSING IN working_usage_collector (expansion opportunities):");
    let mut missing_sorted: Vec<_> = missing_in_collector.into_iter().collect();
    missing_sorted.sort();
    for (i, symbol) in missing_sorted.iter().take(30).enumerate() {
        // Find which modules use this symbol
        let mut using_modules = Vec::new();
        for (module, symbols) in &other_symbols {
            if symbols.contains(*symbol) {
                using_modules.push(module);
            }
        }
        println!("  {:2}. {} (used by: {:?})", i + 1, symbol, using_modules);
    }
    
    println!("\n🔧 COLLECTOR-ONLY SYMBOLS (unique to working_usage_collector):");
    let mut collector_sorted: Vec<_> = collector_only.into_iter().collect();
    collector_sorted.sort();
    for (i, symbol) in collector_sorted.iter().take(15).enumerate() {
        println!("  {:2}. {}", i + 1, symbol);
    }
    
    println!("\n📋 ALL USAGE TYPES FOUND:");
    let mut usage_types_sorted: Vec<_> = all_usage_types.into_iter().collect();
    usage_types_sorted.sort();
    for usage_type in &usage_types_sorted {
        println!("  - {}", usage_type);
    }
    
    println!("\n📋 ALL NODE TYPES FOUND:");
    let mut node_types_sorted: Vec<_> = all_node_types.into_iter().collect();
    node_types_sorted.sort();
    for node_type in &node_types_sorted {
        println!("  - {}", node_type);
    }
    
    // Analyze by module coverage
    println!("\n🏗️  MODULE COVERAGE ANALYSIS:");
    for (module, symbols) in &other_symbols {
        let coverage = (symbols.intersection(&collector_symbols).count() as f64 / symbols.len() as f64) * 100.0;
        println!("  {} - {} symbols, {:.1}% covered by collector", module, symbols.len(), coverage);
    }
    
    println!("\n💡 EXPANSION RECOMMENDATIONS:");
    println!("============================================");
    
    // Group missing symbols by likely AST/HIR categories
    let ast_related: Vec<_> = missing_sorted.iter()
        .filter(|s| s.to_lowercase().contains("ast") || s.to_lowercase().contains("node") || s.to_lowercase().contains("expr"))
        .collect();
    let type_related: Vec<_> = missing_sorted.iter()
        .filter(|s| s.to_lowercase().contains("ty") || s.to_lowercase().contains("type"))
        .collect();
    let pattern_related: Vec<_> = missing_sorted.iter()
        .filter(|s| s.to_lowercase().contains("pat") || s.to_lowercase().contains("match"))
        .collect();
    
    if !ast_related.is_empty() {
        println!("\n🌳 AST/Expression Related:");
        for symbol in ast_related.iter().take(10) {
            println!("  + {}", symbol);
        }
    }
    
    if !type_related.is_empty() {
        println!("\n🔧 Type System Related:");
        for symbol in type_related.iter().take(10) {
            println!("  + {}", symbol);
        }
    }
    
    if !pattern_related.is_empty() {
        println!("\n🎯 Pattern Matching Related:");
        for symbol in pattern_related.iter().take(10) {
            println!("  + {}", symbol);
        }
    }
    
    Ok(())
}
