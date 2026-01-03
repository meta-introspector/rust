#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_hir;

use rustc_driver::Callbacks;
use rustc_interface::{interface, Queries};

struct SymbolExtractor;

impl Callbacks for SymbolExtractor {
    fn after_analysis(&mut self, _compiler: &interface::Compiler, queries: &Queries) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE);
            eprintln!("SYMBOL_EXTRACT: Analyzing crate '{}'", crate_name);
            
            // Get the HIR crate
            let hir = tcx.hir();
            let krate = hir.krate();
            
            let mut item_count = 0;
            
            // Walk through all items in the crate
            for item_id in krate.items() {
                let item = hir.item(*item_id);
                item_count += 1;
                
                // Extract basic item information
                let visibility = match item.vis_span.is_empty() {
                    true => "private",
                    false => "public"
                };
                
                let item_type = match &item.kind {
                    rustc_hir::ItemKind::Fn(..) => "function",
                    rustc_hir::ItemKind::Struct(..) => "struct", 
                    rustc_hir::ItemKind::Enum(..) => "enum",
                    rustc_hir::ItemKind::Mod(..) => "module",
                    rustc_hir::ItemKind::Trait(..) => "trait",
                    rustc_hir::ItemKind::Impl(..) => "impl",
                    rustc_hir::ItemKind::Use(..) => "use",
                    rustc_hir::ItemKind::Static(..) => "static",
                    rustc_hir::ItemKind::Const(..) => "const",
                    rustc_hir::ItemKind::Macro(..) => "macro",
                    _ => "other"
                };
                
                eprintln!("ITEM: {} {} '{}'", visibility, item_type, item.ident.name);
                
                // For modules, recursively analyze contents
                if let rustc_hir::ItemKind::Mod(module) = &item.kind {
                    for &item_id in module.item_ids {
                        let nested_item = hir.item(item_id);
                        let nested_vis = match nested_item.vis_span.is_empty() {
                            true => "private",
                            false => "public"
                        };
                        let nested_type = match &nested_item.kind {
                            rustc_hir::ItemKind::Fn(..) => "function",
                            rustc_hir::ItemKind::Struct(..) => "struct",
                            _ => "other"
                        };
                        eprintln!("  NESTED: {} {} '{}'", nested_vis, nested_type, nested_item.ident.name);
                    }
                }
            }
            
            eprintln!("SYMBOL_EXTRACT: Found {} top-level items", item_count);
        });
        
        rustc_driver::Compilation::Continue
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
    
    // Use symbol extractor for actual compilation
    let mut callbacks = SymbolExtractor;
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    std::process::exit(match result { Ok(_) => 0, Err(_) => 1 });
}
