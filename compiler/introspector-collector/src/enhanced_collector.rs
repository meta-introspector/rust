#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface, Queries};
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct CompilerData {
    // Clean call graph (function -> function only)
    call_graph: HashMap<String, Vec<String>>,
    
    // HIR cache
    hir_bodies: HashMap<String, String>,
    hir_items: HashMap<String, String>,
    
    // Symbol table
    def_paths: HashMap<String, String>,
    symbol_names: HashMap<String, String>,
    
    // Type information
    type_info: HashMap<String, String>,
    
    // Source locations
    source_map: HashMap<String, String>,
}

impl CompilerData {
    fn new() -> Self {
        Self {
            call_graph: HashMap::new(),
            hir_bodies: HashMap::new(),
            hir_items: HashMap::new(),
            def_paths: HashMap::new(),
            symbol_names: HashMap::new(),
            type_info: HashMap::new(),
            source_map: HashMap::new(),
        }
    }
    
    fn save_to_disk(&self, crate_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let filename = format!("compiler_data_{}.json", crate_name);
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&filename, json)?;
        println!("💾 Saved compiler data to {}", filename);
        Ok(())
    }
}

struct EnhancedCollector {
    data: CompilerData,
}

impl EnhancedCollector {
    fn new() -> Self {
        Self {
            data: CompilerData::new(),
        }
    }
    
    fn collect_all_data(&mut self, tcx: TyCtxt<'_>) {
        println!("🔍 Collecting comprehensive compiler data...");
        
        self.collect_hir_data(tcx);
        self.collect_symbol_table(tcx);
        self.collect_type_info(tcx);
        self.collect_source_locations(tcx);
        self.build_clean_call_graph(tcx);
    }
    
    fn collect_hir_data(&mut self, tcx: TyCtxt<'_>) {
        let hir = tcx.hir();
        
        // Collect HIR bodies
        for (body_id, body) in hir.krate().bodies.iter() {
            let body_str = format!("{:#?}", body.value);
            self.data.hir_bodies.insert(format!("{:?}", body_id), body_str);
        }
        
        // Collect HIR items
        for item_id in hir.items() {
            let item = hir.item(item_id);
            let item_str = format!("{:#?}", item);
            self.data.hir_items.insert(format!("{:?}", item_id.owner_id), item_str);
        }
        
        println!("  HIR bodies: {}", self.data.hir_bodies.len());
        println!("  HIR items: {}", self.data.hir_items.len());
    }
    
    fn collect_symbol_table(&mut self, tcx: TyCtxt<'_>) {
        for item_id in tcx.hir().items() {
            let def_id = item_id.owner_id.to_def_id();
            let def_path = tcx.def_path_str(def_id);
            let symbol_name = tcx.item_name(def_id).to_string();
            
            let def_id_str = format!("{:?}", def_id);
            self.data.def_paths.insert(def_id_str.clone(), def_path);
            self.data.symbol_names.insert(def_id_str, symbol_name);
        }
        
        println!("  Symbol entries: {}", self.data.def_paths.len());
    }
    
    fn collect_type_info(&mut self, tcx: TyCtxt<'_>) {
        for item_id in tcx.hir().items() {
            let def_id = item_id.owner_id.to_def_id();
            if let Ok(ty) = tcx.type_of(def_id).try_instantiate_identity() {
                let type_str = format!("{:?}", ty);
                self.data.type_info.insert(format!("{:?}", def_id), type_str);
            }
        }
        
        println!("  Type entries: {}", self.data.type_info.len());
    }
    
    fn collect_source_locations(&mut self, tcx: TyCtxt<'_>) {
        let source_map = tcx.sess.source_map();
        
        for item_id in tcx.hir().items() {
            let def_id = item_id.owner_id.to_def_id();
            let span = tcx.def_span(def_id);
            if let Ok(loc) = source_map.span_to_string(span) {
                self.data.source_map.insert(format!("{:?}", def_id), loc);
            }
        }
        
        println!("  Source locations: {}", self.data.source_map.len());
    }
    
    fn build_clean_call_graph(&mut self, tcx: TyCtxt<'_>) {
        let hir = tcx.hir();
        
        for (body_id, body) in hir.krate().bodies.iter() {
            let owner_def_id = tcx.hir().body_owner_def_id(*body_id);
            let caller = format!("{:?}", owner_def_id.to_def_id());
            
            let mut visitor = CallVisitor::new(tcx, caller.clone());
            visitor.visit_body(body);
            
            if !visitor.callees.is_empty() {
                self.data.call_graph.insert(caller, visitor.callees);
            }
        }
        
        println!("  Call graph edges: {}", self.data.call_graph.len());
    }
}

struct CallVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    caller: String,
    callees: Vec<String>,
}

impl<'tcx> CallVisitor<'tcx> {
    fn new(tcx: TyCtxt<'tcx>, caller: String) -> Self {
        Self {
            tcx,
            caller,
            callees: Vec::new(),
        }
    }
}

impl<'tcx> rustc_hir::intravisit::Visitor<'tcx> for CallVisitor<'tcx> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        use rustc_hir::ExprKind;
        
        match &expr.kind {
            ExprKind::Call(func, _args) => {
                if let Some(def_id) = self.resolve_expr_def_id(func) {
                    let callee = format!("{:?}", def_id);
                    if !self.callees.contains(&callee) {
                        self.callees.push(callee);
                    }
                }
            }
            ExprKind::MethodCall(_, _, _, _) => {
                let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
                if let Some(def_id) = typeck_results.type_dependent_def_id(expr.hir_id) {
                    let callee = format!("{:?}", def_id);
                    if !self.callees.contains(&callee) {
                        self.callees.push(callee);
                    }
                }
            }
            _ => {}
        }
        
        rustc_hir::intravisit::walk_expr(self, expr);
    }
}

impl<'tcx> CallVisitor<'tcx> {
    fn resolve_expr_def_id(&self, expr: &rustc_hir::Expr<'_>) -> Option<DefId> {
        match &expr.kind {
            rustc_hir::ExprKind::Path(qpath) => {
                let typeck_results = self.tcx.typeck(expr.hir_id.owner.def_id);
                typeck_results.qpath_res(qpath, expr.hir_id).opt_def_id()
            }
            _ => None,
        }
    }
}

struct EnhancedCompilerCalls;

impl Callbacks for EnhancedCompilerCalls {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string();
            println!("=== COLLECTING ENHANCED DATA FOR CRATE: {} ===", crate_name);
            
            let mut collector = EnhancedCollector::new();
            collector.collect_all_data(tcx);
            
            if let Err(e) = collector.data.save_to_disk(&crate_name) {
                eprintln!("Error saving data: {}", e);
            }
            
            println!("✅ ENHANCED COLLECTION COMPLETE");
        });
        
        Compilation::Continue
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && (args[1].ends_with(".rs") || args.contains(&"--crate-name".to_string())) {
        RunCompiler::new(&args, &mut EnhancedCompilerCalls).run().unwrap();
    } else {
        args[0] = "rustc".to_string();
        RunCompiler::new(&args, &mut rustc_driver::DefaultCallbacks).run().unwrap();
    }
}
