#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::TypeckResults;
use rustc_hir::{def_id::LOCAL_CRATE, HirId, Node};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
struct UsageEntry {
    usage: String,
    node_type: String,
    expr_type: Option<String>,
    count: usize,
}

struct UsageCollector {
    module_data: HashMap<String, Vec<UsageEntry>>,
}

impl UsageCollector {
    fn new() -> Self {
        Self {
            module_data: HashMap::new(),
        }
    }
    
    fn add_usage(&mut self, module: &str, usage: String) {
        self.module_data.entry(module.to_string()).or_insert_with(Vec::new).push(usage);
    }
    
    fn save_to_files(&self, crate_name: &str) {
        let output_dir = format!("{}/usage_data", std::env::current_dir().unwrap().display());
        std::fs::create_dir_all(&output_dir).unwrap();
        
        for (module, usages) in &self.module_data {
            // Use hash for long names to avoid filesystem limits
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let full_name = format!("{}_{}", crate_name, module);
            let mut hasher = DefaultHasher::new();
            full_name.hash(&mut hasher);
            let hash = hasher.finish();
            
            let clean_name = format!("{}_{}", crate_name, module.replace("::", "_").replace("<", "_").replace(">", "_").replace(" ", "_").replace(",", "_").replace("'", "_"));
            let file_name = if clean_name.len() > 64 {
                format!("{}_{:x}.json", &clean_name[..40], hash)
            } else {
                format!("{}.json", clean_name)
            };
            
            let filename = format!("{}/{}", output_dir, file_name);
            let mut file = File::create(&filename).unwrap();
            
            writeln!(file, "{{").unwrap();
            writeln!(file, "  \"crate\": \"{}\",", crate_name).unwrap();
            writeln!(file, "  \"module\": \"{}\",", module).unwrap();
            writeln!(file, "  \"usages\": [").unwrap();
            
            for (i, usage) in usages.iter().enumerate() {
                let comma = if i == usages.len() - 1 { "" } else { "," };
                writeln!(file, "    \"{}\"{}", usage, comma).unwrap();
            }
            
            writeln!(file, "  ]").unwrap();
            writeln!(file, "}}").unwrap();
            

        }
    }
}

impl Callbacks for UsageCollector {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let local_crate = tcx.crate_name(LOCAL_CRATE);
        let all_items = tcx.hir_crate_items(());
        
        eprintln!("=== COLLECTING USAGE DATA FOR CRATE: {} ===", local_crate);
        
        for item_id in all_items.free_items() {
            let def_id = item_id.owner_id.to_def_id();
            let item_path = tcx.def_path_str(def_id);
            let def_kind = tcx.def_kind(def_id);
            
            let (module, decl) = if item_path.contains("::") {
                let parts: Vec<&str> = item_path.split("::").collect();
                (parts[..parts.len()-1].join("::"), parts[parts.len()-1])
            } else {
                ("root".to_string(), item_path.as_str())
            };
            
            if !matches!(def_kind, rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::Const | rustc_hir::def::DefKind::Static { .. }) {
                continue;
            }
            
            let user_location = format!("crate::{}::{}::{}", local_crate, module, decl);
            let typeck = tcx.typeck(item_id.owner_id.def_id);
            
            for (_local_id, result) in typeck.type_dependent_defs().items_in_stable_order() {
                if let Ok((def_kind, used_def_id)) = result {
                    let used_crate = tcx.crate_name(used_def_id.krate);
                    let used_path = tcx.def_path_str(*used_def_id);
                    let used_location = format!("crate::{}::{}", used_crate, used_path);
                    let usage = format!("{} USES {} ({:?})", user_location, used_location, def_kind);
                    
                    self.add_usage(&module, usage);
                }
            }
        }
        
        self.save_to_files(&local_crate.to_string());
        eprintln!("✅ COLLECTION COMPLETE");
        
        Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // When used as RUSTC_WRAPPER, cargo calls us like: wrapper rustc [args...]
    let rustc_args: Vec<String> = if args.len() > 1 && args[1] == "rustc" {
        args[2..].to_vec()
    } else {
        args[1..].to_vec()
    };
    
    // For --print queries and other non-compilation tasks, just pass through to rustc
    if rustc_args.iter().any(|arg| arg.starts_with("--print") || arg == "--version" || arg == "-V") {
        let mut cmd = std::process::Command::new("rustc");
        cmd.args(&rustc_args);
        std::process::exit(cmd.status().unwrap().code().unwrap_or(1));
    }
    
    let mut callbacks = UsageCollector::new();
    let result = rustc_driver::catch_fatal_errors(|| {
        let mut full_args = vec!["rustc".to_string()];
        full_args.extend(rustc_args);
        rustc_driver::run_compiler(&full_args, &mut callbacks)
    });
    
    std::process::exit(match result { Ok(_) => 0, Err(_) => 1 });
}
