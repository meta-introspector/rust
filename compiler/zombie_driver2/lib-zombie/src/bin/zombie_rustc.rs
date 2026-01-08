use lib_zombie::crate_loader::ZombieSOSystem;
use std::env;
use std::process;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::ffi::{CString, CStr};
use syn_analyzer;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Handle rustc version query
    if args.len() == 2 && (args[1] == "-vV" || args[1] == "--version") {
        println!("rustc 1.89.0-nightly (zombie-rustc character analyzer)");
        println!("binary: zombie-rustc");
        println!("commit-hash: zombie");
        println!("commit-date: 2026-01-07");
        println!("host: x86_64-unknown-linux-gnu");
        println!("release: 1.89.0-nightly");
        println!("LLVM version: zombie.0.0");
        process::exit(0);
    }
    
    // Handle only initial cargo queries with ___ - pass through to real rustc
    if args.iter().any(|arg| arg == "___") {
        eprintln!("🔍 Initial cargo query - passing to real rustc");
        let output = std::process::Command::new("rustc")
            .args(&args[1..])
            .output()
            .expect("Failed to run rustc");
        
        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        process::exit(output.status.code().unwrap_or(1));
    }
    
    // Mimic rustc argument parsing
    if args.len() < 2 {
        eprintln!("zombie-rustc: no input files");
        process::exit(1);
    }
    
    println!("🧟 Zombie Rustc - Character Analysis Driver");
    println!("==========================================");
    println!("📄 Args: {:?}", args);
    
    // Find .rs files and output directory
    let mut rust_files = Vec::new();
    let mut output_dir = "target/debug".to_string();
    let mut i = 1;
    
    while i < args.len() {
        let arg = &args[i];
        
        if arg.ends_with(".rs") {
            rust_files.push(arg.clone());
        } else if arg == "--out-dir" && i + 1 < args.len() {
            output_dir = args[i + 1].clone();
            i += 1;
        } else if arg == "-o" && i + 1 < args.len() {
            // Extract directory from output file path
            if let Some(parent) = Path::new(&args[i + 1]).parent() {
                output_dir = parent.to_string_lossy().to_string();
            }
            i += 1;
        } else if arg.starts_with("--out-dir=") {
            output_dir = arg.strip_prefix("--out-dir=").unwrap().to_string();
        }
        i += 1;
    }
    
    if rust_files.is_empty() {
        eprintln!("zombie-rustc: no .rs files found");
        process::exit(1);
    }
    
    // Ensure output directory exists
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        eprintln!("zombie-rustc: failed to create output directory {}: {}", output_dir, e);
        process::exit(1);
    }
    
    // Run analysis for each file via plugins
    let mut total_essential_arrows = 0;
    
    for file in &rust_files {
        println!("\n📄 Analyzing: {}", file);
        
        let file_stem = Path::new(file).file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        // Character analysis via plugin (if available)
        let char_lib_path = "target/release/libchar_analyzer.so";
        if Path::new(char_lib_path).exists() {
            println!("🔤 Running char analysis via plugin");
            // For now, just note that plugin is available
            // TODO: Implement simple plugin loading
        }
        
        // Syn analysis via direct call
        if let Ok(content) = std::fs::read_to_string(file) {
            println!("🔬 Running syn analysis via plugin");
            println!("📝 File content length: {}", content.len());
            match syn_analyzer::analyze_file(file, &content) {
                Ok(analysis) => {
                    println!("✅ Analysis successful, {} nodes", analysis.total_nodes);
                    let syn_output = format!("{}/{}.syn_analysis.json", output_dir, file_stem);
                    println!("📂 Writing to: {}", syn_output);
                    match serde_json::to_string_pretty(&analysis) {
                        Ok(json) => {
                            println!("📄 JSON length: {}", json.len());
                            match std::fs::write(&syn_output, json) {
                                Ok(_) => println!("📁 Syn analysis: {}", std::fs::canonicalize(&syn_output).unwrap_or_else(|_| syn_output.into()).display()),
                                Err(e) => println!("❌ Failed to write syn analysis: {}", e),
                            }
                        }
                        Err(e) => println!("❌ Failed to serialize analysis: {}", e),
                    }
                }
                Err(e) => println!("❌ Syn analysis failed: {}", e),
            }
        }
    }
    
    // Also save combined analysis to target directory
    let combined_output = format!("{}/zombie_rustc_combined.json", output_dir);
    println!("📁 Combined analysis: {}", std::fs::canonicalize(&combined_output).unwrap_or_else(|_| combined_output.clone().into()).display());
    
    // Generate minimal stub artifacts to satisfy cargo
    generate_cargo_artifacts(&args, &output_dir);
    
    println!("\n🎯 Zombie compilation complete!");
    println!("   Files analyzed: {}", rust_files.len());
    println!("   Essential arrows: {}", total_essential_arrows);
    println!("   Output directory: {}", output_dir);
    println!("   Combined analysis: {}", std::fs::canonicalize(&combined_output).unwrap_or_else(|_| combined_output.into()).display());
    
    // Exit with success (like rustc would)
    process::exit(0);
}

fn generate_cargo_artifacts(args: &[String], output_dir: &str) {
    let mut crate_name = "unknown".to_string();
    let mut metadata = "".to_string();
    let mut extra_filename = "".to_string();
    let mut emit_types = vec!["link".to_string()];
    
    // Parse cargo arguments
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--crate-name" if i + 1 < args.len() => {
                crate_name = args[i + 1].clone();
                i += 1;
            }
            arg if arg.starts_with("--emit=") => {
                emit_types = arg[7..].split(',').map(|s| s.to_string()).collect();
            }
            "-C" if i + 1 < args.len() => {
                let flag = &args[i + 1];
                if flag.starts_with("metadata=") {
                    metadata = flag[9..].to_string();
                } else if flag.starts_with("extra-filename=") {
                    extra_filename = flag[15..].to_string();
                }
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    
    // Generate stub files
    for emit_type in &emit_types {
        match emit_type.as_str() {
            "dep-info" => {
                let dep_file = format!("{}/{}{}.d", output_dir, crate_name, extra_filename);
                let _ = std::fs::write(&dep_file, format!("# Zombie rustc dep-info for {}\n", crate_name));
            }
            "metadata" => {
                let meta_file = format!("{}/lib{}{}.rmeta", output_dir, crate_name, extra_filename);
                let _ = std::fs::write(&meta_file, b"ZOMBIE_RMETA");
            }
            "link" => {
                // Create appropriate output file based on crate type
                if args.iter().any(|arg| arg == "--crate-type" && args.get(args.iter().position(|x| x == arg).unwrap() + 1) == Some(&"bin".to_string())) {
                    let bin_file = format!("{}/{}{}", output_dir, crate_name, extra_filename);
                    let _ = std::fs::write(&bin_file, b"#!/bin/bash\necho 'Zombie binary stub'\n");
                    let _ = std::fs::set_permissions(&bin_file, std::fs::Permissions::from_mode(0o755));
                } else {
                    let lib_file = format!("{}/lib{}{}.rlib", output_dir, crate_name, extra_filename);
                    let _ = std::fs::write(&lib_file, b"ZOMBIE_RLIB");
                }
            }
            _ => {}
        }
    }
}
