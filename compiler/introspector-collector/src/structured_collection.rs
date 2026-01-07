use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub name: String,
    pub module: String,
    pub size_hint: Option<usize>,
}

/// Macro to define structured data collection for any Rust type
#[macro_export]
macro_rules! collect_structured {
    ($collector:expr, $category:expr, $item:expr, $type_name:ident) => {
        {
            let structured_data = StructuredData {
                category: $category.to_string(),
                type_name: stringify!($type_name).to_string(),
                canonical_form: canonical_form!($item),
                content_hash: content_hash!($item),
                fields: extract_fields!($item),
                variants: extract_variants!($item),
                usage_context: current_context!(),
            };
            
            $collector.add_structured_usage(structured_data);
        }
    };
}

/// Macro to extract all fields from any structure
#[macro_export]
macro_rules! extract_fields {
    // For HIR Items
    (rustc_hir::Item) => {
        vec![
            field_data!("owner_id", DefId),
            field_data!("ident", Ident), 
            field_data!("kind", ItemKind),
            field_data!("vis_span", Span),
            field_data!("span", Span)
        ]
    };
    
    // For HIR Expressions  
    (rustc_hir::Expr) => {
        vec![
            field_data!("hir_id", HirId),
            field_data!("kind", ExprKind),
            field_data!("span", Span)
        ]
    };
    
    // For DefKind (enum)
    (rustc_hir::def::DefKind) => {
        vec![
            field_data!("discriminant", u8),
            field_data!("variant_data", VariantData)
        ]
    };
    
    // Generic fallback
    ($item:expr) => {
        extract_fields_generic!($item)
    };
}

/// Macro to extract variants from enums
#[macro_export]
macro_rules! extract_variants {
    (rustc_hir::ItemKind) => {
        vec![
            "Const", "Static", "Fn", "Macro", "Use", "ExternCrate",
            "Mod", "ForeignMod", "GlobalAsm", "TyAlias", "OpaqueTy", 
            "Enum", "Struct", "Union", "Trait", "TraitAlias", "Impl"
        ].into_iter().map(|s| s.to_string()).collect()
    };
    
    (rustc_hir::ExprKind) => {
        vec![
            "Box", "ConstBlock", "Array", "Call", "MethodCall", "Tup",
            "Binary", "Unary", "Lit", "Cast", "Type", "DropTemps",
            "Let", "If", "Loop", "Match", "Closure", "Block", "Assign"
        ].into_iter().map(|s| s.to_string()).collect()
    };
    
    ($item:expr) => { vec![] };
}

/// Macro to create field metadata
#[macro_export]
macro_rules! field_data {
    ($name:expr, $type:ty) => {
        FieldMetadata {
            name: $name.to_string(),
            type_name: stringify!($type).to_string(),
            size: std::mem::size_of::<$type>(),
            alignment: std::mem::align_of::<$type>(),
        }
    };
}

/// Macro to wrap any Rust structure for collection
#[macro_export]
macro_rules! wrap_for_collection {
    ($item:expr, $category:expr) => {
        {
            let wrapper = DataWrapper {
                category: $category.to_string(),
                raw_data: format!("{:?}", $item),
                type_info: TypeInfo::from_type(&$item),
                canonical_hash: content_hash!(canonical_form!($item)),
                collection_timestamp: std::time::SystemTime::now(),
            };
            wrapper
        }
    };
}

/// Unified data structure for all collected items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredData {
    pub category: String,           // "hir_item", "ast_expr", "def_kind", etc.
    pub type_name: String,          // "ItemKind", "ExprKind", etc.
    pub canonical_form: String,     // Minimal canonical representation
    pub content_hash: String,       // Content-addressable hash
    pub fields: Vec<FieldMetadata>, // All field information
    pub variants: Vec<String>,      // For enums: all variants
    pub usage_context: UsageContext, // Where/how it's used
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub name: String,
    pub type_name: String,
    pub size: usize,
    pub alignment: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWrapper {
    pub category: String,
    pub raw_data: String,
    pub type_info: TypeInfo,
    pub canonical_hash: String,
    pub collection_timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageContext {
    pub module: String,
    pub function: String,
    pub line: u32,
    pub frequency: u64,
}
