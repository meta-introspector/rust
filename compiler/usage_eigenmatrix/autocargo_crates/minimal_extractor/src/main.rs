use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let mut processed_files = Vec::new();
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

    // First pass: collect all imports
    for file_path in &processed_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            collect_imports(&content, &mut all_imports);
        }
    }

    // Second pass: generate declaration files
    for file_path in &processed_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            let module_name = extract_module_name(file_path);
            let decl_file = format!("src/generated_{}_decl.rs", module_name);
            
            let mut output = format!("// Generated declarations for {} module\n\n", module_name);
            extract_minimal_stubs(&content, &mut output);
            
            fs::write(&decl_file, output)?;
            println!("Generated {}", decl_file);
        }
    }
    
    generate_skeleton(&processed_files, &all_imports)?;
    Ok(())
}

fn extract_module_name(path: &str) -> String {
    path.split('_').last().unwrap_or("unknown").replace(".rs", "")
}

fn collect_imports(content: &str, imports: &mut Vec<String>) {
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.contains("super::") && trimmed.starts_with("use ") {
            // Parse: use super::module::Type;
            if let Some(start) = trimmed.find("super::") {
                let after_super = &trimmed[start + 7..];
                let import_part = after_super.trim_end_matches(';');
                
                if let Some(double_colon) = import_part.find("::") {
                    let module_name = &import_part[..double_colon];
                    let type_part = &import_part[double_colon + 2..];
                    
                    // Handle {self, Type} syntax
                    if type_part.starts_with('{') && type_part.ends_with('}') {
                        let types = &type_part[1..type_part.len()-1];
                        let import_key = format!("{}::{}", module_name, types);
                        println!("DEBUG: Found import {}", import_key);
                        if !imports.contains(&import_key) {
                            imports.push(import_key);
                        }
                    } else {
                        let import_key = format!("{}::{}", module_name, type_part);
                        println!("DEBUG: Found import {}", import_key);
                        if !imports.contains(&import_key) {
                            imports.push(import_key);
                        }
                    }
                }
            }
        }
    }
}

fn extract_minimal_stubs(content: &str, output: &mut String) {
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Only extract complete struct/enum declarations with semicolon
        if (trimmed.starts_with("pub struct ") || trimmed.starts_with("pub enum ")) && trimmed.ends_with(';') {
            output.push_str(line);
            output.push('\n');
        }
        
        // Extract simple method signatures that are complete on one line
        if (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ")) 
           && trimmed.contains("(") && trimmed.contains(")") && !trimmed.contains("{") {
            let stub = format!("{} {{ unimplemented!() }}", trimmed.trim_end_matches(';'));
            output.push_str(&stub);
            output.push('\n');
        }
    }
}

fn generate_skeleton(processed_files: &[String], imports: &[String]) -> Result<()> {
    let mut skeleton = String::from("// Auto-generated minimal skeleton\n");
    
    for file_path in processed_files {
        let module_name = extract_module_name(file_path);
        skeleton.push_str(&format!("include!(\"generated_{}_decl.rs\");\n", module_name));
    }
    
    // Add discovered modules with specific types
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
            // Split comma-separated types that weren't in braces
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
    
    fs::write("src/generated_complete_skeleton.rs", skeleton)?;
    println!("Generated src/generated_complete_skeleton.rs");
    Ok(())
}
