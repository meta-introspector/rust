#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_driver::Callbacks;
use rustc_interface::{interface, queries};
use rustc_middle::ty::TyCtxt;
use rustc_hir as hir;
use rustc_ast::ast;
use rustc_span::Span;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

struct HirExtractor;

impl Callbacks for HirExtractor {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            extract_and_serialize_hir(tcx);
        });
        
        rustc_driver::Compilation::Continue
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CrateData {
    name: String,
    items: Vec<ItemData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemData {
    name: String,
    kind: String,
    visibility: String,
    span: SpanData,
    def_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpanData {
    file: String,
    line: usize,
    column: usize,
    snippet: String,
}

fn extract_and_serialize_hir(tcx: TyCtxt<'_>) {
    println!("=== Extracting HIR Data ===");
    
    let crate_name = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
    let hir_crate_items = tcx.hir_crate_items(());
    let mut items = Vec::new();
    
    // Walk through all top-level items
    for item_id in hir_crate_items.free_items() {
        let item = tcx.hir().item(item_id);
        
        let item_data = ItemData {
            name: item.ident.to_string(),
            kind: get_item_kind(&item.kind),
            visibility: get_visibility(&item.vis),
            span: get_span_data(tcx, item.span),
            def_id: format!("{:?}", item.owner_id.to_def_id()),
        };
        
        items.push(item_data);
    }
    
    let crate_data = CrateData {
        name: crate_name.clone(),
        items,
    };
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&crate_data)
        .expect("Failed to serialize to JSON");
    
    let output_file = format!("{}_hir.json", crate_name);
    let mut file = File::create(&output_file)
        .expect("Failed to create output file");
    
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
    
    println!("HIR data serialized to: {}", output_file);
    println!("Total top-level items: {}", crate_data.items.len());
    
    // Also print to stderr for easy capture
    eprintln!("HIR_JSON: {}", json);
}

fn get_item_kind(kind: &hir::ItemKind) -> String {
    match kind {
        hir::ItemKind::Fn { .. } => "function".to_string(),
        hir::ItemKind::Struct(..) => "struct".to_string(),
        hir::ItemKind::Enum(..) => "enum".to_string(),
        hir::ItemKind::Const(..) => "const".to_string(),
        hir::ItemKind::Static(..) => "static".to_string(),
        hir::ItemKind::Trait(..) => "trait".to_string(),
        hir::ItemKind::Impl(..) => "impl".to_string(),
        hir::ItemKind::Mod(..) => "module".to_string(),
        hir::ItemKind::Use(..) => "use".to_string(),
        hir::ItemKind::TyAlias(..) => "type_alias".to_string(),
        hir::ItemKind::Macro(..) => "macro".to_string(),
        _ => format!("other({:?})", std::mem::discriminant(kind)),
    }
}

fn get_visibility(vis: &hir::Visibility) -> String {
    match vis.node {
        hir::VisibilityKind::Public => "pub".to_string(),
        hir::VisibilityKind::Restricted { .. } => "pub(restricted)".to_string(),
        hir::VisibilityKind::Inherited => "private".to_string(),
    }
}

fn get_span_data(tcx: TyCtxt<'_>, span: Span) -> SpanData {
    let source_map = tcx.sess.source_map();
    let loc = source_map.lookup_char_pos(span.lo());
    let snippet = source_map.span_to_snippet(span)
        .unwrap_or_else(|_| "<unavailable>".to_string());
    
    SpanData {
        file: loc.file.name.prefer_local().to_string(),
        line: loc.line,
        column: loc.col.0,
        snippet: snippet.chars().take(200).collect(), // Limit snippet size
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
    
    let mut callbacks = HirExtractor;
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks, None, None)
    });
    
    std::process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
