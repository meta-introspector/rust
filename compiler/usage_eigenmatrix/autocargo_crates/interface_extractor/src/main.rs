// interface_extractor.rs - AST-based mock generator for processed files
use std::fs;
use std::env;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <processed_file.rs>", args[0]);
        return Ok(());
    }
    
    let processed_file = &args[1];
    let content = fs::read_to_string(processed_file)?;
    
    println!("// Generated mocks for {}", processed_file);
    
    // Parse the processed file to find super:: imports
    let mut required_modules = HashSet::new();
    for line in content.lines() {
        if line.trim().starts_with("use super::") {
            if let Some(module) = extract_module_name(line) {
                required_modules.insert(module);
            }
        }
    }
    
    // For each required module, extract interface from real source
    for module in &required_modules {
        generate_mock_from_source(module)?;
    }
    
    Ok(())
}

fn extract_module_name(line: &str) -> Option<String> {
    if let Some(start) = line.find("super::") {
        let after_super = &line[start + 7..];
        if let Some(end) = after_super.find("::").or_else(|| after_super.find(";")) {
            return Some(after_super[..end].to_string());
        }
    }
    None
}

fn generate_mock_from_source(module: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = match module {
        "merge_iter" => "../rust/library/alloc/src/collections/btree/merge_iter.rs",
        "node" => "../rust/library/alloc/src/collections/btree/node.rs", 
        "utils" => "../rust/compiler/rustc_macros/src/diagnostics/utils.rs",
        _ => {
            println!("mkmod! {{ {} {{ /* TODO: Unknown module */ }} }}", module);
            return Ok(());
        }
    };
    
    if let Ok(source_content) = fs::read_to_string(source_path) {
        println!("mkmod! {{ {} {{", module);
        extract_interface_from_source(&source_content, module);
        println!("}}");
    } else {
        println!("mkmod! {{ {} {{ /* Source not found: {} */ }} }}", module, source_path);
    }
    
    Ok(())
}

fn extract_interface_from_source(content: &str, module: &str) {
    // Extract struct definitions
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") {
            if let Some(struct_name) = extract_struct_name(trimmed) {
                println!("    pub struct {};", struct_name);
            }
        }
        if trimmed.starts_with("pub const ") {
            println!("    {}", trimmed);
        }
    }
    
    // Extract method signatures (simplified)
    let mut in_impl = false;
    let mut impl_type = String::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("impl") {
            in_impl = true;
            impl_type = extract_impl_type(trimmed);
            if !impl_type.is_empty() {
                println!("    impl {} {{", impl_type);
            }
        } else if in_impl && trimmed == "}" {
            if !impl_type.is_empty() {
                println!("    }}");
            }
            in_impl = false;
            impl_type.clear();
        } else if in_impl && (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ")) {
            let stub_method = create_stub_method(trimmed);
            println!("        {}", stub_method);
        }
    }
}

fn extract_struct_name(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 && (parts[0] == "pub" || parts[0] == "struct") {
        let struct_idx = if parts[0] == "pub" { 2 } else { 1 };
        if let Some(name) = parts.get(struct_idx) {
            return Some(name.trim_end_matches(';').trim_end_matches('{').to_string());
        }
    }
    None
}

fn extract_impl_type(line: &str) -> String {
    if let Some(start) = line.find("impl") {
        let after_impl = &line[start + 4..].trim();
        if let Some(end) = after_impl.find('{').or_else(|| after_impl.find("where")) {
            return after_impl[..end].trim().to_string();
        }
        return after_impl.to_string();
    }
    String::new()
}

fn create_stub_method(line: &str) -> String {
    if let Some(fn_start) = line.find("fn ") {
        let after_fn = &line[fn_start + 3..];
        if let Some(paren_pos) = after_fn.find('(') {
            let fn_name = &after_fn[..paren_pos];
            return format!("pub fn {}(&self) {{ unimplemented!() }}", fn_name);
        }
    }
    format!("// {}", line)
}
