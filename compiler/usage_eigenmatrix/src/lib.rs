use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

/// LatticePointDerive proc macro
#[proc_macro_derive(LatticePointDerive)]
pub fn lattice_point_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Macro that applies Morse-harmonic analysis to any Rust type
#[proc_macro_derive(MorseHarmonic)]
pub fn morse_harmonic_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics.params;
    
    let prime_signature = generate_prime_signature(&input);
    let harmonic_freq = calculate_harmonic_frequency(&prime_signature);
    let morse_value = calculate_morse_critical_value(&input);
    
    let expanded = quote! {
        impl<#generics> #name<#generics> {
            pub fn prime_signature() -> Vec<u64> {
                vec![#(#prime_signature),*]
            }
            
            pub fn harmonic_frequency() -> f64 {
                #harmonic_freq
            }
            
            pub fn morse_critical_value() -> f64 {
                #morse_value
            }
            
            pub fn topological_type() -> &'static str {
                match #morse_value {
                    x if x > 0.1 => "Peak (Maximum)",
                    x if x > 0.05 => "Saddle Point", 
                    x if x > 0.01 => "Local Minimum",
                    _ => "Degenerate"
                }
            }
            
            pub fn semantic_resonance() -> u64 {
                (#harmonic_freq as u64) % 1000
            }
        }
        
        impl<#generics> std::fmt::Display for #name<#generics> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} [Prime: {:?}, Harmonic: {:.0}, Morse: {:.6}, Type: {}]",
                       stringify!(#name),
                       Self::prime_signature(),
                       Self::harmonic_frequency(),
                       Self::morse_critical_value(),
                       Self::topological_type())
            }
        }
    };
    
    TokenStream::from(expanded)
}

fn generate_prime_signature(input: &DeriveInput) -> Vec<u64> {
    let primes = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let mut signature = Vec::new();
    
    // Base prime for type category
    match &input.data {
        Data::Struct(_) => signature.push(primes[0]), // 2 for structs
        Data::Enum(_) => signature.push(primes[1]),   // 3 for enums  
        Data::Union(_) => signature.push(primes[2]),  // 5 for unions
    }
    
    // Field count prime
    let field_count = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => fields.named.len(),
            Fields::Unnamed(fields) => fields.unnamed.len(),
            Fields::Unit => 0,
        },
        Data::Enum(data_enum) => data_enum.variants.len(),
        Data::Union(data_union) => data_union.fields.named.len(),
    };
    
    if field_count < primes.len() {
        signature.push(primes[field_count]);
    }
    
    // Semantic primes based on name patterns
    let name_str = input.ident.to_string();
    if name_str.contains("Option") { signature.push(11); }
    if name_str.contains("Result") { signature.push(13); }
    if name_str.contains("Vec") { signature.push(17); }
    if name_str.contains("HashMap") { signature.push(19); }
    if name_str.contains("Iterator") { signature.push(23); }
    if name_str.contains("Future") { signature.push(29); }
    
    signature
}

fn calculate_harmonic_frequency(signature: &[u64]) -> f64 {
    signature.iter().product::<u64>() as f64
}

fn calculate_morse_critical_value(input: &DeriveInput) -> f64 {
    let name_len = input.ident.to_string().len() as f64;
    let field_count = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => fields.named.len(),
            Fields::Unnamed(fields) => fields.unnamed.len(), 
            Fields::Unit => 0,
        },
        Data::Enum(data_enum) => data_enum.variants.len(),
        Data::Union(data_union) => data_union.fields.named.len(),
    } as f64;
    
    // Morse function: complexity vs simplicity balance
    if field_count > 0.0 {
        (name_len / 100.0) * (1.0 / field_count) * (field_count / (field_count + 1.0))
    } else {
        name_len / 1000.0
    }
}

/// Macro for inline harmonic analysis
#[proc_macro]
pub fn harmonic_analyze(input: TokenStream) -> TokenStream {
    let input_tokens: TokenStream2 = input.into();
    
    let expanded = quote! {
        {
            let prime_sig = vec![2u64, 3, 5];
            let harmonic = prime_sig.iter().product::<u64>() as f64;
            let morse = 0.001f64;
            
            println!("🌊 Harmonic Analysis | Prime: {:?} | Harmonic: {:.0} | Morse: {:.6}", 
                     prime_sig, harmonic, morse);
            
            #input_tokens
        }
    };
    
    TokenStream::from(expanded)
}
