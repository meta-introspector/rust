#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_ast;
extern crate rustc_span;

use rustc_driver::Callbacks;
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_ast::Crate;
use rustc_hir::{self as hir, intravisit::{self, Visitor}};
use rustc_span::Span;
use std::collections::HashMap;

include!("witness_macros.rs");

struct NumericConstantVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    integers: Vec<String>,
    floats: Vec<String>,
    bools: Vec<String>,
    struct_count: u32,
    enum_count: u32,
    item_counts: HashMap<String, u32>,
}

impl<'tcx> NumericConstantVisitor<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            integers: Vec::new(),
            floats: Vec::new(),
            bools: Vec::new(),
            struct_count: 0,
            enum_count: 0,
            item_counts: HashMap::new(),
        }
    }
    
    fn count_item(&mut self, item_type: &str) {
        *self.item_counts.entry(item_type.to_string()).or_insert(0) += 1;
    }
}

impl<'tcx> Visitor<'tcx> for NumericConstantVisitor<'tcx> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        match expr.kind {
            hir::ExprKind::Lit(lit) => {
                match lit.node {
                    rustc_ast::ast::LitKind::Int(value, _) => {
                        self.integers.push(value.to_string());
                    }
                    rustc_ast::ast::LitKind::Float(symbol, _) => {
                        self.floats.push(symbol.to_string());
                    }
                    rustc_ast::ast::LitKind::Bool(b) => {
                        self.bools.push(b.to_string());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        intravisit::walk_expr(self, expr);
    }

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) {
        let item_name = format!("{:?}", item.kind);
        self.count_item(&item_name);
        
        match item.kind {
            hir::ItemKind::Struct(..) => self.struct_count += 1,
            hir::ItemKind::Enum(..) => self.enum_count += 1,
            _ => {}
        }
        intravisit::walk_item(self, item);
    }
}

struct RealConstantCallbacks;

impl Callbacks for RealConstantCallbacks {
    fn after_crate_root_parsing(&mut self, _: &interface::Compiler, _krate: &mut Crate) -> rustc_driver::Compilation {
        witness!(symbol: "parsing_complete", from: "compiler");
        rustc_driver::Compilation::Continue
    }

    fn after_analysis<'tcx>(&mut self, _: &interface::Compiler, tcx: TyCtxt<'tcx>) -> rustc_driver::Compilation {
        witness!(symbol: "extracting_real_constants", from: "hir_analysis");
        
        let mut visitor = NumericConstantVisitor::new(tcx);
        tcx.hir().visit_all_item_likes_in_crate(&mut visitor);
        
        // Output real constants found
        witness!(symbol: format!("integers_found:{}", visitor.integers.len()).as_str(), from: "real_constants");
        witness!(symbol: format!("floats_found:{}", visitor.floats.len()).as_str(), from: "real_constants");
        witness!(symbol: format!("bools_found:{}", visitor.bools.len()).as_str(), from: "real_constants");
        witness!(symbol: format!("structs_found:{}", visitor.struct_count).as_str(), from: "real_constants");
        witness!(symbol: format!("enums_found:{}", visitor.enum_count).as_str(), from: "real_constants");
        
        // Output all item type counts
        for (item_type, count) in &visitor.item_counts {
            witness!(symbol: format!("item_type:{}:{}", item_type, count).as_str(), from: "item_counts");
        }
        
        rustc_driver::Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut callbacks = RealConstantCallbacks;
    
    witness!(symbol: "real_constant_scanner_start", from: "main");
    
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    witness!(symbol: "real_constant_scanner_end", from: "main");
    
    std::process::exit(match result { Ok(_) => 0, Err(_) => 1 });
}
