use std::collections::{HashMap, HashSet};
use serde_json;
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Debug)]
struct UseStatement {
    module_path: String,
    use_stmt: String,
}

#[derive(Debug)]
struct SymbolInfo {
    name: String,
    symbol_type: String,
    source_file: String,
    crate_name: String,
    dependencies: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Quasi Meta-Interpretation: Symbol Resolution");
    
    // Parse extracted USE statements
    let uses = parse_extracted_uses("extract_output2.log")?;
    println!("📊 Found {} USE statements", uses.len());
    
    // Load symbol map
    let symbol_map = load_symbol_map("symbol_map.json.gz")?;
    println!("🗺️  Loaded {} symbols from map", symbol_map.len());
    
    // Resolve each USE statement
    for use_stmt in &uses {
        println!("\n🔎 Resolving: {}", use_stmt.use_stmt);
        let resolved = resolve_use_statement(use_stmt, &symbol_map);
        
        if !resolved.is_empty() {
            println!("  ✅ Found {} matching symbols:", resolved.len());
            for symbol in &resolved {
                println!("    📦 {} ({})", symbol.name, symbol.crate_name);
                println!("       📁 {}", symbol.source_file);
                if !symbol.dependencies.is_empty() {
                    println!("       🔗 Deps: {}", symbol.dependencies.join(", "));
                }
            }
        } else {
            println!("  ❌ No matching symbols found");
        }
    }
    
    // Generate export suggestions
    generate_export_suggestions(&uses, &symbol_map)?;
    
    Ok(())
}

fn parse_extracted_uses(log_file: &str) -> Result<Vec<UseStatement>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(log_file)?;
    let mut uses = Vec::new();
    
    for line in content.lines() {
        if line.contains("error: USE|") {
            if let Some(use_part) = line.split("error: USE|").nth(1) {
                let parts: Vec<&str> = use_part.split('|').collect();
                if parts.len() >= 2 {
                    uses.push(UseStatement {
                        module_path: parts[0].to_string(),
                        use_stmt: parts[1].to_string(),
                    });
                }
            }
        }
    }
    
    Ok(uses)
}

fn load_symbol_map(file_path: &str) -> Result<HashMap<String, SymbolInfo>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: serde_json::Value = serde_json::from_str(&contents)?;
    let mut symbol_map = HashMap::new();
    
    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            if let Some(symbol_obj) = value.as_object() {
                let symbol = SymbolInfo {
                    name: symbol_obj.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    symbol_type: symbol_obj.get("symbol_type").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    source_file: symbol_obj.get("source_file").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    crate_name: symbol_obj.get("crate_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    dependencies: symbol_obj.get("dependencies")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default(),
                };
                symbol_map.insert(key.clone(), symbol);
            }
        }
    }
    
    Ok(symbol_map)
}

fn resolve_use_statement<'a>(use_stmt: &UseStatement, symbol_map: &'a HashMap<String, SymbolInfo>) -> Vec<&'a SymbolInfo> {
    let mut matches = Vec::new();
    
    // Extract crate name from use statement
    let crate_name = extract_crate_name(&use_stmt.use_stmt);
    
    for (_, symbol) in symbol_map {
        if let Some(ref crate_name) = crate_name {
            if symbol.crate_name == *crate_name {
                matches.push(symbol);
            }
        }
    }
    
    matches
}

fn extract_crate_name(use_stmt: &str) -> Option<String> {
    // Parse "use rustc_target::spec::{Target, TargetTuple};" -> "rustc_target"
    if let Some(use_part) = use_stmt.strip_prefix("use ") {
        if let Some(first_part) = use_part.split("::").next() {
            let crate_name = first_part.trim();
            if crate_name.starts_with("rustc_") || crate_name == "getopts" {
                return Some(crate_name.to_string());
            }
        }
    }
    None
}

fn generate_export_suggestions(uses: &[UseStatement], symbol_map: &HashMap<String, SymbolInfo>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Export Suggestions:");
    
    let mut crate_exports: HashMap<String, HashSet<String>> = HashMap::new();
    
    for use_stmt in uses {
        if let Some(crate_name) = extract_crate_name(&use_stmt.use_stmt) {
            let exports = crate_exports.entry(crate_name.clone()).or_insert_with(HashSet::new);
            
            // Find all symbols from this crate
            for (_, symbol) in symbol_map {
                if symbol.crate_name == crate_name {
                    exports.insert(symbol.name.clone());
                }
            }
        }
    }
    
    for (crate_name, exports) in &crate_exports {
        println!("\n📦 extern crate {};", crate_name);
        println!("   Exports {} symbols:", exports.len());
        for export in exports.iter().take(10) {
            println!("     • {}", export);
        }
        if exports.len() > 10 {
            println!("     ... and {} more", exports.len() - 10);
        }
    }
    
    Ok(())
}
