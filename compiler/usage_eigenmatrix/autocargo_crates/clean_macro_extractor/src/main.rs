use std::fs;
use std::path::Path;
use std::io::Result;

fn main() -> Result<()> {
    let mut processed_files = Vec::new();
    let mut all_imports = Vec::new();
    
    for entry in fs::read_dir("src")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with("processed_") && name.ends_with(".rs") {
                processed_files.push(path.to_string_lossy().to_string());
            }
        }
    }

    // Create mirror structure and collect imports
    for file_path in &processed_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            create_mirror_file(&content)?;
            collect_imports(&content, &mut all_imports);
        }
    }
    
    generate_wrap_types(&all_imports)?;
    Ok(())
}

fn create_mirror_file(content: &str) -> Result<()> {
    if let Some(original_path) = extract_original_path(content) {
        let mirror_path = create_mirror_path(&original_path);
        
        if let Some(parent) = Path::new(&mirror_path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&mirror_path, content)?;
        println!("Mirrored -> {}", mirror_path);
    }
    Ok(())
}

fn extract_original_path(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with("// SRC: ") {
            return Some(line[8..].to_string());
        }
    }
    None
}

fn create_mirror_path(original_path: &str) -> String {
    // Convert ../rust/library/alloc/src/collections/btree/append.rs
    // to submodules/rust/library/alloc/src/collections/btree/append.rs
    let path = original_path.strip_prefix("../rust/").unwrap_or(original_path);
    format!("submodules/rust/{}", path)
}

fn collect_imports(content: &str, imports: &mut Vec<String>) {
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.contains("super::") && trimmed.starts_with("use ") {
            if let Some(start) = trimmed.find("super::") {
                let after_super = &trimmed[start + 7..];
                let import_part = after_super.trim_end_matches(';');
                
                if let Some(double_colon) = import_part.find("::") {
                    let module_name = &import_part[..double_colon];
                    let type_part = &import_part[double_colon + 2..];
                    
                    let import_key = format!("{}::{}", module_name, type_part);
                    if !imports.contains(&import_key) {
                        imports.push(import_key);
                    }
                }
            }
        }
    }
}

fn generate_wrap_types(imports: &[String]) -> Result<()> {
    let mut wrap_types = String::from("// wrap_types.rs - Dynamically generated support types\n\n");
    
    // Generate modules from imports
    let mut modules = std::collections::HashMap::new();
    for import in imports {
        if let Some(colon_pos) = import.find("::") {
            let module_name = &import[..colon_pos];
            let type_part = &import[colon_pos + 2..];
            
            modules.entry(module_name.to_string())
                   .or_insert_with(Vec::new)
                   .push(type_part.to_string());
        }
    }
    
    let mut seen_types = std::collections::HashSet::new();
    
    for (module, types) in modules {
        if module == "super" {
            continue; // Skip 'super' module as it's a keyword
        }
        wrap_types.push_str(&format!("pub mod {} {{\n", module));
        for type_name in types {
            for t in type_name.split(',') {
                let t = t.trim()
                    .replace("{", "")
                    .replace("}", "")
                    .replace("*", "Star")
                    .replace(":", "");
                
                if t == "self" {
                    let stub_name = format!("{}Stub", module);
                    if seen_types.insert(stub_name.clone()) {
                        wrap_types.push_str(&format!("    pub struct {};\n", stub_name));
                    }
                } else if !t.is_empty() && t.chars().all(|c| c.is_alphanumeric() || c == '_') && t != "super" {
                    if seen_types.insert(t.clone()) {
                        wrap_types.push_str(&format!("    pub struct {};\n", t));
                    }
                }
            }
        }
        wrap_types.push_str("}\n");
    }
    
    fs::write("src/wrap_types.rs", wrap_types)?;
    println!("Generated src/wrap_types.rs");
    Ok(())
}
