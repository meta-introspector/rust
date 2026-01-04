// this code should be in working_usage_collector.rs now
// 
// #![feature(rustc_private)]

// extern crate rustc_driver;
// extern crate rustc_interface;
// extern crate rustc_middle;
// extern crate rustc_hir;

// use rustc_driver::{Callbacks, Compilation, run_compiler};
// use rustc_interface::interface;
// use rustc_middle::ty::TyCtxt;
// use rustc_hir::def_id::DefId;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fs;

// #[derive(Serialize, Deserialize, Debug)]
// struct EnumUsageData {
//     enum_definitions: HashMap<String, String>,
//     enum_usages: HashMap<String, Vec<String>>,
// }

// impl EnumUsageData {
//     fn new() -> Self {
//         Self {
//             enum_definitions: HashMap::new(),
//             enum_usages: HashMap::new(),
//         }
//     }
// }

// struct EnumUsageCollector {
//     data: EnumUsageData,
// }

// impl EnumUsageCollector {
//     fn new() -> Self {
//         Self {
//             data: EnumUsageData::new(),
//         }
//     }
    
//     fn collect_enum_data(&mut self, tcx: TyCtxt<'_>) {
//         for item_id in tcx.hir_crate_items(()).owners() {
//             let item = tcx.hir().expect_item(item_id.def_id);
            
//             if let rustc_hir::ItemKind::Enum(enum_def, _) = &item.kind {
//                 let enum_name = item.ident.to_string();
//                 let def_id = item_id.def_id.to_def_id();
                
//                 // Store enum definition
//                 self.data.enum_definitions.insert(
//                     enum_name.clone(),
//                     format!("{:?}", enum_def)
//                 );
                
//                 // Collect variants
//                 let mut variants = Vec::new();
//                 for variant in enum_def.variants {
//                     variants.push(variant.ident.to_string());
//                 }
                
//                 self.data.enum_usages.insert(enum_name, variants);
//             }
//         }
//     }
    
//     fn save_data(&self) {
//         let json = serde_json::to_string_pretty(&self.data).unwrap();
//         fs::write("enum_usage_data.json", json).unwrap();
//         println!("✅ Saved enum usage data to enum_usage_data.json");
//     }
// }

// struct EnumCompilerCalls;

// impl Callbacks for EnumCompilerCalls {
//     fn after_analysis<'tcx>(
//         &mut self,
//         _compiler: &interface::Compiler,
//         queries: &'tcx interface::Queries<'tcx>,
//     ) -> Compilation {
//         queries.global_ctxt().unwrap().enter(|tcx| {
//             let mut collector = EnumUsageCollector::new();
//             collector.collect_enum_data(tcx);
//             collector.save_data();
//         });
        
//         Compilation::Stop
//     }
// }

 fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     let mut callbacks = EnumCompilerCalls;
//     run_compiler(&args, &mut callbacks, None, None);
 }
