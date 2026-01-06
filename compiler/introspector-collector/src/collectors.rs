use crate::usage_collector::UsageCollector;
use crate::data_structures::*;
use crate::visitors::{LiteralVisitor, TypeUsageVisitor};

use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_hir::intravisit::{self, Visitor};
use std::io::Write;

impl UsageCollector {
    pub fn collect_constants<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
        
        for item_id in tcx.hir_crate_items(()).free_items() {
            let item = tcx.hir_expect_item(item_id.owner_id.def_id);
            
            if std::env::var("DUMP_HIR").is_ok() {
                eprintln!("=== ITEM DUMP ===");
                eprintln!("Item ID: {:?}", item_id);
                eprintln!("Item Kind: {:?}", std::mem::discriminant(&item.kind));
                eprintln!("Item: {:#?}", item);
                eprintln!("=== END ITEM DUMP ===");
            }
            
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
                    
                    let body = tcx.hir_body(*body_id);
                    self.collect_literals_from_expr(&body.value, &format!("const {}", item_name));
                }
                
                rustc_hir::ItemKind::Static(_, _, _, body_id) => {
                    self.add_usage(
                        "constants",
                        item_name.clone(),
                        "static_decl".to_string(),
                        "StaticDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None,
                        Some("Static".to_string()),
                        Some("Static".to_string())
                    );
                    
                    let body = tcx.hir_body(*body_id);
                    self.collect_literals_from_expr(&body.value, &format!("static {}", item_name));
                }
                
                rustc_hir::ItemKind::Struct(variant_data, _generics, _) => {
                    self.add_usage(
                        "structs",
                        item_name.clone(),
                        "struct_decl".to_string(),
                        "StructDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None,
                        Some("Struct".to_string()),
                        Some("Struct".to_string())
                    );
                    
                    let fields = self.extract_struct_fields(variant_data);
                    let complexity = match fields.len() {
                        0..=3 => "simple",
                        4..=8 => "medium", 
                        _ => "complex"
                    };
                    
                    self.item_complexity.insert(item_name.clone(), ItemComplexity {
                        name: item_name.clone(),
                        item_type: "struct".to_string(),
                        complexity: complexity.to_string(),
                        field_count: Some(fields.len()),
                        variant_count: None,
                        param_count: None,
                        fields: Some(fields),
                        variants: None,
                        parameters: None,
                    });
                }
                
                rustc_hir::ItemKind::Enum(enum_def, _generics, _) => {
                    self.add_usage(
                        "enums",
                        item_name.clone(),
                        "enum_decl".to_string(),
                        "EnumDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None,
                        Some("Enum".to_string()),
                        Some("Enum".to_string())
                    );
                    
                    let variants = self.extract_enum_variants(enum_def);
                    let complexity = match variants.len() {
                        0..=3 => "simple",
                        4..=8 => "medium",
                        _ => "complex"
                    };
                    
                    self.item_complexity.insert(item_name.clone(), ItemComplexity {
                        name: item_name.clone(),
                        item_type: "enum".to_string(),
                        complexity: complexity.to_string(),
                        field_count: None,
                        variant_count: Some(variants.len()),
                        param_count: None,
                        fields: None,
                        variants: Some(variants),
                        parameters: None,
                    });
                }
                
                rustc_hir::ItemKind::Fn { sig, .. } => {
                    let params = self.extract_function_params(sig);
                    let complexity = match params.len() {
                        0..=2 => "simple",
                        3..=5 => "medium",
                        _ => "complex"
                    };
                    
                    self.item_complexity.insert(item_name.clone(), ItemComplexity {
                        name: item_name.clone(),
                        item_type: "function".to_string(),
                        complexity: complexity.to_string(),
                        field_count: None,
                        variant_count: None,
                        param_count: Some(params.len()),
                        fields: None,
                        variants: None,
                        parameters: Some(params),
                    });
                }
                
                _ => {}
            }
        }
    }
    
    pub fn collect_literals_from_expr<'tcx>(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>, context: &str) {
        let mut visitor = LiteralVisitor { 
            collector: self, 
            context: context.to_string()
        };
        visitor.visit_expr(expr);
    }
    
    pub fn collect_type_usage<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
        
        for item_id in tcx.hir_crate_items(()).free_items() {
            let item = tcx.hir_expect_item(item_id.owner_id.def_id);
            
            let mut visitor = TypeUsageVisitor { 
                collector: self, 
                crate_name: crate_name.clone(),
                tcx
            };
            
            intravisit::walk_item(&mut visitor, item);
        }
    }
    
    pub fn extract_struct_fields(&self, variant_data: &rustc_hir::VariantData) -> Vec<FieldInfo> {
        variant_data.fields().iter().map(|field| {
            FieldInfo {
                name: field.ident.name.to_string(),
                field_type: "unknown".to_string(),
                is_public: field.vis_span.is_dummy(),
            }
        }).collect()
    }
    
    pub fn extract_enum_variants(&self, enum_def: &rustc_hir::EnumDef) -> Vec<VariantInfo> {
        enum_def.variants.iter().map(|variant| {
            let field_count = variant.data.fields().len();
            VariantInfo {
                name: variant.ident.name.to_string(),
                has_fields: field_count > 0,
                field_count,
            }
        }).collect()
    }
    
    pub fn extract_function_params(&self, sig: &rustc_hir::FnSig) -> Vec<ParamInfo> {
        sig.decl.inputs.iter().enumerate().map(|(i, _param)| {
            ParamInfo {
                name: format!("param_{}", i),
                param_type: "unknown".to_string(),
            }
        }).collect()
    }
}
