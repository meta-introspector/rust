use std::collections::HashMap;
use std::process::Command;
use regex::Regex;

#[derive(Debug, Clone)]
struct CompilationError {
    error_type: String,
    location: String,
    message: String,
    suggestion: Option<String>,
}

fn main() {
    println!("🔍 Error Analyzer - Auto-fixing unified_rustc_wrapped");
    
    // Get compilation errors
    let errors = get_compilation_errors();
    
    // Categorize errors
    let categorized = categorize_errors(errors);
    
    // Generate minimal test cases
    for (error_type, error_list) in &categorized {
        println!("\n📊 {} errors: {}", error_type, error_list.len());
        
        // Take first 3 examples for analysis
        for (i, error) in error_list.iter().take(3).enumerate() {
            create_minimal_test(error_type, i, error);
        }
    }
    
    // Generate fixes
    generate_auto_fixes(&categorized);
}

fn get_compilation_errors() -> Vec<CompilationError> {
    let output = Command::new("cargo")
        .args(&["check", "--bin", "unified_rustc_wrapped", "--message-format=json"])
        .output()
        .expect("Failed to run cargo check");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    parse_errors(&stderr)
}

fn parse_errors(stderr: &str) -> Vec<CompilationError> {
    let mut errors = Vec::new();
    
    // Parse different error patterns
    let patterns = vec![
        (r"error\[E(\d+)\]: (.+)", "compile_error"),
        (r"error: duplicate diagnostic item.+: `(.+)`", "duplicate_diagnostic"),
        (r"error\[E0658\]: use of unstable library feature `(.+)`", "unstable_feature"),
        (r"error\[E0433\]: failed to resolve: use of unresolved.+`(.+)`", "unresolved_import"),
        (r"error\[E0255\]: the name `(.+)` is defined multiple times", "name_conflict"),
    ];
    
    for line in stderr.lines() {
        for (pattern, error_type) in &patterns {
            let re = Regex::new(pattern).unwrap();
            if let Some(captures) = re.captures(line) {
                errors.push(CompilationError {
                    error_type: error_type.to_string(),
                    location: extract_location(line),
                    message: line.to_string(),
                    suggestion: None,
                });
            }
        }
    }
    
    errors
}

fn extract_location(line: &str) -> String {
    if let Some(pos) = line.find(" --> ") {
        if let Some(end) = line[pos + 5..].find(':') {
            return line[pos + 5..pos + 5 + end].to_string();
        }
    }
    "unknown".to_string()
}

fn categorize_errors(errors: Vec<CompilationError>) -> HashMap<String, Vec<CompilationError>> {
    let mut categorized = HashMap::new();
    
    for error in errors {
        categorized.entry(error.error_type.clone())
            .or_insert_with(Vec::new)
            .push(error);
    }
    
    categorized
}

fn create_minimal_test(error_type: &str, index: usize, error: &CompilationError) {
    let test_content = match error_type {
        "unstable_feature" => generate_unstable_feature_test(error),
        "duplicate_diagnostic" => generate_duplicate_diagnostic_test(error),
        "unresolved_import" => generate_unresolved_import_test(error),
        "name_conflict" => generate_name_conflict_test(error),
        _ => format!("// Unknown error type: {}\n// {}", error_type, error.message),
    };
    
    std::fs::write(
        format!("error_tests/{}_{}.rs", error_type, index),
        test_content
    ).unwrap_or_else(|_| {
        std::fs::create_dir_all("error_tests").unwrap();
        std::fs::write(
            format!("error_tests/{}_{}.rs", error_type, index),
            test_content
        ).unwrap();
    });
}

fn generate_unstable_feature_test(error: &CompilationError) -> String {
    if error.message.contains("assert_matches") {
        r#"// Test: unstable feature assert_matches
// Fix: Add feature flag or use alternative

#![feature(assert_matches)]
use std::assert_matches::assert_matches;

fn test_fix() {
    // Alternative without unstable feature:
    // match result { Ok(_) => {}, Err(_) => panic!("failed") }
}
"#.to_string()
    } else if error.message.contains("error_reporter") {
        r#"// Test: unstable feature error_reporter  
// Fix: Add feature flag or use alternative

#![feature(error_reporter)]
use std::error::Report;

fn test_fix() {
    // Alternative: use anyhow::Error or Box<dyn std::error::Error>
}
"#.to_string()
    } else {
        format!("// Unstable feature test\n// {}", error.message)
    }
}

fn generate_duplicate_diagnostic_test(error: &CompilationError) -> String {
    r#"// Test: duplicate diagnostic item
// Fix: Remove duplicate #[rustc_diagnostic_item] attributes

// This will fail:
// #[rustc_diagnostic_item = "DiagMessage"]
// pub enum DiagMessage { ... }

// Fix: Only keep one definition or use different names
pub enum DiagMessage {
    Str(String),
}
"#.to_string()
}

fn generate_unresolved_import_test(error: &CompilationError) -> String {
    r#"// Test: unresolved import
// Fix: Add missing extern crate or create stub

// This will fail:
// use missing_crate::SomeType;

// Fix: Add extern crate or create stub
pub struct SomeType;
"#.to_string()
}

fn generate_name_conflict_test(error: &CompilationError) -> String {
    r#"// Test: name conflict
// Fix: Rename conflicting items or use qualified paths

// This will fail:
// use crate1::Item;
// use crate2::Item;

// Fix: Use qualified names
use crate1::Item as Item1;
use crate2::Item as Item2;
"#.to_string()
}

fn generate_auto_fixes(categorized: &HashMap<String, Vec<CompilationError>>) {
    let mut fixes = Vec::new();
    
    for (error_type, errors) in categorized {
        match error_type.as_str() {
            "unstable_feature" => {
                fixes.push("Add missing feature flags to Cargo.toml and lib.rs".to_string());
            },
            "duplicate_diagnostic" => {
                fixes.push("Remove duplicate #[rustc_diagnostic_item] attributes".to_string());
            },
            "unresolved_import" => {
                fixes.push("Add missing extern crate declarations".to_string());
            },
            "name_conflict" => {
                fixes.push("Rename conflicting imports with aliases".to_string());
            },
            _ => {}
        }
    }
    
    println!("\n🔧 Auto-fix plan:");
    for (i, fix) in fixes.iter().enumerate() {
        println!("{}. {}", i + 1, fix);
    }
}
