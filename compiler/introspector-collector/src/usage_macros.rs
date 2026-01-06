use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Macro to compile usage patterns into executable code
#[proc_macro]
pub fn usage_to_code(input: TokenStream) -> TokenStream {
    let usage_pattern = parse_macro_input!(input as LitStr);
    let pattern = usage_pattern.value();
    
    let generated = match pattern.as_str() {
        "parse_file" => quote! {
            |content: &str| -> syn::File {
                syn::parse_file(content).expect("Failed to parse file")
            }
        },
        "visit_items" => quote! {
            |items: &[syn::Item]| {
                for item in items {
                    match item {
                        syn::Item::Fn(func) => println!("Function: {}", func.sig.ident),
                        syn::Item::Struct(s) => println!("Struct: {}", s.ident),
                        _ => {}
                    }
                }
            }
        },
        "extract_tokens" => quote! {
            |item: &syn::Item| -> String {
                use quote::ToTokens;
                item.to_token_stream().to_string()
            }
        },
        _ => quote! {
            || { println!("Unknown usage pattern: {}", #pattern); }
        }
    };
    
    TokenStream::from(generated)
}

/// Macro to generate usage-driven walker from usage data
#[proc_macro]
pub fn generate_walker(input: TokenStream) -> TokenStream {
    let usage_file = parse_macro_input!(input as LitStr);
    
    let generated = quote! {
        pub struct GeneratedWalker {
            usage_weights: std::collections::HashMap<String, f64>,
        }
        
        impl GeneratedWalker {
            pub fn from_usage_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
                let usage_data = introspector_collector::libusagedata::load_crate_usage("syn", path)?;
                let weights = introspector_collector::libusagedata::get_symbol_weights(&usage_data);
                Ok(Self { usage_weights: weights })
            }
            
            pub fn walk_with_pattern<F>(&self, items: &[syn::Item], mut executor: F) 
            where F: FnMut(&syn::Item, f64) {
                for item in items {
                    let name = match item {
                        syn::Item::Fn(f) => f.sig.ident.to_string(),
                        syn::Item::Struct(s) => s.ident.to_string(),
                        syn::Item::Enum(e) => e.ident.to_string(),
                        _ => continue,
                    };
                    
                    let weight = self.usage_weights.get(&name).copied().unwrap_or(0.0);
                    executor(item, weight);
                }
            }
        }
    };
    
    TokenStream::from(generated)
}
