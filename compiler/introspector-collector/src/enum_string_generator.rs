/// Enum-to-String Macro Generator
/// Extracts usage data and generates mk-enum-to-string-{name}! macros

use std::collections::HashMap;
use syn::{visit::Visit, ItemEnum, Ident};

pub struct EnumStringGenerator {
    pub enums: HashMap<String, EnumInfo>,
    pub usage_data: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct EnumInfo {
    pub name: String,
    pub variants: Vec<String>,
    pub usage_functions: Vec<String>,
}

impl EnumStringGenerator {
    pub fn new() -> Self {
        Self {
            enums: HashMap::new(),
            usage_data: HashMap::new(),
        }
    }

    /// Extract all enums from the codebase
    pub fn extract_enums(&mut self, file: &syn::File) {
        let mut visitor = EnumVisitor::new();
        visitor.visit_file(file);
        
        for (name, variants) in visitor.enums {
            self.enums.insert(name.clone(), EnumInfo {
                name: name.clone(),
                variants,
                usage_functions: Vec::new(),
            });
        }
    }

    /// Find functions that map enums to strings
    pub fn find_enum_string_functions(&mut self) {
        for (enum_name, enum_info) in &mut self.enums {
            // Look for functions like switch_{enum_name}2string
            let function_name = format!("switch_{}2string", enum_name.to_lowercase());
            enum_info.usage_functions.push(function_name);
            
            // Look for match expressions that convert variants to strings
            for variant in &enum_info.variants {
                let usage_key = format!("{}::{}", enum_name, variant);
                self.usage_data.entry(usage_key).or_insert_with(Vec::new)
                    .push("string_conversion".to_string());
            }
        }
    }

    /// Generate mk-enum-to-string-{name}! macro
    pub fn generate_enum_string_macro(&self, enum_name: &str) -> proc_macro2::TokenStream {
        let enum_info = match self.enums.get(enum_name) {
            Some(info) => info,
            None => return quote::quote! { compile_error!("Enum not found"); },
        };

        let macro_name = format!("mk_enum_to_string_{}", enum_name.to_lowercase());
        let macro_ident = syn::Ident::new(&macro_name, proc_macro2::Span::call_site());
        
        let enum_ident = syn::Ident::new(enum_name, proc_macro2::Span::call_site());
        
        // Generate match arms for each variant
        let match_arms: Vec<proc_macro2::TokenStream> = enum_info.variants.iter().map(|variant| {
            let variant_ident = syn::Ident::new(variant, proc_macro2::Span::call_site());
            let variant_str = variant.as_str();
            
            quote::quote! {
                #enum_ident::#variant_ident => #variant_str,
            }
        }).collect();

        quote::quote! {
            #[macro_export]
            macro_rules! #macro_ident {
                ($expr:expr) => {
                    match $expr {
                        #(#match_arms)*
                        _ => "Unknown",
                    }
                };
                ($expr:expr, { $($custom_pattern:pat => $custom_result:expr),* $(,)? }) => {
                    match $expr {
                        $($custom_pattern => $custom_result,)*
                        #(#match_arms)*
                        _ => "Unknown",
                    }
                };
            }
        }
    }

    /// Generate all enum-to-string macros
    pub fn generate_all_macros(&self) -> proc_macro2::TokenStream {
        let macros: Vec<proc_macro2::TokenStream> = self.enums.keys()
            .map(|enum_name| self.generate_enum_string_macro(enum_name))
            .collect();

        quote::quote! {
            // Auto-generated enum-to-string macros
            #(#macros)*
        }
    }
}

struct EnumVisitor {
    enums: HashMap<String, Vec<String>>,
}

impl EnumVisitor {
    fn new() -> Self {
        Self {
            enums: HashMap::new(),
        }
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        let enum_name = node.ident.to_string();
        let variants: Vec<String> = node.variants.iter()
            .map(|v| v.ident.to_string())
            .collect();
        
        self.enums.insert(enum_name, variants);
        syn::visit::visit_item_enum(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_extraction() {
        let code = r#"
            enum Color {
                Red,
                Green,
                Blue,
            }
        "#;
        
        let file = syn::parse_str::<syn::File>(code).unwrap();
        let mut generator = EnumStringGenerator::new();
        generator.extract_enums(&file);
        
        assert!(generator.enums.contains_key("Color"));
        let color_enum = &generator.enums["Color"];
        assert_eq!(color_enum.variants, vec!["Red", "Green", "Blue"]);
    }
}
