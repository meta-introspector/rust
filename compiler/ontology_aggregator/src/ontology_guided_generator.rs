use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Load existing ontology to guide enhanced collection
#[derive(Debug, Deserialize)]
struct ExistingOntology {
    symbols: BTreeMap<String, SymbolData>,
    statistics: OntologyStatistics,
}

#[derive(Debug, Deserialize)]
struct SymbolData {
    usage_count: usize,
    owl_type: String,
    contexts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OntologyStatistics {
    top_symbols: Vec<TopSymbol>,
}

#[derive(Debug, Deserialize)]
struct TopSymbol {
    symbol: String,
    count: usize,
    owl_type: String,
}

/// Enhanced usage collector that uses ontology knowledge
pub struct OntologyGuidedCollector {
    /// High-priority symbols to focus on (from ontology)
    priority_symbols: HashMap<String, usize>,
    /// Symbol type classifications
    symbol_types: HashMap<String, String>,
    /// Patterns to look for
    target_patterns: Vec<String>,
}

impl OntologyGuidedCollector {
    /// Create collector guided by existing ontology
    pub fn from_ontology(ontology_path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(ontology_path)?;
        let ontology: ExistingOntology = serde_json::from_str(&content)?;
        
        let mut priority_symbols = HashMap::new();
        let mut symbol_types = HashMap::new();
        
        // Focus on top symbols for detailed analysis
        for top_symbol in &ontology.statistics.top_symbols {
            priority_symbols.insert(top_symbol.symbol.clone(), top_symbol.count);
            symbol_types.insert(top_symbol.symbol.clone(), top_symbol.owl_type.clone());
        }
        
        // Add all symbols with significant usage
        for (symbol, data) in &ontology.symbols {
            if data.usage_count > 10 {  // Focus on frequently used symbols
                priority_symbols.insert(symbol.clone(), data.usage_count);
                symbol_types.insert(symbol.clone(), data.owl_type.clone());
            }
        }
        
        // Define target patterns based on ontology insights
        let target_patterns = vec![
            // TypeckResults patterns (from our learnings)
            "TypeckResults::node_type".to_string(),
            "TypeckResults::type_dependent_def_id".to_string(),
            "TypeckResults::expr_ty".to_string(),
            "LateContext::typeck_results".to_string(),
            "TyCtxt::typeck".to_string(),
            
            // Top symbols from ontology
            "rustc_proc_macro::quote::quote".to_string(),
            "rustc_attr_parsing::context".to_string(),
            "TyCtxt::<'tcx>::debug_stats".to_string(),
            "layout_of_uncached".to_string(),
            
            // Method call patterns
            "AssocFn".to_string(),
            "AssocConst".to_string(),
            "Variant".to_string(),
        ];
        
        println!("🎯 Ontology-guided collector initialized:");
        println!("   Priority symbols: {}", priority_symbols.len());
        println!("   Target patterns: {}", target_patterns.len());
        
        Ok(OntologyGuidedCollector {
            priority_symbols,
            symbol_types,
            target_patterns,
        })
    }
    
    /// Generate enhanced usage collector code
    pub fn generate_enhanced_collector(&self) -> String {
        format!(r#"
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;

use rustc_driver::{{Callbacks, Compilation}};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::TypeckResults;
use rustc_hir::{{def_id::LOCAL_CRATE, HirId, Node}};
use std::collections::{{HashMap, BTreeMap}};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
struct EnhancedUsageEntry {{
    usage: String,
    node_type: String,
    expr_type: Option<String>,
    count: usize,
    priority_score: usize,
    semantic_context: String,
}}

struct EnhancedUsageCollector {{
    module_data: HashMap<String, Vec<EnhancedUsageEntry>>,
    priority_symbols: HashMap<String, usize>,
    pattern_matches: HashMap<String, usize>,
}}

impl EnhancedUsageCollector {{
    fn new() -> Self {{
        let mut priority_symbols = HashMap::new();
        
        // Load priority symbols from ontology
        {}
        
        EnhancedUsageCollector {{
            module_data: HashMap::new(),
            priority_symbols,
            pattern_matches: HashMap::new(),
        }}
    }}
    
    fn collect_enhanced_usage(&mut self, tcx: TyCtxt<'_>) {{
        let hir = tcx.hir();
        
        for def_id in hir.body_owners() {{
            let local_def_id = def_id;
            let def_id = local_def_id.to_def_id();
            
            if let Ok(typeck_results) = tcx.typeck(local_def_id) {{
                let module_name = tcx.def_path_str(def_id);
                
                // Enhanced collection with semantic context
                for (hir_id, def_id_opt) in typeck_results.type_dependent_defs() {{
                    if let Some(target_def_id) = def_id_opt {{
                        let symbol_path = tcx.def_path_str(target_def_id);
                        let usage = format!("{{}} ({{}})", symbol_path, tcx.def_kind(target_def_id).descr(target_def_id));
                        
                        // Get detailed node information
                        let node = tcx.hir().get(*hir_id);
                        let node_type = self.classify_node_type(&node);
                        
                        // Get expression type with better error handling
                        let expr_type = typeck_results.node_type_opt(*hir_id)
                            .map(|ty| ty.to_string());
                        
                        // Calculate priority score
                        let priority_score = self.calculate_priority(&symbol_path);
                        
                        // Enhanced semantic context
                        let semantic_context = self.build_semantic_context(
                            &module_name, 
                            &node_type, 
                            expr_type.as_deref()
                        );
                        
                        let entry = EnhancedUsageEntry {{
                            usage,
                            node_type,
                            expr_type,
                            count: 1,
                            priority_score,
                            semantic_context,
                        }};
                        
                        self.module_data.entry(module_name.clone()).or_default().push(entry);
                        
                        // Track pattern matches
                        self.track_pattern_matches(&symbol_path);
                    }}
                }}
                
                // Collect additional TypeckResults patterns
                self.collect_typeck_patterns(tcx, &typeck_results, &module_name);
            }}
        }}
    }}
    
    fn classify_node_type(&self, node: &Node) -> String {{
        match node {{
            Node::Expr(expr) => format!("Expr::{{}}", expr.kind.variant_name()),
            Node::Stmt(stmt) => format!("Stmt::{{}}", stmt.kind.variant_name()),
            Node::Item(item) => format!("Item::{{}}", item.kind.variant_name()),
            Node::TraitItem(item) => format!("TraitItem::{{}}", item.kind.variant_name()),
            Node::ImplItem(item) => format!("ImplItem::{{}}", item.kind.variant_name()),
            Node::Pat(pat) => format!("Pat::{{}}", pat.kind.variant_name()),
            Node::Ty(ty) => format!("Ty::{{}}", ty.kind.variant_name()),
            _ => format!("{{:?}}", node).split('(').next().unwrap_or("Unknown").to_string(),
        }}
    }}
    
    fn calculate_priority(&self, symbol_path: &str) -> usize {{
        // Check exact match first
        if let Some(&count) = self.priority_symbols.get(symbol_path) {{
            return count * 10; // High priority for exact matches
        }}
        
        // Check partial matches for important patterns
        let high_priority_patterns = [
            "TypeckResults", "TyCtxt", "DefId", "HirId",
            "quote", "typeck", "layout", "intrinsic"
        ];
        
        for pattern in &high_priority_patterns {{
            if symbol_path.contains(pattern) {{
                return 100; // Medium priority for pattern matches
            }}
        }}
        
        1 // Default priority
    }}
    
    fn build_semantic_context(&self, module: &str, node_type: &str, expr_type: Option<&str>) -> String {{
        format!("{{}}::{{}}{{}}",
            module,
            node_type,
            expr_type.map(|t| format!("::{{}}", t)).unwrap_or_default()
        )
    }}
    
    fn track_pattern_matches(&mut self, symbol_path: &str) {{
        let target_patterns = [
            "TypeckResults::node_type",
            "TypeckResults::type_dependent_def_id", 
            "TypeckResults::expr_ty",
            "LateContext::typeck_results",
            "TyCtxt::typeck",
        ];
        
        for pattern in &target_patterns {{
            if symbol_path.contains(pattern) {{
                *self.pattern_matches.entry(pattern.to_string()).or_insert(0) += 1;
            }}
        }}
    }}
    
    fn collect_typeck_patterns(&mut self, tcx: TyCtxt<'_>, typeck_results: &TypeckResults<'_>, module_name: &str) {{
        // This is where we'd add specific TypeckResults pattern collection
        // based on our ontology learnings
        
        // Example: collect all node_type calls
        // Example: collect all type_dependent_def_id calls  
        // Example: collect expression type flows
    }}
    
    {}
}}

impl Callbacks for EnhancedUsageCollector {{
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {{
        queries.global_ctxt().unwrap().enter(|tcx| {{
            let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
            eprintln!("=== ENHANCED COLLECTION FOR CRATE: {{}} ===", crate_name);
            
            self.collect_enhanced_usage(tcx);
            self.save_enhanced_files(&crate_name);
            
            eprintln!("✅ ENHANCED COLLECTION COMPLETE");
            eprintln!("   Pattern matches: {{:?}}", self.pattern_matches);
        }});
        
        Compilation::Stop
    }}
}}

fn main() {{
    let mut collector = EnhancedUsageCollector::new();
    let mut args: Vec<String> = std::env::args().collect();
    args.insert(1, "--edition=2021".to_string());
    
    rustc_driver::RunCompiler::new(&args, &mut collector).run().unwrap();
}}
"#,
            self.generate_priority_symbols_code(),
            self.generate_save_method()
        )
    }
    
    fn generate_priority_symbols_code(&self) -> String {
        let mut code = Vec::new();
        for (symbol, count) in &self.priority_symbols {
            code.push(format!(
                r#"        priority_symbols.insert(r#"{}"#.to_string(), {});"#,
                symbol, count
            ));
        }
        code.join("\n")
    }
    
    fn generate_save_method(&self) -> String {
        r#"
    fn save_enhanced_files(&self, crate_name: &str) {
        let output_dir = format!("{}/enhanced_usage_data", std::env::current_dir().unwrap().display());
        std::fs::create_dir_all(&output_dir).unwrap();
        
        for (module, entries) in &self.module_data {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let full_name = format!("{}_{}", crate_name, module);
            let mut hasher = DefaultHasher::new();
            full_name.hash(&mut hasher);
            let hash = hasher.finish();
            
            let file_name = if full_name.len() > 100 {
                format!("{:x}.json", hash)
            } else {
                format!("{}_{}.json", crate_name, module.replace("::", "_").replace("<", "_").replace(">", "_").replace(" ", "_").replace(",", "_").replace("'", "_"))
            };
            
            let filename = format!("{}/{}", output_dir, file_name);
            let mut file = File::create(&filename).unwrap();
            
            writeln!(file, "{{").unwrap();
            writeln!(file, "  \"crate\": \"{}\",", crate_name).unwrap();
            writeln!(file, "  \"module\": \"{}\",", module).unwrap();
            writeln!(file, "  \"enhanced_entries\": [").unwrap();
            
            for (i, entry) in entries.iter().enumerate() {
                let comma = if i == entries.len() - 1 { "" } else { "," };
                writeln!(file, "    {{").unwrap();
                writeln!(file, "      \"usage\": \"{}\",", entry.usage.replace("\"", "\\\"")).unwrap();
                writeln!(file, "      \"node_type\": \"{}\",", entry.node_type).unwrap();
                writeln!(file, "      \"expr_type\": {},", 
                    entry.expr_type.as_ref().map(|t| format!("\"{}\"", t.replace("\"", "\\\"")))
                        .unwrap_or_else(|| "null".to_string())).unwrap();
                writeln!(file, "      \"count\": {},", entry.count).unwrap();
                writeln!(file, "      \"priority_score\": {},", entry.priority_score).unwrap();
                writeln!(file, "      \"semantic_context\": \"{}\"", entry.semantic_context).unwrap();
                writeln!(file, "    }}{}", comma).unwrap();
            }
            
            writeln!(file, "  ]").unwrap();
            writeln!(file, "}}").unwrap();
            
            eprintln!("Saved {} enhanced entries to {}", entries.len(), filename);
        }
    }"#.to_string()
    }
}

fn main() -> anyhow::Result<()> {
    println!("🔬 Generating Ontology-Guided Usage Collector");
    
    let ontology_path = Path::new("rust_ontology.json");
    let collector = OntologyGuidedCollector::from_ontology(ontology_path)?;
    
    let enhanced_code = collector.generate_enhanced_collector();
    fs::write("enhanced_usage_collector.rs", enhanced_code)?;
    
    println!("✅ Generated enhanced_usage_collector.rs");
    println!("   This collector focuses on high-priority symbols from the ontology");
    println!("   It captures detailed semantic context and type information");
    
    Ok(())
}
"#,
        self.generate_priority_symbols_code(),
        self.generate_save_method()
    )
}

fn generate_priority_symbols_code(&self) -> String {
    let mut code = Vec::new();
    for (symbol, count) in &self.priority_symbols {
        code.push(format!(
            r#"        priority_symbols.insert(r#"{}"#.to_string(), {});"#,
            symbol, count
        ));
    }
    code.join("\n")
}

fn generate_save_method(&self) -> String {
    r#"
fn save_enhanced_files(&self, crate_name: &str) {
    let output_dir = format!("{}/enhanced_usage_data", std::env::current_dir().unwrap().display());
    std::fs::create_dir_all(&output_dir).unwrap();
    
    for (module, entries) in &self.module_data {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let full_name = format!("{}_{}", crate_name, module);
        let mut hasher = DefaultHasher::new();
        full_name.hash(&mut hasher);
        let hash = hasher.finish();
        
        let file_name = if full_name.len() > 100 {
            format!("{:x}.json", hash)
        } else {
            format!("{}_{}.json", crate_name, module.replace("::", "_").replace("<", "_").replace(">", "_").replace(" ", "_").replace(",", "_").replace("'", "_"))
        };
        
        let filename = format!("{}/{}", output_dir, file_name);
        let mut file = File::create(&filename).unwrap();
        
        writeln!(file, "{{").unwrap();
        writeln!(file, "  \"crate\": \"{}\",", crate_name).unwrap();
        writeln!(file, "  \"module\": \"{}\",", module).unwrap();
        writeln!(file, "  \"enhanced_entries\": [").unwrap();
        
        for (i, entry) in entries.iter().enumerate() {
            let comma = if i == entries.len() - 1 { "" } else { "," };
            writeln!(file, "    {{").unwrap();
            writeln!(file, "      \"usage\": \"{}\",", entry.usage.replace("\"", "\\\"")).unwrap();
            writeln!(file, "      \"node_type\": \"{}\",", entry.node_type).unwrap();
            writeln!(file, "      \"expr_type\": {},", 
                entry.expr_type.as_ref().map(|t| format!("\"{}\"", t.replace("\"", "\\\"")))
                    .unwrap_or_else(|| "null".to_string())).unwrap();
            writeln!(file, "      \"count\": {},", entry.count).unwrap();
            writeln!(file, "      \"priority_score\": {},", entry.priority_score).unwrap();
            writeln!(file, "      \"semantic_context\": \"{}\"", entry.semantic_context).unwrap();
            writeln!(file, "    }}{}", comma).unwrap();
        }
        
        writeln!(file, "  ]").unwrap();
        writeln!(file, "}}").unwrap();
        
        eprintln!("Saved {} enhanced entries to {}", entries.len(), filename);
    }
}"#.to_string()
}

fn main() -> anyhow::Result<()> {
    println!("🔬 Generating Ontology-Guided Usage Collector");
    
    let ontology_path = Path::new("rust_ontology.json");
    let collector = OntologyGuidedCollector::from_ontology(ontology_path)?;
    
    let enhanced_code = collector.generate_enhanced_collector();
    fs::write("enhanced_usage_collector.rs", enhanced_code)?;
    
    println!("✅ Generated enhanced_usage_collector.rs");
    println!("   This collector focuses on high-priority symbols from the ontology");
    println!("   It captures detailed semantic context and type information");
    
    Ok(())
}
