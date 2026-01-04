// extract_mod_structure.rs - Extract module structure from processed files
use std::fs;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning processed files for module structure...");
    
    let mut super_imports: HashSet<String> = HashSet::new();
    let mut mod_declarations: HashSet<String> = HashSet::new();
    let mut file_paths: HashMap<String, String> = HashMap::new();
    
    // Scan all processed files
    for entry in fs::read_dir("src")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with("temp_processed_") && name.ends_with(".rs") {
                let content = fs::read_to_string(&path)?;
                
                // Extract original file path from comment
                if let Some(src_line) = content.lines().next() {
                    if src_line.starts_with("// SRC: ") {
                        let src_path = src_line.strip_prefix("// SRC: ").unwrap_or("");
                        file_paths.insert(name.to_string(), src_path.to_string());
                    }
                }
                
                // Extract super:: imports
                for line in content.lines() {
                    if line.trim().starts_with("use super::") {
                        if let Some(module) = line.split("::").nth(2) {
                            let module = module.split(":").next().unwrap_or(module);
                            let module = module.split("{").next().unwrap_or(module);
                            let module = module.split(";").next().unwrap_or(module);
                            super_imports.insert(module.trim().to_string());
                        }
                    }
                }
                
                // Extract mod declarations
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("pub mod ") || trimmed.starts_with("mod ") {
                        if let Some(mod_name) = trimmed.split_whitespace().nth(2) {
                            let mod_name = mod_name.split("{").next().unwrap_or(mod_name);
                            mod_declarations.insert(mod_name.to_string());
                        }
                    }
                }
            }
        }
    }
    
    println!("\n📋 Module structure analysis:");
    println!("Super imports needed: {:?}", super_imports);
    println!("Mod declarations found: {:?}", mod_declarations);
    
    // Generate mkmod structure
    println!("\n🏗️ Suggested mkmod structure:");
    for import in &super_imports {
        println!("mkmod! {{ {} {{ pub struct {}; }} }}", import, import);
    }
    
    // Show file paths that need these modules
    println!("\n📁 Files requiring module structure:");
    for (file, src_path) in &file_paths {
        let content = fs::read_to_string(format!("src/{}", file))?;
        if content.contains("use super::") {
            println!("  {} -> {}", src_path, file);
        }
    }
    
    Ok(())
}
