#![recursion_limit = "512"]
#![allow(internal_features)]
#![allow(unused)]
#![allow(rustc::untranslatable_diagnostic)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(panic_backtrace_config)]
#![feature(panic_update_hook)]
#![feature(rustdoc_internals)]
#![feature(try_blocks)]
#![feature(assert_matches)]
#![feature(error_reporter)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use syn::{visit::Visit, ItemFn, ItemStruct, ItemEnum, ItemType, ItemConst};
use lib_introspector_core::transformations::process_content;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: String,
    pub crate_name: String,
    pub file_path: String,
}

#[derive(Debug, Clone)]
pub struct Usage {
    pub symbol: String,
    pub used_in_crate: String,
    pub used_in_file: String,
    pub usage_type: String,
}

struct SymbolExtractor {
    symbols: Vec<Symbol>,
    usages: Vec<Usage>,
    current_crate: String,
    current_file: String,
}

impl SymbolExtractor {
    fn new(crate_name: String, file_path: String) -> Self {
        Self {
            symbols: Vec::new(),
            usages: Vec::new(),
            current_crate: crate_name,
            current_file: file_path,
        }
    }
}

impl<'ast> Visit<'ast> for SymbolExtractor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.symbols.push(Symbol {
            name: node.sig.ident.to_string(),
            symbol_type: "function".to_string(),
            crate_name: self.current_crate.clone(),
            file_path: self.current_file.clone(),
        });
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.symbols.push(Symbol {
            name: node.ident.to_string(),
            symbol_type: "struct".to_string(),
            crate_name: self.current_crate.clone(),
            file_path: self.current_file.clone(),
        });
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        self.symbols.push(Symbol {
            name: node.ident.to_string(),
            symbol_type: "enum".to_string(),
            crate_name: self.current_crate.clone(),
            file_path: self.current_file.clone(),
        });
        syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        self.symbols.push(Symbol {
            name: node.ident.to_string(),
            symbol_type: "type_alias".to_string(),
            crate_name: self.current_crate.clone(),
            file_path: self.current_file.clone(),
        });
        syn::visit::visit_item_type(self, node);
    }

    fn visit_item_const(&mut self, node: &'ast ItemConst) {
        self.symbols.push(Symbol {
            name: node.ident.to_string(),
            symbol_type: "const".to_string(),
            crate_name: self.current_crate.clone(),
            file_path: self.current_file.clone(),
        });
        syn::visit::visit_item_const(self, node);
    }
}

fn preprocess_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Add all necessary features and macro definitions
    let prelude = r#"
// Injected macro definitions for parsing
macro_rules! mkitem { ($($item:tt)*) => { $($item)* }; }
macro_rules! mkfn { ($($fn:tt)*) => { $($fn)* }; }
macro_rules! mkmod { ($($mod:tt)*) => { $($mod)* }; }
macro_rules! mkuse { ($($use:tt)*) => { $($use)* }; }

"#;
    let with_prelude = format!("{}{}", prelude, content);
    
    // Apply lib-introspector transformations
    process_content(&with_prelude)
}

fn analyze_crate(crate_name: &str, base_path: &Path) -> Result<(Vec<Symbol>, Vec<Usage>), Box<dyn std::error::Error>> {
    let mut all_symbols = Vec::new();
    let mut all_usages = Vec::new();
    
    let crate_path = base_path.join("submodules/rust/compiler").join(crate_name);
    
    if !crate_path.exists() {
        println!("⚠️  Crate path not found: {}", crate_path.display());
        return Ok((all_symbols, all_usages));
    }

    for entry in WalkDir::new(&crate_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let file_path = entry.path();
            let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);
            
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    match preprocess_content(&content) {
                        Ok(processed_content) => {
                            match syn::parse_file(&processed_content) {
                                Ok(ast) => {
                                    let mut extractor = SymbolExtractor::new(
                                        crate_name.to_string(),
                                        relative_path.to_string_lossy().to_string()
                                    );
                                    extractor.visit_file(&ast);
                                    all_symbols.extend(extractor.symbols);
                                    all_usages.extend(extractor.usages);
                                }
                                Err(e) => {
                                    println!("⚠️  Parse error in {}: {}", relative_path.display(), e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("⚠️  Preprocessing error in {}: {}", relative_path.display(), e);
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️  Read error {}: {}", relative_path.display(), e);
                }
            }
        }
    }
    
    Ok((all_symbols, all_usages))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let level_0_crates = vec![
        "rustc_middle", "rustc_macros", "rustc_data_structures", "rustc_session", 
        "rustc_hir", "rustc_ast", "rustc_serialize", "rustc_index", "rustc_arena",
        "rustc_fs_util", "rustc_lexer", "rustc_graphviz", "rustc_log", "rustc_thread_pool",
        "rustc_error_codes", "rustc_baked_icu_data", "rustc_llvm", "rustc_infer",
        "rustc_trait_selection", "rustc_pattern_analysis", "rustc_expand", "rustc_ast_passes",
        "rustc_ast_pretty", "rustc_metadata", "rustc_interface", "rustc_driver",
        "rustc_attr_parsing", "rustc_fluent_macro", "rustc_parse_format", "rustc_index_macros",
        "rustc_type_ir_macros", "rustc_next_trait_solver", "rustc_sanitizers", "rustc_query_system"
    ];

    let base_path = Path::new(".");
    let mut total_symbols = 0;
    let mut total_usages = 0;
    let mut crate_stats = HashMap::new();

    println!("🔍 Analyzing Level 0 Crates (35 independent crates)");
    println!("{}", "=".repeat(60));

    for (i, crate_name) in level_0_crates.iter().enumerate() {
        print!("📦 [{:2}/35] Analyzing {} ... ", i + 1, crate_name);
        
        match analyze_crate(crate_name, &base_path) {
            Ok((symbols, usages)) => {
                let symbol_count = symbols.len();
                let usage_count = usages.len();
                
                crate_stats.insert(crate_name.to_string(), (symbol_count, usage_count));
                total_symbols += symbol_count;
                total_usages += usage_count;
                
                println!("✅ {} symbols, {} usages", symbol_count, usage_count);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                crate_stats.insert(crate_name.to_string(), (0, 0));
            }
        }
    }

    println!("\n📊 Level 0 Analysis Summary");
    println!("{}", "=".repeat(60));
    println!("Total symbols extracted: {}", total_symbols);
    println!("Total usages found: {}", total_usages);
    println!("Crates processed: {}/35", crate_stats.len());

    println!("\n🏆 Top Symbol Producers:");
    let mut sorted_crates: Vec<_> = crate_stats.iter().collect();
    sorted_crates.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    
    for (crate_name, (symbols, usages)) in sorted_crates.iter().take(10) {
        println!("  {:25} {:6} symbols, {:6} usages", crate_name, symbols, usages);
    }

    // Save results for Level 1 reuse
    let results = serde_json::json!({
        "level": 0,
        "total_symbols": total_symbols,
        "total_usages": total_usages,
        "crate_stats": crate_stats,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    fs::write("level_0_analysis.json", serde_json::to_string_pretty(&results)?)?;
    println!("\n💾 Results saved to level_0_analysis.json");
    println!("🚀 Ready for Level 1 analysis");

    Ok(())
}
