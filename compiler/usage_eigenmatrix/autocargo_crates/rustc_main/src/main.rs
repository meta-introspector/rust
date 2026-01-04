use std::env;
use std::process;

fn main() {
    println!("🚀 SRC: src/bin/rustc_main.rs - Starting rustc_main");
    
    // Get target from command line or use default
    let target = env::args().nth(1).unwrap_or_else(|| {
        "rustc_driver_impl::lib::main".to_string()
    });

    println!("🎯 SRC: src/bin/rustc_main.rs - Running rustc main with target: {}", target);

    // Load the module tree for the target
    let module_file = format!("src/bin_trees/{}.rs", 
        target.replace("::", "_"));
    
    if !std::path::Path::new(&module_file).exists() {
        eprintln!("❌ Module tree not found: {}", module_file);
        eprintln!("Available targets:");
        if let Ok(entries) = std::fs::read_dir("src/bin_trees") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".rs") {
                        let target_name = name.trim_end_matches(".rs").replace("_", "::");
                        eprintln!("  - {}", target_name);
                    }
                }
            }
        }
        process::exit(1);
    }

    // Generate the complete dependency tree
    match generate_complete_code(&target) {
        Ok(code) => {
            println!("✅ SRC: src/bin/rustc_main.rs - Generated complete code ({} bytes)", code.len());
            
            // Write to current.rs as a binary
            if let Err(e) = std::fs::write("src/bin/current.rs", &code) {
                eprintln!("❌ SRC: src/bin/rustc_main.rs - Failed to write current.rs: {}", e);
                process::exit(1);
            }
            
            println!("📝 SRC: src/bin/rustc_main.rs - Written to src/bin/current.rs");
            
            // Try to compile and run
            match compile_and_run() {
                Ok(_) => println!("🎉 SRC: src/bin/rustc_main.rs - Success!"),
                Err(e) => {
                    eprintln!("❌ SRC: src/bin/rustc_main.rs - Compilation failed: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to generate code: {}", e);
            process::exit(1);
        }
    }
}

fn generate_complete_code(target: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("📝 SRC: src/bin/rustc_main.rs - Generating complete code for target: {}", target);
    
    // Base library setup
    let mut code = String::from(r#"#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(no_core)]
#![feature(generic_atomic)]
#![feature(allocator_internals)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(lang_items)]
#![feature(rustc_attrs)]
#![feature(test)]
#![feature(bench_black_box)]
#![feature(custom_test_frameworks)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(never_type)]
#![feature(try_trait_v2)]
#![feature(generator_trait)]
#![feature(async_closure)]

extern crate rustc_ast;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_driver;
extern crate rustc_driver_impl;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_errors;
extern crate rustc_data_structures;
extern crate rustc_interface;

include!("../wrap_types.rs");

"#);

    // Load the module tree (already contains main function)
    let module_file = format!("src/bin_trees/{}.rs", 
        target.replace("::", "_"));
    
    println!("📂 SRC: src/bin/rustc_main.rs - Loading module tree from: {}", module_file);
    
    let module_content = std::fs::read_to_string(&module_file)?;
    
    println!("✅ SRC: src/bin/rustc_main.rs - Loaded {} bytes from module tree", module_content.len());
    
    code.push_str(&module_content);

    Ok(code)
}

fn compile_and_run() -> Result<(), Box<dyn std::error::Error>> {
    // Compile
    let output = process::Command::new("cargo")
        .args(&["build", "--bin", "current"])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Compilation failed:\n{}", 
            String::from_utf8_lossy(&output.stderr)).into());
    }
    
    println!("✅ Compilation successful");
    
    // Run the compiled rustc with test.rs
    let run_output = process::Command::new("./target/debug/current")
        .args(&["test.rs", "-o", "test_output"])
        .output();
    
    match run_output {
        Ok(output) => {
            if output.status.success() {
                println!("📤 Rustc output:\n{}", String::from_utf8_lossy(&output.stdout));
                println!("✅ Successfully compiled test.rs!");
            } else {
                println!("⚠️  Rustc error:\n{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("❌ Failed to run rustc: {}", e);
        }
    }
    
    Ok(())
}
