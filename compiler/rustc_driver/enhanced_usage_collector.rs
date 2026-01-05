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
struct UsageEntry {
    usage: String,
    def_id: Option<String>,
    tyctxt_method: Option<String>,
}

struct EnhancedUsageCollector {
    module_data: HashMap<String, Vec<UsageEntry>>,
    tyctxt_usage: HashMap<String, u32>,
}

impl EnhancedUsageCollector {
    fn new() -> Self {
        Self {
            module_data: HashMap::new(),
            tyctxt_usage: HashMap::new(),
        }
    }
    
    fn add_tyctxt_usage(&mut self, method: &str, def_id: Option<String>) {
        *self.tyctxt_usage.entry(method.to_string()).or_insert(0) += 1;
        
        let entry = UsageEntry {
            usage: format!("TyCtxt::{}", method),
            def_id,
            tyctxt_method: Some(method.to_string()),
        };
        
        self.module_data.entry("tyctxt".to_string()).or_insert_with(Vec::new).push(entry);
    }
    
    fn track_tyctxt_usage<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let local_def_id = rustc_hir::def_id::DefId::local(rustc_hir::def_id::DefIndex::from_u32(0));
        
        // Track critical methods
        let _path = tcx.def_path_str(local_def_id);
        self.add_tyctxt_usage("def_path_str", Some(format!("{:?}", local_def_id)));
        
        let _kind = tcx.def_kind(local_def_id);
        self.add_tyctxt_usage("def_kind", Some(format!("{:?}", local_def_id)));
        
        if tcx.def_kind(local_def_id).has_type() {
            let _ty = tcx.type_of(local_def_id);
            self.add_tyctxt_usage("type_of", Some(format!("{:?}", local_def_id)));
        }
        
        let _hir = tcx.hir();
        self.add_tyctxt_usage("hir", None);
    }
    
    fn save_to_files(&self, crate_name: &str) {
        let output_dir = format!("usage_data");
        std::fs::create_dir_all(&output_dir).unwrap();
        
        // Generate usage bitmap
        let bitmap_file = format!("{}/{}_usage_bitmap.txt", output_dir, crate_name);
        let mut file = File::create(&bitmap_file).unwrap();
        
        writeln!(file, "USAGE BITMAP FOR {}", crate_name).unwrap();
        writeln!(file, "TyCtxt Methods:").unwrap();
        for (method, count) in &self.tyctxt_usage {
            writeln!(file, "{}: {}", method, count).unwrap();
        }
    }
}

impl Callbacks for EnhancedUsageCollector {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            self.track_tyctxt_usage(tcx);
        });
        Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file> [--crate-name <name>]", args[0]);
        std::process::exit(1);
    }
    
    let mut collector = EnhancedUsageCollector::new();
    let crate_name = args.get(3).unwrap_or(&"unknown".to_string()).clone();
    
    let rustc_args = vec![args[1].clone()];
    
    rustc_driver::RunCompiler::new(&rustc_args, &mut collector).run().unwrap();
    
    collector.save_to_files(&crate_name);
    println!("✅ Enhanced collection complete!");
}
