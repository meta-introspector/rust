use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::Path;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Rust Semantic Web Ontology - Aggregated usage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustOntology {
    pub metadata: OntologyMetadata,
    pub symbols: BTreeMap<String, SymbolData>,
    pub patterns: BTreeMap<String, usize>,
    pub type_flows: BTreeMap<String, Vec<String>>,
    pub statistics: OntologyStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyMetadata {
    pub version: String,
    pub generated_from: String,
    pub total_files_processed: usize,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolData {
    pub usage_count: usize,
    pub node_types: BTreeMap<String, usize>,
    pub expr_types: BTreeMap<String, usize>,
    pub contexts: Vec<String>,
    pub owl_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyStatistics {
    pub total_symbols: usize,
    pub total_usages: usize,
    pub top_symbols: Vec<TopSymbol>,
    pub top_patterns: Vec<TopPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSymbol {
    pub symbol: String,
    pub count: usize,
    pub owl_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopPattern {
    pub pattern: String,
    pub count: usize,
}

#[derive(Debug, Deserialize)]
struct UsageFile {
    #[serde(default)]
    crate_name: String,
    #[serde(default)]
    module: String,
    #[serde(default)]
    entries: Vec<UsageEntry>,
    #[serde(default)]
    usages: Vec<String>, // Legacy format
}

#[derive(Debug, Deserialize)]
struct UsageEntry {
    usage: String,
    #[serde(default)]
    node_type: String,
    expr_type: Option<String>,
    #[serde(default = "default_count")]
    count: usize,
}

fn default_count() -> usize { 1 }

impl RustOntology {
    /// Create ontology by aggregating all JSON files in directory
    pub fn from_usage_data(usage_data_dir: &Path) -> Result<Self> {
        let mut symbols: BTreeMap<String, SymbolData> = BTreeMap::new();
        let mut patterns: BTreeMap<String, usize> = BTreeMap::new();
        let mut type_flows: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut files_processed = 0;

        // Process all JSON files
        for entry in fs::read_dir(usage_data_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(usage_file) = serde_json::from_str::<UsageFile>(&content) {
                        Self::process_usage_file(&usage_file, &mut symbols, &mut patterns, &mut type_flows);
                        files_processed += 1;
                    }
                }
            }
        }

        // Convert type_flows to deduplicated vectors
        let type_flows: BTreeMap<String, Vec<String>> = type_flows
            .into_iter()
            .map(|(k, mut v)| {
                v.sort();
                v.dedup();
                (k, v)
            })
            .collect();

        // Generate statistics
        let total_symbols = symbols.len();
        let total_usages: usize = symbols.values().map(|s| s.usage_count).sum();
        
        let mut top_symbols: Vec<_> = symbols
            .iter()
            .map(|(symbol, data)| TopSymbol {
                symbol: symbol.clone(),
                count: data.usage_count,
                owl_type: data.owl_type.clone(),
            })
            .collect();
        top_symbols.sort_by(|a, b| b.count.cmp(&a.count));
        top_symbols.truncate(50);

        let mut top_patterns: Vec<_> = patterns
            .iter()
            .map(|(pattern, count)| TopPattern {
                pattern: pattern.clone(),
                count: *count,
            })
            .collect();
        top_patterns.sort_by(|a, b| b.count.cmp(&a.count));
        top_patterns.truncate(50);

        Ok(RustOntology {
            metadata: OntologyMetadata {
                version: "1.0".to_string(),
                generated_from: usage_data_dir.display().to_string(),
                total_files_processed: files_processed,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            symbols,
            patterns,
            type_flows,
            statistics: OntologyStatistics {
                total_symbols,
                total_usages,
                top_symbols,
                top_patterns,
            },
        })
    }

    fn process_usage_file(
        usage_file: &UsageFile,
        symbols: &mut BTreeMap<String, SymbolData>,
        patterns: &mut BTreeMap<String, usize>,
        type_flows: &mut BTreeMap<String, Vec<String>>,
    ) {
        let context = format!("{}::{}", usage_file.crate_name, usage_file.module);

        // Process structured entries
        for entry in &usage_file.entries {
            let symbol = Self::extract_symbol(&entry.usage);
            let owl_type = Self::classify_owl_type(&entry.usage);

            let symbol_data = symbols.entry(symbol.clone()).or_insert_with(|| SymbolData {
                usage_count: 0,
                node_types: BTreeMap::new(),
                expr_types: BTreeMap::new(),
                contexts: Vec::new(),
                owl_type: owl_type.clone(),
            });

            symbol_data.usage_count += entry.count;
            *symbol_data.node_types.entry(entry.node_type.clone()).or_insert(0) += entry.count;
            
            if let Some(ref expr_type) = entry.expr_type {
                *symbol_data.expr_types.entry(expr_type.clone()).or_insert(0) += entry.count;
                
                // Track type flows
                type_flows.entry(expr_type.clone()).or_default().push(symbol.clone());
            }

            if !symbol_data.contexts.contains(&context) {
                symbol_data.contexts.push(context.clone());
            }

            // Track patterns
            let pattern = format!("{} -> {}", entry.node_type, symbol);
            *patterns.entry(pattern).or_insert(0) += entry.count;
        }

        // Process legacy usages
        for usage in &usage_file.usages {
            let symbol = Self::extract_symbol(usage);
            let owl_type = Self::classify_owl_type(usage);

            let symbol_data = symbols.entry(symbol.clone()).or_insert_with(|| SymbolData {
                usage_count: 0,
                node_types: BTreeMap::new(),
                expr_types: BTreeMap::new(),
                contexts: Vec::new(),
                owl_type,
            });

            symbol_data.usage_count += 1;
            if !symbol_data.contexts.contains(&context) {
                symbol_data.contexts.push(context.clone());
            }
        }
    }

    fn extract_symbol(usage: &str) -> String {
        if usage.contains("crate::") {
            for part in usage.split_whitespace() {
                if part.contains("crate::") && !part.contains('(') {
                    return part.to_string();
                }
            }
        }
        usage.split_whitespace().next().unwrap_or("unknown").to_string()
    }

    fn classify_owl_type(usage: &str) -> String {
        if usage.contains("(AssocFn)") {
            "owl:Method".to_string()
        } else if usage.contains("(AssocConst)") {
            "owl:Constant".to_string()
        } else if usage.contains("(Variant)") {
            "owl:EnumVariant".to_string()
        } else if usage.contains("(Struct)") {
            "owl:Class".to_string()
        } else if usage.contains("(Trait)") {
            "owl:Interface".to_string()
        } else if usage.contains("NODE_TYPE:") {
            "owl:DataProperty".to_string()
        } else {
            "owl:Thing".to_string()
        }
    }

    /// Generate RDF/Turtle representation
    pub fn to_rdf_turtle(&self) -> String {
        let mut rdf = Vec::new();
        
        rdf.push("@prefix rust: <http://rust-lang.org/ontology#> .".to_string());
        rdf.push("@prefix owl: <http://www.w3.org/2002/07/owl#> .".to_string());
        rdf.push("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .".to_string());
        rdf.push("".to_string());
        
        rdf.push("# Rust Semantic Web Ontology".to_string());
        rdf.push(format!("# Generated: {}", self.metadata.timestamp));
        rdf.push(format!("# Total symbols: {}", self.statistics.total_symbols));
        rdf.push("".to_string());

        // Top symbols only to keep manageable
        for symbol_info in &self.statistics.top_symbols {
            if let Some(data) = self.symbols.get(&symbol_info.symbol) {
                let symbol_uri = symbol_info.symbol
                    .replace("::", "_")
                    .replace('<', "_")
                    .replace('>', "_")
                    .replace(' ', "_");
                
                rdf.push(format!("rust:{} a {} ;", symbol_uri, data.owl_type));
                rdf.push(format!("    rdfs:label \"{}\" ;", symbol_info.symbol));
                rdf.push(format!("    rust:usageCount {} ;", data.usage_count));
                
                // Top node types
                for (node_type, count) in data.node_types.iter().take(3) {
                    rdf.push(format!("    rust:usedByNodeType \"{}\" ;", node_type));
                }
                
                rdf.push("    .".to_string());
                rdf.push("".to_string());
            }
        }
        
        rdf.join("\n")
    }

    /// Save ontology in multiple formats
    pub fn save_all_formats(&self, base_path: &str) -> Result<()> {
        // JSON
        let json_content = serde_json::to_string_pretty(self)?;
        fs::write(format!("{}.json", base_path), json_content)?;

        // TOML
        let toml_content = toml::to_string_pretty(self)?;
        fs::write(format!("{}.toml", base_path), toml_content)?;

        // RDF/Turtle
        let rdf_content = self.to_rdf_turtle();
        fs::write(format!("{}.ttl", base_path), rdf_content)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    println!("🔬 Rust Semantic Web - Ontology Aggregator");
    
    let usage_data_dir = Path::new("../../usage_data");
    let ontology = RustOntology::from_usage_data(usage_data_dir)?;
    
    ontology.save_all_formats("rust_ontology")?;
    
    println!("✅ Generated ontology files:");
    println!("   - rust_ontology.json");
    println!("   - rust_ontology.toml"); 
    println!("   - rust_ontology.ttl");
    println!();
    println!("📊 Statistics:");
    println!("   Files processed: {}", ontology.metadata.total_files_processed);
    println!("   Total symbols: {}", ontology.statistics.total_symbols);
    println!("   Total usages: {}", ontology.statistics.total_usages);
    
    if let Some(top) = ontology.statistics.top_symbols.first() {
        println!("   Top symbol: {} ({} uses)", top.symbol, top.count);
    }
    
    Ok(())
}
