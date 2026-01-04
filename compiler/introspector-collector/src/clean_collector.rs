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
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fs;

// #[derive(Serialize, Deserialize, Debug)]
// struct CleanCompilerData {
//     // Clean call graph (DefId -> DefId only)
//     call_graph: HashMap<String, Vec<String>>,
    
//     // Symbol table (DefId -> readable name)
//     symbol_names: HashMap<String, String>,
    
//     // DefId -> source path
//     def_paths: HashMap<String, String>,
    
//     // Metadata
//     crate_name: String,
//     total_items: usize,
// }

// impl CleanCompilerData {
//     fn new(crate_name: String) -> Self {
//         Self {
//             call_graph: HashMap::new(),
//             symbol_names: HashMap::new(),
//             def_paths: HashMap::new(),
//             crate_name,
//             total_items: 0,
//         }
//     }
    
//     fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
//         let filename = format!("clean_compiler_data_{}.json", self.crate_name);
//         let json = serde_json::to_string_pretty(self)?;
//         fs::write(&filename, json)?;
//         println!("💾 Saved clean compiler data to {}", filename);
//         Ok(())
//     }
// }

// struct CleanCollector {
//     data: CleanCompilerData,
// }

// impl CleanCollector {
//     fn new(crate_name: String) -> Self {
//         Self {
//             data: CleanCompilerData::new(crate_name),
//         }
//     }
    
//     fn collect_clean_data(&mut self, tcx: TyCtxt<'_>) {
//         println!("🔍 Collecting clean compiler data...");
        
//         // Use the compatibility macros from the existing collector
//         use crate::{get_crate_items, get_item_name};
        
//         // Collect all items and build symbol table
//         for owner_id in get_crate_items!(tcx) {
//             let def_id = owner_id.to_def_id();
//             let def_id_str = format!("{:?}", def_id);
            
//             // Get symbol name
//             let symbol_name = tcx.item_name(def_id).to_string();
//             self.data.symbol_names.insert(def_id_str.clone(), symbol_name);
            
//             // Get def path
//             let def_path = tcx.def_path_str(def_id);
//             self.data.def_paths.insert(def_id_str, def_path);
            
//             self.data.total_items += 1;
//         }
        
//         println!("  Items collected: {}", self.data.total_items);
//         println!("  Symbol names: {}", self.data.symbol_names.len());
//         println!("  Def paths: {}", self.data.def_paths.len());
//     }
// }

// // Compatibility macros (copied from existing collector)
// macro_rules! get_crate_items {
//     ($tcx:expr) => {
//         $tcx.hir_crate_items(()).owners()
//     };
// }

// macro_rules! get_item_name {
//     ($tcx:expr, $item:expr) => {
//         $tcx.item_name($item.owner_id.to_def_id()).to_string()
//     };
// }

// struct CleanCompilerCalls;

// impl Callbacks for CleanCompilerCalls {
//     fn after_analysis<'tcx>(
//         &mut self,
//         _compiler: &interface::Compiler,
//         queries: &'tcx rustc_interface::Queries<'tcx>,
//     ) -> Compilation {
//         queries.global_ctxt().unwrap().enter(|tcx| {
//             let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
//             println!("=== COLLECTING CLEAN DATA FOR CRATE: {} ===", crate_name);
            
//             let mut collector = CleanCollector::new(crate_name);
//             collector.collect_clean_data(tcx);
            
//             if let Err(e) = collector.data.save_to_disk() {
//                 eprintln!("Error saving clean data: {}", e);
//             }
            
//             println!("✅ CLEAN COLLECTION COMPLETE");
//         });
        
//         Compilation::Continue
//     }
// }

fn main() {
//     let mut args: Vec<String> = std::env::args().collect();
    
//     if args.len() > 1 && (args[1].ends_with(".rs") || args.contains(&"--crate-name".to_string())) {
//         rustc_driver::RunCompiler::new(&args, &mut CleanCompilerCalls).run().unwrap();
//     } else {
//         args[0] = "rustc".to_string();
//         rustc_driver::RunCompiler::new(&args, &mut rustc_driver::DefaultCallbacks).run().unwrap();
//     }
}
