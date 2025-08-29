use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn, Ident, FnArg, Pat, PatType};

#[proc_macro_attribute]
pub fn instrument_syscall(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let instrumented_fn_name = format_ident!("instrumented_{}", fn_name);
    let fn_args = &input.sig.inputs;
    let fn_block = &input.block;
    let fn_return_type = &input.sig.output;

    let arg_names: Vec<Ident> = fn_args.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { pat, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                Some(pat_ident.ident.clone())
            } else {
                None
            }
        } else {
            None
        }
    }).collect();

    let expanded = quote! {
        #input

        // This is a simplified example. In a real scenario, you'd want to
        // capture arguments and return values more robustly and send them
        // to a dedicated logging/tracing system.
        // For now, we'll just print to stdout/stderr.
        #[allow(dead_code)]
        fn #instrumented_fn_name(#fn_args) #fn_return_type {
            println!("Syscall instrumented: Calling function `{}` with arguments: {:?}", stringify!(#fn_name), (#(#arg_names),*));
            let result = #fn_block;
            println!("Syscall instrumented: Function `{}` returned: {:?}", stringify!(#fn_name), result);
            result
        }
    };

    expanded.into()
}