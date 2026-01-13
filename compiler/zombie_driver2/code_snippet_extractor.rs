use std::collections::{HashMap, HashSet};
use std::fs;
use syn::{parse_file, visit::Visit, Type, FnArg, ReturnType, Pat, Item, spanned::Spanned};
use serde_json::json;
use std::hash::{Hash, Hasher, DefaultHasher};

#[derive(Debug, Clone)]
struct CodeSnippet {
    function_code: String,
    surrounding_context: String,
    imports: Vec<String>,
    dependencies: Vec<String>,
    fingerprint: u64,
    file_path: String,
    line_range: (usize, usize),
}

#[derive(Debug, Clone)]
struct UnknownTypeContext {
    unknown_signature: String,
    snippets: Vec<CodeSnippet>,
    common_patterns: Vec<String>,
    inferred_generics: Vec<String>,
    test_cases: Vec<String>,
}

struct CodeExtractor {
    current_file: String,
    file_content: String,
    file_lines: Vec<String>,
    imports: Vec<String>,
    contexts: Vec<UnknownTypeContext>,
}

impl<'ast> Visit<'ast> for CodeExtractor {
    fn visit_file(&mut self, node: &'ast syn::File) {
        // Extract imports first
        for item in &node.items {
            if let Item::Use(use_item) = item {
                self.imports.push(quote::ToTokens::to_token_stream(use_item).to_string());
            }
        }
        syn::visit::visit_file(self, node);
    }
    
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let function_name = node.sig.ident.to_string();
        let function_code = quote::ToTokens::to_token_stream(node).to_string();
        
        // Check for Unknown types in parameters and return
        let mut has_unknown = false;
        let mut unknown_contexts = Vec::new();
        
        // Check parameters
        for input in &node.sig.inputs {
            if let FnArg::Typed(pat_type) = input {
                let type_str = type_to_string(&pat_type.ty);
                if type_str.contains("Unknown") {
                    has_unknown = true;
                    let var_name = extract_pattern_name(&pat_type.pat);
                    unknown_contexts.push(format!("param_{}_{}", var_name, type_str));
                }
            }
        }
        
        // Check return type
        if let ReturnType::Type(_, ty) = &node.sig.output {
            let type_str = type_to_string(ty);
            if type_str.contains("Unknown") {
                has_unknown = true;
                unknown_contexts.push(format!("return_{}", type_str));
            }
        }
        
        if has_unknown {
            // Extract surrounding context (5 lines before and after)
            let start_line = node.span().start().line.saturating_sub(5);
            let end_line = (node.span().end().line + 5).min(self.file_lines.len());
            
            let surrounding_context = self.file_lines[start_line..end_line].join("\n");
            
            // Generate fingerprint
            let fingerprint = self.generate_fingerprint(&function_code, &surrounding_context);
            
            // Extract dependencies from imports and function body
            let dependencies = self.extract_dependencies(&function_code);
            
            let snippet = CodeSnippet {
                function_code: function_code.clone(),
                surrounding_context,
                imports: self.imports.clone(),
                dependencies,
                fingerprint,
                file_path: self.current_file.clone(),
                line_range: (start_line, end_line),
            };
            
            // Create or update context for each unknown type
            for unknown_sig in unknown_contexts {
                if let Some(context) = self.contexts.iter_mut().find(|c| c.unknown_signature == unknown_sig) {
                    context.snippets.push(snippet.clone());
                } else {
                    let mut new_context = UnknownTypeContext {
                        unknown_signature: unknown_sig,
                        snippets: vec![snippet.clone()],
                        common_patterns: Vec::new(),
                        inferred_generics: Vec::new(),
                        test_cases: Vec::new(),
                    };
                    
                    // Analyze for generic patterns
                    new_context.inferred_generics = self.infer_generics(&function_code);
                    new_context.common_patterns = self.extract_patterns(&function_code);
                    
                    self.contexts.push(new_context);
                }
            }
        }
        
        syn::visit::visit_item_fn(self, node);
    }
}

impl CodeExtractor {
    fn generate_fingerprint(&self, function_code: &str, context: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        function_code.hash(&mut hasher);
        context.hash(&mut hasher);
        self.current_file.hash(&mut hasher);
        hasher.finish()
    }
    
    fn extract_dependencies(&self, function_code: &str) -> Vec<String> {
        let mut deps = Vec::new();
        
        // Common crate patterns
        let patterns = [
            "serde", "tokio", "syn", "quote", "goblin", "nalgebra",
            "HashMap", "Vec", "Result", "Option", "String", "PathBuf"
        ];
        
        for pattern in &patterns {
            if function_code.contains(pattern) {
                deps.push(pattern.to_string());
            }
        }
        
        deps.sort();
        deps.dedup();
        deps
    }
    
    fn infer_generics(&self, function_code: &str) -> Vec<String> {
        let mut generics = Vec::new();
        
        // Look for generic patterns
        if function_code.contains("<T>") || function_code.contains("<T,") {
            generics.push("T".to_string());
        }
        if function_code.contains("<K,") || function_code.contains("<K>") {
            generics.push("K".to_string());
        }
        if function_code.contains("<V>") || function_code.contains(",V>") {
            generics.push("V".to_string());
        }
        if function_code.contains("<E>") || function_code.contains(",E>") {
            generics.push("E".to_string());
        }
        
        // Infer from common patterns
        if function_code.contains("Result<") {
            generics.push("Result<T,E>".to_string());
        }
        if function_code.contains("Option<") {
            generics.push("Option<T>".to_string());
        }
        if function_code.contains("Vec<") {
            generics.push("Vec<T>".to_string());
        }
        if function_code.contains("HashMap<") {
            generics.push("HashMap<K,V>".to_string());
        }
        
        generics
    }
    
    fn extract_patterns(&self, function_code: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Common Rust patterns
        if function_code.contains("match ") {
            patterns.push("pattern_matching".to_string());
        }
        if function_code.contains("if let ") {
            patterns.push("if_let_binding".to_string());
        }
        if function_code.contains("for ") && function_code.contains(" in ") {
            patterns.push("iterator_loop".to_string());
        }
        if function_code.contains("map(") || function_code.contains("filter(") {
            patterns.push("functional_programming".to_string());
        }
        if function_code.contains("unwrap()") || function_code.contains("expect(") {
            patterns.push("error_handling".to_string());
        }
        if function_code.contains("clone()") {
            patterns.push("cloning".to_string());
        }
        
        patterns
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 CODE SNIPPET EXTRACTOR FOR UNKNOWN TYPES");
    println!("============================================");
    
    let rust_files = find_rust_files(".")?;
    println!("📁 Processing {} files for Unknown type contexts", rust_files.len());
    
    let mut all_contexts: HashMap<String, UnknownTypeContext> = HashMap::new();
    
    // Process each file
    for file_path in &rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(syntax_tree) = parse_file(&content) {
                let file_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                
                let mut extractor = CodeExtractor {
                    current_file: file_path.clone(),
                    file_content: content,
                    file_lines,
                    imports: Vec::new(),
                    contexts: Vec::new(),
                };
                
                extractor.visit_file(&syntax_tree);
                
                // Merge contexts
                for context in extractor.contexts {
                    if let Some(existing) = all_contexts.get_mut(&context.unknown_signature) {
                        existing.snippets.extend(context.snippets);
                        existing.common_patterns.extend(context.common_patterns);
                        existing.inferred_generics.extend(context.inferred_generics);
                    } else {
                        all_contexts.insert(context.unknown_signature.clone(), context);
                    }
                }
            }
        }
    }
    
    println!("🎯 Found {} unique Unknown type contexts", all_contexts.len());
    
    // Generate test files for each context
    let test_dir = "unknown_type_tests";
    fs::create_dir_all(&test_dir)?;
    
    for (signature, context) in &all_contexts {
        // Create test file for this unknown type
        let test_filename = format!("{}/test_{}.rs", test_dir, 
                                   signature.replace(['<', '>', '&', ' ', ':', '(', ')'], "_"));
        
        let test_content = generate_test_file(signature, context)?;
        fs::write(&test_filename, test_content)?;
        
        println!("📝 Generated test file: {}", test_filename);
    }
    
    // Create comprehensive report
    let report = json!({
        "analysis_type": "unknown_type_code_extraction",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "files_processed": rust_files.len(),
            "unique_unknown_contexts": all_contexts.len(),
            "total_code_snippets": all_contexts.values().map(|c| c.snippets.len()).sum::<usize>(),
            "test_files_generated": all_contexts.len()
        },
        "contexts": all_contexts.iter().map(|(sig, context)| json!({
            "signature": sig,
            "snippet_count": context.snippets.len(),
            "common_patterns": context.common_patterns,
            "inferred_generics": context.inferred_generics,
            "fingerprints": context.snippets.iter().map(|s| s.fingerprint).collect::<Vec<_>>(),
            "files": context.snippets.iter().map(|s| s.file_path.clone()).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>(),
            "sample_code": context.snippets.first().map(|s| s.function_code.chars().take(200).collect::<String>())
        })).collect::<Vec<_>>()
    });
    
    fs::write("unknown_type_code_extraction.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Code extraction report saved to: unknown_type_code_extraction.json");
    println!("📂 Test files generated in: {}/", test_dir);
    
    Ok(())
}

fn generate_test_file(signature: &str, context: &UnknownTypeContext) -> Result<String, Box<dyn std::error::Error>> {
    let mut test_content = String::new();
    
    // Add common imports
    test_content.push_str("// Auto-generated test file for Unknown type: ");
    test_content.push_str(signature);
    test_content.push('\n');
    test_content.push_str("// Fingerprints: ");
    test_content.push_str(&context.snippets.iter().map(|s| s.fingerprint.to_string()).collect::<Vec<_>>().join(", "));
    test_content.push_str("\n\n");
    
    // Add dependencies
    let all_deps: HashSet<String> = context.snippets.iter()
        .flat_map(|s| s.dependencies.iter().cloned())
        .collect();
    
    if !all_deps.is_empty() {
        test_content.push_str("// Dependencies detected: ");
        test_content.push_str(&all_deps.into_iter().collect::<Vec<_>>().join(", "));
        test_content.push_str("\n\n");
    }
    
    // Add common imports
    test_content.push_str("use std::collections::HashMap;\n");
    test_content.push_str("use std::path::PathBuf;\n");
    
    // Add inferred generics as type aliases
    if !context.inferred_generics.is_empty() {
        test_content.push_str("\n// Inferred generic types:\n");
        for generic in &context.inferred_generics {
            if generic.contains("Result") {
                test_content.push_str("type TestResult<T> = Result<T, Box<dyn std::error::Error>>;\n");
            } else if generic.contains("Option") {
                test_content.push_str("type TestOption<T> = Option<T>;\n");
            } else if generic.contains("Vec") {
                test_content.push_str("type TestVec<T> = Vec<T>;\n");
            } else if generic.contains("HashMap") {
                test_content.push_str("type TestMap<K, V> = HashMap<K, V>;\n");
            }
        }
    }
    
    test_content.push_str("\n// Test functions reproducing Unknown type usage:\n\n");
    
    // Add simplified versions of the original functions
    for (i, snippet) in context.snippets.iter().take(3).enumerate() {
        test_content.push_str(&format!("// Test case {} - Fingerprint: {}\n", i + 1, snippet.fingerprint));
        test_content.push_str(&format!("// Source: {}\n", snippet.file_path));
        
        // Simplify the function for testing
        let simplified = simplify_function_for_test(&snippet.function_code, signature);
        test_content.push_str(&simplified);
        test_content.push_str("\n\n");
    }
    
    // Add main test function
    test_content.push_str("#[cfg(test)]\nmod tests {\n    use super::*;\n\n");
    test_content.push_str("    #[test]\n    fn test_unknown_type_usage() {\n");
    test_content.push_str("        // Test the Unknown type patterns\n");
    test_content.push_str("        println!(\"Testing Unknown type: ");
    test_content.push_str(signature);
    test_content.push_str("\");\n");
    test_content.push_str("    }\n}\n");
    
    Ok(test_content)
}

fn simplify_function_for_test(function_code: &str, _signature: &str) -> String {
    // Create a simplified version that compiles
    let mut simplified = function_code.to_string();
    
    // Replace Unknown with concrete types for testing
    simplified = simplified.replace("Unknown", "String");
    simplified = simplified.replace("&String", "&str");
    
    // Add basic implementation if function is empty
    if !simplified.contains('{') || simplified.matches('{').count() == simplified.matches('}').count() {
        if simplified.contains("->") && !simplified.contains("()") {
            simplified = simplified.replace("}", "    todo!(\"Implementation needed\")\n}");
        }
    }
    
    simplified
}

fn find_rust_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            files.push(path.to_string_lossy().to_string());
        } else if path.is_dir() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
            if let Ok(mut subfiles) = find_rust_files(&path.to_string_lossy()) {
                files.append(&mut subfiles);
            }
        }
    }
    
    Ok(files)
}

fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            type_path.path.segments.iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::")
        }
        Type::Reference(type_ref) => {
            format!("&{}", type_to_string(&type_ref.elem))
        }
        Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                "()".to_string()
            } else {
                let elem_types: Vec<String> = type_tuple.elems.iter()
                    .map(type_to_string)
                    .collect();
                format!("({})", elem_types.join(", "))
            }
        }
        _ => "Unknown".to_string(),
    }
}

fn extract_pattern_name(pat: &Pat) -> String {
    match pat {
        Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
        _ => "unnamed".to_string(),
    }
}
