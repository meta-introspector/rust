use std::collections::{HashSet, HashMap};
use std::fs;
use analyzeme::ProfilingData;
use serde_json::Value;

fn main() {
    println!("🔍 Enhanced Function Matcher - Static vs Runtime Correlation");
    println!("===========================================================");
    
    // Load static usage symbols
    let mut static_symbols = HashSet::new();
    let mut static_usage_counts = HashMap::new();
    
    println!("📊 Loading static usage data...");
    if let Ok(entries) = fs::read_dir("../../test_usage_data") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") 
                && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                            for usage in usages {
                                if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                    static_symbols.insert(symbol.to_string());
                                    if let Some(count) = usage.get("usage_count").and_then(|c| c.as_u64()) {
                                        *static_usage_counts.entry(symbol.to_string()).or_insert(0) += count as usize;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Load runtime functions
    let mut runtime_functions = HashSet::new();
    let mut runtime_call_counts = HashMap::new();
    
    println!("🔧 Loading runtime profile data...");
    if let Ok(entries) = fs::read_dir("../../compilation_trace") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mm_profdata") {
                if let Ok(profiling_data) = ProfilingData::new(&path) {
                    let results = profiling_data.perform_analysis();
                    for query in &results.query_data {
                        let function_name = query.label.to_string();
                        runtime_functions.insert(function_name.clone());
                        *runtime_call_counts.entry(function_name).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    println!("📈 Analysis complete:");
    println!("   Static symbols: {}", static_symbols.len());
    println!("   Runtime functions: {}", runtime_functions.len());
    
    // Find exact matches
    let exact_matches: Vec<_> = static_symbols.intersection(&runtime_functions).collect();
    
    // Find partial matches (contains/substring)
    let mut partial_matches = Vec::new();
    for static_sym in &static_symbols {
        for runtime_func in &runtime_functions {
            if static_sym != runtime_func {
                // Check if one contains the other (case insensitive)
                let static_lower = static_sym.to_lowercase();
                let runtime_lower = runtime_func.to_lowercase();
                
                if static_lower.contains(&runtime_lower) || runtime_lower.contains(&static_lower) {
                    partial_matches.push((static_sym, runtime_func));
                }
            }
        }
    }
    
    // Find semantic matches (common patterns)
    let mut semantic_matches = Vec::new();
    let patterns = [
        ("new", "create"),
        ("encode", "encode"),
        ("decode", "decode"),
        ("meta", "metadata"),
        ("parse", "parse"),
        ("span", "span"),
        ("error", "error"),
        ("ast", "ast"),
        ("trait", "trait"),
        ("impl", "impl"),
    ];
    
    for (static_pattern, runtime_pattern) in &patterns {
        let static_matches: Vec<_> = static_symbols.iter()
            .filter(|s| s.to_lowercase().contains(static_pattern))
            .collect();
        let runtime_matches: Vec<_> = runtime_functions.iter()
            .filter(|f| f.to_lowercase().contains(runtime_pattern))
            .collect();
            
        if !static_matches.is_empty() && !runtime_matches.is_empty() {
            semantic_matches.push((static_pattern, static_matches, runtime_matches));
        }
    }
    
    // Report results
    println!("\n🎯 MATCHING RESULTS:");
    println!("==================");
    
    println!("\n🔗 EXACT MATCHES ({}):", exact_matches.len());
    for symbol in exact_matches.iter().take(10) {
        let static_count = static_usage_counts.get(*symbol).unwrap_or(&0);
        let runtime_count = runtime_call_counts.get(*symbol).unwrap_or(&0);
        println!("  ✅ {} (static: {}, runtime: {})", symbol, static_count, runtime_count);
    }
    
    println!("\n🔗 PARTIAL MATCHES ({}):", partial_matches.len());
    for (static_sym, runtime_func) in partial_matches.iter().take(10) {
        let static_count = static_usage_counts.get(*static_sym).unwrap_or(&0);
        let runtime_count = runtime_call_counts.get(*runtime_func).unwrap_or(&0);
        println!("  🔄 {} ↔ {} (static: {}, runtime: {})", static_sym, runtime_func, static_count, runtime_count);
    }
    
    println!("\n🔗 SEMANTIC MATCHES:");
    for (pattern, static_matches, runtime_matches) in &semantic_matches {
        println!("  🎯 Pattern '{}': {} static ↔ {} runtime", pattern, static_matches.len(), runtime_matches.len());
        
        // Show top examples
        for static_sym in static_matches.iter().take(3) {
            let static_count = static_usage_counts.get(*static_sym).unwrap_or(&0);
            println!("    📊 Static: {} ({})", static_sym, static_count);
        }
        for runtime_func in runtime_matches.iter().take(3) {
            let runtime_count = runtime_call_counts.get(*runtime_func).unwrap_or(&0);
            println!("    ⚡ Runtime: {} ({})", runtime_func, runtime_count);
        }
        println!();
    }
    
    // Summary statistics
    let total_matches = exact_matches.len() + partial_matches.len();
    let match_rate = if !static_symbols.is_empty() {
        (total_matches as f64 / static_symbols.len() as f64) * 100.0
    } else {
        0.0
    };
    
    println!("📊 SUMMARY:");
    println!("  Total matches: {}", total_matches);
    println!("  Match rate: {:.1}%", match_rate);
    println!("  Semantic patterns: {}", semantic_matches.len());
}
