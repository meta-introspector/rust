use std::collections::{HashMap, HashSet};
use std::fs;
use syn::{parse_file, visit::Visit, ItemFn, ItemConst, Expr, Type, ReturnType, FnArg};
use serde_json::json;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct FunctionSignature {
    input_arity: usize,
    output_type: String,
    complexity_class: ComplexityClass,
    lattice_position: LatticePosition,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum ComplexityClass {
    Constant,           // Returns constant value
    Identity,           // f(x) = x
    Projection,         // f(x,y,z) = x (selects one input)
    Transformation,     // f(x) = g(x) (transforms input)
    Aggregation,        // f(x,y) = x+y (combines inputs)
    Conditional,        // f(x) = if p(x) then a else b
    Recursive,          // f(x) = f(g(x))
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct LatticePosition {
    input_dimension: usize,
    output_cardinality: Option<usize>, // For enums/finite types
    purity_level: PurityLevel,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum PurityLevel {
    Pure,        // No side effects
    IO,          // Has I/O side effects
    Stateful,    // Modifies state
    Unsafe,      // Unsafe operations
}

#[derive(Debug, Clone)]
struct FunctionNode {
    name: String,
    signature: FunctionSignature,
    source_code: String,
    file_path: String,
    similar_functions: Vec<String>,
}

#[derive(Debug, Clone)]
struct ConstantNode {
    name: String,
    value_type: String,
    signature: FunctionSignature,
    source_code: String,
}

struct LatticeAnalyzer {
    functions: Vec<FunctionNode>,
    constants: Vec<ConstantNode>,
    lattice_classes: HashMap<FunctionSignature, Vec<String>>,
    current_file: String,
}

impl<'ast> Visit<'ast> for LatticeAnalyzer {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let function_name = node.sig.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        
        // Analyze function signature
        let input_arity = node.sig.inputs.len();
        let output_type = match &node.sig.output {
            ReturnType::Default => "()".to_string(),
            ReturnType::Type(_, ty) => type_to_string(ty),
        };
        
        // Classify complexity
        let complexity_class = classify_function_complexity(&source_code, &function_name);
        
        // Determine lattice position
        let lattice_position = LatticePosition {
            input_dimension: input_arity,
            output_cardinality: estimate_output_cardinality(&output_type),
            purity_level: analyze_purity(&source_code),
        };
        
        let signature = FunctionSignature {
            input_arity,
            output_type,
            complexity_class,
            lattice_position,
        };
        
        let function_node = FunctionNode {
            name: function_name.clone(),
            signature: signature.clone(),
            source_code,
            file_path: self.current_file.clone(),
            similar_functions: Vec::new(),
        };
        
        // Add to lattice class
        self.lattice_classes.entry(signature).or_default().push(function_name);
        self.functions.push(function_node);
    }
    
    fn visit_item_const(&mut self, node: &'ast ItemConst) {
        let const_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        let value_type = type_to_string(&node.ty);
        
        // Constants are 0-arity functions returning their value
        let signature = FunctionSignature {
            input_arity: 0,
            output_type: value_type.clone(),
            complexity_class: ComplexityClass::Constant,
            lattice_position: LatticePosition {
                input_dimension: 0,
                output_cardinality: Some(1), // Single value
                purity_level: PurityLevel::Pure,
            },
        };
        
        let constant_node = ConstantNode {
            name: const_name.clone(),
            value_type,
            signature: signature.clone(),
            source_code,
        };
        
        self.lattice_classes.entry(signature).or_default().push(const_name);
        self.constants.push(constant_node);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 FUNCTION LATTICE ANALYZER");
    println!("============================");
    
    // Get split-decls output files
    let split_decls_files = find_split_decls_files()?;
    println!("📁 Found {} split-decls files to analyze", split_decls_files.len());
    
    let mut analyzer = LatticeAnalyzer {
        functions: Vec::new(),
        constants: Vec::new(),
        lattice_classes: HashMap::new(),
        current_file: String::new(),
    };
    
    // Process each split-decls file
    for file_path in &split_decls_files {
        analyzer.current_file = file_path.clone();
        
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(syntax_tree) = parse_file(&content) {
                analyzer.visit_file(&syntax_tree);
            }
        }
    }
    
    // Also analyze original source files
    let original_files = vec![
        "./semantic_signature_generator.rs",
        "./basic_block_analyzer.rs", 
        "./io_matrix_analyzer.rs",
        "./split_decls_applicator.rs",
    ];
    
    for file_path in &original_files {
        analyzer.current_file = file_path.to_string();
        let full_path = format!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_driver2/{}", file_path.trim_start_matches("./"));
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            if let Ok(syntax_tree) = parse_file(&content) {
                analyzer.visit_file(&syntax_tree);
            }
        }
    }
    
    // Build similarity relationships
    build_similarity_graph(&mut analyzer);
    
    // Generate lattice analysis
    println!("\n🔗 FUNCTION LATTICE ANALYSIS:");
    println!("==============================");
    println!("Total functions: {}", analyzer.functions.len());
    println!("Total constants: {}", analyzer.constants.len());
    println!("Lattice classes: {}", analyzer.lattice_classes.len());
    
    // Analyze lattice structure
    println!("\n📊 LATTICE CLASS DISTRIBUTION:");
    let mut sorted_classes: Vec<_> = analyzer.lattice_classes.iter().collect();
    sorted_classes.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (i, (signature, functions)) in sorted_classes.iter().take(15).enumerate() {
        println!("{}. {:?} → {} ({} functions)", 
                 i + 1,
                 signature.complexity_class,
                 signature.output_type,
                 functions.len());
        println!("   Input arity: {}, Purity: {:?}", 
                 signature.input_arity, 
                 signature.lattice_position.purity_level);
        println!("   Functions: {}", functions.join(", "));
        println!();
    }
    
    // Analyze by complexity class
    println!("🏗️  COMPLEXITY CLASS ANALYSIS:");
    let mut complexity_counts: HashMap<ComplexityClass, usize> = HashMap::new();
    for function in &analyzer.functions {
        *complexity_counts.entry(function.signature.complexity_class.clone()).or_insert(0) += 1;
    }
    
    let mut sorted_complexity: Vec<_> = complexity_counts.iter().collect();
    sorted_complexity.sort_by(|a, b| b.1.cmp(a.1));
    
    for (complexity, count) in &sorted_complexity {
        println!("   {:?}: {} functions", complexity, count);
    }
    
    // Analyze lattice dimensions
    println!("\n📐 LATTICE DIMENSIONS:");
    let mut dimension_analysis: HashMap<(usize, Option<usize>), usize> = HashMap::new();
    for function in &analyzer.functions {
        let key = (
            function.signature.lattice_position.input_dimension,
            function.signature.lattice_position.output_cardinality,
        );
        *dimension_analysis.entry(key).or_insert(0) += 1;
    }
    
    let mut sorted_dimensions: Vec<_> = dimension_analysis.iter().collect();
    sorted_dimensions.sort_by(|a, b| b.1.cmp(a.1));
    
    for ((input_dim, output_card), count) in sorted_dimensions.iter().take(10) {
        let output_str = match output_card {
            Some(n) => format!("finite({})", n),
            None => "infinite".to_string(),
        };
        println!("   {}D → {}: {} functions", input_dim, output_str, count);
    }
    
    // Generate comprehensive report
    let report = json!({
        "analysis_type": "function_lattice",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "total_functions": analyzer.functions.len(),
            "total_constants": analyzer.constants.len(),
            "lattice_classes": analyzer.lattice_classes.len(),
            "complexity_classes": sorted_complexity.len()
        },
        "lattice_classes": sorted_classes.iter().take(20).map(|(sig, funcs)| json!({
            "signature": {
                "input_arity": sig.input_arity,
                "output_type": sig.output_type,
                "complexity_class": format!("{:?}", sig.complexity_class),
                "input_dimension": sig.lattice_position.input_dimension,
                "output_cardinality": sig.lattice_position.output_cardinality,
                "purity_level": format!("{:?}", sig.lattice_position.purity_level)
            },
            "function_count": funcs.len(),
            "functions": funcs
        })).collect::<Vec<_>>(),
        "complexity_distribution": sorted_complexity.iter().map(|(complexity, count)| json!({
            "complexity_class": format!("{:?}", complexity),
            "function_count": count
        })).collect::<Vec<_>>(),
        "dimension_analysis": sorted_dimensions.iter().take(15).map(|((input_dim, output_card), count)| json!({
            "input_dimension": input_dim,
            "output_cardinality": output_card,
            "function_count": count
        })).collect::<Vec<_>>()
    });
    
    fs::write("function_lattice_analysis.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Function lattice analysis saved to: function_lattice_analysis.json");
    
    Ok(())
}

fn find_split_decls_files() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir("split_decls_output") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Ok(subentries) = fs::read_dir(&path) {
                        for subentry in subentries {
                            if let Ok(subentry) = subentry {
                                let subpath = subentry.path();
                                if subpath.extension().map_or(false, |ext| ext == "rs") {
                                    files.push(subpath.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(files)
}

fn classify_function_complexity(source_code: &str, function_name: &str) -> ComplexityClass {
    if source_code.contains("const ") || function_name.starts_with("get_") && !source_code.contains("(") {
        ComplexityClass::Constant
    } else if source_code.contains("if ") || source_code.contains("match ") {
        ComplexityClass::Conditional
    } else if source_code.contains("for ") || source_code.contains("while ") || source_code.contains("loop ") {
        ComplexityClass::Aggregation
    } else if source_code.contains(&format!("{}(", function_name)) {
        ComplexityClass::Recursive
    } else if source_code.matches("->").count() == 1 && !source_code.contains("{") {
        ComplexityClass::Identity
    } else {
        ComplexityClass::Transformation
    }
}

fn estimate_output_cardinality(output_type: &str) -> Option<usize> {
    match output_type {
        "()" => Some(1),
        "bool" => Some(2),
        "Option" => Some(2), // None or Some
        "Result" => Some(2), // Ok or Err
        _ if output_type.contains("enum") => Some(10), // Estimate for enums
        _ => None, // Infinite or unknown
    }
}

fn analyze_purity(source_code: &str) -> PurityLevel {
    if source_code.contains("unsafe") {
        PurityLevel::Unsafe
    } else if source_code.contains("println!") || source_code.contains("fs::") || source_code.contains("std::io") {
        PurityLevel::IO
    } else if source_code.contains("mut ") || source_code.contains("&mut") {
        PurityLevel::Stateful
    } else {
        PurityLevel::Pure
    }
}

fn build_similarity_graph(analyzer: &mut LatticeAnalyzer) {
    // Build similarity relationships based on lattice position
    for i in 0..analyzer.functions.len() {
        let mut similar = Vec::new();
        let current_sig = &analyzer.functions[i].signature;
        
        for j in 0..analyzer.functions.len() {
            if i != j {
                let other_sig = &analyzer.functions[j].signature;
                
                // Functions are similar if they have same lattice characteristics
                if current_sig.lattice_position.input_dimension == other_sig.lattice_position.input_dimension
                    && current_sig.lattice_position.output_cardinality == other_sig.lattice_position.output_cardinality
                    && current_sig.complexity_class == other_sig.complexity_class {
                    similar.push(analyzer.functions[j].name.clone());
                }
            }
        }
        
        analyzer.functions[i].similar_functions = similar;
    }
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
                "tuple".to_string()
            }
        }
        _ => "Unknown".to_string(),
    }
}
