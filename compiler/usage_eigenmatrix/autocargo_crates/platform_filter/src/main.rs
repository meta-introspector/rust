// platform_filter.rs - Strip Windows, Mac, and test symbols for x86 Linux only
use std::collections::BTreeMap;
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Platform Filter: x86 Linux only (no Windows, Mac, tests)");
    
    // Load used symbols
    let file = fs::File::open("used_symbols.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: Value = serde_json::from_str(&contents)?;
    let mut filtered_symbols = BTreeMap::new();
    let mut total_symbols = 0;
    let mut filtered_count = 0;
    
    if let Value::Object(symbols) = json {
        total_symbols = symbols.len();
        
        for (symbol_key, symbol_data) in symbols {
            if should_keep_symbol(&symbol_key, &symbol_data) {
                filtered_symbols.insert(symbol_key, symbol_data);
                filtered_count += 1;
            }
        }
    }
    
    // Save filtered symbols
    let output_file = fs::File::create("x86_linux_symbols.json.gz")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let json_output = serde_json::to_string_pretty(&filtered_symbols)?;
    encoder.write_all(json_output.as_bytes())?;
    encoder.finish()?;
    
    println!("✅ Platform filtering complete:");
    println!("  📊 Original: {} symbols", total_symbols);
    println!("  📊 Filtered: {} symbols ({:.1}%)", filtered_count, (filtered_count as f32 / total_symbols as f32) * 100.0);
    println!("  📊 Removed: {} symbols ({:.1}%)", total_symbols - filtered_count, ((total_symbols - filtered_count) as f32 / total_symbols as f32) * 100.0);
    println!("  💾 Saved: x86_linux_symbols.json.gz ({} KB)", fs::metadata("x86_linux_symbols.json.gz")?.len() / 1024);
    
    Ok(())
}

fn should_keep_symbol(symbol_key: &str, symbol_data: &Value) -> bool {
    let symbol_name = symbol_key.to_lowercase();
    
    // Skip test-related symbols
    if symbol_name.contains("test") || 
       symbol_name.contains("bench") ||
       symbol_name.contains("mock") ||
       symbol_name.contains("assert") {
        return false;
    }
    
    // Skip Windows-specific
    if symbol_name.contains("windows") ||
       symbol_name.contains("win32") ||
       symbol_name.contains("msvc") ||
       symbol_name.contains("pe32") ||
       symbol_name.contains("coff") {
        return false;
    }
    
    // Skip Mac-specific  
    if symbol_name.contains("macos") ||
       symbol_name.contains("darwin") ||
       symbol_name.contains("mach") ||
       symbol_name.contains("apple") ||
       symbol_name.contains("osx") {
        return false;
    }
    
    // Skip non-x86 architectures
    if symbol_name.contains("aarch64") ||
       symbol_name.contains("arm") ||
       symbol_name.contains("mips") ||
       symbol_name.contains("riscv") ||
       symbol_name.contains("sparc") ||
       symbol_name.contains("powerpc") ||
       symbol_name.contains("s390x") ||
       symbol_name.contains("wasm") ||
       symbol_name.contains("lasx") ||  // MIPS SIMD
       symbol_name.contains("lsx") ||   // MIPS SIMD
       symbol_name.contains("msa") ||   // MIPS SIMD
       symbol_name.contains("neon") ||  // ARM SIMD
       symbol_name.contains("sve") {    // ARM SVE
        return false;
    }
    
    // Check crate name for platform-specific crates
    if let Some(obj) = symbol_data.as_object() {
        if let Some(crate_name) = obj.get("crate_name").and_then(|v| v.as_str()) {
            let crate_lower = crate_name.to_lowercase();
            
            // Skip test crates
            if crate_lower.contains("test") ||
               crate_lower.contains("bench") {
                return false;
            }
            
            // Skip platform-specific crates
            if crate_lower.contains("windows") ||
               crate_lower.contains("darwin") ||
               crate_lower.contains("macos") {
                return false;
            }
        }
        
        // Check source file path
        if let Some(source_file) = obj.get("source_file").and_then(|v| v.as_str()) {
            let source_lower = source_file.to_lowercase();
            
            if source_lower.contains("test") ||
               source_lower.contains("bench") ||
               source_lower.contains("windows") ||
               source_lower.contains("darwin") ||
               source_lower.contains("macos") {
                return false;
            }
        }
    }
    
    true
}
