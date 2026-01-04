use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load cached patterns
    let data = fs::read_to_string("ast_patterns.json")?;
    let patterns: HashMap<String, u64> = serde_json::from_str(&data)?;
    
    // Calculate global symbol weights
    let mut symbol_weights: HashMap<String, u64> = HashMap::new();
    for (pattern, count) in &patterns {
        for symbol in pattern.split("::") {
            *symbol_weights.entry(symbol.to_string()).or_insert(0) += count;
        }
    }
    
    // Find common subsequences across different lengths
    let mut subsequence_usage: HashMap<String, Vec<(String, u64)>> = HashMap::new();
    
    for (pattern, count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        
        // Generate all subsequences of length 2-7
        for len in 2..=7 {
            if parts.len() >= len {
                for start in 0..=(parts.len() - len) {
                    let subseq = parts[start..start+len].join("::");
                    subsequence_usage.entry(subseq)
                        .or_insert_with(Vec::new)
                        .push((pattern.clone(), *count));
                }
            }
        }
    }
    
    // Find subsequences used in multiple different contexts
    let mut common_subsequences: Vec<(String, u64, usize)> = subsequence_usage
        .iter()
        .filter(|(_, usages)| usages.len() > 1)
        .map(|(subseq, usages)| {
            let total_count: u64 = usages.iter().map(|(_, count)| count).sum();
            (subseq.clone(), total_count, usages.len())
        })
        .collect();
    
    common_subsequences.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("🔗 Common Subsequences Across Multiple Patterns:");
    for (subseq, total_count, pattern_count) in common_subsequences.iter().take(20) {
        println!("  {} → {} total uses in {} different patterns", subseq, total_count, pattern_count);
    }
    
    // Calculate weighted significance for pattern naming
    let mut weighted_patterns: Vec<(String, f64)> = Vec::new();
    
    for (pattern, local_count) in &patterns {
        let parts: Vec<&str> = pattern.split("::").collect();
        let mut total_weight = 0.0;
        
        for symbol in &parts {
            if let Some(global_weight) = symbol_weights.get(*symbol) {
                total_weight += (*local_count as f64) * (*global_weight as f64);
            }
        }
        
        weighted_patterns.push((pattern.clone(), total_weight));
    }
    
    weighted_patterns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🏆 Top 15 Weighted Significant Patterns:");
    for (pattern, weight) in weighted_patterns.iter().take(15) {
        let parts: Vec<&str> = pattern.split("::").collect();
        let dominant_symbol = parts.iter()
            .max_by_key(|symbol| symbol_weights.get(**symbol).unwrap_or(&0))
            .unwrap_or(&"unknown");
        
        println!("  {} → weight: {:.0} (dominant: {})", pattern, weight, dominant_symbol);
    }
    
    // Generate macro names based on dominant symbols
    println!("\n🎯 Generated Macro Names:");
    for (pattern, _) in weighted_patterns.iter().take(10) {
        let parts: Vec<&str> = pattern.split("::").collect();
        let dominant = parts.iter()
            .max_by_key(|symbol| symbol_weights.get(**symbol).unwrap_or(&0))
            .unwrap_or(&"unknown");
        
        let macro_name = format!("use_pattern_{}_{}", dominant, parts.len());
        let args = parts.iter()
            .filter(|&s| s != dominant)
            .take(3)
            .map(|s| format!("${}", s))
            .collect::<Vec<_>>()
            .join(", ");
        
        println!("  macro_rules! {} {{ ({}) => {{ /* {} */ }}; }}", 
            macro_name, args, pattern);
    }
    
    Ok(())
}
