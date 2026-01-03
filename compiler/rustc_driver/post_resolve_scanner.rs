use rustc_driver::{Compilation, RunCompiler};
use rustc_interface::{interface, Queries};
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use rustc_span::symbol::Symbol;
use std::env;

struct PostResolveScanner;

impl rustc_driver::Callbacks for PostResolveScanner {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            extract_resolved_symbols(tcx);
        });
        Compilation::Continue
    }
}

fn extract_resolved_symbols(tcx: TyCtxt<'_>) {
    let output_dir = env::var("SCAN_OUTPUT_DIR").unwrap_or_else(|_| "scan_results".to_string());
    let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string();
    
    println!("POST_RESOLVE: Scanning {} with full symbol resolution", crate_name);
    
    // Get all local definitions
    for def_id in tcx.hir().body_owners() {
        let def_path = tcx.def_path_str(def_id.to_def_id());
        let span = tcx.def_span(def_id.to_def_id());
        
        println!("RESOLVED_SYMBOL: {{\"symbol\":\"{}\",\"def_path\":\"{}\",\"crate\":\"{}\",\"span\":\"{:?}\"}}", 
                 tcx.def_path_str(def_id.to_def_id()), def_path, crate_name, span);
    }
    
    // Get all external dependencies with their resolved paths
    for &cnum in tcx.crates(()).iter() {
        let crate_name_ext = tcx.crate_name(cnum);
        println!("EXTERNAL_CRATE: {{\"crate\":\"{}\",\"cnum\":{}}}", crate_name_ext, cnum.as_u32());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        return;
    }

    let mut rustc_args = vec![
        "rustc".to_string(),
        args[1].clone(),
        "--crate-type".to_string(),
        "lib".to_string(),
    ];

    RunCompiler::new(&rustc_args, &mut PostResolveScanner).run().unwrap();
}
