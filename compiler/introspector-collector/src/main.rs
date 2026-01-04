// this code should be mergeed ino working_usage_collector.rs
// #![feature(rustc_private)]
// use rustc_hir::ItemKind;
// extern crate rustc_driver;
// extern crate rustc_interface;
// extern crate rustc_middle;
// extern crate rustc_hir;

// use rustc_driver::{Callbacks, Compilation};
// use rustc_interface::interface;
// use rustc_middle::ty::TyCtxt;
// use rustc_middle::ty::TypeckResults;
// use rustc_hir::{def_id::LOCAL_CRATE, Node};
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::Write;

// // Compatibility macros for API changes
// macro_rules! get_item_name {
//     ($tcx:expr, $item:expr) => {
//         $tcx.item_name($item.owner_id.to_def_id()).to_string()
//     };
// }

// macro_rules! get_typeck_results {
//     ($tcx:expr, $local_def_id:expr) => {
//         $tcx.typeck($local_def_id)
//     };
// }

// macro_rules! iterate_type_dependent_defs {
//     ($typeck_results:expr) => {
//         $typeck_results.type_dependent_defs().iter()
//     };
// }

// macro_rules! get_hir_node {
//     ($tcx:expr, $hir_id:expr) => {
//         $tcx.hir_node($hir_id)
//     };
// }

// macro_rules! get_crate_items {
//     ($tcx:expr) => {
//         $tcx.hir_crate_items(()).owners()
//     };
// }

// #[derive(Debug, Clone)]
// struct EnhancedUsageEntry {
//     usage: String,
//     node_type: String,
//     expr_type: Option<String>,
//     def_kind: String,
//     is_definition: bool,
//     is_usage: bool,
//     count: usize,
// }

// struct EnhancedUsageCollector {
//     module_data: HashMap<String, Vec<EnhancedUsageEntry>>,
// }

// impl EnhancedUsageCollector {
//     fn new() -> Self {
//         EnhancedUsageCollector {
//             module_data: HashMap::new(),
//         }
//     }

//     fn collect_enhanced_usage(&mut self, tcx: TyCtxt<'_>) {
//         // First pass: collect all definitions
//         self.collect_definitions(tcx);
        
//         // Second pass: collect all usages
//         for def_id in tcx.hir_crate_items(()).owners() {
//             let local_def_id = def_id;
//             let def_id = local_def_id.to_def_id();
            
//             let typeck_results = tcx.typeck(local_def_id);
//                 let module_name = tcx.def_path_str(def_id);
                
//                 // Collect method calls and symbol usages
//                 for (hir_id, def_id_opt) in typeck_results.type_dependent_defs() {
//                     if let Some(target_def_id) = def_id_opt {
//                         let symbol_path = tcx.def_path_str(target_def_id);
//                         let def_kind = tcx.def_kind(target_def_id);
//                         let usage = format!("{} ({})", symbol_path, def_kind.descr(target_def_id));
                        
//                         let node = tcx.hir_node(*hir_id);
//                         let node_type = self.classify_node_type(&node);
//                         let expr_type = typeck_results.node_type_opt(*hir_id).map(|ty| ty.to_string());
                        
//                         let entry = EnhancedUsageEntry {
//                             usage,
//                             node_type,
//                             expr_type,
//                             def_kind: format!("{:?}", def_kind),
//                             is_definition: false,
//                             is_usage: true,
//                             count: 1,
//                         };
                        
//                         self.module_data.entry(module_name.clone()).or_default().push(entry);
//                     }
//                 }
                
//                 // Collect path resolutions (constants, enums, etc.)
//             self.collect_path_resolutions(tcx, &typeck_results, &module_name);
//         }
//     }// collect
    
//     fn collect_definitions(&mut self, tcx: TyCtxt<'_>) {
//         // Iterate through all items in the crate using the correct API
//         for item_id in tcx.hir_crate_items(()).items() {
//             let item = tcx.hir_expect_item(item_id.owner_id.def_id);
//             let item_name = tcx.item_name(item.owner_id.to_def_id()).to_string();
//             let module_name = tcx.def_path_str(item.owner_id.to_def_id());
            
//             let (usage, def_kind) = match &item.kind {
//                 ItemKind::Const(ty, _, _, _) => {
//                     (format!("CONST_DEF: {}", item_name), "Const")
//                 },
//                 ItemKind::Static(ty, mutability, _, _) => {
//                     (format!("STATIC_DEF: {} {:?}", item_name, mutability), "Static")
//                 },
//                 ItemKind::Enum(enum_def, _, _) => {
//                     let variants: Vec<_> = enum_def.variants.iter().map(|v| v.ident.to_string()).collect();
//                     (format!("ENUM_DEF: {} {{ {} }}", item_name, variants.join(", ")), "Enum")
//                 },
//                 ItemKind::Struct(variant_data, _) => {
//                     let fields = match variant_data {
//                         rustc_hir::VariantData::Struct { fields, .. } => {
//                             fields.iter().map(|f| f.ident.to_string()).collect::<Vec<_>>().join(", ")
//                         },
//                         rustc_hir::VariantData::Tuple(fields, ..) => {
//                             format!("{} tuple fields", fields.len())
//                         },
//                         rustc_hir::VariantData::Unit(..) => "unit struct".to_string(),
//                     };
//                     (format!("STRUCT_DEF: {} {{ {} }}", item.ident, fields), "Struct")
//                 },
//                 ItemKind::Union(variant_data, _) => {
//                     let fields = variant_data.fields().iter().map(|f| f.ident.to_string()).collect::<Vec<_>>().join(", ");
//                     (format!("UNION_DEF: {} {{ {} }}", item.ident, fields), "Union")
//                 },
//                 ItemKind::Trait(_, _, _, _, trait_items) => {
//                     let items: Vec<_> = trait_items.iter().map(|ti| ti.ident.to_string()).collect();
//                     (format!("TRAIT_DEF: {} {{ {} }}", item.ident, items.join(", ")), "Trait")
//                 },
//                 ItemKind::Impl(impl_block) => {
//                     let trait_name = impl_block.of_trait.as_ref()
//                         .map(|t| tcx.hir().node_to_string(t.hir_ref_id))
//                         .unwrap_or_else(|| "inherent".to_string());
//                     let type_name = tcx.hir().node_to_string(impl_block.self_ty.hir_id);
//                     (format!("IMPL_DEF: {} for {}", trait_name, type_name), "Impl")
//                 },
//                 ItemKind::Fn(sig, _, _) => {
//                     let inputs: Vec<_> = sig.decl.inputs.iter().map(|input| {
//                         tcx.hir().node_to_string(input.hir_id)
//                     }).collect();
//                     let output = match &sig.decl.output {
//                         rustc_hir::FnRetTy::Return(ty) => tcx.hir().node_to_string(ty.hir_id),
//                         rustc_hir::FnRetTy::DefaultReturn(_) => "()".to_string(),
//                     };
//                     (format!("FN_DEF: {}({}) -> {}", item.ident, inputs.join(", "), output), "Fn")
//                 },
//                 ItemKind::Macro(macro_def, _) => {
//                     (format!("MACRO_DEF: {} rules", item.ident), "Macro")
//                 },
//                 ItemKind::TyAlias(ty, _) => {
//                     (format!("TYPE_ALIAS_DEF: {} = {}", item.ident, tcx.hir().node_to_string(ty.hir_id)), "TyAlias")
//                 },
//                 _ => (format!("OTHER_DEF: {}", item.ident), "Other"),
//             };
            
//             let entry = EnhancedUsageEntry {
//                 usage,
//                 node_type: "ItemDef".to_string(),
//                 expr_type: None,
//                 def_kind: def_kind.to_string(),
//                 is_definition: true,
//                 is_usage: false,
//                 count: 1,
//             };
            
//             self.module_data.entry(module_name).or_default().push(entry);
//         }
        
//         // Also collect trait items and impl items
//         self.collect_associated_items(tcx);
//     }
    
//     fn collect_associated_items(&mut self, tcx: TyCtxt<'_>) {
//         // Collect trait items
//         for trait_item_id in tcx.hir_crate_items(()).trait_items() {
//             let trait_item = tcx.hir().trait_item(trait_item_id);
//             let module_name = tcx.def_path_str(trait_item.owner_id.to_def_id());
            
//             let (usage, def_kind) = match &trait_item.kind {
//                 rustc_hir::TraitItemKind::Const(ty, _) => {
//                     (format!("TRAIT_CONST_DEF: {} : {}", trait_item.ident, tcx.hir().node_to_string(ty.hir_id)), "AssocConst")
//                 },
//                 rustc_hir::TraitItemKind::Fn(sig, _) => {
//                     (format!("TRAIT_FN_DEF: {}", trait_item.ident), "AssocFn")
//                 },
//                 rustc_hir::TraitItemKind::Type(_, _) => {
//                     (format!("TRAIT_TYPE_DEF: {}", trait_item.ident), "AssocTy")
//                 },
//             };
            
//             let entry = EnhancedUsageEntry {
//                 usage,
//                 node_type: "TraitItemDef".to_string(),
//                 expr_type: None,
//                 def_kind: def_kind.to_string(),
//                 is_definition: true,
//                 is_usage: false,
//                 count: 1,
//             };
            
//             self.module_data.entry(module_name).or_default().push(entry);
//         }
        
//         // Collect impl items
//         for impl_item_id in tcx.hir_crate_items(()).impl_items() {
//             let impl_item = tcx.hir().impl_item(impl_item_id);
//             let module_name = tcx.def_path_str(impl_item.owner_id.to_def_id());
            
//             let (usage, def_kind) = match &impl_item.kind {
//                 rustc_hir::ImplItemKind::Const(ty, _) => {
//                     (format!("IMPL_CONST_DEF: {} : {}", impl_item.ident, tcx.hir().node_to_string(ty.hir_id)), "AssocConst")
//                 },
//                 rustc_hir::ImplItemKind::Fn(sig, _) => {
//                     (format!("IMPL_FN_DEF: {}", impl_item.ident), "AssocFn")
//                 },
//                 rustc_hir::ImplItemKind::Type(ty) => {
//                     (format!("IMPL_TYPE_DEF: {} = {}", impl_item.ident, tcx.hir().node_to_string(ty.hir_id)), "AssocTy")
//                 },
//             };
            
//             let entry = EnhancedUsageEntry {
//                 usage,
//                 node_type: "ImplItemDef".to_string(),
//                 expr_type: None,
//                 def_kind: def_kind.to_string(),
//                 is_definition: true,
//                 is_usage: false,
//                 count: 1,
//             };
            
//             self.module_data.entry(module_name).or_default().push(entry);
//         }
//     }
    
//     fn collect_path_resolutions(&mut self, tcx: TyCtxt<'_>, typeck_results: &TypeckResults<'_>, module_name: &str) {
//         // Collect path resolutions by examining all expressions in the body
//         // This is a simplified version - in practice we'd need a visitor
//         for (hir_id, _) in typeck_results.type_dependent_defs() {
//             let node = tcx.hir_node(*hir_id);
//             if let rustc_hir::Node::Expr(expr) = node {
//                 match &expr.kind {
//                     rustc_hir::ExprKind::Path(qpath) => {
//                         if let Some(res) = typeck_results.qpath_res(qpath, *hir_id).opt_def_id() {
//                             let def_kind = tcx.def_kind(res);
//                             let symbol_path = tcx.def_path_str(res);
                            
//                             let usage = match def_kind {
//                                 rustc_hir::def::DefKind::Const => format!("CONST_USE: {}", symbol_path),
//                                 rustc_hir::def::DefKind::Static { .. } => format!("STATIC_USE: {}", symbol_path),
//                                 rustc_hir::def::DefKind::Variant => format!("VARIANT_USE: {}", symbol_path),
//                                 _ => format!("PATH_USE: {} ({:?})", symbol_path, def_kind),
//                             };
                            
//                             let entry = EnhancedUsageEntry {
//                                 usage,
//                                 node_type: "Expr::Path".to_string(),
//                                 expr_type: typeck_results.node_type_opt(*hir_id).map(|ty| ty.to_string()),
//                                 def_kind: format!("{:?}", def_kind),
//                                 is_definition: false,
//                                 is_usage: true,
//                                 count: 1,
//                             };
                            
//                             self.module_data.entry(module_name.to_string()).or_default().push(entry);
//                         }
//                     },
//                     _ => {}
//                 }
//             }
//         }
//     }
    
//     fn classify_node_type(&self, node: &Node) -> String {
//         match node {
//             Node::Expr(expr) => {
//                 format!("Expr::{}", match &expr.kind {
//                     rustc_hir::ExprKind::Call(..) => "Call",
//                     rustc_hir::ExprKind::MethodCall(..) => "MethodCall",
//                     rustc_hir::ExprKind::Path(..) => "Path",
//                     rustc_hir::ExprKind::Struct(..) => "Struct",
//                     rustc_hir::ExprKind::Field(..) => "Field",
//                     _ => "Other",
//                 })
//             },
//             Node::Stmt(_) => "Stmt".to_string(),
//             Node::Item(_) => "Item".to_string(),
//             Node::TraitItem(_) => "TraitItem".to_string(),
//             Node::ImplItem(_) => "ImplItem".to_string(),
//             Node::Pat(_) => "Pat".to_string(),
//             Node::Ty(_) => "Ty".to_string(),
//             _ => "Other".to_string(),
//         }
//     }
    
//     fn save_enhanced_files(&self, crate_name: &str) {
//         let output_dir = format!("{}/enhanced_usage_data", std::env::current_dir().unwrap().display());
//         std::fs::create_dir_all(&output_dir).unwrap();
        
//         for (module, entries) in &self.module_data {
//             use std::collections::hash_map::DefaultHasher;
//             use std::hash::{Hash, Hasher};
            
//             let full_name = format!("{}_{}", crate_name, module);
//             let mut hasher = DefaultHasher::new();
//             full_name.hash(&mut hasher);
//             let hash = hasher.finish();
            
//             let file_name = if full_name.len() > 100 {
//                 format!("{:x}.json", hash)
//             } else {
//                 format!("{}_{}.json", crate_name, module.replace("::", "_").replace("<", "_").replace(">", "_").replace(" ", "_").replace(",", "_").replace("'", "_"))
//             };
            
//             let filename = format!("{}/{}", output_dir, file_name);
//             let mut file = File::create(&filename).unwrap();
            
//             writeln!(file, "{{").unwrap();
//             writeln!(file, "  \"crate\": \"{}\",", crate_name).unwrap();
//             writeln!(file, "  \"module\": \"{}\",", module).unwrap();
//             writeln!(file, "  \"enhanced_entries\": [").unwrap();
            
//             for (i, entry) in entries.iter().enumerate() {
//                 let comma = if i == entries.len() - 1 { "" } else { "," };
//                 writeln!(file, "    {{").unwrap();
//                 writeln!(file, "      \"usage\": \"{}\",", entry.usage.replace("\"", "\\\"")).unwrap();
//                 writeln!(file, "      \"node_type\": \"{}\",", entry.node_type).unwrap();
//                 writeln!(file, "      \"expr_type\": {},", 
//                     entry.expr_type.as_ref().map(|t| format!("\"{}\"", t.replace("\"", "\\\"")))
//                         .unwrap_or_else(|| "null".to_string())).unwrap();
//                 writeln!(file, "      \"def_kind\": \"{}\",", entry.def_kind).unwrap();
//                 writeln!(file, "      \"is_definition\": {},", entry.is_definition).unwrap();
//                 writeln!(file, "      \"is_usage\": {},", entry.is_usage).unwrap();
//                 writeln!(file, "      \"count\": {}", entry.count).unwrap();
//                 writeln!(file, "    }}{}", comma).unwrap();
//             }
            
//             writeln!(file, "  ]").unwrap();
//             writeln!(file, "}}").unwrap();
            
//             eprintln!("Saved {} enhanced entries to {}", entries.len(), filename);
//         }
//     }// enhand
// }

// impl Callbacks for EnhancedUsageCollector {
//     fn after_analysis<'tcx>(
//         &mut self,
//         _compiler: &interface::Compiler,
//         queries: &'tcx rustc_interface::interface::Queries<'tcx>,
//     ) -> Compilation {
//         queries.global_ctxt().unwrap().enter(|tcx| {
//             let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
//             eprintln!("=== ENHANCED COLLECTION FOR CRATE: {} ===", crate_name);
            
//             self.collect_enhanced_usage(tcx);
//             self.save_enhanced_files(&crate_name);
            
//             eprintln!("✅ ENHANCED COLLECTION COMPLETE");
//         });
        
//         Compilation::Stop
//     }
// }

fn main() {
    // let mut collector = EnhancedUsageCollector::new();
    // let mut args: Vec<String> = std::env::args().collect();
    // args.insert(1, "--edition=2021".to_string());
    
    // let mut callbacks = collector;
    // rustc_driver::catch_with_exit_code(|| {
    //     RunCompiler::new(&args, &mut callbacks).run()
    // }).unwrap();
}
