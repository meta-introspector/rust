use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let mut processed_files = Vec::new();
    let mut all_macros = Vec::new();
    let mut all_imports = Vec::new();
    
    for entry in fs::read_dir("src")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with("temp_processed_") && name.ends_with(".rs") {
                processed_files.push(path.to_string_lossy().to_string());
            }
        }
    }

    // Extract macros and imports from all files
    for file_path in &processed_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            extract_macro_defs(&content, &mut all_macros);
            collect_imports(&content, &mut all_imports);
        }
    }
    
    generate_macro_skeleton(&all_macros, &all_imports)?;
    Ok(())
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

fn generate_macro_skeleton(macros: &[String], imports: &[String]) -> Result<()> {
    let mut skeleton = String::from("// Macro-based skeleton\n\n");
    
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
    
    for (module, types) in modules {
        skeleton.push_str(&format!("pub mod {} {{\n", module));
        for type_name in types {
            for t in type_name.split(',') {
                let t = t.trim();
                if t == "self" {
                    skeleton.push_str(&format!("    pub struct {}Stub;\n", module));
                } else if !t.is_empty() {
                    skeleton.push_str(&format!("    pub struct {};\n", t));
                }
            }
        }
        skeleton.push_str("}\n");
    }
    
    skeleton.push('\n');
    
    // Add file content macros
    for macro_def in macros {
        skeleton.push_str(&macro_def);
        skeleton.push('\n');
    }
    
    fs::write("src/generated_macro_skeleton.rs", skeleton)?;
    println!("Generated macro-based skeleton");
    Ok(())
}

fn extract_macro_defs(content: &str, macros: &mut Vec<String>) {
    // Create deterministic macro name from content hash
    let file_hash = content.len(); // Simple but deterministic
    let macro_name = format!("include_file_{}", file_hash);
    
    let macro_def = format!("macro_rules! {} {{ () => {{ {} }} }}", macro_name, content);
    
    if !macros.contains(&macro_def) {
        macros.push(macro_def);
    }
}

fn generate_macro_skeleton_v2(macros: &[String]) -> Result<()> {
    let mut skeleton = String::from("// Macro-based skeleton\n\n");
    
    for macro_def in macros {
        skeleton.push_str(&macro_def);
        skeleton.push('\n');
    }
    
    skeleton.push_str("\n// Expand all macros\n");
    for macro_def in macros {
        if let Some(name_end) = macro_def.find("_decl") {
            if let Some(name_start) = macro_def.find("macro_rules! ") {
                let macro_name = &macro_def[name_start + 13..name_end + 5];
                skeleton.push_str(&format!("{}!();\n", macro_name));
            }
        }
    }
    
    fs::write("src/generated_macro_skeleton.rs", skeleton)?;
    println!("Generated macro-based skeleton");
    Ok(())
}
