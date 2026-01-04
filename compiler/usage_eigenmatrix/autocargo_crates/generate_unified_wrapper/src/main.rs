use std::fs;
use std::collections::HashMap;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load compressed symbol map
    let file = fs::File::open("symbol_map.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut symbol_map_content = String::new();
    decoder.read_to_string(&mut symbol_map_content)?;
    let symbol_map: HashMap<String, Value> = serde_json::from_str(&symbol_map_content)?;
    
    // Find all rustc crates in dependency order
    let mut crate_order = Vec::new();
    let mut seen_crates = std::collections::HashSet::new();
    
    // Analyze symbol map to determine crate dependencies
    for (symbol, entry) in &symbol_map {
        if let Some(crate_name) = entry.get("crate_name").and_then(|c| c.as_str()) {
            if crate_name.starts_with("rustc_") && !seen_crates.contains(crate_name) {
                crate_order.push(crate_name.to_string());
                seen_crates.insert(crate_name.to_string());
            }
        }
    }
    
    // Generate unified_rustc_wrapped.rs with proper macro calls
    let mut code = String::new();
    
    // Header
    code.push_str("#![recursion_limit = \"512\"]\n");
    code.push_str("#![allow(internal_features)]\n");
    code.push_str("#![allow(unused)]\n");
    code.push_str("#![allow(rustc::untranslatable_diagnostic)]\n");
    code.push_str("#![feature(rustc_private)]\n");
    code.push_str("#![feature(core_intrinsics)]\n");
    code.push_str("#![feature(decl_macro)]\n");
    code.push_str("#![feature(panic_backtrace_config)]\n");
    code.push_str("#![feature(panic_update_hook)]\n");
    code.push_str("#![feature(rustdoc_internals)]\n");
    code.push_str("#![feature(try_blocks)]\n\n");
    
    // Extern crates
    for crate_name in &crate_order {
        code.push_str(&format!("extern crate {};\n", crate_name));
    }
    code.push_str("\n");
    
    // Include macro
    code.push_str("// Custom macro to include processed rustc files in dependency order\n");
    code.push_str("macro_rules! include_rustc {\n");
    code.push_str("    ($crate_name:ident) => {\n");
    code.push_str("        include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/submodules/rust/compiler/\", stringify!($crate_name), \"/src/lib.rs\"));\n");
    code.push_str("    };\n");
    code.push_str("    ($crate_name:ident, $file:ident) => {\n");
    code.push_str("        include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/submodules/rust/compiler/\", stringify!($crate_name), \"/src/\", stringify!($file), \".rs\"));\n");
    code.push_str("    };\n");
    code.push_str("}\n\n");
    
    // Include all crates in dependency order using macros
    code.push_str("// Include all rustc crates in proper dependency order\n");
    for crate_name in &crate_order {
        code.push_str(&format!("include_rustc!({});\n", crate_name));
    }
    code.push_str("\n");
    
    // Main function
    code.push_str("fn main() {\n");
    code.push_str("    println!(\"🚀 Unified rustc wrapper with {} crates loaded\");\n");
    code.push_str("    rustc_driver_impl::main();\n");
    code.push_str("}\n");
    
    // Write the new unified_rustc_wrapped.rs
    fs::write("unified_rustc_wrapped.rs", &code)?;
    
    println!("✅ Generated unified_rustc_wrapped.rs with {} crates in dependency order:", crate_order.len());
    for (i, crate_name) in crate_order.iter().enumerate() {
        println!("  {}: {}", i + 1, crate_name);
    }
    
    Ok(())
}
