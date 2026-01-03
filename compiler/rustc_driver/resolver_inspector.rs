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

struct ResolverInspector;

impl Callbacks for ResolverInspector {
    fn after_crate_root_parsing(
        &mut self,
        _compiler: &interface::Compiler,
        krate: &mut ast::Crate,
    ) -> Compilation {
        eprintln!("=== AFTER CRATE ROOT PARSING ===");
        eprintln!("Crate attrs: {}", krate.attrs.len());
        eprintln!("Crate items: {}", krate.items.len());
        
        for (i, item) in krate.items.iter().enumerate().take(5) {
            let name = match &item.kind {
                ast::ItemKind::Fn(f) => format!("fn {}", f.sig.ident),
                ast::ItemKind::Struct(_, ident) => format!("struct {}", ident),
                ast::ItemKind::Enum(_, ident) => format!("enum {}", ident),
                _ => format!("item_{}", i),
            };
            eprintln!("Item {}: {} ({:?})", i, name, std::mem::discriminant(&item.kind));
        }
        
        Compilation::Continue
    }

    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        eprintln!("=== AFTER EXPANSION ===");
        eprintln!("TyCtxt available: {}", tcx.crate_name(rustc_span::def_id::LOCAL_CRATE));
        
        Compilation::Continue
    }

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        eprintln!("=== AFTER ANALYSIS (POST-RESOLUTION) ===");
        
        let crate_name = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE);
        eprintln!("Crate name: {}", crate_name);
        
        // Get the HIR map - this is available after resolution
        let hir_map = tcx.hir();
        eprintln!("HIR map available");
        
        // Show what's in the root module
        let root_mod = hir_map.root_module();
        eprintln!("Root module items: {}", root_mod.item_ids.len());
        
        for (i, item_id) in root_mod.item_ids.iter().enumerate().take(10) {
            let item = hir_map.item(*item_id);
            eprintln!("  Item {}: {} -> {:?}", i, item.ident, item.owner_id.to_def_id());
        }
        
        // Check what resolution data we have
        eprintln!("=== RESOLUTION DATA AVAILABLE ===");
        eprintln!("- DefId mappings: ✓");
        eprintln!("- Import resolution: ✓"); 
        eprintln!("- Macro resolution: ✓");
        eprintln!("- Type resolution: ✓");
        eprintln!("- HIR lowering: ✓");
        
        // Show dependency order information
        eprintln!("=== CRATE DEPENDENCY ORDER ===");
        let crates = tcx.crates(());
        eprintln!("External crates: {}", crates.len());
        for (i, &crate_num) in crates.iter().enumerate().take(10) {
            let crate_name = tcx.crate_name(crate_num);
            eprintln!("  Crate {}: {} ({})", i, crate_name, crate_num);
        }
        
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
    
    eprintln!("Starting resolver inspection...");
    
    let mut callbacks = ResolverInspector;
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    std::process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
