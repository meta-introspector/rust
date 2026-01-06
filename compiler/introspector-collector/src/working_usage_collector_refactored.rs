#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_ast;

mod usage_types;
mod usage_classifier;
mod generic_tracker;
mod ast_extractor;
mod usage_collector;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_hir::intravisit::{self, Visitor};
use std::io::Write;
use serde::{Serialize, Deserialize};

use usage_types::*;
use usage_collector::UsageCollector;
use ast_extractor::AstExtractor;

struct UsageVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    collector: UsageCollector,
}

impl<'tcx> UsageVisitor<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            collector: UsageCollector::new(),
        }
    }
    
    fn collect_literals_from_expr<'a>(&mut self, expr: &'a rustc_hir::Expr<'a>, context: &str) {
        match &expr.kind {
            rustc_hir::ExprKind::Lit(lit) => {
                let literal_value = match &lit.node {
                    rustc_ast::LitKind::Str(s, _) => format!("\"{}\"", s),
                    rustc_ast::LitKind::Int(i, _) => format!("{}", i),
                    rustc_ast::LitKind::Float(f, _) => format!("{}", f),
                    rustc_ast::LitKind::Bool(b) => format!("{}", b),
                    rustc_ast::LitKind::Char(c) => format!("'{}'", c),
                    rustc_ast::LitKind::Byte(b) => format!("{}", b),
                    _ => "unknown_literal".to_string(),
                };
                
                self.collector.add_usage(
                    "literals",
                    literal_value,
                    "literal".to_string(),
                    "NumericLiteral".to_string(),
                    "Expression".to_string(),
                    context.to_string(),
                    "literal".to_string(),
                    None,
                    None
                );
            }
            _ => {}
        }
    }
}

impl<'tcx> Visitor<'tcx> for UsageVisitor<'tcx> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;
    
    fn nested_visit_map(&mut self) -> Self::NestedFilter {
        rustc_middle::hir::nested_filter::All
    }
    
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        self.collect_literals_from_expr(expr, "expr_context");
        intravisit::walk_expr(self, expr);
    }
}

struct UsageCallbacks;

impl Callbacks for UsageCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let mut visitor = UsageVisitor::new(tcx);
            let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
            
            // Debug environment
            eprintln!("=== ENV DEBUG ===");
            eprintln!("PWD: {:?}", std::env::var("PWD"));
            eprintln!("CARGO_MANIFEST_DIR: {:?}", std::env::var("CARGO_MANIFEST_DIR"));
            eprintln!("=== END ENV DEBUG ===");
            
            // Output directory setup
            let output_dir = std::env::var("USAGE_OUTPUT_DIR")
                .or_else(|_| {
                    std::env::var("CARGO_TARGET_DIR")
                        .map(|target_dir| format!("{}/harmonic/usage", target_dir))
                })
                .or_else(|_| {
                    std::env::var("CARGO_MANIFEST_DIR")
                        .and_then(|manifest_dir| {
                            std::env::var("PROFILE")
                                .map(|profile| format!("{}/target/{}/harmonic/usage", manifest_dir, profile))
                        })
                })
                .unwrap_or_else(|_| "usage_data".to_string());
            
            std::fs::create_dir_all(&output_dir).unwrap();
            
            // Process all items in the crate
            for item_id in tcx.hir().items() {
                let item = tcx.hir().item(item_id);
                let item_name = item.ident.to_string();
                
                match &item.kind {
                    // Struct definitions with GENERIC TRACKING (HIGH IMPACT)
                    rustc_hir::ItemKind::Struct(variant_data, generics, _) => {
                        visitor.collector.add_usage(
                            "structs",
                            item_name.clone(),
                            "struct_decl".to_string(),
                            "StructDecl".to_string(),
                            "Item".to_string(),
                            crate_name.clone(),
                            item_name.clone(),
                            Some(crate_name.clone()),
                            None
                        );
                        
                        // HIGH IMPACT: Track generic parameters (16 occurrences found)
                        visitor.collector.track_generics(generics, &item_name, &crate_name, "struct");
                        
                        // Extract struct fields
                        let fields = AstExtractor::extract_struct_fields(variant_data);
                        let complexity = match fields.len() {
                            0..=3 => "simple",
                            4..=8 => "medium", 
                            _ => "complex"
                        };
                        
                        visitor.collector.item_complexity.insert(item_name.clone(), ItemComplexity {
                            name: item_name.clone(),
                            item_type: "struct".to_string(),
                            complexity: complexity.to_string(),
                            field_count: Some(fields.len()),
                            variant_count: None,
                            param_count: None,
                            fields: Some(fields),
                            variants: None,
                            parameters: None,
                        });
                    }
                    
                    // Enum definitions with GENERIC TRACKING (HIGH IMPACT)
                    rustc_hir::ItemKind::Enum(enum_def, generics, _) => {
                        visitor.collector.add_usage(
                            "enums",
                            item_name.clone(),
                            "enum_decl".to_string(),
                            "EnumDecl".to_string(),
                            "Item".to_string(),
                            crate_name.clone(),
                            item_name.clone(),
                            Some(crate_name.clone()),
                            None
                        );
                        
                        // HIGH IMPACT: Track generic parameters (16 occurrences found)
                        visitor.collector.track_generics(generics, &item_name, &crate_name, "enum");
                        
                        // Extract enum variants
                        let variants = AstExtractor::extract_enum_variants(enum_def);
                        let complexity = match variants.len() {
                            0..=3 => "simple",
                            4..=8 => "medium",
                            _ => "complex"
                        };
                        
                        visitor.collector.item_complexity.insert(item_name.clone(), ItemComplexity {
                            name: item_name.clone(),
                            item_type: "enum".to_string(),
                            complexity: complexity.to_string(),
                            field_count: None,
                            variant_count: Some(variants.len()),
                            param_count: None,
                            fields: None,
                            variants: Some(variants.into_iter().map(|v| VariantInfo {
                                name: v.name,
                                has_fields: v.has_fields,
                                field_count: v.field_count,
                            }).collect()),
                            parameters: None,
                        });
                    }
                    
                    // Function definitions
                    rustc_hir::ItemKind::Fn(sig, generics, _) => {
                        visitor.collector.add_usage(
                            "functions",
                            item_name.clone(),
                            "fn_decl".to_string(),
                            "FnDecl".to_string(),
                            "Item".to_string(),
                            crate_name.clone(),
                            item_name.clone(),
                            Some(crate_name.clone()),
                            None
                        );
                        
                        // Track function generics
                        visitor.collector.track_generics(generics, &item_name, &crate_name, "function");
                        
                        // Extract function parameters
                        let params = AstExtractor::extract_function_params(sig);
                        visitor.collector.item_complexity.insert(item_name.clone(), ItemComplexity {
                            name: item_name.clone(),
                            item_type: "function".to_string(),
                            complexity: "simple".to_string(),
                            field_count: None,
                            variant_count: None,
                            param_count: Some(params.len()),
                            fields: None,
                            variants: None,
                            parameters: Some(params),
                        });
                    }
                    
                    // Constants
                    rustc_hir::ItemKind::Const(_, _, _, body_id) => {
                        visitor.collector.add_usage(
                            "constants",
                            item_name.clone(),
                            "const_decl".to_string(),
                            "ConstDecl".to_string(),
                            "Item".to_string(),
                            crate_name.clone(),
                            item_name.clone(),
                            Some(crate_name.clone()),
                            None
                        );
                        
                        let body = tcx.hir_body(*body_id);
                        visitor.collect_literals_from_expr(&body.value, &format!("const {}", item_name));
                    }
                    
                    // Static items
                    rustc_hir::ItemKind::Static(_, _, _, body_id) => {
                        visitor.collector.add_usage(
                            "statics",
                            item_name.clone(),
                            "static_decl".to_string(),
                            "StaticDecl".to_string(),
                            "Item".to_string(),
                            crate_name.clone(),
                            item_name.clone(),
                            Some(crate_name.clone()),
                            None
                        );
                        
                        let body = tcx.hir_body(*body_id);
                        visitor.collect_literals_from_expr(&body.value, &format!("static {}", item_name));
                    }
                    
                    _ => {}
                }
            }
            
            // Save collected data
            let total_usages: usize = visitor.collector.module_data.values().map(|v| v.len()).sum();
            eprintln!("=== SAVED {} COMPLEXITY ITEMS FOR CRATE: {} ===", visitor.collector.item_complexity.len(), crate_name);
            
            // Save manifest
            let manifest_path = format!("{}/{}_manifest.json", output_dir, crate_name);
            let manifest_data = serde_json::json!({
                "crate": crate_name,
                "complexity_items": visitor.collector.item_complexity.len(),
                "total_usages": total_usages,
                "rustc_version": std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            
            if let Ok(mut file) = std::fs::File::create(&manifest_path) {
                if let Ok(json_str) = serde_json::to_string_pretty(&manifest_data) {
                    let _ = file.write_all(json_str.as_bytes());
                }
            }
            eprintln!("=== MANIFEST SAVED: {} ===", manifest_path);
            
            // Save usage data for each module
            for (module, usages) in &visitor.collector.module_data {
                let module_data = ModuleData {
                    crate_name: crate_name.clone(),
                    module: module.clone(),
                    usages: usages.clone(),
                };
                
                let filename = format!("{}_{}.json", crate_name, module);
                let filepath = format!("{}/{}", output_dir, filename);
                
                if let Ok(mut file) = std::fs::File::create(&filepath) {
                    if let Ok(json_str) = serde_json::to_string_pretty(&module_data) {
                        let _ = file.write_all(json_str.as_bytes());
                    }
                }
            }
            
            eprintln!("=== COLLECTING USAGE DATA FOR CRATE: {} === ({} total usages)", crate_name, total_usages);
        });
        
        Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut callbacks = UsageCallbacks;
    
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    std::process::exit(result.unwrap_or(1));
}
