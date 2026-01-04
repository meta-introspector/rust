use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let processed_files = [
        "src/temp_processed_.._rust_library_alloc_src_collections_btree_append.rs",
        "src/temp_processed_.._rust_compiler_rustc_macros_src_diagnostics_diagnostic_builder.rs",
        "src/temp_processed_.._rust_library_alloc_src_alloc.rs",
        "src/temp_processed_dot_dot_rust_compiler_rustc_session_src_filesearch.rs"
    ];

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
    
    generate_skeleton()?;
    Ok(())
}

fn extract_module_name(path: &str) -> String {
    if path.contains("merge_iter") { "merge_iter".to_string() }
    else if path.contains("node") { "node".to_string() }
    else if path.contains("utils") { "utils".to_string() }
    else { "append".to_string() }
}

fn extract_clean_declarations(content: &str, output: &mut String) {
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") || line.starts_with("#[") {
            i += 1;
            continue;
        }
        
        // Extract struct/enum declarations
        if line.starts_with("pub struct ") || line.starts_with("pub enum ") {
            output.push_str(lines[i]);
            output.push('\n');
        }
        
        // Extract impl blocks properly
        else if line.starts_with("impl") {
            let impl_start = i;
            let mut brace_count = 0;
            let mut found_opening = false;
            
            // Find the complete impl block
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
            
            // Extract just the impl signature and method signatures
            output.push_str(lines[impl_start]);
            output.push_str(" {\n");
            
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

fn generate_skeleton() -> Result<()> {
    let skeleton = r#"// Auto-generated complete skeleton
include!("generated_merge_iter_decl.rs");
include!("generated_node_decl.rs"); 
include!("generated_utils_decl.rs");
"#;
    
    fs::write("src/generated_complete_skeleton.rs", skeleton)?;
    println!("Generated src/generated_complete_skeleton.rs");
    Ok(())
}
