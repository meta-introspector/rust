// reorder_symbol_map.rs - Reorder existing symbol_map.json.gz by dependency count
use std::fs;
use std::io::{Write, Read};
use std::collections::{HashMap, BTreeMap};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json::{Value, Map};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Reordering symbol_map.json.gz by dependency count...");
    
    // Load and decompress existing symbol map
    let file = fs::File::open("symbol_map.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut content = String::new();
    decoder.read_to_string(&mut content)?;
    
    // Parse JSON
    let symbol_map: Map<String, Value> = serde_json::from_str(&content)?;
    println!("✅ Loaded {} symbols", symbol_map.len());
    
    // Group symbols by source file and calculate total dependencies
    let mut file_deps: HashMap<String, usize> = HashMap::new();
    let mut file_symbols: HashMap<String, Vec<(String, Value)>> = HashMap::new();
    
    for (symbol_name, symbol_data) in symbol_map {
        if let Some(source_file) = symbol_data.get("source_file").and_then(|v| v.as_str()) {
            let deps_count = symbol_data.get("dependencies")
                .and_then(|v| v.as_array())
                .map(|arr| arr.len())
                .unwrap_or(0);
            
            *file_deps.entry(source_file.to_string()).or_insert(0) += deps_count;
            file_symbols.entry(source_file.to_string())
                .or_insert_with(Vec::new)
                .push((symbol_name, symbol_data));
        }
    }
    
    // Sort files by dependency count (0 first, then ascending)
    let mut sorted_files: Vec<(String, usize)> = file_deps.into_iter().collect();
    sorted_files.sort_by_key(|(_, deps)| *deps);
    
    println!("📊 Dependency distribution:");
    let zero_dep_files = sorted_files.iter().filter(|(_, deps)| *deps == 0).count();
    println!("  - {} files with 0 dependencies", zero_dep_files);
    if let Some((_, max_deps)) = sorted_files.last() {
        println!("  - Max dependencies: {}", max_deps);
    }
    
    // Create ordered symbol map
    let mut ordered_symbols = BTreeMap::new();
    for (file, _) in &sorted_files {
        if let Some(symbols) = file_symbols.get(file) {
            for (symbol_name, symbol_data) in symbols {
                ordered_symbols.insert(symbol_name.clone(), symbol_data.clone());
            }
        }
    }
    
    // Export reordered JSON
    let json_output = serde_json::to_string_pretty(&ordered_symbols)?;
    fs::write("symbol_map_ordered.json", &json_output)?;
    
    // Compress the reordered JSON
    let compressed_file = fs::File::create("symbol_map_ordered.json.gz")?;
    let mut encoder = GzEncoder::new(compressed_file, Compression::default());
    encoder.write_all(json_output.as_bytes())?;
    encoder.finish()?;
    
    println!("📄 Exported symbol_map_ordered.json ({} symbols)", ordered_symbols.len());
    println!("📦 Exported symbol_map_ordered.json.gz (compressed)");
    
    // Show first few files (should be 0 dependency files)
    println!("\n🔍 First files in dependency order:");
    for (file, deps) in sorted_files.iter().take(5) {
        println!("  {} dependencies: {}", deps, file);
    }
    
    Ok(())
}
