/// Bootstrap switch macros - minimal stubs for missing generators
/// These will be replaced by the full generator system

#[macro_export]
macro_rules! bootstrap_switch_litkind2string {
    ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
        match $expr {
            $($custom_pattern => ($custom_result).to_string(),)*
            rustc_ast::LitKind::Str(_, _) => ("String", "str"),
            rustc_ast::LitKind::Int(_, _) => ("Integer", "int"), 
            rustc_ast::LitKind::Float(_, _) => ("Float", "float"),
            rustc_ast::LitKind::Bool(_) => ("Boolean", "bool"),
            rustc_ast::LitKind::Char(_) => ("Character", "char"),
            _ => ("Other", "other")
        }
    };
    ($expr:expr) => {
        match $expr {
            rustc_ast::LitKind::Str(_, _) => ("String", "str"),
            rustc_ast::LitKind::Int(_, _) => ("Integer", "int"),
            rustc_ast::LitKind::Float(_, _) => ("Float", "float"), 
            rustc_ast::LitKind::Bool(_) => ("Boolean", "bool"),
            rustc_ast::LitKind::Char(_) => ("Character", "char"),
            _ => ("Other", "other")
        }
    };
}

#[macro_export]
macro_rules! bootstrap_switch_litinttype2string {
    ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
        match $expr {
            $($custom_pattern => ($custom_result).to_string(),)*
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I8) => "i8".to_string(),
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I16) => "i16".to_string(),
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I32) => "i32".to_string(),
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I64) => "i64".to_string(),
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I128) => "i128".to_string(),
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::Isize) => "isize".to_string(),
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U8) => "u8".to_string(),
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U16) => "u16".to_string(),
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U32) => "u32".to_string(),
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U64) => "u64".to_string(),
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U128) => "u128".to_string(),
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::Usize) => "usize".to_string(),
            rustc_ast::LitIntType::Unsuffixed => "unsuffixed".to_string(),
        }
    };
}

#[macro_export]
macro_rules! bootstrap_switch_litfloattype2string {
    ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
        match $expr {
            $($custom_pattern => ($custom_result).to_string(),)*
            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F32) => "f32".to_string(),
            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F64) => "f64".to_string(),
            rustc_ast::LitFloatType::Unsuffixed => "unsuffixed_float".to_string(),
        }
    };
}

#[macro_export]
macro_rules! bootstrap_switch_itemkind {
    ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
        match $expr {
            $($custom_pattern => $custom_result,)*
            rustc_hir::ItemKind::Fn { .. } => "Fn",
            rustc_hir::ItemKind::Struct(..) => "Struct",
            rustc_hir::ItemKind::Enum(..) => "Enum",
            rustc_hir::ItemKind::Const(..) => "Const",
            rustc_hir::ItemKind::Static(..) => "Static",
            rustc_hir::ItemKind::Trait(..) => "Trait",
            rustc_hir::ItemKind::Impl(..) => "Impl",
            rustc_hir::ItemKind::Mod(..) => "Mod",
            rustc_hir::ItemKind::Use(..) => "Use",
            _ => "Other"
        }
    };
}

#[macro_export]
macro_rules! bootstrap_switch_itemkind_custom {
    ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
        bootstrap_switch_itemkind!($expr, { $($custom_pattern => $custom_result),* })
    };
}

#[macro_export]
macro_rules! bootstrap_switch_node2string {
    ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
        match $expr {
            $($custom_pattern => ($custom_result).to_string(),)*
            _ => format!("{:?}", $expr)
        }
    };
    ($expr:expr) => {
        format!("{:?}", $expr)
    };
}
