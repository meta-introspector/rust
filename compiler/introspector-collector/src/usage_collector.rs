#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_ast;

use crate::data_structures::*;
use crate::file_manager::FileManager;
use crate::collectors::*;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_hir::intravisit::{self, Visitor};
use std::io::Write;
use std::collections::HashMap;

pub struct UsageCollector {
    pub module_data: HashMap<String, Vec<UsageEntry>>,
    pub enum_data: HashMap<String, EnumInfo>,
    pub item_complexity: HashMap<String, ItemComplexity>,
}

impl UsageCollector {
    pub fn new() -> Self {
        Self {
            module_data: HashMap::new(),
            enum_data: HashMap::new(),
            item_complexity: HashMap::new(),
        }
    }
    
    pub fn add_usage(
        &mut self,
        module: &str,
        symbol: String,
        kind: String,
        usage_type: String,
        node_type: String,
        user_def_id: String,
        used_def_id: String,
        user_crate: Option<String>,
        used_crate: Option<String>,
        used_def_kind: Option<String>,
        user_def_kind: Option<String>,
    ) {
        let entry = UsageEntry {
            symbol,
            kind,
            usage_count: 1,
            usage_type,
            node_type,
            user_def_id,
            used_def_id,
            user_crate,
            used_crate,
            used_def_kind,
            user_def_kind,
        };
        
        self.module_data.entry(module.to_string()).or_insert_with(Vec::new).push(entry);
    }
    
    pub fn get_defkind_from_item<'tcx>(&self, tcx: TyCtxt<'tcx>, item: &rustc_hir::Item) -> String {
        match &item.kind {
            rustc_hir::ItemKind::Const(_, _, _, body_id) => {
                let body = tcx.hir_body(*body_id);
                match &body.value.kind {
                    rustc_hir::ExprKind::Lit(lit) => match lit.node {
                        rustc_ast::LitKind::Str(..) => "StringLiteral",
                        rustc_ast::LitKind::Int(..) => "IntLiteral", 
                        rustc_ast::LitKind::Float(..) => "FloatLiteral",
                        rustc_ast::LitKind::Bool(..) => "BoolLiteral",
                        _ => "Const"
                    },
                    _ => "Const"
                }
            },
            rustc_hir::ItemKind::Static(..) => "Static",
            rustc_hir::ItemKind::Struct(..) => "Struct",
            rustc_hir::ItemKind::Enum(..) => "Enum",
            rustc_hir::ItemKind::Fn { .. } => "Fn",
            _ => "Other"
        }.to_string()
    }
    
    pub fn classify_usage(&self, usage: &str, usage_type: &str, _used_def_id: &str) -> UsageClassification {
        let mut classification = UsageClassification::default();
        
        if usage.contains("to_string") || usage.contains("Display") || usage.contains("fmt") || 
           usage.contains("format!") || usage.contains("write!") {
            classification.string_conversion = 1;
        }
        else if usage.contains("match") || usage.contains("if let") || usage_type == "PatternMatch" {
            classification.pattern_matching = 1;
        }
        else if usage.contains("::") && !usage.contains("(") {
            classification.construction = 1;
        }
        else if usage.contains("==") || usage.contains("!=") || usage.contains("cmp") {
            classification.comparison = 1;
        }
        else if usage.contains("Debug") || usage.contains("{:?}") || usage.contains("dbg!") {
            classification.debug_format = 1;
        }
        else if usage.contains("serde") || usage.contains("serialize") || usage.contains("json") {
            classification.serialization = 1;
        }
        else {
            classification.general = 1;
        }
        
        classification
    }
    
    pub fn extract_enum_variant(&self, def_id: &str) -> Option<(String, String)> {
        if let Some(path_part) = def_id.strip_prefix("DefId(").and_then(|s| s.split(" ~ ").nth(1)) {
            if let Some(clean_path) = path_part.strip_suffix(")") {
                let parts: Vec<&str> = clean_path.split("::").collect();
                if parts.len() >= 2 {
                    let variant = parts[parts.len() - 1].to_string();
                    let enum_name = parts[parts.len() - 2].to_string();
                    
                    if !variant.contains("(") && !variant.contains("<") && 
                       (variant.chars().next().unwrap_or('a').is_uppercase() || 
                        variant == "true" || variant == "false") {
                        return Some((enum_name, variant));
                    }
                }
            }
        }
        None
    }
    
    pub fn update_enum_usage(&mut self, enum_name: String, variant_name: String, 
                        classification: UsageClassification, usage_type: &str, converter_fn: &str) {
        let enum_info = self.enum_data.entry(enum_name.clone()).or_insert_with(|| EnumInfo {
            name: enum_name.clone(),
            variants: Vec::new(),
            total_usage_classes: UsageClassification::default(),
        });
        
        if let Some(variant) = enum_info.variants.iter_mut().find(|v| v.variant_name == variant_name) {
            variant.usage_classes.string_conversion += classification.string_conversion;
            variant.usage_classes.pattern_matching += classification.pattern_matching;
            variant.usage_classes.construction += classification.construction;
            variant.usage_classes.comparison += classification.comparison;
            variant.usage_classes.debug_format += classification.debug_format;
            variant.usage_classes.serialization += classification.serialization;
            variant.usage_classes.general += classification.general;
            
            let converters = variant.top_converters.entry(usage_type.to_string()).or_insert_with(Vec::new);
            if let Some(existing) = converters.iter_mut().find(|(fn_name, _)| fn_name == converter_fn) {
                existing.1 += 1;
            } else {
                converters.push((converter_fn.to_string(), 1));
            }
            converters.sort_by(|a, b| b.1.cmp(&a.1));
            converters.truncate(3);
        } else {
            let mut top_converters = HashMap::new();
            top_converters.insert(usage_type.to_string(), vec![(converter_fn.to_string(), 1)]);
            
            enum_info.variants.push(EnumVariantUsage {
                enum_name: enum_name.clone(),
                variant_name: variant_name.clone(),
                usage_classes: classification.clone(),
                top_converters,
            });
        }
        
        enum_info.total_usage_classes.string_conversion += classification.string_conversion;
        enum_info.total_usage_classes.pattern_matching += classification.pattern_matching;
        enum_info.total_usage_classes.construction += classification.construction;
        enum_info.total_usage_classes.comparison += classification.comparison;
        enum_info.total_usage_classes.debug_format += classification.debug_format;
        enum_info.total_usage_classes.serialization += classification.serialization;
        enum_info.total_usage_classes.general += classification.general;
    }
    
    pub fn save_to_files(&self, crate_name: &str) {
        FileManager::save_to_files(&self.module_data, &self.enum_data, &self.item_complexity, crate_name);
    }
    
    pub fn collect_constants<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
        
        for item_id in tcx.hir_crate_items(()).free_items() {
            let item = tcx.hir_expect_item(item_id.owner_id.def_id);
            
            let item_name = match &item.kind {
                rustc_hir::ItemKind::Use(..) => continue,
                rustc_hir::ItemKind::ExternCrate(..) => continue,
                rustc_hir::ItemKind::Impl(..) => continue,
                rustc_hir::ItemKind::ForeignMod { .. } => continue,
                _ => tcx.item_name(item.owner_id.to_def_id()).to_string(),
            };
            
            match &item.kind {
                rustc_hir::ItemKind::Const(_, _, _, body_id) => {
                    let const_defkind = self.get_defkind_from_item(tcx, item);
                    self.add_usage(
                        "constants",
                        item_name.clone(),
                        "const_decl".to_string(),
                        "ConstDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None,
                        Some(const_defkind),
                        Some("Const".to_string())
                    );
                }
                _ => {}
            }
        }
    }
}

impl Callbacks for UsageCollector {

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let local_crate = tcx.crate_name(LOCAL_CRATE);
        let all_items = tcx.hir_crate_items(());
        
        if std::env::var("DUMP_HIR").is_ok() {
            eprintln!("🔍 Creating HIR dump...");
            let hir_dump_path = "hir_dump.json";
            if let Ok(mut hir_dump) = std::fs::File::create(hir_dump_path) {
                let crate_items = tcx.hir_crate_items(());
                let _ = writeln!(hir_dump, "{{\"crate_name\": \"{}\", \"items\": [", local_crate);
                
                let items: Vec<_> = crate_items.free_items().collect();
                for (i, item_id) in items.iter().enumerate() {
                    let item = tcx.hir_expect_item(item_id.owner_id.def_id);
                    let comma = if i == items.len() - 1 { "" } else { "," };
                    let _ = writeln!(hir_dump, "  {{\"id\": {}, \"kind\": \"{:?}\"}}{}", 
                        item_id.owner_id.def_id.local_def_index.index(), 
                        std::mem::discriminant(&item.kind), 
                        comma);
                }
                let _ = writeln!(hir_dump, "]}}");
                eprintln!("🔍 HIR dump saved to {}", hir_dump_path);
            }
        }
        
        for item_id in all_items.free_items() {
            let def_id = item_id.owner_id.to_def_id();
            let item_path = tcx.def_path_str(def_id);
            let def_kind = tcx.def_kind(def_id);
                
            let (module, decl) = if item_path.contains("::") {
                let parts: Vec<&str> = item_path.split("::").collect();
                (parts[..parts.len()-1].join("::"), parts[parts.len()-1])
            } else {
                ("root".to_string(), item_path.as_str())
            };
            
            if !matches!(def_kind, 
                rustc_hir::def::DefKind::Fn | 
                rustc_hir::def::DefKind::Const | 
                rustc_hir::def::DefKind::Static { .. } |
                rustc_hir::def::DefKind::AssocFn |
                rustc_hir::def::DefKind::AssocConst
            ) {
                continue;
            }
            
            let user_location = format!("crate::{}::{}::{}", local_crate, module, decl);
            let typeck = tcx.typeck(item_id.owner_id.def_id);
            
            for (_local_id, result) in typeck.type_dependent_defs().items_in_stable_order() {
                if let Ok((def_kind, used_def_id)) = result {
                    let used_crate = tcx.crate_name(used_def_id.krate);
                    let used_path = tcx.def_path_str(*used_def_id);
                    let _used_location = format!("crate::{}::{}", used_crate, used_path);
                    
                    let node_type = match tcx.hir_node_by_def_id(item_id.owner_id.def_id) {
                        rustc_hir::Node::Item(_) => "Item",
                        rustc_hir::Node::Expr(_) => "Expr", 
                        rustc_hir::Node::Stmt(_) => "Stmt",
                        rustc_hir::Node::TraitItem(_) => "TraitItem",
                        rustc_hir::Node::ImplItem(_) => "ImplItem",
                        rustc_hir::Node::Pat(_) => "Pat",
                        rustc_hir::Node::Ty(_) => "Ty",
                        _ => "Other",
                    }.to_string();
                    
                    let usage_type = match def_kind {
                        rustc_hir::def::DefKind::AssocFn => "MethodCall",
                        rustc_hir::def::DefKind::AssocConst => "AssocConstAccess",
                        rustc_hir::def::DefKind::Variant => "EnumVariant", 
                        rustc_hir::def::DefKind::Struct => "StructUsage",
                        rustc_hir::def::DefKind::Enum => "EnumUsage",
                        rustc_hir::def::DefKind::Fn => "FunctionCall",
                        rustc_hir::def::DefKind::Const => "ConstantAccess",
                        rustc_hir::def::DefKind::Static { .. } => "StaticAccess",
                        rustc_hir::def::DefKind::Macro(_) => "MacroCall",
                        rustc_hir::def::DefKind::TyAlias => "TypeAlias",
                        rustc_hir::def::DefKind::Union => "UnionUsage",
                        rustc_hir::def::DefKind::Trait => "TraitUsage",
                        _ => "Other",
                    }.to_string();
                    
                    let symbol = used_path.split("::").last().unwrap_or(&used_path).to_string();
                    let kind = match def_kind {
                        rustc_hir::def::DefKind::Variant => "enum_variant_usage",
                        rustc_hir::def::DefKind::Struct => "struct_usage", 
                        rustc_hir::def::DefKind::Enum => "enum_usage",
                        rustc_hir::def::DefKind::Macro(_) => "macro_usage",
                        rustc_hir::def::DefKind::AssocFn => "method_call",
                        rustc_hir::def::DefKind::Fn => "function_call",
                        rustc_hir::def::DefKind::Const => "const_usage",
                        rustc_hir::def::DefKind::Static { .. } => "static_usage",
                        rustc_hir::def::DefKind::TyAlias => "type_alias_usage",
                        rustc_hir::def::DefKind::Union => "union_usage",
                        rustc_hir::def::DefKind::Trait => "trait_usage",
                        rustc_hir::def::DefKind::AssocTy => "assoc_type_usage",
                        rustc_hir::def::DefKind::AssocConst => "assoc_const_usage",
                        rustc_hir::def::DefKind::Mod => "module_usage",
                        rustc_hir::def::DefKind::Field => "field_usage",
                        rustc_hir::def::DefKind::Ctor(_, _) => "constructor_usage",
                        _ => "unknown_usage",
                    }.to_string();
                    
                    let user_def_id_str = format!("{:?}", item_id.owner_id.to_def_id());
                    let used_def_id_str = format!("{:?}", used_def_id);
                    let used_def_kind_str = format!("{:?}", def_kind);
                    let user_def_kind_str = format!("{:?}", tcx.def_kind(item_id.owner_id.to_def_id()));
                    
                    self.add_usage(&module, symbol, kind, usage_type, node_type, user_def_id_str, used_def_id_str, Some(local_crate.to_string()), Some(used_crate.to_string()), Some(used_def_kind_str), Some(user_def_kind_str));
                }
            }
        }
        
        self.collect_constants(tcx);
        self.save_to_files(&local_crate.to_string());
        
        Compilation::Continue
    }

}
