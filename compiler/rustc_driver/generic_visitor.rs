#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_hir;
extern crate rustc_serialize;
extern crate rustc_span;

use rustc_driver::Callbacks;
use rustc_interface::{interface, Queries};
use rustc_hir::intravisit::{self, Visitor};
use rustc_serialize::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SymbolInfo {
    name: String,
    kind: String,
    visibility: String,
    span: String,
    def_id: Option<String>,
    children: Vec<SymbolInfo>,
}

impl rustc_serialize::Encodable for SymbolInfo {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("SymbolInfo", 6, |s| {
            s.emit_struct_field("name", 0, |s| self.name.encode(s))?;
            s.emit_struct_field("kind", 1, |s| self.kind.encode(s))?;
            s.emit_struct_field("visibility", 2, |s| self.visibility.encode(s))?;
            s.emit_struct_field("span", 3, |s| self.span.encode(s))?;
            s.emit_struct_field("def_id", 4, |s| self.def_id.encode(s))?;
            s.emit_struct_field("children", 5, |s| self.children.encode(s))
        })
    }
}

struct SymbolVisitor<'tcx> {
    tcx: rustc_hir::def_id::TyCtxt<'tcx>,
    symbols: Vec<SymbolInfo>,
    current_module: Vec<String>,
}

impl<'tcx> SymbolVisitor<'tcx> {
    fn new(tcx: rustc_hir::def_id::TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            symbols: Vec::new(),
            current_module: Vec::new(),
        }
    }

    fn get_visibility(&self, vis: &rustc_hir::Visibility) -> String {
        match vis.node {
            rustc_hir::VisibilityKind::Public => "pub".to_string(),
            rustc_hir::VisibilityKind::Restricted { .. } => "pub(restricted)".to_string(),
            rustc_hir::VisibilityKind::Inherited => "private".to_string(),
        }
    }

    fn span_to_string(&self, span: rustc_span::Span) -> String {
        if span.is_dummy() {
            "dummy".to_string()
        } else {
            let source_map = self.tcx.sess.source_map();
            match source_map.span_to_string(span) {
                Ok(s) => s,
                Err(_) => format!("span:{:?}", span),
            }
        }
    }

    fn create_symbol_info(&self, name: &str, kind: &str, vis: &rustc_hir::Visibility, span: rustc_span::Span, def_id: Option<rustc_hir::def_id::DefId>) -> SymbolInfo {
        SymbolInfo {
            name: name.to_string(),
            kind: kind.to_string(),
            visibility: self.get_visibility(vis),
            span: self.span_to_string(span),
            def_id: def_id.map(|id| format!("{:?}", id)),
            children: Vec::new(),
        }
    }
}

impl<'tcx> Visitor<'tcx> for SymbolVisitor<'tcx> {
    type NestedFilter = rustc_hir::intravisit::nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_item(&mut self, item: &'tcx rustc_hir::Item<'tcx>) {
        let symbol = match &item.kind {
            rustc_hir::ItemKind::Fn(sig, generics, _) => {
                let mut info = self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "function",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                );
                
                // Add function signature details
                info.children.push(SymbolInfo {
                    name: "signature".to_string(),
                    kind: "fn_sig".to_string(),
                    visibility: "".to_string(),
                    span: self.span_to_string(sig.span),
                    def_id: None,
                    children: Vec::new(),
                });
                
                Some(info)
            },
            rustc_hir::ItemKind::Struct(variant_data, generics) => {
                Some(self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "struct",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                ))
            },
            rustc_hir::ItemKind::Enum(enum_def, generics) => {
                let mut info = self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "enum",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                );
                
                // Add enum variants
                for variant in enum_def.variants {
                    info.children.push(SymbolInfo {
                        name: variant.ident.name.to_string(),
                        kind: "enum_variant".to_string(),
                        visibility: "".to_string(),
                        span: self.span_to_string(variant.span),
                        def_id: Some(format!("{:?}", variant.def_id)),
                        children: Vec::new(),
                    });
                }
                
                Some(info)
            },
            rustc_hir::ItemKind::Mod(module) => {
                self.current_module.push(item.ident.name.to_string());
                let info = self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "module",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                );
                Some(info)
            },
            rustc_hir::ItemKind::Trait(is_auto, safety, generics, bounds, items) => {
                let mut info = self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "trait",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                );
                
                // Add trait items
                for trait_item_ref in *items {
                    info.children.push(SymbolInfo {
                        name: trait_item_ref.ident.name.to_string(),
                        kind: "trait_item".to_string(),
                        visibility: "".to_string(),
                        span: self.span_to_string(trait_item_ref.span),
                        def_id: Some(format!("{:?}", trait_item_ref.id.owner_id.to_def_id())),
                        children: Vec::new(),
                    });
                }
                
                Some(info)
            },
            rustc_hir::ItemKind::Impl(impl_item) => {
                Some(self.create_symbol_info(
                    &format!("impl_{}", item.owner_id.to_def_id().index.as_u32()),
                    "impl",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                ))
            },
            rustc_hir::ItemKind::Use(path, kind) => {
                Some(self.create_symbol_info(
                    &format!("use_{}", item.owner_id.to_def_id().index.as_u32()),
                    "use",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                ))
            },
            rustc_hir::ItemKind::Static(ty, mutability, _) => {
                Some(self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "static",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                ))
            },
            rustc_hir::ItemKind::Const(ty, generics, _) => {
                Some(self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "const",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                ))
            },
            rustc_hir::ItemKind::Macro(macro_def, _) => {
                Some(self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "macro",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                ))
            },
            _ => {
                Some(self.create_symbol_info(
                    &item.ident.name.to_string(),
                    "other",
                    &item.vis,
                    item.span,
                    Some(item.owner_id.to_def_id()),
                ))
            }
        };

        if let Some(sym) = symbol {
            self.symbols.push(sym);
        }

        // Continue visiting nested items
        intravisit::walk_item(self, item);

        // Pop module name when leaving module
        if matches!(item.kind, rustc_hir::ItemKind::Mod(_)) {
            self.current_module.pop();
        }
    }

    fn visit_impl_item(&mut self, impl_item: &'tcx rustc_hir::ImplItem<'tcx>) {
        let kind = match &impl_item.kind {
            rustc_hir::ImplItemKind::Fn(..) => "impl_fn",
            rustc_hir::ImplItemKind::Const(..) => "impl_const",
            rustc_hir::ImplItemKind::Type(..) => "impl_type",
        };

        let symbol = SymbolInfo {
            name: impl_item.ident.name.to_string(),
            kind: kind.to_string(),
            visibility: self.get_visibility(&impl_item.vis),
            span: self.span_to_string(impl_item.span),
            def_id: Some(format!("{:?}", impl_item.owner_id.to_def_id())),
            children: Vec::new(),
        };

        self.symbols.push(symbol);
        intravisit::walk_impl_item(self, impl_item);
    }

    fn visit_trait_item(&mut self, trait_item: &'tcx rustc_hir::TraitItem<'tcx>) {
        let kind = match &trait_item.kind {
            rustc_hir::TraitItemKind::Fn(..) => "trait_fn",
            rustc_hir::TraitItemKind::Const(..) => "trait_const",
            rustc_hir::TraitItemKind::Type(..) => "trait_type",
        };

        let symbol = SymbolInfo {
            name: trait_item.ident.name.to_string(),
            kind: kind.to_string(),
            visibility: "".to_string(), // Trait items don't have explicit visibility
            span: self.span_to_string(trait_item.span),
            def_id: Some(format!("{:?}", trait_item.owner_id.to_def_id())),
            children: Vec::new(),
        };

        self.symbols.push(symbol);
        intravisit::walk_trait_item(self, trait_item);
    }
}

struct GenericSymbolExtractor;

impl Callbacks for GenericSymbolExtractor {
    fn after_analysis(&mut self, _compiler: &interface::Compiler, queries: &Queries) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE);
            
            let mut visitor = SymbolVisitor::new(tcx);
            let hir = tcx.hir();
            
            // Visit the entire crate
            hir.walk_toplevel_module(&mut visitor);
            
            // Create the final output structure
            let output = HashMap::from([
                ("crate_name".to_string(), json::encode(&crate_name.to_string()).unwrap()),
                ("symbols".to_string(), json::encode(&visitor.symbols).unwrap()),
                ("total_symbols".to_string(), json::encode(&visitor.symbols.len()).unwrap()),
            ]);
            
            // Output as JSON
            match json::encode(&output) {
                Ok(json_str) => {
                    eprintln!("SYMBOL_DATA: {}", json_str);
                },
                Err(e) => {
                    eprintln!("SYMBOL_ERROR: Failed to encode JSON: {:?}", e);
                }
            }
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
    
    // Use generic symbol extractor for actual compilation
    let mut callbacks = GenericSymbolExtractor;
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    std::process::exit(match result { Ok(_) => 0, Err(_) => 1 });
}
