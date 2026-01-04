use std::collections::HashMap;
use std::fs;
use std::process::Command;
use flate2::read::GzDecoder;
use std::io::Read;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Symbol Extraction & Histogram Analysis");
    
    // 1. Extract symbols from source (symbol map)
    let mut source_symbols = HashMap::new();
    if let Ok(file) = fs::File::open("symbol_map.json.gz") {
        let mut decoder = GzDecoder::new(file);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents)?;
        
        let symbol_map: HashMap<String, Value> = serde_json::from_str(&contents)?;
        for (symbol, entry) in &symbol_map {
            if let Some(symbol_type) = entry.get("symbol_type").and_then(|s| s.as_str()) {
                *source_symbols.entry(symbol_type.to_string()).or_insert(0) += 1;
            }
        }
        println!("📊 Source symbols: {} total", symbol_map.len());
    }
    
    // 2. Extract symbols from binary (compile and analyze)
    println!("🔨 Compiling current.rs for binary analysis...");
    let output = Command::new("cargo")
        .args(&["build", "--lib"])
        .output()?;
    
    let mut binary_symbols = HashMap::new();
    if output.status.success() {
        // Use nm to extract symbols from binary
        if let Ok(nm_output) = Command::new("nm")
            .args(&["target/debug/deps/libsplit_decls_genesis-*.rlib"])
            .output() 
        {
            let nm_str = String::from_utf8_lossy(&nm_output.stdout);
            for line in nm_str.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let symbol_type = parts[1];
                    *binary_symbols.entry(symbol_type.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    // 3. Generate histogram
    println!("\n📈 SYMBOL HISTOGRAM");
    println!("==================");
    
    println!("\n🔤 SOURCE SYMBOLS (from symbol_map.json.gz):");
    let mut source_sorted: Vec<_> = source_symbols.iter().collect();
    source_sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (symbol_type, count) in source_sorted.iter().take(10) {
        println!("  {:30} {:>8}", symbol_type, count);
    }
    
    println!("\n🔧 BINARY SYMBOLS (from compiled output):");
    let mut binary_sorted: Vec<_> = binary_symbols.iter().collect();
    binary_sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (symbol_type, count) in binary_sorted.iter().take(10) {
        println!("  {:30} {:>8}", symbol_type, count);
    }
    
    // 4. Coverage analysis
    let source_total: i32 = source_symbols.values().sum();
    let binary_total: i32 = binary_symbols.values().sum();
    
    println!("\n📊 COVERAGE ANALYSIS:");
    println!("  Source symbols:  {:>8}", source_total);
    println!("  Binary symbols:  {:>8}", binary_total);
    println!("  Coverage ratio:  {:>7.1}%", 
             if source_total > 0 { (binary_total as f64 / source_total as f64) * 100.0 } else { 0.0 });
    
    Ok(())
}
