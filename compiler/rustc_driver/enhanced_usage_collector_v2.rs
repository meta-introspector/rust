#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::LOCAL_CRATE;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
struct EnhancedUsageEntry {
    usage: String,
    usage_count: usize,
    usage_type: String,
    node_type: String,
}

struct EnhancedUsageCollector {
    module_data: HashMap<String, Vec<EnhancedUsageEntry>>,
}

impl EnhancedUsageCollector {
    fn new() -> Self {
        Self {
            module_data: HashMap::new(),
        }
    }
    
    fn add_usage(&mut self, module: &str, usage: String, usage_type: String, node_type: String) {
        let entry = EnhancedUsageEntry {
            usage,
            usage_count: 1,
            usage_type,
            node_type,
        };
        self.module_data.entry(module.to_string()).or_insert_with(Vec::new).push(entry);
    }
    
    fn save_to_files(&self, crate_name: &str) {
        let output_dir = format!("{}/usage_data", std::env::current_dir().unwrap().display());
        std::fs::create_dir_all(&output_dir).unwrap();
        
        for (module, entries) in &self.module_data {
            let filename = format!("{}/{}_{}.json", output_dir, crate_name, module.replace("::", "_"));
            let mut file = File::create(&filename).unwrap();
            
            writeln!(file, "{{").unwrap();
            writeln!(file, "  \"crate\": \"{}\",", crate_name).unwrap();
            writeln!(file, "  \"module\": \"{}\",", module).unwrap();
            writeln!(file, "  \"enhanced_entries\": [").unwrap();
            
            for (i, entry) in entries.iter().enumerate() {
                let comma = if i == entries.len() - 1 { "" } else { "," };
                writeln!(file, "    {{").unwrap();
                writeln!(file, "      \"usage\": \"{}\",", entry.usage.replace("\"", "\\\"")).unwrap();
                writeln!(file, "      \"usage_count\": {},", entry.usage_count).unwrap();
                writeln!(file, "      \"usage_type\": \"{}\",", entry.usage_type).unwrap();
                writeln!(file, "      \"node_type\": \"{}\"", entry.node_type).unwrap();
                writeln!(file, "    }}{}", comma).unwrap();
            }
            
            writeln!(file, "  ]").unwrap();
            writeln!(file, "}}").unwrap();
            
            eprintln!("Saved {} enhanced entries to {}", entries.len(), filename);
        }
    }
}

impl Callbacks for EnhancedUsageCollector {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
            eprintln!("=== ENHANCED COLLECTION FOR CRATE: {} ===", crate_name);
            
            // Use the same approach as the working collector
            let hir = tcx.hir();
            for def_id in hir.body_owners() {
                let local_def_id = def_id;
                let def_id = local_def_id.to_def_id();
                
                if let Ok(typeck_results) = tcx.typeck(local_def_id) {
                    let module_name = tcx.def_path_str(def_id);
                    
                    for (hir_id, def_id) in typeck_results.type_dependent_defs() {
                        if let Some(def_id) = def_id {
                            let symbol_path = tcx.def_path_str(def_id);
                            let def_kind = tcx.def_kind(def_id);
                            let usage = format!("{} USES {} ({})", 
                                module_name, 
                                symbol_path, 
                                def_kind.descr(def_id)
                            );
                            
                            // Get node type
                            let node = tcx.hir().get(hir_id);
                            let node_type = format!("{:?}", node).split('(').next().unwrap_or("Unknown").to_string();
                            
                            // Classify usage type
                            let usage_type = match def_kind.descr(def_id) {
                                "AssocFn" => "MethodCall",
                                "AssocConst" => "ConstantAccess", 
                                "Variant" => "EnumVariant",
                                "Struct" => "StructUsage",
                                _ => "Other",
                            }.to_string();
                            
                            self.add_usage(&module_name, usage, usage_type, node_type);
                        }
                    }
                }
            }
            
            self.save_to_files(&crate_name);
            eprintln!("✅ ENHANCED COLLECTION COMPLETE");
        });
        
        Compilation::Stop
    }
}

fn main() {
    let mut collector = EnhancedUsageCollector::new();
    let mut args: Vec<String> = std::env::args().collect();
    args.insert(1, "--edition=2021".to_string());
    
    rustc_driver::RunCompiler::new(&args, &mut collector).run().unwrap();
}
