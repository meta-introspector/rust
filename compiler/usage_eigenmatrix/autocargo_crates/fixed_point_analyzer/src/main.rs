use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("ast_patterns.json")?;
    let patterns: HashMap<String, u64> = serde_json::from_str(&data)?;
    
    // Find fixed points - symbols that appear at multiple depths
    let mut symbol_depths: HashMap<String, Vec<usize>> = HashMap::new();
    
    for (pattern, count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        let depth = parts.len();
        
        for symbol in &parts {
            symbol_depths.entry(symbol.to_string())
                .or_insert_with(Vec::new)
                .push(depth);
        }
    }
    
    // Find symbols that appear at 3+ different depths (fixed points)
    let mut fixed_points: Vec<(String, Vec<usize>, u64)> = Vec::new();
    
    for (symbol, depths) in &symbol_depths {
        let mut unique_depths: Vec<usize> = depths.clone();
        unique_depths.sort();
        unique_depths.dedup();
        
        if unique_depths.len() >= 3 {
            let total_usage: u64 = patterns.iter()
                .filter(|(pattern, _)| pattern.contains(symbol))
                .map(|(_, count)| count)
                .sum();
            
            fixed_points.push((symbol.clone(), unique_depths, total_usage));
        }
    }
    
    fixed_points.sort_by(|a, b| b.2.cmp(&a.2));
    
    println!("🔄 Fixed Points - Symbols Connecting Multiple System Levels:");
    for (symbol, depths, usage) in fixed_points.iter().take(15) {
        println!("  {} → depths {:?} → {} total uses", symbol, depths, usage);
    }
    
    // Find convergence patterns - where different systems meet
    let mut convergence_points: HashMap<String, Vec<String>> = HashMap::new();
    
    for (pattern, count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        
        // Look for transitions between different system types
        for i in 0..parts.len()-1 {
            let from = parts[i];
            let to = parts[i+1];
            
            // Identify system boundaries
            let from_type = classify_symbol(from);
            let to_type = classify_symbol(to);
            
            if from_type != to_type {
                let transition = format!("{}→{}", from_type, to_type);
                convergence_points.entry(transition)
                    .or_insert_with(Vec::new)
                    .push(format!("{}::{}", from, to));
            }
        }
    }
    
    println!("\n🌐 System Convergence Points:");
    for (transition, examples) in convergence_points.iter() {
        if examples.len() > 5 {
            println!("  {} → {} examples: {:?}", transition, examples.len(), 
                &examples[..3.min(examples.len())]);
        }
    }
    
    // Find equivalence classes - symbols with similar usage patterns
    let mut equivalence_classes: HashMap<String, Vec<String>> = HashMap::new();
    
    for (symbol1, depths1) in &symbol_depths {
        for (symbol2, depths2) in &symbol_depths {
            if symbol1 != symbol2 && depths1.len() >= 3 && depths2.len() >= 3 {
                let mut d1 = depths1.clone();
                let mut d2 = depths2.clone();
                d1.sort(); d1.dedup();
                d2.sort(); d2.dedup();
                
                // Check if they have similar depth patterns
                let intersection: Vec<_> = d1.iter().filter(|&x| d2.contains(x)).collect();
                if intersection.len() >= 2 {
                    let key = format!("depths_{:?}", d1);
                    equivalence_classes.entry(key)
                        .or_insert_with(Vec::new)
                        .push(symbol1.clone());
                }
            }
        }
    }
    
    println!("\n≡ Equivalence Classes - Symbols with Similar Depth Patterns:");
    for (pattern, symbols) in equivalence_classes.iter() {
        if symbols.len() > 2 {
            println!("  {} → {:?}", pattern, &symbols[..3.min(symbols.len())]);
        }
    }
    
    Ok(())
}

fn classify_symbol(symbol: &str) -> &'static str {
    match symbol {
        "tcx" | "self" | "dcx" => "context",
        "call" | "emit" | "emit_err" => "action", 
        "Some" | "Ok" | "Err" | "None" => "constructor",
        "span" | "def_id" | "ty" => "data",
        "map" | "collect" | "filter" => "transform",
        _ => "other"
    }
}
