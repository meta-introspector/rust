// usage_analyzer.rs - Build N-level usage dependency trees from symbol map
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fs;
use serde_json::{Value, Map};
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Debug, Clone)]
struct UsageContext {
    used_by: String,
    field_context: String,
    usage_count: usize,
}

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    crate_name: String,
    symbol_type: String,
    source_file: String,
    usage_count: usize,
    dependencies: Vec<String>,
    used_by: Vec<UsageContext>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Usage Analyzer: Building N-level dependency trees");
    
    // Load symbol map
    println!("📖 Loading symbol map...");
    let symbol_map = load_symbol_map()?;
    
    // Filter used symbols only
    let used_symbols = filter_used_symbols(&symbol_map);
    println!("✅ Filtered to {} used symbols (from {})", used_symbols.len(), symbol_map.len());
    
    // Build reverse dependency map (who uses what)
    println!("🔗 Building usage chains...");
    let usage_chains = build_usage_chains(&used_symbols);
    
    // Generate N-level usage trees
    let depth = 3; // Start with depth 3
    println!("🌳 Generating {}-level usage trees...", depth);
    let usage_trees = generate_usage_trees(&used_symbols, &usage_chains, depth);
    
    // Save results
    save_usage_analysis(&used_symbols, &usage_trees)?;
    
    println!("✅ Usage analysis complete!");
    Ok(())
}

fn load_symbol_map() -> Result<BTreeMap<String, Symbol>, Box<dyn std::error::Error>> {
    let file = fs::File::open("symbol_map_original.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: Value = serde_json::from_str(&contents)?;
    let mut symbols = BTreeMap::new();
    
    if let Value::Object(map) = json {
        for (key, value) in map {
            if let Value::Object(obj) = value {
                let symbol = Symbol {
                    name: obj.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    crate_name: obj.get("crate_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    symbol_type: obj.get("symbol_type").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    source_file: obj.get("source_file").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    usage_count: obj.get("usage_count").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
                    dependencies: obj.get("dependencies")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                        .unwrap_or_default(),
                    used_by: Vec::new(),
                };
                symbols.insert(key, symbol);
            }
        }
    }
    
    Ok(symbols)
}

fn filter_used_symbols(symbols: &BTreeMap<String, Symbol>) -> BTreeMap<String, Symbol> {
    symbols.iter()
        .filter(|(_, symbol)| symbol.usage_count > 0)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

fn build_usage_chains(symbols: &BTreeMap<String, Symbol>) -> HashMap<String, Vec<UsageContext>> {
    let mut usage_chains: HashMap<String, Vec<UsageContext>> = HashMap::new();
    
    // For each symbol, find what uses it by scanning dependencies
    for (symbol_key, symbol) in symbols {
        for (other_key, other_symbol) in symbols {
            if other_key != symbol_key {
                // Check if other_symbol depends on this symbol
                for dep in &other_symbol.dependencies {
                    if dep.contains(&symbol.name) || symbol_key.contains(dep) {
                        let context = UsageContext {
                            used_by: other_key.clone(),
                            field_context: format!("dependency[{}]", dep),
                            usage_count: other_symbol.usage_count,
                        };
                        usage_chains.entry(symbol_key.clone()).or_default().push(context);
                    }
                }
            }
        }
    }
    
    usage_chains
}

fn generate_usage_trees(
    symbols: &BTreeMap<String, Symbol>,
    usage_chains: &HashMap<String, Vec<UsageContext>>,
    max_depth: usize,
) -> HashMap<String, Value> {
    let mut trees = HashMap::new();
    
    for (symbol_key, symbol) in symbols {
        let tree = build_tree_recursive(symbol_key, symbols, usage_chains, max_depth, 0, &mut HashSet::new());
        trees.insert(symbol_key.clone(), tree);
    }
    
    trees
}

fn build_tree_recursive(
    symbol_key: &str,
    symbols: &BTreeMap<String, Symbol>,
    usage_chains: &HashMap<String, Vec<UsageContext>>,
    max_depth: usize,
    current_depth: usize,
    visited: &mut HashSet<String>,
) -> Value {
    if current_depth >= max_depth || visited.contains(symbol_key) {
        return serde_json::json!({
            "symbol": symbol_key,
            "depth_limit": true
        });
    }
    
    visited.insert(symbol_key.to_string());
    
    let symbol = symbols.get(symbol_key);
    let mut node = serde_json::json!({
        "symbol": symbol_key,
        "usage_count": symbol.map(|s| s.usage_count).unwrap_or(0),
        "symbol_type": symbol.map(|s| s.symbol_type.as_str()).unwrap_or("unknown"),
        "used_by": []
    });
    
    if let Some(contexts) = usage_chains.get(symbol_key) {
        let mut used_by = Vec::new();
        for context in contexts {
            let child_tree = build_tree_recursive(
                &context.used_by,
                symbols,
                usage_chains,
                max_depth,
                current_depth + 1,
                visited,
            );
            used_by.push(serde_json::json!({
                "context": context.field_context,
                "usage_count": context.usage_count,
                "tree": child_tree
            }));
        }
        node["used_by"] = serde_json::json!(used_by);
    }
    
    visited.remove(symbol_key);
    node
}

fn save_usage_analysis(
    symbols: &BTreeMap<String, Symbol>,
    trees: &HashMap<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Save trimmed symbols (used only)
    let trimmed_symbols: BTreeMap<String, Value> = symbols.iter()
        .map(|(k, v)| (k.clone(), serde_json::json!({
            "name": v.name,
            "crate_name": v.crate_name,
            "symbol_type": v.symbol_type,
            "source_file": v.source_file,
            "usage_count": v.usage_count,
            "dependencies": v.dependencies
        })))
        .collect();
    
    fs::write("used_symbols.json", serde_json::to_string_pretty(&trimmed_symbols)?)?;
    
    // Save usage trees
    fs::write("usage_trees.json", serde_json::to_string_pretty(&trees)?)?;
    
    // Save summary
    let summary = serde_json::json!({
        "total_used_symbols": symbols.len(),
        "total_usage_trees": trees.len(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    fs::write("usage_analysis_summary.json", serde_json::to_string_pretty(&summary)?)?;
    
    println!("💾 Saved:");
    println!("  - used_symbols.json ({} symbols)", symbols.len());
    println!("  - usage_trees.json ({} trees)", trees.len());
    println!("  - usage_analysis_summary.json");
    
    Ok(())
}
