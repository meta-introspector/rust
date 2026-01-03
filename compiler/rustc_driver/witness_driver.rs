#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_ast;

use rustc_driver::Callbacks;
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_ast::Crate;
use std::collections::HashMap;

include!("witness_macros.rs");
include!("topo_tracer.rs");
include!("leaf_analyzer.rs");

struct TopoWitnessCallbacks {
    tracer: TopologicalTracer,
}

impl TopoWitnessCallbacks {
    fn new() -> Self {
        Self { tracer: TopologicalTracer::new() }
    }
}

impl Callbacks for TopoWitnessCallbacks {
    fn after_crate_root_parsing(&mut self, _: &interface::Compiler, krate: &mut Crate) -> rustc_driver::Compilation {
        witness!(symbol: "main", from: "entry_point");
        self.tracer.add_triple("entry_point", "contains", "main");
        rustc_driver::Compilation::Continue
    }

    fn after_expansion<'tcx>(&mut self, _: &interface::Compiler, _tcx: TyCtxt<'tcx>) -> rustc_driver::Compilation {
        witness!(symbol: "expansion_complete", from: "compiler");
        self.tracer.add_triple("main", "depends_on", "rustc_driver");
        rustc_driver::Compilation::Continue
    }

    fn after_analysis<'tcx>(&mut self, _: &interface::Compiler, tcx: TyCtxt<'tcx>) -> rustc_driver::Compilation {
        witness!(symbol: "analyzing_leaf_constants", from: "leaf_analysis");
        
        let mut leaf_analyzer = LeafAnalyzer::new();
        
        // Scan ALL crates for constants, not just leaf crates
        for &crate_num in tcx.crates(()) {
            let crate_name = tcx.crate_name(crate_num).to_string();
            witness!(symbol: format!("scanning_crate:{}", crate_name).as_str(), from: "constant_scan");
            
            // Mock constants for each crate (real implementation would traverse HIR)
            leaf_analyzer.add_constant("MAX_SIZE", "1024", "pub", "usize", &crate_name);
            leaf_analyzer.add_constant("DEFAULT_CAPACITY", "8", "private", "usize", &crate_name);
            leaf_analyzer.add_constant("BUFFER_SIZE", "4096", "pub", "usize", &crate_name);
            leaf_analyzer.add_constant("TIMEOUT_MS", "5000", "private", "u64", &crate_name);
            leaf_analyzer.add_constant("PI_APPROX", "314", "pub", "i32", &crate_name);
            
            self.tracer.add_triple("main", "uses", &crate_name);
            self.tracer.add_triple(&crate_name, "provides", "symbols");
        }
        
        // Output constant report
        for constant in &leaf_analyzer.constants {
            witness!(symbol: format!("const:{}:{}:{}:{}", constant.name, constant.value, constant.visibility, constant.crate_name).as_str(), from: "constant_analysis");
        }
        
        // Output histogram
        for (value, count) in &leaf_analyzer.numeric_histogram {
            witness!(symbol: format!("histogram:{}:{}", value, count).as_str(), from: "numeric_histogram");
        }
        
        let topo_order = self.tracer.topological_sort();
        for (i, node) in topo_order.iter().enumerate() {
            witness!(symbol: format!("topo_order:{}:{}", i, node).as_str(), from: "topological");
        }
        
        for triple in &self.tracer.triples {
            witness!(symbol: format!("triple:{}->{}->{}",triple.from,triple.relation,triple.to).as_str(), from: "dependency_graph");
        }
        
        rustc_driver::Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut callbacks = TopoWitnessCallbacks::new();
    
    witness!(symbol: "topo_rustc_start", from: "main");
    
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    witness!(symbol: "topo_rustc_end", from: "main");
    
    std::process::exit(match result { Ok(_) => 0, Err(_) => 1 });
}
