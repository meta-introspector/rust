#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_driver::Callbacks;
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_hir as hir;
use rustc_span::Span;
use std::fs::File;
use std::io::Write;

struct HirExtractor;

impl Callbacks for HirExtractor {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            extract_and_serialize_hir(tcx);
        });
        
        rustc_driver::Compilation::Continue
    }
}

// Simple JSON-like serialization without external dependencies
fn to_json_string(s: &str) -> String {
    format!("\"{}\"", s.replace("\"", "\\\"").replace("\n", "\\n").replace("\r", "\\r"))
}

fn extract_and_serialize_hir(tcx: TyCtxt<'_>) {
    eprintln!("=== Extracting HIR Data ===");
    
    let crate_name = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
    let hir = tcx.hir();
    let mut items_json = Vec::new();
    
    // Walk through all top-level items
    for item_id in hir.root_module().item_ids {
        let item = hir.item(*item_id);
        
        let name = item.ident.to_string();
        let kind = get_item_kind(&item.kind);
        let visibility = get_visibility(&item.vis);
        let span_data = get_span_data(tcx, item.span);
        let def_id = format!("{:?}", item.owner_id.to_def_id());
        
        let item_json = format!(
            r#"{{
  "name": {},
  "kind": {},
  "visibility": {},
  "def_id": {},
  "span": {{
    "file": {},
    "line": {},
    "column": {},
    "snippet": {}
  }}
}}"#,
            to_json_string(&name),
            to_json_string(&kind),
            to_json_string(&visibility),
            to_json_string(&def_id),
            to_json_string(&span_data.0),
            span_data.1,
            span_data.2,
            to_json_string(&span_data.3)
        );
        
        items_json.push(item_json);
    }
    
    let crate_json = format!(
        r#"{{
  "name": {},
  "total_items": {},
  "items": [
{}
  ]
}}"#,
        to_json_string(&crate_name),
        items_json.len(),
        items_json.join(",\n")
    );
    
    // Write to file
    let output_file = format!("{}_hir.json", crate_name);
    if let Ok(mut file) = File::create(&output_file) {
        let _ = file.write_all(crate_json.as_bytes());
        eprintln!("HIR data written to: {}", output_file);
    }
    
    // Also output to stderr for capture
    eprintln!("HIR_JSON_START");
    eprintln!("{}", crate_json);
    eprintln!("HIR_JSON_END");
    
    eprintln!("Total top-level items: {}", items_json.len());
}

fn get_item_kind(kind: &hir::ItemKind) -> String {
    match kind {
        hir::ItemKind::Fn(..) => "function".to_string(),
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
        _ => "other".to_string(),
    }
}

fn get_visibility(vis: &hir::Visibility) -> String {
    match vis.node {
        hir::VisibilityKind::Public => "pub".to_string(),
        hir::VisibilityKind::Restricted { .. } => "pub(restricted)".to_string(),
        hir::VisibilityKind::Inherited => "private".to_string(),
    }
}

fn get_span_data(tcx: TyCtxt<'_>, span: Span) -> (String, usize, usize, String) {
    let source_map = tcx.sess.source_map();
    let loc = source_map.lookup_char_pos(span.lo());
    let snippet = source_map.span_to_snippet(span)
        .unwrap_or_else(|_| "<unavailable>".to_string());
    
    (
        loc.file.name.prefer_local().to_string(),
        loc.line,
        loc.col.0,
        snippet.chars().take(100).collect(), // Limit snippet size
    )
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
        rustc_driver::RunCompiler::new(&args, &mut callbacks).run()
    });
    
    std::process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
