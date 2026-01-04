// frequency_analyzer.rs - Analyze term frequency in used symbols
use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Frequency Analysis of Used Symbols");
    
    // Load used symbols
    let file = fs::File::open("used_symbols.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: Value = serde_json::from_str(&contents)?;
    
    let mut term_frequency = HashMap::new();
    let mut crate_frequency = HashMap::new();
    let mut type_frequency = HashMap::new();
    let mut usage_by_term = HashMap::new();
    
    if let Value::Object(symbols) = json {
        for (symbol_key, symbol_data) in symbols {
            if let Value::Object(obj) = symbol_data {
                // Extract crate name
                if let Some(crate_name) = obj.get("crate_name").and_then(|v| v.as_str()) {
                    *crate_frequency.entry(crate_name.to_string()).or_insert(0) += 1;
                }
                
                // Extract symbol type
                if let Some(symbol_type) = obj.get("symbol_type").and_then(|v| v.as_str()) {
                    *type_frequency.entry(symbol_type.to_string()).or_insert(0) += 1;
                }
                
                // Extract usage count
                let usage_count = obj.get("usage_count").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                
                // Analyze symbol name terms
                if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                    // Split by common separators and analyze terms
                    let terms: Vec<&str> = name.split(&['_', ':'][..]).collect();
                    for term in terms {
                        if term.len() > 1 { // Skip single chars
                            *term_frequency.entry(term.to_string()).or_insert(0) += 1;
                            *usage_by_term.entry(term.to_string()).or_insert(0) += usage_count;
                        }
                    }
                }
            }
        }
    }
    
    // Sort and display results
    println!("\n🏆 Top 20 Most Frequent Terms:");
    let mut sorted_terms: Vec<_> = term_frequency.iter().collect();
    sorted_terms.sort_by(|a, b| b.1.cmp(a.1));
    for (i, (term, count)) in sorted_terms.iter().take(20).enumerate() {
        let usage = usage_by_term.get(*term).unwrap_or(&0);
        println!("  {:2}. {:20} {:6} symbols, {:10} usages", i+1, term, count, usage);
    }
    
    println!("\n🏆 Top 15 Most Active Crates:");
    let mut sorted_crates: Vec<_> = crate_frequency.iter().collect();
    sorted_crates.sort_by(|a, b| b.1.cmp(a.1));
    for (i, (crate_name, count)) in sorted_crates.iter().take(15).enumerate() {
        println!("  {:2}. {:30} {:6} symbols", i+1, crate_name, count);
    }
    
    println!("\n🏆 Top 15 Symbol Types:");
    let mut sorted_types: Vec<_> = type_frequency.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    for (i, (symbol_type, count)) in sorted_types.iter().take(15).enumerate() {
        println!("  {:2}. {:20} {:6} symbols", i+1, symbol_type, count);
    }
    
    // Save detailed frequency analysis
    let analysis = serde_json::json!({
        "term_frequency": term_frequency,
        "crate_frequency": crate_frequency,
        "type_frequency": type_frequency,
        "usage_by_term": usage_by_term,
        "total_symbols": sorted_terms.len(),
        "total_crates": sorted_crates.len(),
        "total_types": sorted_types.len()
    });
    
    fs::write("frequency_analysis.json", serde_json::to_string_pretty(&analysis)?)?;
    
    println!("\n💾 Detailed analysis saved to frequency_analysis.json");
    println!("📊 Summary: {} unique terms, {} crates, {} symbol types", 
        sorted_terms.len(), sorted_crates.len(), sorted_types.len());
    
    Ok(())
}
