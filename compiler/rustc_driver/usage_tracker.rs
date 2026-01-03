#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;
extern crate rustc_ast;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_ast as ast;
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use std::collections::HashMap;

struct UsageTracker {
    usage_map: HashMap<String, Vec<String>>,
}

impl UsageTracker {
    fn new() -> Self {
        Self {
            usage_map: HashMap::new(),
        }
    }

    fn record_usage(&mut self, user: String, used: String) {
        self.usage_map.entry(user).or_insert_with(Vec::new).push(used);
    }

    fn format_location(&self, tcx: TyCtxt<'_>, def_id: DefId) -> String {
        let crate_name = tcx.crate_name(def_id.krate);
        let def_path = tcx.def_path_str(def_id);
        format!("{}::{}", crate_name, def_path)
    }
}

impl Callbacks for UsageTracker {
    fn after_crate_root_parsing(
        &mut self,
        _compiler: &interface::Compiler,
        krate: &mut ast::Crate,
    ) -> Compilation {
        eprintln!("=== PARSING STAGE ===");
        eprintln!("Crate has {} items", krate.items.len());
        
        // At this stage we only have AST, no resolution yet
        for (i, item) in krate.items.iter().enumerate().take(5) {
            let kind_name = match &item.kind {
                ast::ItemKind::Fn(_) => "function",
                ast::ItemKind::Struct(..) => "struct", 
                ast::ItemKind::Enum(..) => "enum",
                ast::ItemKind::Mod(..) => "module",
                ast::ItemKind::Use(..) => "use",
                _ => "other",
            };
            eprintln!("  AST Item {}: {} (id: {:?})", i, kind_name, item.id);
        }
        
        Compilation::Continue
    }

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        eprintln!("=== POST-RESOLUTION ANALYSIS ===");
        
        let local_crate_name = tcx.crate_name(LOCAL_CRATE);
        eprintln!("Analyzing crate: {}", local_crate_name);
        
        // Get all external crates in dependency order
        let external_crates = tcx.crates(());
        eprintln!("External crates ({}): ", external_crates.len());
        for &crate_num in external_crates.iter() {
            let crate_name = tcx.crate_name(crate_num);
            eprintln!("  - {} ({})", crate_name, crate_num);
        }
        
        // Now we can access resolved information
        eprintln!("\n=== SYMBOL USAGE TRACKING ===");
        
        // Get all local definitions - let's try a different approach
        eprintln!("TyCtxt methods available for symbol tracking:");
        eprintln!("- crate_name: ✓");
        eprintln!("- def_path_str: ✓"); 
        eprintln!("- crates(): ✓");
        
        // Try to get local definitions through a different path
        let all_local_items = tcx.hir_crate_items(());
        
        let items_count = all_local_items.free_items().count();
        let impl_items_count = all_local_items.impl_items().count();
        let trait_items_count = all_local_items.trait_items().count();
        let foreign_items_count = all_local_items.foreign_items().count();
        
        eprintln!("Local items found: {} items, {} impl items, {} trait items, {} foreign items",
                 items_count, impl_items_count, trait_items_count, foreign_items_count);
        
        // Show first few items
        for (i, item_id) in all_local_items.free_items().enumerate().take(5) {
            let def_id = item_id.owner_id.to_def_id();
            let def_path = tcx.def_path_str(def_id);
            let user_location = format!("{}::{}", local_crate_name, def_path);
            eprintln!("  Item {}: {}", i, user_location);
        }
        
        eprintln!("\n=== USAGE SUMMARY ===");
        eprintln!("Ready to track: crate::module::decl::ast USES crate::module::decl::ast");
        eprintln!("Next step: Implement HIR visitor to find all DefId references");
        
        Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Handle cargo queries normally  
    if args.iter().any(|arg| arg.starts_with("--print") || arg == "--version" || arg == "-V") {
        let mut cmd = std::process::Command::new("rustc");
        cmd.args(&args[1..]);
        std::process::exit(cmd.status().unwrap().code().unwrap_or(1));
    }
    
    eprintln!("Starting usage tracking...");
    
    let mut callbacks = UsageTracker::new();
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    std::process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
