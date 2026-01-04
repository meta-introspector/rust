// usage_levels.rs - Calculate 8 levels of symbol usage dependencies
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Calculating 8-level usage dependencies for x86 Linux symbols");
    
    // Load x86 Linux symbols
    let file = fs::File::open("x86_linux_symbols.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: Value = serde_json::from_str(&contents)?;
    let mut symbols = HashMap::new();
    let mut dependencies = HashMap::new();
    
    // Parse symbols and dependencies
    if let Value::Object(symbol_map) = json {
        for (symbol_key, symbol_data) in symbol_map {
            if let Value::Object(obj) = symbol_data {
                let usage_count = obj.get("usage_count").and_then(|v| v.as_u64()).unwrap_or(0);
                symbols.insert(symbol_key.clone(), usage_count);
                
                if let Some(deps) = obj.get("dependencies").and_then(|v| v.as_array()) {
                    let dep_list: Vec<String> = deps.iter()
                        .filter_map(|d| d.as_str().map(|s| s.to_string()))
                        .collect();
                    dependencies.insert(symbol_key, dep_list);
                }
            }
        }
    }
    
    println!("📊 Loaded {} symbols with dependencies", symbols.len());
    
    // Build reverse dependency map (who uses what)
    let mut used_by: HashMap<String, Vec<String>> = HashMap::new();
    for (symbol, deps) in &dependencies {
        for dep in deps {
            used_by.entry(dep.clone()).or_default().push(symbol.clone());
        }
    }
    
    // Calculate 8-level usage chains for each symbol
    let mut level_analysis = HashMap::new();
    let mut processed = 0;
    
    for (symbol, usage_count) in &symbols {
        if processed % 1000 == 0 {
            println!("  📊 Processed {}/{} symbols ({:.1}%)", 
                processed, symbols.len(), 
                (processed as f32 / symbols.len() as f32) * 100.0);
        }
        
        let levels = calculate_usage_levels(symbol, &used_by, &symbols, 8);
        level_analysis.insert(symbol.clone(), serde_json::json!({
            "symbol": symbol,
            "direct_usage": usage_count,
            "levels": levels
        }));
        
        processed += 1;
    }
    
    // Save 8-level analysis
    let output_file = fs::File::create("usage_levels_8.json.gz")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let json_output = serde_json::to_string_pretty(&level_analysis)?;
    encoder.write_all(json_output.as_bytes())?;
    encoder.finish()?;
    
    // Generate summary statistics
    let mut level_stats = vec![0; 8];
    let mut max_usage_per_level = vec![0u64; 8];
    
    for analysis in level_analysis.values() {
        if let Some(levels) = analysis.get("levels").and_then(|v| v.as_array()) {
            for (i, level) in levels.iter().enumerate() {
                if i >= 8 { break; }
                if let Some(count) = level.get("total_usage").and_then(|v| v.as_u64()) {
                    if count > 0 {
                        level_stats[i] += 1;
                        max_usage_per_level[i] = max_usage_per_level[i].max(count);
                    }
                }
            }
        }
    }
    
    println!("\n✅ 8-Level Usage Analysis Complete!");
    println!("📊 Results:");
    for (level, count) in level_stats.iter().enumerate() {
        println!("  Level {}: {} symbols with usage (max: {})", 
            level + 1, count, max_usage_per_level[level]);
    }
    println!("💾 Saved: usage_levels_8.json.gz ({} KB)", 
        fs::metadata("usage_levels_8.json.gz")?.len() / 1024);
    
    Ok(())
}

fn calculate_usage_levels(
    symbol: &str,
    used_by: &HashMap<String, Vec<String>>,
    symbols: &HashMap<String, u64>,
    max_levels: usize,
) -> Vec<Value> {
    let mut levels = Vec::new();
    let mut current_level = HashSet::new();
    let mut visited = HashSet::new();
    
    current_level.insert(symbol.to_string());
    visited.insert(symbol.to_string());
    
    for level in 0..max_levels {
        let mut next_level = HashSet::new();
        let mut total_usage = 0u64;
        let mut symbol_count = 0;
        
        for sym in &current_level {
            if let Some(users) = used_by.get(sym) {
                for user in users {
                    if !visited.contains(user) {
                        next_level.insert(user.clone());
                        visited.insert(user.clone());
                        
                        if let Some(usage) = symbols.get(user) {
                            total_usage += usage;
                        }
                        symbol_count += 1;
                    }
                }
            }
        }
        
        levels.push(serde_json::json!({
            "level": level + 1,
            "symbol_count": symbol_count,
            "total_usage": total_usage,
            "symbols": if symbol_count <= 10 { 
                next_level.iter().cloned().collect::<Vec<_>>() 
            } else { 
                next_level.iter().take(10).cloned().collect::<Vec<_>>() 
            }
        }));
        
        if next_level.is_empty() {
            break;
        }
        
        current_level = next_level;
    }
    
    levels
}
