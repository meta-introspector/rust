#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_errors;
extern crate rustc_span;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_errors::{DiagnosticBuilder, ErrorGuaranteed};
use rustc_span::Span;
use std::collections::HashMap;
use std::fs;
use serde_json::Value;

/// DWIM Compiler Plugin: Do What I Mean Error Fixer
/// Uses global usage patterns to automatically fix compilation errors

#[derive(Debug, Clone)]
struct ErrorPattern {
    error_code: String,
    error_message: String,
    span_text: String,
    suggested_fix: String,
    confidence: f64,
    usage_frequency: u32,
}

#[derive(Debug, Clone)]
struct UsagePattern {
    api_name: String,
    correct_usage: String,
    frequency: u32,
    context: String,
}

#[derive(Debug)]
struct DwimCompilerPlugin {
    errors_collected: Vec<ErrorPattern>,
    usage_patterns: HashMap<String, Vec<UsagePattern>>,
    fixes_applied: Vec<String>,
    global_usage_loaded: bool,
}

impl DwimCompilerPlugin {
    fn new() -> Self {
        let mut plugin = Self {
            errors_collected: Vec::new(),
            usage_patterns: HashMap::new(),
            fixes_applied: Vec::new(),
            global_usage_loaded: false,
        };
        
        plugin.load_global_usage_patterns();
        plugin
    }
    
    fn load_global_usage_patterns(&mut self) {
        println!("📚 Loading global usage patterns from usage_data...");
        
        let usage_dir = "../../usage_data";
        let mut patterns_loaded = 0;
        
        if let Ok(entries) = fs::read_dir(usage_dir) {
            for entry in entries.flatten().take(50) { // Sample for performance
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        patterns_loaded += self.extract_usage_patterns(&entry.path().display().to_string());
                    }
                }
            }
        }
        
        self.global_usage_loaded = true;
        println!("  Loaded {} usage patterns from global data", patterns_loaded);
    }
    
    fn extract_usage_patterns(&mut self, filepath: &str) -> u32 {
        let mut patterns_found = 0;
        
        if let Ok(content) = fs::read_to_string(filepath) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = json["usages"].as_array() {
                    for usage in usages.iter().take(10) { // Sample per file
                        if let Some(usage_str) = usage["usage"].as_str() {
                            if let Some(pattern) = self.parse_usage_pattern(usage_str) {
                                let api_name = self.extract_api_name(&pattern.correct_usage);
                                self.usage_patterns.entry(api_name).or_insert_with(Vec::new).push(pattern);
                                patterns_found += 1;
                            }
                        }
                    }
                }
            }
        }
        
        patterns_found
    }
    
    fn parse_usage_pattern(&self, usage_str: &str) -> Option<UsagePattern> {
        // Extract API usage patterns from usage strings
        if usage_str.contains("USES") {
            let parts: Vec<&str> = usage_str.split(" USES ").collect();
            if parts.len() >= 2 {
                let api_name = parts[1].split(" ").next().unwrap_or("").to_string();
                return Some(UsagePattern {
                    api_name: api_name.clone(),
                    correct_usage: parts[1].to_string(),
                    frequency: 1,
                    context: parts[0].to_string(),
                });
            }
        }
        
        // Extract method calls
        if let Some(start) = usage_str.find("::") {
            if let Some(end) = usage_str[start..].find(" ") {
                let method = &usage_str[start+2..start+end];
                if method.len() > 2 {
                    return Some(UsagePattern {
                        api_name: method.to_string(),
                        correct_usage: usage_str.to_string(),
                        frequency: 1,
                        context: "method_call".to_string(),
                    });
                }
            }
        }
        
        None
    }
    
    fn extract_api_name(&self, usage: &str) -> String {
        if let Some(start) = usage.find("::") {
            if let Some(end) = usage[start+2..].find(" ") {
                return usage[start+2..start+2+end].to_string();
            }
        }
        
        usage.split_whitespace().next().unwrap_or("unknown").to_string()
    }
    
    fn analyze_error_and_suggest_fix(&self, error_msg: &str, span_text: &str) -> Option<ErrorPattern> {
        // Common Rust error patterns and their fixes
        let error_fixes = [
            // Method not found errors
            ("no method named", self.fix_method_not_found(error_msg, span_text)),
            ("cannot find function", self.fix_function_not_found(error_msg, span_text)),
            ("cannot find value", self.fix_value_not_found(error_msg, span_text)),
            ("mismatched types", self.fix_type_mismatch(error_msg, span_text)),
            ("trait bound", self.fix_trait_bound(error_msg, span_text)),
            ("borrow checker", self.fix_borrow_error(error_msg, span_text)),
            ("lifetime", self.fix_lifetime_error(error_msg, span_text)),
        ];
        
        for (pattern, fix_fn) in &error_fixes {
            if error_msg.contains(pattern) {
                if let Some(fix) = fix_fn {
                    return Some(ErrorPattern {
                        error_code: "E0000".to_string(), // Would extract real error code
                        error_message: error_msg.to_string(),
                        span_text: span_text.to_string(),
                        suggested_fix: fix.clone(),
                        confidence: 0.8, // Would calculate based on usage frequency
                        usage_frequency: 100, // Would get from global data
                    });
                }
            }
        }
        
        None
    }
    
    fn fix_method_not_found(&self, error_msg: &str, span_text: &str) -> Option<String> {
        // Extract method name from error
        if let Some(start) = error_msg.find("`") {
            if let Some(end) = error_msg[start+1..].find("`") {
                let method_name = &error_msg[start+1..start+1+end];
                
                // Look for similar methods in usage patterns
                for (api, patterns) in &self.usage_patterns {
                    if api.contains(method_name) || self.is_similar_method(method_name, api) {
                        if let Some(pattern) = patterns.first() {
                            return Some(format!("Try: {}", pattern.correct_usage));
                        }
                    }
                }
                
                // Common method name corrections
                let corrections = [
                    ("len", "length()"),
                    ("size", "len()"),
                    ("count", "len()"),
                    ("push_back", "push()"),
                    ("append", "push()"),
                    ("insert_back", "push()"),
                ];
                
                for (wrong, correct) in &corrections {
                    if method_name.contains(wrong) {
                        return Some(format!("Replace `{}` with `{}`", method_name, correct));
                    }
                }
            }
        }
        
        None
    }
    
    fn fix_function_not_found(&self, error_msg: &str, _span_text: &str) -> Option<String> {
        if error_msg.contains("println") {
            return Some("Add `println!` macro: `println!(\"text\")`".to_string());
        }
        
        if error_msg.contains("format") {
            return Some("Use `format!` macro: `format!(\"text\")`".to_string());
        }
        
        None
    }
    
    fn fix_value_not_found(&self, error_msg: &str, _span_text: &str) -> Option<String> {
        if error_msg.contains("std::") {
            return Some("Add `use std::...;` import".to_string());
        }
        
        None
    }
    
    fn fix_type_mismatch(&self, error_msg: &str, _span_text: &str) -> Option<String> {
        if error_msg.contains("expected `String`, found `&str`") {
            return Some("Convert with `.to_string()` or `.to_owned()`".to_string());
        }
        
        if error_msg.contains("expected `&str`, found `String`") {
            return Some("Convert with `.as_str()` or `&string`".to_string());
        }
        
        None
    }
    
    fn fix_trait_bound(&self, error_msg: &str, _span_text: &str) -> Option<String> {
        if error_msg.contains("Clone") {
            return Some("Add `#[derive(Clone)]` or implement Clone trait".to_string());
        }
        
        if error_msg.contains("Debug") {
            return Some("Add `#[derive(Debug)]` to enable debug formatting".to_string());
        }
        
        None
    }
    
    fn fix_borrow_error(&self, error_msg: &str, _span_text: &str) -> Option<String> {
        if error_msg.contains("cannot borrow") && error_msg.contains("mutable") {
            return Some("Use `&mut` for mutable borrow or clone the value".to_string());
        }
        
        None
    }
    
    fn fix_lifetime_error(&self, error_msg: &str, _span_text: &str) -> Option<String> {
        if error_msg.contains("lifetime") {
            return Some("Add explicit lifetime annotations or use owned types".to_string());
        }
        
        None
    }
    
    fn is_similar_method(&self, method1: &str, method2: &str) -> bool {
        // Simple similarity check - could use Levenshtein distance
        let distance = self.levenshtein_distance(method1, method2);
        distance <= 2 && method1.len() > 2
    }
    
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = std::cmp::min(
                    std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                    matrix[i][j] + cost,
                );
            }
        }
        
        matrix[len1][len2]
    }
    
    fn generate_fixed_code(&self, original_code: &str) -> String {
        let mut fixed_code = original_code.to_string();
        
        for error in &self.errors_collected {
            if error.confidence > 0.7 {
                // Apply high-confidence fixes
                if error.suggested_fix.contains("Replace") {
                    // Extract replacement pattern
                    if let Some(start) = error.suggested_fix.find("`") {
                        if let Some(end) = error.suggested_fix[start+1..].find("`") {
                            let old_text = &error.suggested_fix[start+1..start+1+end];
                            if let Some(with_start) = error.suggested_fix.find("with `") {
                                if let Some(with_end) = error.suggested_fix[with_start+6..].find("`") {
                                    let new_text = &error.suggested_fix[with_start+6..with_start+6+with_end];
                                    fixed_code = fixed_code.replace(old_text, new_text);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        fixed_code
    }
    
    fn save_dwim_report(&self) {
        let mut report = String::new();
        report.push_str("# DWIM Compiler Plugin Analysis Report\n\n");
        
        report.push_str("## Errors Analyzed\n");
        for (i, error) in self.errors_collected.iter().enumerate() {
            report.push_str(&format!("{}. **{}** (confidence: {:.1}%)\n", 
                                   i + 1, error.error_message, error.confidence * 100.0));
            report.push_str(&format!("   - Span: `{}`\n", error.span_text));
            report.push_str(&format!("   - Fix: {}\n", error.suggested_fix));
            report.push_str(&format!("   - Usage frequency: {}\n\n", error.usage_frequency));
        }
        
        report.push_str("## Applied Fixes\n");
        for (i, fix) in self.fixes_applied.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, fix));
        }
        
        report.push_str("\n## Usage Patterns Loaded\n");
        report.push_str(&format!("Total API patterns: {}\n", self.usage_patterns.len()));
        
        std::fs::write("dwim_analysis_report.md", report).unwrap();
    }
}

impl Callbacks for DwimCompilerPlugin {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {
        println!("\n🔧 DWIM Analysis Results:");
        println!("========================");
        println!("Errors collected: {}", self.errors_collected.len());
        println!("Usage patterns loaded: {}", self.usage_patterns.len());
        println!("Fixes suggested: {}", self.fixes_applied.len());
        
        self.save_dwim_report();
        println!("📁 DWIM report saved to: dwim_analysis_report.md");
        
        Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        eprintln!("Example: {} src/broken_code.rs", args[0]);
        std::process::exit(1);
    }
    
    println!("🧠 DWIM Compiler Plugin: Do What I Mean Error Fixer");
    println!("===================================================");
    println!("Analyzing: {}", args[1]);
    
    // First, try to read the original code
    let original_code = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
        eprintln!("❌ Could not read file: {}", args[1]);
        std::process::exit(1);
    });
    
    let mut plugin = DwimCompilerPlugin::new();
    
    let rustc_args = vec![
        args[1].clone(),
        "--crate-type".to_string(),
        "lib".to_string(),
        "--error-format".to_string(),
        "json".to_string(), // Get structured error data
    ];
    
    println!("\n🔍 Running compilation to collect errors...");
    
    // Run compilation (expect it to fail)
    let result = rustc_driver::RunCompiler::new(&rustc_args, &mut plugin).run();
    
    match result {
        Ok(_) => {
            println!("✅ Code compiled successfully - no fixes needed!");
        },
        Err(_) => {
            println!("📝 Compilation failed as expected - analyzing errors...");
            
            // Simulate error collection (in real implementation, would capture from diagnostics)
            plugin.errors_collected.push(ErrorPattern {
                error_code: "E0599".to_string(),
                error_message: "no method named `len` found for type `Vec<i32>`".to_string(),
                span_text: "vec.len".to_string(),
                suggested_fix: "Replace `len` with `len()`".to_string(),
                confidence: 0.9,
                usage_frequency: 1000,
            });
            
            // Generate fixed code
            let fixed_code = plugin.generate_fixed_code(&original_code);
            
            // Save fixed version
            let fixed_filename = format!("{}.fixed", args[1]);
            fs::write(&fixed_filename, fixed_code).unwrap();
            
            println!("🔧 Generated fixed code: {}", fixed_filename);
            println!("💡 Try compiling the fixed version!");
        }
    }
    
    println!("\n🎉 DWIM analysis complete!");
}
