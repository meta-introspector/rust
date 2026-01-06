use std::collections::HashMap;
use std::fs;
use std::process::Command;
use serde_json::Value;

/// Enhanced DWIM Error Corrector using Introspector-Collector Usage Patterns
/// Leverages type-prefixed literals and AST pattern analysis for intelligent fixes

#[derive(Debug, Clone)]
struct ErrorPattern {
    error_code: String,
    error_message: String,
    suggested_fix: String,
    confidence: f64,
    line_number: u32,
    usage_frequency: u32,
}

#[derive(Debug, Clone)]
struct TypedUsagePattern {
    pattern_type: String,  // field_access, function_call, memory_ref, etc.
    api_signature: String,
    correct_usage: String,
    type_prefix: String,   // From our type prefix system
    frequency: u32,
    context: String,
}

#[derive(Debug)]
struct EnhancedDwimCorrector {
    typed_patterns: HashMap<String, Vec<TypedUsagePattern>>,
    error_fixes: HashMap<String, String>,
    fixes_applied: Vec<String>,
    correlation_data: HashMap<String, f64>,
}

impl EnhancedDwimCorrector {
    fn new() -> Self {
        let mut corrector = Self {
            typed_patterns: HashMap::new(),
            error_fixes: HashMap::new(),
            fixes_applied: Vec::new(),
            correlation_data: HashMap::new(),
        };
        
        corrector.load_introspector_patterns();
        corrector.initialize_enhanced_fixes();
        corrector
    }
    
    fn load_introspector_patterns(&mut self) {
        println!("🔬 Loading enhanced usage patterns from introspector-collector...");
        
        // Load from our enhanced collector data
        let data_sources = [
            "../introspector-collector/test_usage_data",
            "../../usage_data",
            "./static.txt",
            "./runtime.txt"
        ];
        
        let mut patterns_loaded = 0;
        
        for source in &data_sources {
            patterns_loaded += self.extract_typed_patterns(source);
        }
        
        println!("  Loaded {} typed usage patterns", patterns_loaded);
    }
    
    fn extract_typed_patterns(&mut self, source_path: &str) -> u32 {
        let mut patterns_found = 0;
        
        // Handle different source types
        if source_path.ends_with(".txt") {
            patterns_found += self.parse_text_patterns(source_path);
        } else if let Ok(entries) = fs::read_dir(source_path) {
            for entry in entries.flatten().take(30) {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        patterns_found += self.parse_json_patterns(&entry.path().display().to_string());
                    }
                }
            }
        }
        
        patterns_found
    }
    
    fn parse_text_patterns(&mut self, file_path: &str) -> u32 {
        let mut patterns_found = 0;
        
        if let Ok(content) = fs::read_to_string(file_path) {
            for line in content.lines().take(100) {
                if let Some(pattern) = self.parse_enhanced_usage_line(line) {
                    let key = format!("{}_{}", pattern.pattern_type, pattern.api_signature);
                    self.typed_patterns.entry(key).or_insert_with(Vec::new).push(pattern);
                    patterns_found += 1;
                }
            }
        }
        
        patterns_found
    }
    
    fn parse_json_patterns(&mut self, file_path: &str) -> u32 {
        let mut patterns_found = 0;
        
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = json["usages"].as_array() {
                    for usage in usages.iter().take(10) {
                        if let Some(usage_str) = usage["usage"].as_str() {
                            if let Some(pattern) = self.parse_enhanced_usage_line(usage_str) {
                                let key = format!("{}_{}", pattern.pattern_type, pattern.api_signature);
                                self.typed_patterns.entry(key).or_insert_with(Vec::new).push(pattern);
                                patterns_found += 1;
                            }
                        }
                    }
                }
            }
        }
        
        patterns_found
    }
    
    fn parse_enhanced_usage_line(&self, line: &str) -> Option<TypedUsagePattern> {
        // Parse our enhanced usage patterns with type prefixes
        
        // Field access patterns: field_access, field_name
        if line.contains("field_access") {
            return Some(TypedUsagePattern {
                pattern_type: "field_access".to_string(),
                api_signature: self.extract_field_name(line),
                correct_usage: format!("obj.{}", self.extract_field_name(line)),
                type_prefix: self.extract_type_prefix(line),
                frequency: 1,
                context: "field_access".to_string(),
            });
        }
        
        // Function call patterns: call_N_args
        if line.contains("call_") && line.contains("_args") {
            let arg_count = self.extract_arg_count(line);
            return Some(TypedUsagePattern {
                pattern_type: "function_call".to_string(),
                api_signature: format!("call_{}_args", arg_count),
                correct_usage: format!("func({})", "arg, ".repeat(arg_count).trim_end_matches(", ")),
                type_prefix: "fn_".to_string(),
                frequency: 1,
                context: "function_call".to_string(),
            });
        }
        
        // Memory reference patterns: addr_ref, addr_mut_ref
        if line.contains("addr_") {
            let ref_type = if line.contains("mut_ref") { "mut" } else { "immut" };
            return Some(TypedUsagePattern {
                pattern_type: "memory_ref".to_string(),
                api_signature: format!("addr_{}_ref", ref_type),
                correct_usage: format!("&{} value", if ref_type == "mut" { "mut" } else { "" }),
                type_prefix: "ref_".to_string(),
                frequency: 1,
                context: "memory_safety".to_string(),
            });
        }
        
        // Pattern matching: match_N_arms
        if line.contains("match_") && line.contains("_arms") {
            let arm_count = self.extract_arm_count(line);
            return Some(TypedUsagePattern {
                pattern_type: "pattern_match".to_string(),
                api_signature: format!("match_{}_arms", arm_count),
                correct_usage: "match expr { pattern => result, }".to_string(),
                type_prefix: "match_".to_string(),
                frequency: 1,
                context: "patterns".to_string(),
            });
        }
        
        // Type-prefixed literals
        if let Some(prefix) = self.extract_literal_prefix(line) {
            return Some(TypedUsagePattern {
                pattern_type: "literal".to_string(),
                api_signature: prefix.clone(),
                correct_usage: self.generate_literal_usage(&prefix),
                type_prefix: prefix,
                frequency: 1,
                context: "literals".to_string(),
            });
        }
        
        None
    }
    
    fn extract_field_name(&self, line: &str) -> String {
        if let Some(start) = line.find("field_") {
            if let Some(end) = line[start..].find(" ") {
                return line[start+6..start+end].to_string();
            }
        }
        "field_name".to_string()
    }
    
    fn extract_arg_count(&self, line: &str) -> usize {
        if let Some(start) = line.find("call_") {
            if let Some(end) = line[start+5..].find("_args") {
                if let Ok(count) = line[start+5..start+5+end].parse::<usize>() {
                    return count;
                }
            }
        }
        0
    }
    
    fn extract_arm_count(&self, line: &str) -> usize {
        if let Some(start) = line.find("match_") {
            if let Some(end) = line[start+6..].find("_arms") {
                if let Ok(count) = line[start+6..start+6+end].parse::<usize>() {
                    return count;
                }
            }
        }
        2
    }
    
    fn extract_type_prefix(&self, line: &str) -> String {
        let prefixes = ["str_", "int_", "u8_", "i32_", "f64_", "b", "c"];
        for prefix in &prefixes {
            if line.contains(prefix) {
                return prefix.to_string();
            }
        }
        "unknown_".to_string()
    }
    
    fn extract_literal_prefix(&self, line: &str) -> Option<String> {
        let prefixes = ["str_", "int_", "u8_", "i32_", "f64_", "byte_", "bstr_"];
        for prefix in &prefixes {
            if line.contains(prefix) {
                return Some(prefix.to_string());
            }
        }
        None
    }
    
    fn generate_literal_usage(&self, prefix: &str) -> String {
        match prefix {
            "str_" => "\"string\"".to_string(),
            "int_" => "42".to_string(),
            "u8_" => "255u8".to_string(),
            "i32_" => "42i32".to_string(),
            "f64_" => "3.14f64".to_string(),
            "b" => "true".to_string(),
            "c" => "'a'".to_string(),
            _ => "value".to_string(),
        }
    }
    
    fn initialize_enhanced_fixes(&mut self) {
        // Enhanced fixes using our pattern analysis
        self.error_fixes.insert(
            "no method named `len` found".to_string(),
            "Use `.len()` method call pattern".to_string()
        );
        
        self.error_fixes.insert(
            "expected `String`, found `&str`".to_string(),
            "Apply str_ to String conversion: `.to_string()`".to_string()
        );
        
        self.error_fixes.insert(
            "field `{}` of struct `{}` is private".to_string(),
            "Use field access pattern with getter method".to_string()
        );
        
        self.error_fixes.insert(
            "cannot borrow `{}` as mutable".to_string(),
            "Apply memory_ref pattern: use `&mut`".to_string()
        );
        
        self.error_fixes.insert(
            "pattern doesn't bind `{}`".to_string(),
            "Use pattern_match template with proper binding".to_string()
        );
    }
    
    fn suggest_enhanced_fix(&self, error_message: &str) -> String {
        // Enhanced fix suggestions using typed patterns
        
        // Method not found - use function call patterns
        if error_message.contains("no method named") {
            if let Some(method) = self.extract_method_name(error_message) {
                // Look for similar patterns in our typed data
                for (key, patterns) in &self.typed_patterns {
                    if key.contains("function_call") {
                        for pattern in patterns {
                            if self.is_similar_method(&method, &pattern.api_signature) {
                                return format!("Try function call pattern: {}", pattern.correct_usage);
                            }
                        }
                    }
                }
                
                // Common method corrections with type awareness
                match method.as_str() {
                    "len" => return "Use function_call pattern: `.len()`".to_string(),
                    "size" => return "Use function_call pattern: `.len()` (size → len)".to_string(),
                    "count" => return "Use function_call pattern: `.len()` for collections".to_string(),
                    _ => {}
                }
            }
        }
        
        // Field access errors
        if error_message.contains("field") && error_message.contains("private") {
            return "Use field_access pattern with public getter method".to_string();
        }
        
        // Type mismatch - use literal patterns
        if error_message.contains("expected") && error_message.contains("found") {
            if error_message.contains("String") && error_message.contains("&str") {
                return "Apply str_ literal pattern: use `.to_string()` conversion".to_string();
            }
            if error_message.contains("&str") && error_message.contains("String") {
                return "Apply str_ literal pattern: use `.as_str()` or `&string`".to_string();
            }
        }
        
        // Borrow checker - use memory reference patterns
        if error_message.contains("borrow") {
            if error_message.contains("mutable") {
                return "Apply memory_ref pattern: use `&mut` for mutable references".to_string();
            }
            return "Apply memory_ref pattern: check reference lifetimes".to_string();
        }
        
        // Pattern matching errors
        if error_message.contains("pattern") {
            return "Use pattern_match template with proper arm structure".to_string();
        }
        
        "Apply usage pattern analysis for manual fix".to_string()
    }
    
    fn extract_method_name(&self, error_message: &str) -> Option<String> {
        if let Some(start) = error_message.find("`") {
            if let Some(end) = error_message[start+1..].find("`") {
                return Some(error_message[start+1..start+1+end].to_string());
            }
        }
        None
    }
    
    fn is_similar_method(&self, method1: &str, method2: &str) -> bool {
        method1 == method2 || self.levenshtein_distance(method1, method2) <= 2
    }
    
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        for i in 0..=len1 { matrix[i][0] = i; }
        for j in 0..=len2 { matrix[0][j] = j; }
        
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
    
    fn compile_and_extract_errors(&self, file_path: &str) -> Vec<ErrorPattern> {
        println!("🔍 Compiling {} with enhanced error analysis...", file_path);
        
        let output = Command::new("rustc")
            .arg(file_path)
            .arg("--error-format=json")
            .arg("--crate-type=bin")
            .output()
            .expect("Failed to run rustc");
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut errors = Vec::new();
        
        for line in stderr.lines() {
            if let Ok(json) = serde_json::from_str::<Value>(line) {
                if json["level"] == "error" {
                    if let Some(message) = json["message"].as_str() {
                        let error = ErrorPattern {
                            error_code: json["code"]["code"].as_str().unwrap_or("E0000").to_string(),
                            error_message: message.to_string(),
                            suggested_fix: self.suggest_enhanced_fix(message),
                            confidence: self.calculate_enhanced_confidence(message),
                            line_number: json["spans"][0]["line_start"].as_u64().unwrap_or(0) as u32,
                            usage_frequency: self.get_pattern_frequency(message),
                        };
                        errors.push(error);
                    }
                }
            }
        }
        
        println!("  Found {} compilation errors with pattern analysis", errors.len());
        errors
    }
    
    fn calculate_enhanced_confidence(&self, error_message: &str) -> f64 {
        // Enhanced confidence based on pattern frequency and type correlation
        let mut confidence: f64 = 0.3; // Base confidence
        
        // Boost confidence for patterns we have data on
        if error_message.contains("no method named") {
            confidence += 0.4; // We have good function call patterns
        }
        
        if error_message.contains("field") {
            confidence += 0.3; // We have field access patterns
        }
        
        if error_message.contains("expected") && error_message.contains("String") {
            confidence += 0.5; // We have strong type conversion patterns
        }
        
        if error_message.contains("borrow") {
            confidence += 0.3; // We have memory reference patterns
        }
        
        confidence.min(0.95) // Cap at 95%
    }
    
    fn get_pattern_frequency(&self, error_message: &str) -> u32 {
        // Get frequency from our usage patterns
        for (key, patterns) in &self.typed_patterns {
            if error_message.contains("method") && key.contains("function_call") {
                return patterns.iter().map(|p| p.frequency).sum();
            }
            if error_message.contains("field") && key.contains("field_access") {
                return patterns.iter().map(|p| p.frequency).sum();
            }
        }
        1
    }
    
    fn generate_enhanced_fixed_code(&mut self, file_path: &str, errors: &[ErrorPattern]) -> String {
        let original_code = fs::read_to_string(file_path).unwrap_or_default();
        let mut fixed_code = original_code.clone();
        
        for error in errors {
            if error.confidence > 0.6 { // Lower threshold due to enhanced patterns
                let fix_applied = self.apply_enhanced_fix(&mut fixed_code, error);
                if fix_applied {
                    self.fixes_applied.push(format!(
                        "Line {}: {} (confidence: {:.1}%, frequency: {})", 
                        error.line_number, error.suggested_fix, 
                        error.confidence * 100.0, error.usage_frequency
                    ));
                }
            }
        }
        
        fixed_code
    }
    
    fn apply_enhanced_fix(&self, code: &mut String, error: &ErrorPattern) -> bool {
        // Enhanced fix application using pattern templates
        
        if error.suggested_fix.contains("function_call pattern") {
            if error.error_message.contains("len") {
                *code = code.replace(".len", ".len()");
                return true;
            }
        }
        
        if error.suggested_fix.contains("str_ literal pattern") {
            if error.suggested_fix.contains("to_string") {
                // Would need more sophisticated AST-level replacement
                return false;
            }
        }
        
        if error.suggested_fix.contains("memory_ref pattern") {
            // Would need context-aware mutable reference insertion
            return false;
        }
        
        // Apply simple text replacements for now
        if error.suggested_fix.contains("Use `println!` macro") {
            *code = code.replace("println(", "println!(");
            return true;
        }
        
        false
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        eprintln!("Example: {} broken_code.rs", args[0]);
        std::process::exit(1);
    }
    
    println!("🧠 Enhanced DWIM Error Corrector");
    println!("================================");
    println!("Using introspector-collector usage patterns for intelligent fixes");
    
    let mut corrector = EnhancedDwimCorrector::new();
    let file_path = &args[1];
    
    let errors = corrector.compile_and_extract_errors(file_path);
    
    if errors.is_empty() {
        println!("✅ No compilation errors found!");
        return;
    }
    
    println!("\n📊 ENHANCED ERROR ANALYSIS:");
    println!("===========================");
    
    for (i, error) in errors.iter().enumerate() {
        println!("{}. {} (Line {}) - Confidence: {:.1}% - Frequency: {}", 
                i + 1, error.error_code, error.line_number, 
                error.confidence * 100.0, error.usage_frequency);
        println!("   Message: {}", error.error_message);
        println!("   Enhanced Fix: {}", error.suggested_fix);
        println!();
    }
    
    let fixed_code = corrector.generate_enhanced_fixed_code(file_path, &errors);
    let fixed_filename = format!("{}.enhanced_fixed", file_path);
    fs::write(&fixed_filename, &fixed_code).unwrap();
    
    println!("🔧 ENHANCED FIXES APPLIED:");
    println!("==========================");
    for fix in &corrector.fixes_applied {
        println!("✅ {}", fix);
    }
    
    if corrector.fixes_applied.is_empty() {
        println!("⚠️  No automatic fixes applied - complex patterns need manual intervention");
    }
    
    println!("\n📁 Enhanced fixed code saved to: {}", fixed_filename);
    println!("🎯 Pattern-based fixes using {} typed usage patterns", corrector.typed_patterns.len());
    println!("🎉 Enhanced DWIM analysis complete!");
}
