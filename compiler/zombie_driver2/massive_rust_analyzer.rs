use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 MASSIVE RUST ECOSYSTEM ANALYZER");
    println!("==================================");
    println!("📊 Processing 42,132 Cargo.toml files from 33.9M file index");
    
    // Read all Cargo.toml files from master index
    let output = std::process::Command::new("grep")
        .args(&["Cargo.toml", "/mnt/data1/files.txt"])
        .output()?;
    
    let cargo_files = String::from_utf8(output.stdout)?;
    let cargo_paths: Vec<&str> = cargo_files.lines().collect();
    
    println!("📁 Found {} Cargo.toml files", cargo_paths.len());
    
    // Sample analysis on first 1000 files to avoid overwhelming
    let sample_size = 1000.min(cargo_paths.len());
    println!("🔬 Analyzing sample of {} files", sample_size);
    
    let mut crate_types = HashMap::new();
    let mut dependencies = HashMap::new();
    let mut split_decls_projects = Vec::new();
    let mut syn_quote_projects = Vec::new();
    
    for (i, cargo_path) in cargo_paths.iter().take(sample_size).enumerate() {
        if i % 100 == 0 {
            println!("   Progress: {}/{}", i, sample_size);
        }
        
        // Remove leading "./" if present
        let clean_path = cargo_path.strip_prefix("./").unwrap_or(cargo_path);
        let full_path = format!("/mnt/data1/{}", clean_path);
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            analyze_cargo_toml(&content, cargo_path, &mut crate_types, &mut dependencies, 
                             &mut split_decls_projects, &mut syn_quote_projects);
        }
    }
    
    // Report findings
    println!("\n📊 ANALYSIS RESULTS:");
    println!("====================");
    
    println!("\n🏗️  CRATE TYPES:");
    let mut sorted_types: Vec<_> = crate_types.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    for (crate_type, count) in sorted_types.iter().take(10) {
        println!("   {}: {}", crate_type, count);
    }
    
    println!("\n📦 TOP DEPENDENCIES:");
    let mut sorted_deps: Vec<_> = dependencies.iter().collect();
    sorted_deps.sort_by(|a, b| b.1.cmp(a.1));
    for (dep, count) in sorted_deps.iter().take(15) {
        println!("   {}: {}", dep, count);
    }
    
    println!("\n✂️  SPLIT-DECLS PROJECTS: {}", split_decls_projects.len());
    for project in &split_decls_projects {
        println!("   {}", project);
    }
    
    println!("\n🔧 SYN/QUOTE PROJECTS: {}", syn_quote_projects.len());
    for project in syn_quote_projects.iter().take(10) {
        println!("   {}", project);
    }
    
    // Now find Rust source files for detailed analysis
    println!("\n🦀 FINDING RUST SOURCE FILES...");
    let rust_output = std::process::Command::new("grep")
        .args(&[r"\.rs$", "/mnt/data1/files.txt"])
        .output()?;
    
    let rust_files = String::from_utf8(rust_output.stdout)?;
    let rust_count = rust_files.lines().count();
    println!("📄 Found {} Rust source files", rust_count);
    
    // Focus on split-decls related files
    let split_decls_output = std::process::Command::new("grep")
        .args(&["split.*decl", "/mnt/data1/files.txt"])
        .output()?;
    
    let split_decls_files = String::from_utf8(split_decls_output.stdout)?;
    let split_decls_count = split_decls_files.lines().count();
    println!("✂️  Found {} split-decls related files", split_decls_count);
    
    if split_decls_count > 0 {
        println!("\n🎯 SPLIT-DECLS FILES:");
        for file in split_decls_files.lines().take(10) {
            println!("   {}", file);
        }
    }
    
    Ok(())
}

fn analyze_cargo_toml(
    content: &str,
    path: &str,
    crate_types: &mut HashMap<String, usize>,
    dependencies: &mut HashMap<String, usize>,
    split_decls_projects: &mut Vec<String>,
    syn_quote_projects: &mut Vec<String>,
) {
    // Parse basic TOML structure
    let lines: Vec<&str> = content.lines().collect();
    let mut in_dependencies = false;
    let mut current_crate_type = "bin".to_string(); // default
    
    for line in lines {
        let line = line.trim();
        
        // Detect crate type
        if line.starts_with("name =") {
            if line.contains("split") && line.contains("decl") {
                split_decls_projects.push(path.to_string());
            }
        }
        
        // Check for library vs binary
        if line == "[lib]" {
            current_crate_type = "lib".to_string();
        } else if line.starts_with("[[bin]]") {
            current_crate_type = "bin".to_string();
        }
        
        // Track dependencies section
        if line == "[dependencies]" {
            in_dependencies = true;
            continue;
        } else if line.starts_with('[') && line != "[dependencies]" {
            in_dependencies = false;
        }
        
        // Parse dependencies
        if in_dependencies && line.contains('=') {
            if let Some(dep_name) = line.split('=').next() {
                let dep_name = dep_name.trim().trim_matches('"');
                *dependencies.entry(dep_name.to_string()).or_insert(0) += 1;
                
                // Check for syn/quote usage
                if dep_name == "syn" || dep_name == "quote" {
                    if !syn_quote_projects.contains(&path.to_string()) {
                        syn_quote_projects.push(path.to_string());
                    }
                }
            }
        }
    }
    
    *crate_types.entry(current_crate_type).or_insert(0) += 1;
}
