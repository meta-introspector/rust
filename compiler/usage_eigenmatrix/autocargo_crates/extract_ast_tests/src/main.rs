use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting AST Test Cases from Build Errors");
    
    let build_log = fs::read_to_string("build.log")?;
    let mut error_cases = HashMap::new();
    
    // Parse errors and extract file/line info
    for line in build_log.lines() {
        if line.contains("error[E") && line.contains("-->") {
            if let Some(file_info) = line.split("-->").nth(1) {
                let file_info = file_info.trim();
                if let Some((file_path, line_col)) = file_info.split_once(':') {
                    if file_path.starts_with("src/processed_") {
                        // Extract error type
                        let error_type = line.split('[').nth(1)
                            .and_then(|s| s.split(']').next())
                            .unwrap_or("UNKNOWN");
                        
                        error_cases.entry(file_path.to_string())
                            .or_insert_with(Vec::new)
                            .push((error_type.to_string(), line_col.to_string()));
                    }
                }
            }
        }
    }
    
    println!("📊 Found {} files with errors", error_cases.len());
    
    // Extract AST metadata for each failing file
    for (file_path, errors) in error_cases.iter().take(10) {
        println!("\n🎯 File: {}", file_path);
        
        if let Ok(content) = fs::read_to_string(file_path) {
            // Find AST metadata
            let mut ast_blocks = Vec::new();
            let mut current_ast = None;
            
            for (line_num, line) in content.lines().enumerate() {
                if line.contains("AST_META:") {
                    current_ast = Some((line_num + 1, line.to_string()));
                } else if line.trim().is_empty() && current_ast.is_some() {
                    if let Some((ast_line, ast_meta)) = current_ast.take() {
                        ast_blocks.push((ast_line, ast_meta));
                    }
                }
            }
            
            println!("   AST Blocks: {}", ast_blocks.len());
            for error in errors.iter().take(3) {
                println!("   Error: {} at {}", error.0, error.1);
            }
            
            // Show first AST block as test case
            if let Some((line_num, ast_meta)) = ast_blocks.first() {
                println!("   Sample AST: {}", ast_meta);
            }
        }
    }
    
    // Generate test case template
    let test_case = r##"
#[test]
fn test_ast_compilation() {
    let ast_code = r#"
        // AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=test_fn | COMPLEXITY=5 | LINES=3
        pub fn test_fn() {
            println!("test");
        }
    "#;
    
    // Test compilation
    assert!(compile_ast_fragment(ast_code).is_ok());
}
"##;
    
    fs::write("ast_test_template.rs", test_case)?;
    
    Ok(())
}
