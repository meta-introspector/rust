// Function wrapper injection for rustc_driver
// Inject tracing into all compiler functions

use syn::{parse_file, visit_mut::VisitMut, ItemFn, Stmt, parse_quote};
use quote::quote;

pub struct FunctionWrapper;

impl VisitMut for FunctionWrapper {
    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) {
        let fn_name = &node.sig.ident;
        let fn_name_str = fn_name.to_string();
        
        // Skip already wrapped functions
        if fn_name_str.starts_with("traced_") {
            return;
        }
        
        // Add tracing to function body
        let trace_enter: Stmt = parse_quote! {
            println!("TRACE_ENTER: {}", #fn_name_str);
        };
        
        let trace_exit: Stmt = parse_quote! {
            println!("TRACE_EXIT: {}", #fn_name_str);
        };
        
        // Insert at beginning and end of function
        if let Some(block) = &mut node.block.stmts.first_mut() {
            node.block.stmts.insert(0, trace_enter);
        }
        
        // Continue visiting nested items
        syn::visit_mut::visit_item_fn_mut(self, node);
    }
}

pub fn inject_function_tracing(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut ast = parse_file(source)?;
    let mut wrapper = FunctionWrapper;
    wrapper.visit_file_mut(&mut ast);
    Ok(quote!(#ast).to_string())
}
