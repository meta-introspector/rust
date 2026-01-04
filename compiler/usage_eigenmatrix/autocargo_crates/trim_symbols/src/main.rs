// trim_symbols.rs - Fast symbol trimming to remove unused symbols
use std::collections::BTreeMap;
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("✂️  Trimming unused symbols from symbol map");
    
    // Load symbol map
    println!("📖 Loading symbol_map_original.json.gz...");
    let file = fs::File::open("symbol_map_original.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: Value = serde_json::from_str(&contents)?;
    let mut used_symbols = BTreeMap::new();
    let mut total_symbols = 0;
    let mut used_count = 0;
    
    if let Value::Object(map) = json {
        total_symbols = map.len();
        for (key, value) in map {
            if let Value::Object(ref obj) = value {
                if let Some(usage_count) = obj.get("usage_count").and_then(|v| v.as_u64()) {
                    if usage_count > 0 {
                        used_symbols.insert(key, value);
                        used_count += 1;
                    }
                }
            }
        }
    }
    
    // Save trimmed symbols
    let trimmed_json = serde_json::to_string_pretty(&used_symbols)?;
    fs::write("used_symbols.json", trimmed_json)?;
    
    // Compress trimmed symbols
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    let output_file = fs::File::create("used_symbols.json.gz")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let json_output = serde_json::to_string_pretty(&used_symbols)?;
    encoder.write_all(json_output.as_bytes())?;
    encoder.finish()?;
    
    println!("✅ Trimmed symbols:");
    println!("  📊 Original: {} symbols", total_symbols);
    println!("  📊 Used: {} symbols ({:.1}%)", used_count, (used_count as f32 / total_symbols as f32) * 100.0);
    println!("  💾 Saved: used_symbols.json ({} KB)", fs::metadata("used_symbols.json")?.len() / 1024);
    println!("  💾 Saved: used_symbols.json.gz ({} KB)", fs::metadata("used_symbols.json.gz")?.len() / 1024);
    
    Ok(())
}
