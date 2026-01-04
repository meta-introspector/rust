use std::fs;
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 Scanning split-decls-genesis for ingestion\n");
    
    let genesis_path = "../../../split-decls-genesis";
    
    if !Path::new(genesis_path).exists() {
        println!("❌ Genesis path not found: {}", genesis_path);
        return Ok(());
    }
    
    // Scan for Rust files
    scan_rust_files(genesis_path)?;
    
    // Scan for math/data files
    scan_data_files(genesis_path)?;
    
    Ok(())
}

fn scan_rust_files(path: &str) -> Result<()> {
    println!("🦀 Scanning Rust files in {}:", path);
    
    let rust_files = find_files_with_extension(path, "rs")?;
    
    for file in &rust_files {
        println!("  📄 {}", file);
        
        // Read and analyze the file
        if let Ok(content) = fs::read_to_string(file) {
            let lines = content.lines().count();
            let functions = content.matches("fn ").count();
            let structs = content.matches("struct ").count();
            
            println!("    Lines: {}, Functions: {}, Structs: {}", lines, functions, structs);
            
            // Look for interesting patterns
            if content.contains("eigenvalue") || content.contains("matrix") {
                println!("    🧮 Contains math: eigenvalue/matrix");
            }
            if content.contains("DefId") || content.contains("rustc") {
                println!("    🔧 Contains compiler code");
            }
        }
    }
    
    println!("  Total Rust files: {}", rust_files.len());
    Ok(())
}

fn scan_data_files(path: &str) -> Result<()> {
    println!("\n📊 Scanning data files in {}:", path);
    
    let data_extensions = vec!["json", "toml", "yaml", "yml", "txt", "md"];
    
    for ext in data_extensions {
        let files = find_files_with_extension(path, ext)?;
        if !files.is_empty() {
            println!("  {} files ({}): {}", ext.to_uppercase(), files.len(), files.len());
            for file in files.iter().take(3) {
                println!("    📄 {}", file);
            }
            if files.len() > 3 {
                println!("    ... and {} more", files.len() - 3);
            }
        }
    }
    
    Ok(())
}

fn find_files_with_extension(dir: &str, extension: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();
    
    fn visit_dir(dir: &Path, extension: &str, files: &mut Vec<String>) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    visit_dir(&path, extension, files)?;
                } else if let Some(ext) = path.extension() {
                    if ext == extension {
                        if let Some(path_str) = path.to_str() {
                            files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    visit_dir(Path::new(dir), extension, &mut files)?;
    Ok(files)
}
