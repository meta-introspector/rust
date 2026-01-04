use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let mut processed_files = Vec::new();
    
    // Find all processed files
    for entry in fs::read_dir("src")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with("temp_processed_") && name.ends_with(".rs") {
                processed_files.push(path.to_string_lossy().to_string());
            }
        }
    }

    for file_path in &processed_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            let module_name = extract_module_name(file_path);
            let decl_file = format!("src/generated_{}_decl.rs", module_name);
            
            let mut output = format!("// Generated declarations for {} module\n\n", module_name);
            extract_clean_declarations(&content, &mut output);
            
            fs::write(&decl_file, output)?;
            println!("Generated {}", decl_file);
        }
    }
    
    generate_skeleton(&processed_files)?;
    Ok(())
}

fn extract_module_name(path: &str) -> String {
    path.split('_').last().unwrap_or("unknown").replace(".rs", "")
}

fn extract_clean_declarations(content: &str, output: &mut String) {
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.is_empty() || line.starts_with("//") || line.starts_with("#[") {
            i += 1;
            continue;
        }
        
        if line.starts_with("pub struct ") || line.starts_with("pub enum ") {
            output.push_str(lines[i]);
            output.push('\n');
        }
        
        else if line.starts_with("impl") {
            let impl_start = i;
            let mut brace_count = 0;
            let mut found_opening = false;
            
            while i < lines.len() {
                for ch in lines[i].chars() {
                    match ch {
                        '{' => { brace_count += 1; found_opening = true; }
                        '}' => brace_count -= 1,
                        _ => {}
                    }
                }
                
                if found_opening && brace_count == 0 {
                    break;
                }
                i += 1;
            }
            
            // Write impl line without extra brace
            let impl_line = lines[impl_start];
            if impl_line.contains('{') {
                output.push_str(impl_line);
            } else {
                output.push_str(impl_line);
                output.push_str(" {");
            }
            output.push('\n');
            
            for j in (impl_start + 1)..=i {
                if j < lines.len() {
                    let method_line = lines[j].trim();
                    if (method_line.starts_with("pub fn ") || method_line.starts_with("fn ")) 
                       && method_line.contains("(") {
                        let stub = create_method_stub(method_line);
                        output.push_str("    ");
                        output.push_str(&stub);
                        output.push('\n');
                    }
                }
            }
            
            output.push_str("}\n");
        }
        
        i += 1;
    }
}

fn create_method_stub(line: &str) -> String {
    if let Some(end) = line.find(" {") {
        format!("{} {{ unimplemented!() }}", &line[..end])
    } else if line.ends_with(';') {
        line.replace(';', " { unimplemented!() }")
    } else {
        format!("{} {{ unimplemented!() }}", line)
    }
}

fn generate_skeleton(processed_files: &[String]) -> Result<()> {
    let mut skeleton = String::from("// Auto-generated complete skeleton\n");
    
    for file_path in processed_files {
        let module_name = extract_module_name(file_path);
        skeleton.push_str(&format!("include!(\"generated_{}_decl.rs\");\n", module_name));
    }
    
    fs::write("src/generated_complete_skeleton.rs", skeleton)?;
    println!("Generated src/generated_complete_skeleton.rs");
    Ok(())
}
