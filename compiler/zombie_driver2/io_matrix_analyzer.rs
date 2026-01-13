use std::collections::{HashMap, HashSet};
use std::fs;
use syn::{parse_file, visit::Visit, Type, FnArg, ReturnType};
use serde_json::json;

#[derive(Debug, Clone)]
struct IORelation {
    function_name: String,
    input_types: Vec<String>,
    output_types: Vec<String>,
    file_path: String,
    char_frequency: HashMap<char, usize>,
    basic_block_count: usize,
    ast_complexity: usize,
}

#[derive(Debug, Default)]
struct MatrixCell {
    relations: Vec<IORelation>,
    total_functions: usize,
    avg_complexity: f64,
    char_patterns: HashMap<char, f64>,
    unique_files: HashSet<String>,
}

struct IOVisitor {
    relations: Vec<IORelation>,
    current_file: String,
}

impl<'ast> Visit<'ast> for IOVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let function_name = node.sig.ident.to_string();
        
        // Extract input types
        let mut input_types = Vec::new();
        for input in &node.sig.inputs {
            match input {
                FnArg::Typed(pat_type) => {
                    input_types.push(type_to_string(&pat_type.ty));
                }
                FnArg::Receiver(_) => {
                    input_types.push("self".to_string());
                }
            }
        }
        
        // Extract output types
        let mut output_types = Vec::new();
        match &node.sig.output {
            ReturnType::Default => output_types.push("()".to_string()),
            ReturnType::Type(_, ty) => output_types.push(type_to_string(ty)),
        }
        
        // Calculate basic metrics
        let char_freq = calculate_char_frequency(&function_name);
        let ast_complexity = calculate_ast_complexity(node);
        
        self.relations.push(IORelation {
            function_name,
            input_types,
            output_types,
            file_path: self.current_file.clone(),
            char_frequency: char_freq,
            basic_block_count: estimate_basic_blocks(node),
            ast_complexity,
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 IO MATRIX ANALYZER");
    println!("=====================");
    
    // Collect all Rust files in zombie driver
    let rust_files = find_rust_files(".")?;
    println!("📁 Found {} Rust files", rust_files.len());
    
    let mut all_relations = Vec::new();
    let mut type_registry: HashSet<String> = HashSet::new();
    
    // Process each file
    for file_path in &rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(syntax_tree) = parse_file(&content) {
                let mut visitor = IOVisitor {
                    relations: Vec::new(),
                    current_file: file_path.clone(),
                };
                
                visitor.visit_file(&syntax_tree);
                
                // Register all types
                for relation in &visitor.relations {
                    for input_type in &relation.input_types {
                        type_registry.insert(input_type.clone());
                    }
                    for output_type in &relation.output_types {
                        type_registry.insert(output_type.clone());
                    }
                }
                
                all_relations.extend(visitor.relations);
            }
        }
    }
    
    println!("🔍 Processed {} functions across {} files", all_relations.len(), rust_files.len());
    println!("📊 Found {} unique types", type_registry.len());
    
    // Create IO matrix
    let types: Vec<String> = type_registry.into_iter().collect();
    let mut matrix: HashMap<(String, String), MatrixCell> = HashMap::new();
    
    // Populate matrix
    for relation in &all_relations {
        for input_type in &relation.input_types {
            for output_type in &relation.output_types {
                let key = (input_type.clone(), output_type.clone());
                let cell = matrix.entry(key).or_default();
                
                cell.relations.push(relation.clone());
                cell.total_functions += 1;
                cell.unique_files.insert(relation.file_path.clone());
                
                // Aggregate character patterns
                for (ch, count) in &relation.char_frequency {
                    *cell.char_patterns.entry(*ch).or_insert(0.0) += *count as f64;
                }
            }
        }
    }
    
    // Calculate averages
    for cell in matrix.values_mut() {
        if !cell.relations.is_empty() {
            cell.avg_complexity = cell.relations.iter()
                .map(|r| r.ast_complexity as f64)
                .sum::<f64>() / cell.relations.len() as f64;
            
            // Normalize character patterns
            let total_chars: f64 = cell.char_patterns.values().sum();
            if total_chars > 0.0 {
                for freq in cell.char_patterns.values_mut() {
                    *freq /= total_chars;
                }
            }
        }
    }
    
    // Generate matrix report
    println!("\n🎯 IO MATRIX ANALYSIS:");
    println!("======================");
    
    // Show top type pairs
    let mut sorted_pairs: Vec<_> = matrix.iter().collect();
    sorted_pairs.sort_by(|a, b| b.1.total_functions.cmp(&a.1.total_functions));
    
    println!("\n🏆 TOP 10 TYPE RELATIONSHIPS:");
    for (i, ((input, output), cell)) in sorted_pairs.iter().take(10).enumerate() {
        println!("{}. {} → {} ({} functions, {:.1} avg complexity)", 
                 i + 1, input, output, cell.total_functions, cell.avg_complexity);
        println!("   Files: {}", cell.unique_files.len());
        
        // Show top character patterns
        let mut char_pairs: Vec<_> = cell.char_patterns.iter().collect();
        char_pairs.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        let top_chars: String = char_pairs.iter().take(3)
            .map(|(ch, freq)| format!("{}({:.1}%)", ch, *freq * 100.0))
            .collect::<Vec<_>>()
            .join(", ");
        println!("   Char patterns: {}", top_chars);
        println!();
    }
    
    // Diagonal analysis (same input/output type)
    println!("🔄 DIAGONAL ANALYSIS (Same Input/Output Types):");
    let mut diagonal_pairs: Vec<_> = matrix.iter()
        .filter(|((input, output), _)| input == output)
        .collect();
    diagonal_pairs.sort_by(|a, b| b.1.total_functions.cmp(&a.1.total_functions));
    
    for (i, ((type_name, _), cell)) in diagonal_pairs.iter().take(5).enumerate() {
        println!("{}. {} → {} ({} functions)", 
                 i + 1, type_name, type_name, cell.total_functions);
    }
    
    // Create comprehensive JSON report
    let matrix_data: HashMap<String, serde_json::Value> = matrix.iter()
        .map(|((input, output), cell)| {
            let key = format!("{}→{}", input, output);
            let value = json!({
                "input_type": input,
                "output_type": output,
                "total_functions": cell.total_functions,
                "avg_complexity": cell.avg_complexity,
                "unique_files": cell.unique_files.len(),
                "files": cell.unique_files.iter().collect::<Vec<_>>(),
                "char_patterns": cell.char_patterns.iter()
                    .map(|(ch, freq)| json!({
                        "char": ch.to_string(),
                        "frequency": freq
                    }))
                    .collect::<Vec<_>>(),
                "sample_functions": cell.relations.iter().take(3)
                    .map(|r| json!({
                        "name": r.function_name,
                        "file": r.file_path,
                        "complexity": r.ast_complexity,
                        "basic_blocks": r.basic_block_count
                    }))
                    .collect::<Vec<_>>()
            });
            (key, value)
        })
        .collect();
    
    let report = json!({
        "analysis_type": "io_matrix_analysis",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "total_files": rust_files.len(),
            "total_functions": all_relations.len(),
            "unique_types": types.len(),
            "type_relationships": matrix.len()
        },
        "types": types,
        "matrix": matrix_data,
        "diagonal_analysis": diagonal_pairs.iter().take(10).map(|((type_name, _), cell)| json!({
            "type": type_name,
            "self_transformations": cell.total_functions,
            "avg_complexity": cell.avg_complexity,
            "files": cell.unique_files.len()
        })).collect::<Vec<_>>()
    });
    
    fs::write("io_matrix_report.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ IO Matrix analysis saved to: io_matrix_report.json");
    
    Ok(())
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

fn calculate_char_frequency(text: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}

fn calculate_ast_complexity(node: &syn::ItemFn) -> usize {
    // Simple complexity metric: count statements + expressions
    let mut complexity = 1; // Base complexity
    
    if let Some(block) = &node.block.stmts.get(0) {
        complexity += count_statements_recursive(block);
    }
    
    complexity
}

fn count_statements_recursive(_stmt: &syn::Stmt) -> usize {
    // Simplified - just return 1 for each statement
    1
}

fn estimate_basic_blocks(node: &syn::ItemFn) -> usize {
    // Estimate based on control flow statements
    node.block.stmts.len().max(1)
}
