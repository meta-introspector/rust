use std::collections::HashMap;
use std::fs;
use std::process::Command;
use serde_json::Value;

/// DWIM Compiler Plugin: Do What I Mean Error Fixer
/// Uses global usage patterns to automatically fix compilation errors

#[derive(Debug, Clone)]
struct ErrorPattern {
    error_code: String,
    error_message: String,
    suggested_fix: String,
    confidence: f64,
    line_number: u32,
}

#[derive(Debug, Clone)]
struct UsagePattern {
    api_name: String,
    correct_usage: String,
    frequency: u32,
    context: String,
}

#[derive(Debug)]
struct DwimErrorFixer {
    usage_patterns: HashMap<String, Vec<UsagePattern>>,
    error_fixes: HashMap<String, String>,
    fixes_applied: Vec<String>,
}

impl DwimErrorFixer {
    fn new() -> Self {
        let mut fixer = Self {
            usage_patterns: HashMap::new(),
            error_fixes: HashMap::new(),
            fixes_applied: Vec::new(),
        };
        
        fixer.load_global_usage_patterns();
        fixer.initialize_common_fixes();
        fixer
    }
    
    fn load_global_usage_patterns(&mut self) {
        println!("📚 Loading global usage patterns...");
        
        let usage_dir = "../../usage_data";
        let mut patterns_loaded = 0;
        
        if let Ok(entries) = fs::read_dir(usage_dir) {
            for entry in entries.flatten().take(20) { // Sample for performance
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        patterns_loaded += self.extract_usage_patterns(&entry.path().display().to_string());
                    }
                }
            }
        }
        
        println!("  Loaded {} usage patterns", patterns_loaded);
    }
    
    fn extract_usage_patterns(&mut self, filepath: &str) -> u32 {
        let mut patterns_found = 0;
        
        if let Ok(content) = fs::read_to_string(filepath) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = json["usages"].as_array() {
                    for usage in usages.iter().take(5) {
                        if let Some(usage_str) = usage["usage"].as_str() {
                            if let Some(pattern) = self.parse_usage_pattern(usage_str) {
                                let api_name = pattern.api_name.clone();
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
        if usage_str.contains("::") {
            if let Some(start) = usage_str.rfind("::") {
                if let Some(end) = usage_str[start+2..].find(" ") {
                    let method = &usage_str[start+2..start+2+end];
                    if method.len() > 2 {
                        return Some(UsagePattern {
                            api_name: method.to_string(),
                            correct_usage: format!("{}()", method),
                            frequency: 1,
                            context: "method_call".to_string(),
                        });
                    }
                }
            }
        }
        None
    }
    
    fn initialize_common_fixes(&mut self) {
        // Common Rust error patterns and their fixes
        self.error_fixes.insert(
            "no method named `len` found".to_string(),
            "Replace `len` with `len()`".to_string()
        );
        
        self.error_fixes.insert(
            "expected `String`, found `&str`".to_string(),
            "Add `.to_string()` or `.to_owned()`".to_string()
        );
        
        self.error_fixes.insert(
            "expected `&str`, found `String`".to_string(),
            "Add `.as_str()` or use `&`".to_string()
        );
        
        self.error_fixes.insert(
            "cannot find function `println`".to_string(),
            "Use `println!` macro instead".to_string()
        );
        
        self.error_fixes.insert(
            "doesn't implement `Debug`".to_string(),
            "Add `#[derive(Debug)]`".to_string()
        );
        
        self.error_fixes.insert(
            "cannot borrow as mutable".to_string(),
            "Use `&mut` or avoid simultaneous borrows".to_string()
        );
    }
    
    fn compile_and_extract_errors(&self, file_path: &str) -> Vec<ErrorPattern> {
        println!("🔍 Compiling {} to extract errors...", file_path);
        
        let output = Command::new("rustc")
            .arg(file_path)
            .arg("--error-format=json")
            .arg("--crate-type=bin")
            .output()
            .expect("Failed to run rustc");
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut errors = Vec::new();
        
        // Parse JSON error output
        for line in stderr.lines() {
            if let Ok(json) = serde_json::from_str::<Value>(line) {
                if json["level"] == "error" {
                    if let Some(message) = json["message"].as_str() {
                        let error = ErrorPattern {
                            error_code: json["code"]["code"].as_str().unwrap_or("E0000").to_string(),
                            error_message: message.to_string(),
                            suggested_fix: self.suggest_fix(message),
                            confidence: self.calculate_confidence(message),
                            line_number: json["spans"][0]["line_start"].as_u64().unwrap_or(0) as u32,
                        };
                        errors.push(error);
                    }
                }
            }
        }
        
        println!("  Found {} compilation errors", errors.len());
        errors
    }
    
    fn suggest_fix(&self, error_message: &str) -> String {
        // Check our common fixes first
        for (pattern, fix) in &self.error_fixes {
            if error_message.contains(pattern) {
                return fix.clone();
            }
        }
        
        // Check usage patterns for method suggestions
        if error_message.contains("no method named") {
            if let Some(start) = error_message.find("`") {
                if let Some(end) = error_message[start+1..].find("`") {
                    let method_name = &error_message[start+1..start+1+end];
                    
                    // Look for similar methods in usage patterns
                    for (api, patterns) in &self.usage_patterns {
                        if self.is_similar_method(method_name, api) {
                            if let Some(pattern) = patterns.first() {
                                return format!("Try: {}", pattern.correct_usage);
                            }
                        }
                    }
                    
                    // Common corrections
                    match method_name {
                        "len" => return "Add parentheses: `len()`".to_string(),
                        "size" => return "Use `len()` instead".to_string(),
                        "count" => return "Use `len()` for collections".to_string(),
                        _ => {}
                    }
                }
            }
        }
        
        "Manual fix required".to_string()
    }
    
    fn calculate_confidence(&self, error_message: &str) -> f64 {
        // Simple confidence calculation based on pattern matching
        for pattern in self.error_fixes.keys() {
            if error_message.contains(pattern) {
                return 0.9; // High confidence for known patterns
            }
        }
        
        if error_message.contains("no method named") {
            return 0.7; // Medium confidence for method errors
        }
        
        0.3 // Low confidence for unknown patterns
    }
    
    fn is_similar_method(&self, method1: &str, method2: &str) -> bool {
        if method1 == method2 {
            return true;
        }
        
        // Simple similarity check
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
    
    fn generate_fixed_code(&mut self, file_path: &str, errors: &[ErrorPattern]) -> String {
        let original_code = fs::read_to_string(file_path).unwrap_or_default();
        let mut fixed_code = original_code.clone();
        
        for error in errors {
            if error.confidence > 0.7 {
                let fix_applied = self.apply_fix(&mut fixed_code, error);
                if fix_applied {
                    self.fixes_applied.push(format!("Line {}: {}", error.line_number, error.suggested_fix));
                }
            }
        }
        
        fixed_code
    }
    
    fn apply_fix(&self, code: &mut String, error: &ErrorPattern) -> bool {
        // Apply specific fixes based on error patterns
        if error.suggested_fix.contains("Add parentheses") {
            *code = code.replace(".len", ".len()");
            return true;
        }
        
        if error.suggested_fix.contains("Use `println!` macro") {
            *code = code.replace("println(", "println!(");
            return true;
        }
        
        if error.suggested_fix.contains("Add `.to_string()`") {
            // This would need more sophisticated parsing to apply correctly
            return false;
        }
        
        if error.suggested_fix.contains("Add `#[derive(Debug)]`") {
            // Find struct definition and add derive
            if let Some(struct_pos) = code.find("struct ") {
                let before_struct = &code[..struct_pos];
                let after_struct = &code[struct_pos..];
                *code = format!("{}#[derive(Debug)]\n{}", before_struct, after_struct);
                return true;
            }
        }
        
        false
    }
    
    fn save_dwim_report(&self, errors: &[ErrorPattern]) {
        let mut report = String::new();
        report.push_str("# DWIM Error Fixer Analysis Report\n\n");
        
        report.push_str("## Errors Analyzed\n");
        for (i, error) in errors.iter().enumerate() {
            report.push_str(&format!("{}. **{}** (Line {}, Confidence: {:.1}%)\n", 
                                   i + 1, error.error_code, error.line_number, error.confidence * 100.0));
            report.push_str(&format!("   - Message: {}\n", error.error_message));
            report.push_str(&format!("   - Suggested Fix: {}\n\n", error.suggested_fix));
        }
        
        report.push_str("## Fixes Applied\n");
        for (i, fix) in self.fixes_applied.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, fix));
        }
        
        report.push_str(&format!("\n## Statistics\n"));
        report.push_str(&format!("- Total errors: {}\n", errors.len()));
        report.push_str(&format!("- High confidence fixes: {}\n", 
                                errors.iter().filter(|e| e.confidence > 0.7).count()));
        report.push_str(&format!("- Usage patterns loaded: {}\n", self.usage_patterns.len()));
        
        std::fs::write("dwim_analysis_report.md", report).unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        eprintln!("Example: {} test_broken_code.rs", args[0]);
        std::process::exit(1);
    }
    
    println!("🧠 DWIM Error Fixer: Do What I Mean Compiler Plugin");
    println!("==================================================");
    
    let mut fixer = DwimErrorFixer::new();
    let file_path = &args[1];
    
    // Extract errors from compilation
    let errors = fixer.compile_and_extract_errors(file_path);
    
    if errors.is_empty() {
        println!("✅ No compilation errors found!");
        return;
    }
    
    println!("\n📊 ERROR ANALYSIS:");
    println!("==================");
    
    for (i, error) in errors.iter().enumerate() {
        println!("{}. {} (Line {}) - Confidence: {:.1}%", 
                i + 1, error.error_code, error.line_number, error.confidence * 100.0);
        println!("   Message: {}", error.error_message);
        println!("   Fix: {}", error.suggested_fix);
        println!();
    }
    
    // Generate fixed code
    let fixed_code = fixer.generate_fixed_code(file_path, &errors);
    
    // Save fixed version
    let fixed_filename = format!("{}.fixed", file_path);
    fs::write(&fixed_filename, &fixed_code).unwrap();
    
    println!("🔧 FIXES APPLIED:");
    println!("=================");
    for fix in &fixer.fixes_applied {
        println!("✅ {}", fix);
    }
    
    if fixer.fixes_applied.is_empty() {
        println!("⚠️  No automatic fixes could be applied");
        println!("💡 Manual intervention required for complex errors");
    }
    
    // Save analysis report
    fixer.save_dwim_report(&errors);
    
    println!("\n📁 Files generated:");
    println!("  - {}: Fixed code", fixed_filename);
    println!("  - dwim_analysis_report.md: Detailed analysis");
    
    println!("\n🎉 DWIM analysis complete!");
    println!("💡 Try compiling the fixed version: rustc {}", fixed_filename);
}
