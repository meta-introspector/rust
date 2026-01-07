use syn::{parse_quote, ItemEnum, Variant, Fields};
use quote::quote;
use proc_macro2::TokenStream;
use std::fs;

pub struct SynMacroGenerator {
    generated_macros: Vec<TokenStream>,
}

impl SynMacroGenerator {
    pub fn new() -> Self {
        Self {
            generated_macros: Vec::new(),
        }
    }
    
    /// Generate switch macro for a Rust enum using syn
    pub fn generate_enum_switch_macro(&mut self, enum_name: &str, macro_name: &str, rust_path: &str) {
        // Create a synthetic enum definition for common Rust compiler enums
        let enum_def = match enum_name {
            "ItemKind" => parse_quote! {
                pub enum ItemKind {
                    Fn,
                    Struct,
                    Enum,
                    Const,
                    Static,
                    Trait,
                    Impl,
                    Mod,
                    Use,
                    TyAlias,
                    Union,
                }
            },
            "DefKind" => parse_quote! {
                pub enum DefKind {
                    Fn,
                    AssocFn,
                    Struct,
                    Variant,
                    Const,
                    AssocConst,
                    Static,
                    Trait,
                    TyAlias,
                    Mod,
                    Field,
                }
            },
            "ExprKind" => parse_quote! {
                pub enum ExprKind {
                    Lit,
                    Call,
                    Match,
                    If,
                    Loop,
                    Block,
                    Assign,
                    Binary,
                    Unary,
                    Cast,
                    Path,
                    AddrOf,
                    Break,
                    Continue,
                    Ret,
                }
            },
            "Mutability" => parse_quote! {
                pub enum Mutability {
                    Mut,
                    Not,
                }
            },
            "LitKind" => parse_quote! {
                pub enum LitKind {
                    Str,
                    ByteStr,
                    Byte,
                    Char,
                    Int,
                    Float,
                    Bool,
                    Err,
                }
            },
            "Node" => parse_quote! {
                pub enum Node {
                    Item,
                    Expr,
                    Stmt,
                    TraitItem,
                    ImplItem,
                    ForeignItem,
                    Field,
                    AnonConst,
                    Ctor,
                    Pat,
                    Arm,
                    Block,
                    Local,
                    Param,
                    GenericParam,
                    Crate,
                }
            },
            _ => {
                println!("Unknown enum: {}", enum_name);
                return;
            }
        };
        
        self.generate_switch_from_enum(&enum_def, macro_name, rust_path);
    }
    
    fn generate_switch_from_enum(&mut self, enum_def: &ItemEnum, macro_name: &str, rust_path: &str) {
        let macro_ident = syn::Ident::new(macro_name, proc_macro2::Span::call_site());
        
        // Generate match arms for each variant
        let mut default_arms = Vec::new();
        
        for variant in &enum_def.variants {
            let variant_ident = &variant.ident;
            let variant_name = variant_ident.to_string();
            
            // Handle different field types
            let pattern = match &variant.fields {
                Fields::Named(_) => quote! { #rust_path::#variant_ident { .. } },
                Fields::Unnamed(_) => quote! { #rust_path::#variant_ident(..) },
                Fields::Unit => quote! { #rust_path::#variant_ident },
            };
            
            default_arms.push(quote! {
                #pattern => #variant_name.to_string()
            });
        }
        
        // Generate the macro
        let generated_macro = quote! {
            /// Auto-generated switch macro using syn
            #[macro_export]
            macro_rules! #macro_ident {
                ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
                    match $expr {
                        $($custom_pattern => ($custom_result).to_string(),)*
                        #(#default_arms,)*
                        _ => "Other".to_string()
                    }
                };
                ($expr:expr) => {
                    match $expr {
                        #(#default_arms,)*
                        _ => "Other".to_string()
                    }
                };
            }
        };
        
        self.generated_macros.push(generated_macro);
    }
    
    pub fn generate_all_rust_enum_macros(&mut self) {
        println!("🔧 Generating enum switch macros with syn...");
        
        // Generate macros for all major Rust compiler enums
        self.generate_enum_switch_macro("ItemKind", "switch_itemkind2string", "rustc_hir::ItemKind");
        self.generate_enum_switch_macro("DefKind", "switch_defkind2string", "rustc_hir::def::DefKind");
        self.generate_enum_switch_macro("ExprKind", "switch_exprkind2string", "rustc_hir::ExprKind");
        self.generate_enum_switch_macro("Mutability", "switch_mutability2string", "rustc_hir::Mutability");
        self.generate_enum_switch_macro("LitKind", "switch_litkind2string", "rustc_ast::LitKind");
        self.generate_enum_switch_macro("Node", "switch_node2string", "rustc_hir::Node");
        
        println!("✅ Generated {} enum switch macros", self.generated_macros.len());
    }
    
    pub fn save_generated_macros(&self, output_path: &str) -> Result<(), std::io::Error> {
        let mut output = String::new();
        
        output.push_str("// Auto-generated by syn-based macro generator\n");
        output.push_str("// DO NOT EDIT - Generated at build time\n\n");
        
        for macro_tokens in &self.generated_macros {
            output.push_str(&macro_tokens.to_string());
            output.push_str("\n\n");
        }
        
        // Add special helper macros
        output.push_str(&self.generate_special_macros());
        
        fs::write(output_path, output)?;
        println!("💾 Saved generated macros to: {}", output_path);
        Ok(())
    }
    
    fn generate_special_macros(&self) -> String {
        r#"/// Special version for custom pattern matching with ItemKind
#[macro_export]
macro_rules! switch_itemkind_custom {
    ($expr:expr, { $($pattern:pat => $body:expr),* $(,)? }) => {
        match $expr {
            $($pattern => $body,)*
            _ => switch_itemkind2string!($expr)
        }
    };
}

/// Pattern matching switch for ItemKind - executes custom code blocks
#[macro_export]
macro_rules! switch_itemkind {
    ($expr:expr => { $($pattern:pat => $body:block)* }) => {
        match $expr {
            $($pattern => $body)*
        }
    };
}

/// Helper macro for literal types with tuple returns
#[macro_export]
macro_rules! switch_litkind2string {
    ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
        match $expr {
            $($custom_pattern => $custom_result,)*
            rustc_ast::LitKind::Str(..) => ("String".to_string(), "str_"),
            rustc_ast::LitKind::Int(..) => ("Int".to_string(), "int_"),
            rustc_ast::LitKind::Bool(..) => ("Bool".to_string(), "bool_"),
            rustc_ast::LitKind::Float(..) => ("Float".to_string(), "float_"),
            rustc_ast::LitKind::Char(..) => ("Char".to_string(), "char_"),
            _ => ("Other".to_string(), "other_")
        }
    };
}

/// Integer type switch macro
#[macro_export]
macro_rules! switch_litinttype2string {
    ($expr:expr, { $($pattern:pat => $custom:expr),* $(,)? }) => {
        match $expr {
            $($pattern => $custom,)*
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::Isize) => "isize_",
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I8) => "i8_",
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I16) => "i16_",
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I32) => "i32_",
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I64) => "i64_",
            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I128) => "i128_",
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::Usize) => "usize_",
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U8) => "u8_",
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U16) => "u16_",
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U32) => "u32_",
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U64) => "u64_",
            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U128) => "u128_",
            rustc_ast::LitIntType::Unsuffixed => "unsuffixed_",
        }
    };
}

/// Float type switch macro
#[macro_export]
macro_rules! switch_litfloattype2string {
    ($expr:expr, { $($pattern:pat => $custom:expr),* $(,)? }) => {
        match $expr {
            $($pattern => $custom,)*
            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F32) => "f32_",
            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F64) => "f64_",
            rustc_ast::LitFloatType::Unsuffixed => "float_unsuffixed_",
        }
    };
}
"#.to_string()
    }
}
