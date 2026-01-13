use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Expr, Lit};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueUsage {
    file_path: String,
    line_number: Option<usize>,
    context: String,
    usage_type: String,
    complexity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueEntry {
    value: String,
    length: usize,
    total_usages: u32,
    usages: Vec<ValueUsage>,
}

struct ConstantVisitor {
    file_path: String,
    constants: Vec<(String, String, String)>, // (value, context, usage_type)
}

impl ConstantVisitor {
    fn new(file_path: String) -> Self {
        Self { file_path, constants: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ConstantVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Lit(lit_expr) => {
                let (value, usage_type) = match &lit_expr.lit {
                    Lit::Int(int_lit) => (int_lit.base10_digits().to_string(), "integer_literal".to_string()),
                    Lit::Float(float_lit) => (float_lit.base10_digits().to_string(), "float_literal".to_string()),
                    Lit::Str(str_lit) => (str_lit.value(), "string_literal".to_string()),
                    Lit::Bool(bool_lit) => (bool_lit.value.to_string(), "boolean_literal".to_string()),
                    _ => return,
                };
                
                let context = "literal_usage".to_string(); // Simplified context
                self.constants.push((value, context, usage_type));
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }
}

fn main() {
    println!("🔢 VALUE LATTICE DIRECTORY BUILDER");
    println!("=================================");

    let base_dir = "/mnt/data1/meta-introspector/value-lattice";
    fs::create_dir_all(&base_dir).unwrap();

    let mut value_entries: HashMap<String, ValueEntry> = HashMap::new();

    // Load canonical forms and process ALL Rust files on disk
    let canonical_forms_dir = "/mnt/data1/meta-introspector/analysis/canonical-forms";
    
    if let Ok(entries) = fs::read_dir(canonical_forms_dir) {
        let mut all_rust_files = Vec::new();
        
        // Collect all .rs files from canonical forms
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                collect_rust_files(&entry.path(), &mut all_rust_files);
            }
        }
        
        // Also scan the main directories
        let main_dirs = vec![
            "/mnt/data1/nix/vendor/rust",
            "/mnt/data1/meta-introspector",
        ];
        
        for dir in main_dirs {
            collect_rust_files(&std::path::Path::new(dir), &mut all_rust_files);
        }
        
        println!("🦀 Found {} total Rust files to process", all_rust_files.len());
        
        // Process files in parallel batches using all available memory
        let batch_size = 10000; // Larger batches for 30GB RAM
        let total_batches = (all_rust_files.len() + batch_size - 1) / batch_size;
        
        println!("🔥 HIGH-MEMORY MODE: Processing {} files in {} batches", 
                 all_rust_files.len(), total_batches);
        println!("💾 Using up to 30GB RAM for in-memory processing");
        
        for (batch_num, batch) in all_rust_files.chunks(batch_size).enumerate() {
            let progress = ((batch_num + 1) as f64 / total_batches as f64) * 100.0;
            println!("📊 Batch {}/{} ({:.1}% complete) - {} files", 
                     batch_num + 1, total_batches, progress, batch.len());
            
            // Process batch in parallel using rayon if available, otherwise sequential
            for (file_idx, file_path) in batch.iter().enumerate() {
                if file_idx % 1000 == 0 {
                    println!("   File {}/{} in batch {}", file_idx, batch.len(), batch_num + 1);
                }
                
                if let Ok(source) = fs::read_to_string(file_path) {
                    if let Ok(ast) = parse_file(&source) {
                        let mut visitor = ConstantVisitor::new(file_path.clone());
                        visitor.visit_file(&ast);
                        
                        for (value, context, usage_type) in visitor.constants {
                            let complexity = calculate_complexity(&context);
                            
                            let entry = value_entries.entry(value.clone()).or_insert_with(|| {
                                ValueEntry {
                                    value: value.clone(),
                                    length: value.len(),
                                    total_usages: 0,
                                    usages: Vec::new(),
                                }
                            });
                            
                            entry.total_usages += 1;
                            entry.usages.push(ValueUsage {
                                file_path: file_path.clone(),
                                line_number: None,
                                context,
                                usage_type,
                                complexity,
                            });
                        }
                    }
                }
            }
            
            // Memory status update
            if batch_num % 10 == 0 {
                println!("🧠 Memory status: {} unique constants found so far", value_entries.len());
            }
        }
    } else {
        println!("❌ Failed to read canonical forms directory");
        return;
    }

    // Create directory structure: length/value.json
    create_lattice_directories(&base_dir, &value_entries);
    
    println!("\n🏗️  VALUE LATTICE COMPLETE");
    println!("Unique values: {}", value_entries.len());
    println!("Directory structure created at: {}", base_dir);
}

fn calculate_complexity(context: &str) -> u32 {
    let mut complexity = 1;
    
    // Basic complexity heuristics
    if context.contains("match") { complexity += 2; }
    if context.contains("if") { complexity += 1; }
    if context.contains("loop") || context.contains("for") { complexity += 2; }
    if context.contains("async") { complexity += 1; }
    if context.contains("unsafe") { complexity += 3; }
    if context.contains("macro") { complexity += 2; }
    
    // Nesting complexity
    complexity += context.matches('(').count() as u32;
    complexity += context.matches('[').count() as u32;
    complexity += context.matches('{').count() as u32;
    
    complexity
}

fn create_lattice_directories(base_dir: &str, value_entries: &HashMap<String, ValueEntry>) {
    println!("🏗️  Creating lattice directory structure...");
    
    // Group by length
    let mut by_length: HashMap<usize, Vec<&ValueEntry>> = HashMap::new();
    for entry in value_entries.values() {
        by_length.entry(entry.length).or_default().push(entry);
    }
    
    // Create length directories
    for (length, entries) in &by_length {
        let length_dir = format!("{}/length-{}", base_dir, length);
        fs::create_dir_all(&length_dir).unwrap();
        
        // Sort entries by frequency (most used first)
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by(|a, b| b.total_usages.cmp(&a.total_usages));
        
        println!("📁 Length {}: {} unique values", length, sorted_entries.len());
        
        for entry in &sorted_entries {
            create_value_file(&length_dir, entry);
        }
        
        // Create summary for this length
        create_length_summary(&length_dir, entries);
    }
    
    // Create master index
    create_master_index(base_dir, &by_length);
}

fn create_value_file(length_dir: &str, entry: &ValueEntry) {
    let safe_filename = entry.value
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
        .collect::<String>();
    
    let file_path = format!("{}/{}.json", length_dir, safe_filename);
    
    // Create detailed usage report
    let mut report = String::new();
    report.push_str(&format!("# Value: '{}'\n", entry.value));
    report.push_str(&format!("# Length: {}\n", entry.length));
    report.push_str(&format!("# Total Usages: {}\n\n", entry.total_usages));
    
    // Group usages by type
    let mut by_type: HashMap<String, Vec<&ValueUsage>> = HashMap::new();
    for usage in &entry.usages {
        by_type.entry(usage.usage_type.clone()).or_default().push(usage);
    }
    
    for (usage_type, usages) in by_type {
        report.push_str(&format!("## {} ({} usages)\n", usage_type, usages.len()));
        
        // Sort by complexity
        let mut sorted_usages = usages;
        sorted_usages.sort_by(|a, b| b.complexity.cmp(&a.complexity));
        
        for (i, usage) in sorted_usages.iter().take(10).enumerate() {
            report.push_str(&format!("{}. File: {}\n", i + 1, usage.file_path));
            report.push_str(&format!("   Complexity: {}\n", usage.complexity));
            report.push_str(&format!("   Context: {}\n\n", usage.context));
        }
        
        if sorted_usages.len() > 10 {
            report.push_str(&format!("   ... and {} more usages\n\n", sorted_usages.len() - 10));
        }
    }
    
    // Also save as JSON
    let json_data = serde_json::to_string_pretty(entry).unwrap();
    fs::write(&file_path, json_data).unwrap();
    
    // Save readable report
    let report_path = format!("{}/{}.md", length_dir, safe_filename);
    fs::write(report_path, report).unwrap();
}

fn create_length_summary(length_dir: &str, entries: &[&ValueEntry]) {
    let total_usages: u32 = entries.iter().map(|e| e.total_usages).sum();
    let avg_complexity: f64 = entries.iter()
        .flat_map(|e| &e.usages)
        .map(|u| u.complexity as f64)
        .sum::<f64>() / entries.iter().flat_map(|e| &e.usages).count() as f64;
    
    let summary = format!(
        "# Length {} Summary\n\
         Unique values: {}\n\
         Total usages: {}\n\
         Average complexity: {:.2}\n\
         Most frequent: '{}' ({} usages)\n",
        entries[0].length,
        entries.len(),
        total_usages,
        avg_complexity,
        entries[0].value,
        entries[0].total_usages
    );
    
    fs::write(format!("{}/README.md", length_dir), summary).unwrap();
}

fn collect_rust_files(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip common non-source directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !matches!(name, "target" | ".git" | "node_modules" | "build" | "dist") {
                        collect_rust_files(&path, files);
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
}

fn create_master_index(base_dir: &str, by_length: &HashMap<usize, Vec<&ValueEntry>>) {
    let mut index = String::new();
    index.push_str("# Value Lattice Master Index\n\n");
    
    let mut sorted_lengths: Vec<_> = by_length.keys().collect();
    sorted_lengths.sort();
    
    for length in sorted_lengths {
        let entries = &by_length[length];
        let total_usages: u32 = entries.iter().map(|e| e.total_usages).sum();
        
        index.push_str(&format!("## Length {}\n", length));
        index.push_str(&format!("- {} unique values\n", entries.len()));
        index.push_str(&format!("- {} total usages\n", total_usages));
        index.push_str(&format!("- Directory: `length-{}/`\n\n", length));
    }
    
    fs::write(format!("{}/INDEX.md", base_dir), index).unwrap();
    println!("✅ Master index created");
}
