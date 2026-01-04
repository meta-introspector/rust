// this should be merged into working_usage_collector.rs
// #![feature(rustc_private)]

// extern crate rustc_driver;
// extern crate rustc_interface;
// extern crate rustc_middle;
// extern crate rustc_hir;
// extern crate rustc_span;

// use rustc_driver::{Callbacks, Compilation, run_compiler};
// use rustc_interface::interface;
// use rustc_middle::ty::TyCtxt;
// use rustc_hir::def_id::DefId;
// use rustc_hir::intravisit::{self, Visitor};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fs;

// #[derive(Serialize, Deserialize, Debug)]
// struct CompilerData {
//     call_graph: HashMap<String, Vec<String>>,
//     hir_items: HashMap<String, String>,
//     def_paths: HashMap<String, String>,
//     symbol_names: HashMap<String, String>,
//     type_info: HashMap<String, String>,
//     source_map: HashMap<String, String>,
// }

// impl CompilerData {
//     fn new() -> Self {
//         Self {
//             call_graph: HashMap::new(),
//             hir_items: HashMap::new(),
//             def_paths: HashMap::new(),
//             symbol_names: HashMap::new(),
//             type_info: HashMap::new(),
//             source_map: HashMap::new(),
//         }
//     }
// }

// struct EnhancedCollector {
//     data: CompilerData,
// }

// impl EnhancedCollector {
//     fn new() -> Self {
//         Self {
//             data: CompilerData::new(),
//         }
//     }
    
//     fn collect_all_data(&mut self, tcx: TyCtxt<'_>) {
//         println!("🔍 Collecting comprehensive compiler data...");
        
//         self.collect_hir_data(tcx);
//         self.collect_symbol_table(tcx);
//         self.collect_type_info(tcx);
//         self.collect_source_locations(tcx);
//         self.build_clean_call_graph(tcx);
//     }
    
//     fn collect_hir_data(&mut self, tcx: TyCtxt<'_>) {
//         for item_id in tcx.hir_crate_items(()).owners() {
//             let item = tcx.hir_expect_item(item_id.owner_id.def_id);
//             let item_str = format!("{:#?}", item);
//             self.data.hir_items.insert(format!("{:?}", item_id), item_str);
//         }
        
//         println!("  HIR items: {}", self.data.hir_items.len());
//     }
    
//     fn collect_symbol_table(&mut self, tcx: TyCtxt<'_>) {
//         for item_id in tcx.hir_crate_items(()).owners() {
//             let def_id = item_id.owner_id.def_id.to_def_id();
//             let def_path = tcx.def_path_str(def_id);
//             let symbol_name = tcx.item_name(def_id).to_string();
            
//             let def_id_str = format!("{:?}", def_id);
//             self.data.def_paths.insert(def_id_str.clone(), def_path);
//             self.data.symbol_names.insert(def_id_str, symbol_name);
//         }
        
//         println!("  Symbol entries: {}", self.data.def_paths.len());
//     }
    
//     fn collect_type_info(&mut self, tcx: TyCtxt<'_>) {
//         for item_id in tcx.hir_crate_items(()).owners() {
//             let def_id = item_id.owner_id.def_id.to_def_id();
//             let ty = tcx.type_of(def_id).instantiate_identity();
//             let type_str = format!("{:?}", ty);
//             self.data.type_info.insert(format!("{:?}", def_id), type_str);
//         }
        
//         println!("  Type entries: {}", self.data.type_info.len());
//     }
    
//     fn collect_source_locations(&mut self, tcx: TyCtxt<'_>) {
//         let source_map = tcx.sess.source_map();
        
//         for item_id in tcx.hir_crate_items(()).owners() {
//             let def_id = item_id.owner_id.def_id.to_def_id();
//             let span = tcx.def_span(def_id);
//             let loc = source_map.span_to_string(span, rustc_span::FileNameDisplayPreference::Short);
//             self.data.source_map.insert(format!("{:?}", def_id), loc);
//         }
        
//         println!("  Source locations: {}", self.data.source_map.len());
//     }
    
//     fn build_clean_call_graph(&mut self, tcx: TyCtxt<'_>) {
//         for item_id in tcx.hir_crate_items(()).owners() {
//             let def_id = item_id.owner_id.def_id;
//             if let Some(body) = tcx.hir_maybe_body_owned_by(def_id) {
//                 let caller = format!("{:?}", def_id.to_def_id());
                
//                 let mut visitor = CallVisitor::new(tcx, caller.clone());
//                 visitor.visit_body(body);
                
//                 if !visitor.callees.is_empty() {
//                     self.data.call_graph.insert(caller, visitor.callees);
//                 }
//             }
//         }
        
//         println!("  Call graph edges: {}", self.data.call_graph.len());
//     }
    
//     fn save_data(&self) {
//         let json = serde_json::to_string_pretty(&self.data).unwrap();
//         fs::write("enhanced_compiler_data.json", json).unwrap();
//         println!("✅ Saved enhanced compiler data to enhanced_compiler_data.json");
//     }
// }

// struct CallVisitor<'tcx> {
//     tcx: TyCtxt<'tcx>,
//     caller: String,
//     callees: Vec<String>,
// }

// impl<'tcx> CallVisitor<'tcx> {
//     fn new(tcx: TyCtxt<'tcx>, caller: String) -> Self {
//         Self {
//             tcx,
//             caller,
//             callees: Vec::new(),
//         }
//     }
    
//     fn resolve_expr_def_id(&self, expr: &rustc_hir::Expr<'_>) -> Option<DefId> {
//         match &expr.kind {
//             rustc_hir::ExprKind::Path(qpath) => {
//                 let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
//                 typeck_results.qpath_res(qpath, expr.hir_id).opt_def_id()
//             }
//             _ => None,
//         }
//     }
// }

// impl<'tcx> Visitor<'tcx> for CallVisitor<'tcx> {
//     type NestedFilter = rustc_middle::hir::nested_filter::All;

//     fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
//         use rustc_hir::ExprKind;
        
//         match &expr.kind {
//             ExprKind::Call(func, _args) => {
//                 if let Some(def_id) = self.resolve_expr_def_id(func) {
//                     let callee = format!("{:?}", def_id);
//                     if !self.callees.contains(&callee) {
//                         self.callees.push(callee);
//                     }
//                 }
//             }
//             ExprKind::MethodCall(_, _, _, _) => {
//                 let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
//                 if let Some(def_id) = typeck_results.type_dependent_def_id(expr.hir_id) {
//                     let callee = format!("{:?}", def_id);
//                     if !self.callees.contains(&callee) {
//                         self.callees.push(callee);
//                     }
//                 }
//             }
//             _ => {}
//         }
        
//         intravisit::walk_expr(self, expr);
//     }
// }

// struct EnhancedCompilerCalls;

// impl Callbacks for EnhancedCompilerCalls {
//     fn after_analysis<'tcx>(
//         &mut self,
//         _compiler: &interface::Compiler,
//         tcx: TyCtxt<'tcx>,
//     ) -> Compilation {
//         let mut collector = EnhancedCollector::new();
//         collector.collect_all_data(tcx);
//         collector.save_data();
        
//         Compilation::Stop
//     }
// }

 fn main() {
//     let args: Vec<String> = std::env::args().collect();
    
//     if args.len() > 1 && (args[1].ends_with(".rs") || args.contains(&"--crate-name".to_string())) {
//         let mut callbacks = EnhancedCompilerCalls;
//         run_compiler(&args, &mut callbacks);
//     } else {
//         eprintln!("Usage: enhanced_collector <rust_file.rs>");
//         std::process::exit(1);
//     }
}
