use crate::usage_types::*;
use rustc_hir;

pub struct AstExtractor;

impl AstExtractor {
    pub fn extract_struct_fields(variant_data: &rustc_hir::VariantData) -> Vec<FieldInfo> {
        variant_data.fields().iter().map(|field| {
            FieldInfo {
                name: field.ident.to_string(),
                field_type: "unknown".to_string(), // TODO: Extract actual type
                is_public: matches!(field.vis.node, rustc_hir::VisibilityKind::Public),
            }
        }).collect()
    }
    
    pub fn extract_enum_variants(enum_def: &rustc_hir::EnumDef) -> Vec<VariantInfo> {
        enum_def.variants.iter().map(|variant| {
            let field_count = variant.data.fields().len();
            VariantInfo {
                name: variant.ident.to_string(),
                has_fields: field_count > 0,
                field_count,
            }
        }).collect()
    }
    
    pub fn extract_function_params(sig: &rustc_hir::FnSig) -> Vec<ParamInfo> {
        sig.decl.inputs.iter().enumerate().map(|(i, input)| {
            ParamInfo {
                name: format!("param_{}", i),
                param_type: "unknown".to_string(), // TODO: Extract actual type
            }
        }).collect()
    }
}
