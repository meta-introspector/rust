use std::collections::HashMap;
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, Type, TypePath, Path as SynPath};

fn main() {
    println!("🧬 Ty Usage Pattern Extractor - Analyzing rustc type system usage");
    
    // Extract ty usage patterns from actual rustc ASTs
    let ty_patterns = extract_ty_patterns_from_asts();
    
    // Analyze frequencies and contexts
    let frequency_analysis = analyze_frequencies(&ty_patterns);
    
    // Generate macro templates based on patterns
    generate_ty_macros(&frequency_analysis);
    
    println!("✅ Generated ty-macros from {} real usage patterns", ty_patterns.len());
}

fn extract_ty_patterns_from_asts() -> Vec<TyUsagePattern> {
    let mut pattern_counts: HashMap<String, u64> = HashMap::new();
    let mut pattern_contexts: HashMap<String, Vec<String>> = HashMap::new();
    
    // Scan processed rustc files
    let rustc_dir = "submodules/rust";
    if let Ok(entries) = fs::read_dir(rustc_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                scan_directory_for_ty_patterns(&entry.path(), &mut pattern_counts, &mut pattern_contexts);
            }
        }
    }
    
    // Convert to TyUsagePattern structs
    pattern_counts.into_iter().map(|(pattern, frequency)| {
        TyUsagePattern {
            pattern: pattern.clone(),
            frequency,
            depth: calculate_depth(&pattern),
            contexts: pattern_contexts.get(&pattern).cloned().unwrap_or_default(),
        }
    }).collect()
}

fn scan_directory_for_ty_patterns(
    dir: &Path, 
    pattern_counts: &mut HashMap<String, u64>,
    pattern_contexts: &mut HashMap<String, Vec<String>>
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory_for_ty_patterns(&path, pattern_counts, pattern_contexts);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                extract_ty_from_file(&path, pattern_counts, pattern_contexts);
            }
        }
    }
}

fn extract_ty_from_file(
    file_path: &Path,
    pattern_counts: &mut HashMap<String, u64>,
    pattern_contexts: &mut HashMap<String, Vec<String>>
) {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(ast) = parse_file(&content) {
            let mut visitor = TyVisitor::new(file_path.to_string_lossy().to_string());
            visitor.visit_file(&ast);
            
            // Merge results
            for (pattern, count) in visitor.patterns {
                *pattern_counts.entry(pattern.clone()).or_insert(0) += count;
                pattern_contexts.entry(pattern.clone())
                    .or_insert_with(Vec::new)
                    .extend(visitor.contexts.get(&pattern).cloned().unwrap_or_default());
            }
        }
    }
}

struct TyVisitor {
    patterns: HashMap<String, u64>,
    contexts: HashMap<String, Vec<String>>,
    current_file: String,
}

impl TyVisitor {
    fn new(file_path: String) -> Self {
        Self {
            patterns: HashMap::new(),
            contexts: HashMap::new(),
            current_file: file_path,
        }
    }
    
    fn record_pattern(&mut self, pattern: String, context: String) {
        *self.patterns.entry(pattern.clone()).or_insert(0) += 1;
        self.contexts.entry(pattern)
            .or_insert_with(Vec::new)
            .push(format!("{}:{}", self.current_file, context));
    }
}

impl<'ast> Visit<'ast> for TyVisitor {
    fn visit_type(&mut self, ty: &'ast Type) {
        match ty {
            Type::Path(type_path) => {
                let path_str = path_to_string(&type_path.path);
                
                // Extract ty:: patterns
                if path_str.starts_with("ty::") {
                    self.record_pattern(path_str.clone(), "type_path".to_string());
                }
                
                // Extract TyCtxt patterns
                if path_str.contains("TyCtxt") {
                    self.record_pattern("TyCtxt".to_string(), "type_path".to_string());
                }
                
                // Extract Ty<'tcx> patterns
                if path_str.starts_with("Ty<") {
                    self.record_pattern("Ty<'tcx>".to_string(), "type_path".to_string());
                }
            }
            _ => {}
        }
        
        syn::visit::visit_type(self, ty);
    }
}

fn path_to_string(path: &SynPath) -> String {
    path.segments.iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn analyze_frequencies(patterns: &[TyUsagePattern]) -> FrequencyAnalysis {
    let mut analysis = FrequencyAnalysis {
        high_freq: Vec::new(),
        medium_freq: Vec::new(),
        low_freq: Vec::new(),
    };
    
    for pattern in patterns {
        match pattern.frequency {
            f if f > 100 => analysis.high_freq.push(pattern.clone()),
            f if f > 10 => analysis.medium_freq.push(pattern.clone()),
            _ => analysis.low_freq.push(pattern.clone()),
        }
    }
    
    analysis
}

fn generate_ty_macros(analysis: &FrequencyAnalysis) {
    let mut macro_code = String::new();
    
    // Generate high-frequency direct macros
    for pattern in &analysis.high_freq {
        macro_code.push_str(&format!(
            "macro_rules! ty_{} {{\n    () => {{ /* High freq: {} ({} uses) */ }};\n}}\n\n",
            pattern.pattern.replace("::", "_").replace("<", "_").replace(">", "_").replace("'", "_"),
            pattern.pattern,
            pattern.frequency
        ));
    }
    
    // Generate medium-frequency parameterized macros
    for pattern in &analysis.medium_freq {
        macro_code.push_str(&format!(
            "macro_rules! ty_{}_param {{\n    ($param:ty) => {{ /* Medium freq: {} ({} uses) */ }};\n}}\n\n",
            pattern.pattern.replace("::", "_").replace("<", "_").replace(">", "_").replace("'", "_"),
            pattern.pattern,
            pattern.frequency
        ));
    }
    
    // Generate low-frequency generic macros
    macro_code.push_str("macro_rules! ty_generic {\n    ($($tokens:tt)*) => { /* Generic fallback */ };\n}\n");
    
    fs::write("src/generated_ty_macros.rs", macro_code)
        .expect("Failed to write ty macros");
    
    println!("✅ Generated ty macros in src/generated_ty_macros.rs");
}

fn calculate_depth(pattern: &str) -> u8 {
    pattern.matches("::").count() as u8 + 1
}

#[derive(Debug, Clone)]
struct TyUsagePattern {
    pattern: String,
    frequency: u64,
    depth: u8,
    contexts: Vec<String>,
}

#[derive(Debug)]
struct FrequencyAnalysis {
    high_freq: Vec<TyUsagePattern>,
    medium_freq: Vec<TyUsagePattern>,
    low_freq: Vec<TyUsagePattern>,
}
