// this should be merged into working_usage_collector.rs
// #![feature(rustc_private)]

// extern crate rustc_driver;
// extern crate rustc_interface;
// extern crate rustc_middle;
// extern crate rustc_hir;

// use rustc_driver::{Callbacks, Compilation};
// use rustc_interface::interface;
// use rustc_middle::ty::TyCtxt;
// use rustc_hir::def_id::LOCAL_CRATE;
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::Write;

// #[derive(Debug, Clone)]
// struct SimpleUsageEntry {
//     usage: String,
//     count: usize,
// }

// struct SimpleUsageCollector {
//     module_data: HashMap<String, Vec<SimpleUsageEntry>>,
// }

// impl SimpleUsageCollector {
//     fn new() -> Self {
//         SimpleUsageCollector {
//             module_data: HashMap::new(),
//         }
//     }

//     fn collect_usage(&mut self, tcx: TyCtxt<'_>) {
//         // Collect from body owners (functions with bodies)
//         for def_id in tcx.hir_crate_items(()).owners() {
//             let local_def_id = def_id;
//             let def_id = local_def_id.to_def_id();
            
//             let typeck_results = tcx.typeck(local_def_id);
//             let module_name = tcx.def_path_str(def_id);
            
//             // Collect type-dependent definitions (method calls, etc.)
//             for (hir_id, def_id_opt) in typeck_results.type_dependent_defs() {
//                 if let Some(target_def_id) = def_id_opt {
//                     let symbol_path = tcx.def_path_str(*target_def_id);
//                     let def_kind = tcx.def_kind(*target_def_id);
//                     let usage = format!("{} ({})", symbol_path, def_kind.descr(*target_def_id));
                    
//                     let entry = SimpleUsageEntry {
//                         usage,
//                         count: 1,
//                     };
                    
//                     self.module_data.entry(module_name.clone()).or_default().push(entry);
//                 }
//             }
//         }
//     }
    
//     fn save_files(&self, crate_name: &str) {
//         let output_dir = format!("{}/enhanced_usage_data", std::env::current_dir().unwrap().display());
//         std::fs::create_dir_all(&output_dir).unwrap();
        
//         for (module, entries) in &self.module_data {
//             use std::collections::hash_map::DefaultHasher;
//             use std::hash::{Hash, Hasher};
            
//             let full_name = format!("{}_{}", crate_name, module);
//             let mut hasher = DefaultHasher::new();
//             full_name.hash(&mut hasher);
//             let hash = hasher.finish();
            
//             let file_name = if full_name.len() > 100 {
//                 format!("{:x}.json", hash)
//             } else {
//                 format!("{}_{}.json", crate_name, module.replace("::", "_").replace("<", "_").replace(">", "_").replace(" ", "_").replace(",", "_").replace("'", "_"))
//             };
            
//             let filename = format!("{}/{}", output_dir, file_name);
//             let mut file = File::create(&filename).unwrap();
            
//             writeln!(file, "{{").unwrap();
//             writeln!(file, "  \"crate\": \"{}\",", crate_name).unwrap();
//             writeln!(file, "  \"module\": \"{}\",", module).unwrap();
//             writeln!(file, "  \"simple_entries\": [").unwrap();
            
//             for (i, entry) in entries.iter().enumerate() {
//                 let comma = if i == entries.len() - 1 { "" } else { "," };
//                 writeln!(file, "    {{").unwrap();
//                 writeln!(file, "      \"usage\": \"{}\",", entry.usage.replace("\"", "\\\"")).unwrap();
//                 writeln!(file, "      \"count\": {}", entry.count).unwrap();
//                 writeln!(file, "    }}{}", comma).unwrap();
//             }
            
//             writeln!(file, "  ]").unwrap();
//             writeln!(file, "}}").unwrap();
            
//             eprintln!("Saved {} entries to {}", entries.len(), filename);
//         }
//     }
// }

// impl Callbacks for SimpleUsageCollector {
//     fn after_analysis<'tcx>(
//         &mut self,
//         _compiler: &interface::Compiler,
//         queries: &'tcx rustc_interface::interface::Queries<'tcx>,
//     ) -> Compilation {
//         queries.global_ctxt().unwrap().enter(|tcx| {
//             let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
//             eprintln!("=== SIMPLE COLLECTION FOR CRATE: {} ===", crate_name);
            
//             self.collect_usage(tcx);
//             self.save_files(&crate_name);
            
//             eprintln!("✅ SIMPLE COLLECTION COMPLETE");
//         });
        
//         Compilation::Stop
//     }
// }

fn main() {
//     let mut collector = SimpleUsageCollector::new();
//     let mut args: Vec<String> = std::env::args().collect();
//     args.insert(1, "--edition=2021".to_string());
    
//     let exit_code = rustc_driver::catch_with_exit_code(|| {
//         let mut callbacks = collector;
//         rustc_driver::run_compiler(&args, &mut callbacks)
//     });
    
//     std::process::exit(exit_code);
}
