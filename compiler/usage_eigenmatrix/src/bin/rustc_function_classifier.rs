use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔍 RUSTC FUNCTION CLASSIFICATION & TYPE ANALYSIS");
    println!("═══════════════════════════════════════════════");
    
    let graph = load_rustc_graph()?;
    let mut classifier = FunctionClassifier::new();
    
    let analysis = classifier.analyze_all_functions(&graph)?;
    
    // Print classification results
    print_classification_summary(&analysis);
    
    // Save detailed analysis
    save_analysis(&analysis)?;
    
    Ok(())
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    name: String,
    function_type: FunctionType,
    domain: Vec<String>,  // Input types/parameters
    range: String,        // Output type
    in_degree: usize,     // How many functions call this
    out_degree: usize,    // How many functions this calls
    is_entry_point: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FunctionType {
    EntryPoint,      // main, run_compiler, etc.
    Parser,          // parse_*, tokenize_*, etc.
    Analyzer,        // analyze_*, check_*, validate_*
    Transformer,     // lower_*, convert_*, transform_*
    Generator,       // codegen_*, emit_*, generate_*
    Utility,         // helper functions
    DataStructure,   // constructors, accessors
    ErrorHandler,    // error_*, panic_*, etc.
    Unknown,
}

struct FunctionClassifier {
    classifications: HashMap<String, FunctionInfo>,
}

impl FunctionClassifier {
    fn new() -> Self {
        Self {
            classifications: HashMap::new(),
        }
    }
    
    fn analyze_all_functions(&mut self, graph: &HashMap<String, Vec<String>>) -> Result<HashMap<String, FunctionInfo>> {
        println!("🎯 Analyzing {} functions...", graph.len());
        
        // Calculate degrees for each function
        let mut in_degrees = HashMap::new();
        let mut out_degrees = HashMap::new();
        
        for (caller, callees) in graph {
            out_degrees.insert(caller.clone(), callees.len());
            for callee in callees {
                *in_degrees.entry(callee.clone()).or_insert(0) += 1;
            }
        }
        
        // Classify each function
        for (func_name, callees) in graph {
            let function_info = FunctionInfo {
                name: func_name.clone(),
                function_type: self.classify_function(func_name),
                domain: self.infer_domain(func_name, callees),
                range: self.infer_range(func_name),
                in_degree: *in_degrees.get(func_name).unwrap_or(&0),
                out_degree: *out_degrees.get(func_name).unwrap_or(&0),
                is_entry_point: self.is_entry_point(func_name, *in_degrees.get(func_name).unwrap_or(&0)),
            };
            
            self.classifications.insert(func_name.clone(), function_info);
        }
        
        Ok(self.classifications.clone())
    }
    
    fn classify_function(&self, name: &str) -> FunctionType {
        let name_lower = name.to_lowercase();
        
        if name_lower.contains("main") || name_lower.contains("run_compiler") || name_lower.contains("driver") {
            FunctionType::EntryPoint
        } else if name_lower.contains("parse") || name_lower.contains("lex") || name_lower.contains("token") {
            FunctionType::Parser
        } else if name_lower.contains("analyze") || name_lower.contains("check") || name_lower.contains("validate") || name_lower.contains("resolve") {
            FunctionType::Analyzer
        } else if name_lower.contains("lower") || name_lower.contains("transform") || name_lower.contains("convert") {
            FunctionType::Transformer
        } else if name_lower.contains("codegen") || name_lower.contains("emit") || name_lower.contains("generate") {
            FunctionType::Generator
        } else if name_lower.contains("error") || name_lower.contains("panic") || name_lower.contains("abort") {
            FunctionType::ErrorHandler
        } else if name_lower.contains("new") || name_lower.contains("create") || name_lower.contains("build") {
            FunctionType::DataStructure
        } else {
            FunctionType::Utility
        }
    }
    
    fn infer_domain(&self, name: &str, callees: &[String]) -> Vec<String> {
        // Infer input types from function name and callees
        let mut domain = Vec::new();
        
        if name.contains("DefId") {
            domain.push("DefId".to_string());
        }
        if name.contains("Span") {
            domain.push("Span".to_string());
        }
        if name.contains("Ty") || name.contains("Type") {
            domain.push("Type".to_string());
        }
        if name.contains("Expr") {
            domain.push("Expression".to_string());
        }
        if name.contains("Item") {
            domain.push("Item".to_string());
        }
        
        // Infer from callees
        for callee in callees.iter().take(3) {
            if callee.contains("fmt") {
                domain.push("Formatter".to_string());
            }
            if callee.contains("iter") {
                domain.push("Iterator".to_string());
            }
        }
        
        if domain.is_empty() {
            domain.push("Unknown".to_string());
        }
        
        domain
    }
    
    fn infer_range(&self, name: &str) -> String {
        if name.contains("bool") || name.contains("is_") || name.contains("has_") {
            "bool".to_string()
        } else if name.contains("Option") {
            "Option<T>".to_string()
        } else if name.contains("Result") {
            "Result<T, E>".to_string()
        } else if name.contains("Vec") {
            "Vec<T>".to_string()
        } else if name.contains("String") {
            "String".to_string()
        } else {
            "Unknown".to_string()
        }
    }
    
    fn is_entry_point(&self, name: &str, in_degree: usize) -> bool {
        // Entry points: low in-degree + specific names
        (in_degree == 0 || in_degree <= 2) && 
        (name.contains("main") || name.contains("run") || name.contains("start") || name.contains("init"))
    }
}

fn print_classification_summary(analysis: &HashMap<String, FunctionInfo>) {
    let mut type_counts = HashMap::new();
    let mut entry_points = Vec::new();
    
    for func_info in analysis.values() {
        *type_counts.entry(func_info.function_type.clone()).or_insert(0) += 1;
        
        if func_info.is_entry_point {
            entry_points.push(&func_info.name);
        }
    }
    
    println!("\n📊 FUNCTION TYPE DISTRIBUTION:");
    for (func_type, count) in &type_counts {
        println!("  {:?}: {} functions", func_type, count);
    }
    
    println!("\n🚪 ENTRY POINTS ({} total):", entry_points.len());
    for (i, entry) in entry_points.iter().take(10).enumerate() {
        println!("  {}. {}", i+1, entry);
    }
    if entry_points.len() > 10 {
        println!("  ... and {} more", entry_points.len() - 10);
    }
    
    // Find highest degree functions
    let mut by_in_degree: Vec<_> = analysis.values().collect();
    by_in_degree.sort_by(|a, b| b.in_degree.cmp(&a.in_degree));
    
    println!("\n📈 MOST CALLED FUNCTIONS (highest in-degree):");
    for (i, func) in by_in_degree.iter().take(5).enumerate() {
        println!("  {}. {} - called by {} functions", i+1, func.name, func.in_degree);
    }
}

fn save_analysis(analysis: &HashMap<String, FunctionInfo>) -> Result<()> {
    let summary = serde_json::json!({
        "total_functions": analysis.len(),
        "entry_points": analysis.values().filter(|f| f.is_entry_point).count(),
        "type_distribution": {
            "entry_point": analysis.values().filter(|f| f.function_type == FunctionType::EntryPoint).count(),
            "parser": analysis.values().filter(|f| f.function_type == FunctionType::Parser).count(),
            "analyzer": analysis.values().filter(|f| f.function_type == FunctionType::Analyzer).count(),
            "transformer": analysis.values().filter(|f| f.function_type == FunctionType::Transformer).count(),
            "generator": analysis.values().filter(|f| f.function_type == FunctionType::Generator).count(),
            "utility": analysis.values().filter(|f| f.function_type == FunctionType::Utility).count(),
            "data_structure": analysis.values().filter(|f| f.function_type == FunctionType::DataStructure).count(),
            "error_handler": analysis.values().filter(|f| f.function_type == FunctionType::ErrorHandler).count(),
        }
    });
    
    fs::write("rustc_function_analysis.json", serde_json::to_string_pretty(&summary)?)?;
    println!("\n💾 Analysis saved to: rustc_function_analysis.json");
    
    Ok(())
}

fn load_rustc_graph() -> Result<HashMap<String, Vec<String>>> {
    let mut graph = HashMap::new();
    let usage_dir = "../../usage_data";
    let mut file_count = 0;
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let user_id = usage_obj["user_def_id"].as_str().unwrap_or("unknown");
                    let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                    
                    graph.entry(user_id.to_string())
                        .or_insert_with(Vec::new)
                        .push(used_id.to_string());
                }
            }
            
            file_count += 1;
            if file_count >= 200 { // Larger sample for better classification
                break;
            }
        }
    }
    
    Ok(graph)
}
