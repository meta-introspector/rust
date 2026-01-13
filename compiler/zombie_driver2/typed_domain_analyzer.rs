use std::collections::{HashMap, HashSet};
use std::fs;
use syn::{parse_file, visit::Visit, Item, ItemFn, ItemStruct, ItemEnum, ItemConst, ItemType, Type, Expr};
use serde_json::json;

#[derive(Debug, Clone)]
struct TypedDomainConcept {
    name: String,
    concept_type: ConceptType,
    frequency: usize,
    related_items: Vec<String>,
    used_terms: HashSet<String>,
    semantic_category: SemanticCategory,
    abstraction_level: AbstractionLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ConceptType {
    Function,
    Struct,
    Enum,
    Constant,
    TypeAlias,
    Variable,
    Parameter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SemanticCategory {
    Action,        // analyze, generate, extract
    Entity,        // signature, block, matrix
    Property,      // semantic, basic, unknown
    System,        // rust, cargo, split
    Data,          // json, file, string
    Meta,          // meta, introspector
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AbstractionLevel {
    Concrete,      // file, string, json
    Abstract,      // signature, analysis
    Meta,          // introspector, lattice
    System,        // rust, cargo, compiler
}

#[derive(Debug, Clone)]
struct EnhancedFunction {
    name: String,
    used_types: HashSet<String>,
    used_terms: HashSet<String>,
    parameters: Vec<String>,
    return_type: String,
    complexity_score: usize,
}

struct TypedDomainAnalyzer {
    concepts: HashMap<String, TypedDomainConcept>,
    functions: Vec<EnhancedFunction>,
    type_usage_graph: HashMap<String, HashSet<String>>,
    current_file: String,
}

impl<'ast> Visit<'ast> for TypedDomainAnalyzer {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let function_name = node.sig.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        
        // Extract used types and terms
        let used_types = extract_types_from_function(node);
        let used_terms = extract_terms_from_code(&source_code);
        let parameters = extract_parameter_names(node);
        let return_type = extract_return_type(node);
        
        let enhanced_function = EnhancedFunction {
            name: function_name.clone(),
            used_types: used_types.clone(),
            used_terms: used_terms.clone(),
            parameters,
            return_type,
            complexity_score: calculate_complexity(&source_code),
        };
        
        self.functions.push(enhanced_function);
        
        // Create concept for function
        let concept = TypedDomainConcept {
            name: function_name.clone(),
            concept_type: ConceptType::Function,
            frequency: 1,
            related_items: used_types.iter().cloned().collect(),
            used_terms: used_terms.clone(),
            semantic_category: classify_semantic_category(&function_name),
            abstraction_level: classify_abstraction_level(&function_name),
        };
        
        self.concepts.insert(function_name.clone(), concept);
        
        // Update type usage graph
        for used_type in &used_types {
            self.type_usage_graph.entry(used_type.clone())
                .or_default()
                .insert(function_name.clone());
        }
    }
    
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let struct_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        let used_terms = extract_terms_from_code(&source_code);
        let field_types = extract_struct_field_types(node);
        
        let concept = TypedDomainConcept {
            name: struct_name.clone(),
            concept_type: ConceptType::Struct,
            frequency: 1,
            related_items: field_types,
            used_terms,
            semantic_category: classify_semantic_category(&struct_name),
            abstraction_level: classify_abstraction_level(&struct_name),
        };
        
        self.concepts.insert(struct_name, concept);
    }
    
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        let enum_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        let used_terms = extract_terms_from_code(&source_code);
        let variant_names = extract_enum_variants(node);
        
        let concept = TypedDomainConcept {
            name: enum_name.clone(),
            concept_type: ConceptType::Enum,
            frequency: 1,
            related_items: variant_names,
            used_terms,
            semantic_category: classify_semantic_category(&enum_name),
            abstraction_level: classify_abstraction_level(&enum_name),
        };
        
        self.concepts.insert(enum_name, concept);
    }
    
    fn visit_item_const(&mut self, node: &'ast ItemConst) {
        let const_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        let used_terms = extract_terms_from_code(&source_code);
        let const_type = type_to_string(&node.ty);
        
        let concept = TypedDomainConcept {
            name: const_name.clone(),
            concept_type: ConceptType::Constant,
            frequency: 1,
            related_items: vec![const_type],
            used_terms,
            semantic_category: classify_semantic_category(&const_name),
            abstraction_level: classify_abstraction_level(&const_name),
        };
        
        self.concepts.insert(const_name, concept);
    }
    
    fn visit_item_type(&mut self, node: &'ast ItemType) {
        let type_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        let used_terms = extract_terms_from_code(&source_code);
        let aliased_type = type_to_string(&node.ty);
        
        let concept = TypedDomainConcept {
            name: type_name.clone(),
            concept_type: ConceptType::TypeAlias,
            frequency: 1,
            related_items: vec![aliased_type],
            used_terms,
            semantic_category: classify_semantic_category(&type_name),
            abstraction_level: classify_abstraction_level(&type_name),
        };
        
        self.concepts.insert(type_name, concept);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  TYPED DOMAIN MODEL ANALYZER");
    println!("===============================");
    
    let mut analyzer = TypedDomainAnalyzer {
        concepts: HashMap::new(),
        functions: Vec::new(),
        type_usage_graph: HashMap::new(),
        current_file: String::new(),
    };
    
    // Analyze source files
    let source_files = vec![
        "./semantic_signature_generator.rs",
        "./basic_block_analyzer.rs",
        "./io_matrix_analyzer.rs",
        "./split_decls_applicator.rs",
        "./function_lattice_analyzer.rs",
        "./domain_model_analyzer.rs",
    ];
    
    for file_path in &source_files {
        analyzer.current_file = file_path.to_string();
        let full_path = format!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_driver2/{}", file_path.trim_start_matches("./"));
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            if let Ok(syntax_tree) = parse_file(&content) {
                analyzer.visit_file(&syntax_tree);
            }
        }
    }
    
    // Enhance functions with type information
    enhance_functions_with_types(&mut analyzer);
    
    println!("\n🏗️  TYPED DOMAIN ANALYSIS:");
    println!("==========================");
    println!("Total concepts: {}", analyzer.concepts.len());
    println!("Enhanced functions: {}", analyzer.functions.len());
    println!("Type relationships: {}", analyzer.type_usage_graph.len());
    
    // Analyze by concept type
    println!("\n📊 CONCEPT TYPE DISTRIBUTION:");
    let mut type_counts: HashMap<ConceptType, usize> = HashMap::new();
    for concept in analyzer.concepts.values() {
        *type_counts.entry(concept.concept_type.clone()).or_insert(0) += 1;
    }
    
    for (concept_type, count) in &type_counts {
        println!("   {:?}: {} items", concept_type, count);
    }
    
    // Top concepts by type
    println!("\n🏆 TOP CONCEPTS BY TYPE:");
    for concept_type in [ConceptType::Function, ConceptType::Struct, ConceptType::Enum, ConceptType::Constant] {
        let mut concepts_of_type: Vec<_> = analyzer.concepts.values()
            .filter(|c| c.concept_type == concept_type)
            .collect();
        concepts_of_type.sort_by(|a, b| b.used_terms.len().cmp(&a.used_terms.len()));
        
        println!("\n{:?}s:", concept_type);
        for concept in concepts_of_type.iter().take(5) {
            println!("   {} (terms: {}, category: {:?})", 
                     concept.name, 
                     concept.used_terms.len(),
                     concept.semantic_category);
        }
    }
    
    // Enhanced function analysis
    println!("\n🔧 ENHANCED FUNCTION ANALYSIS:");
    let mut sorted_functions = analyzer.functions.clone();
    sorted_functions.sort_by(|a, b| b.used_terms.len().cmp(&a.used_terms.len()));
    
    for function in sorted_functions.iter().take(10) {
        println!("   {} (types: {}, terms: {}, complexity: {})", 
                 function.name,
                 function.used_types.len(),
                 function.used_terms.len(),
                 function.complexity_score);
        
        if !function.used_terms.is_empty() {
            let terms: Vec<String> = function.used_terms.iter().take(5).cloned().collect();
            println!("     Terms: {}", terms.join(", "));
        }
    }
    
    // Type usage analysis
    println!("\n🔗 TYPE USAGE ANALYSIS:");
    let mut sorted_types: Vec<_> = analyzer.type_usage_graph.iter().collect();
    sorted_types.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (type_name, users) in sorted_types.iter().take(10) {
        println!("   {} used by {} functions", type_name, users.len());
    }
    
    // Generate comprehensive report
    let report = json!({
        "analysis_type": "typed_domain_model",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "total_concepts": analyzer.concepts.len(),
            "enhanced_functions": analyzer.functions.len(),
            "type_relationships": analyzer.type_usage_graph.len()
        },
        "concept_types": type_counts.iter().map(|(t, c)| json!({
            "type": format!("{:?}", t),
            "count": c
        })).collect::<Vec<_>>(),
        "enhanced_functions": sorted_functions.iter().take(20).map(|f| json!({
            "name": f.name,
            "used_types_count": f.used_types.len(),
            "used_terms_count": f.used_terms.len(),
            "complexity_score": f.complexity_score,
            "return_type": f.return_type,
            "parameter_count": f.parameters.len(),
            "sample_terms": f.used_terms.iter().take(5).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "type_usage": sorted_types.iter().take(15).map(|(type_name, users)| json!({
            "type": type_name,
            "usage_count": users.len(),
            "used_by": users.iter().take(5).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "concepts_by_category": analyzer.concepts.values()
            .fold(HashMap::new(), |mut acc, concept| {
                *acc.entry(concept.semantic_category.clone()).or_insert(0) += 1;
                acc
            })
            .iter()
            .map(|(cat, count)| json!({
                "category": format!("{:?}", cat),
                "count": count
            }))
            .collect::<Vec<_>>()
    });
    
    fs::write("typed_domain_model_analysis.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Typed domain model analysis saved to: typed_domain_model_analysis.json");
    
    Ok(())
}

fn extract_types_from_function(node: &ItemFn) -> HashSet<String> {
    let mut types = HashSet::new();
    
    // Extract parameter types
    for input in &node.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            types.insert(type_to_string(&pat_type.ty));
        }
    }
    
    // Extract return type
    if let syn::ReturnType::Type(_, ty) = &node.sig.output {
        types.insert(type_to_string(ty));
    }
    
    types
}

fn extract_terms_from_code(code: &str) -> HashSet<String> {
    let mut terms = HashSet::new();
    
    // Extract identifiers and keywords
    for word in code.split_whitespace() {
        let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
        if cleaned.len() > 2 && !cleaned.chars().all(|c| c.is_numeric()) {
            // Split camelCase and snake_case
            for part in split_identifier(cleaned) {
                if part.len() > 2 {
                    terms.insert(part.to_lowercase());
                }
            }
        }
    }
    
    terms
}

fn split_identifier(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    
    // Split on underscores first
    for part in s.split('_') {
        if !part.is_empty() {
            // Then split camelCase
            let mut current = String::new();
            for ch in part.chars() {
                if ch.is_uppercase() && !current.is_empty() {
                    result.push(current);
                    current = String::new();
                }
                current.push(ch);
            }
            if !current.is_empty() {
                result.push(current);
            }
        }
    }
    
    result
}

fn extract_parameter_names(node: &ItemFn) -> Vec<String> {
    let mut params = Vec::new();
    
    for input in &node.sig.inputs {
        match input {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    params.push(pat_ident.ident.to_string());
                }
            }
            syn::FnArg::Receiver(_) => {
                params.push("self".to_string());
            }
        }
    }
    
    params
}

fn extract_return_type(node: &ItemFn) -> String {
    match &node.sig.output {
        syn::ReturnType::Default => "()".to_string(),
        syn::ReturnType::Type(_, ty) => type_to_string(ty),
    }
}

fn extract_struct_field_types(node: &ItemStruct) -> Vec<String> {
    let mut types = Vec::new();
    
    match &node.fields {
        syn::Fields::Named(fields) => {
            for field in &fields.named {
                types.push(type_to_string(&field.ty));
            }
        }
        syn::Fields::Unnamed(fields) => {
            for field in &fields.unnamed {
                types.push(type_to_string(&field.ty));
            }
        }
        syn::Fields::Unit => {}
    }
    
    types
}

fn extract_enum_variants(node: &ItemEnum) -> Vec<String> {
    node.variants.iter()
        .map(|variant| variant.ident.to_string())
        .collect()
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

fn calculate_complexity(code: &str) -> usize {
    let mut complexity = 1;
    complexity += code.matches("if ").count();
    complexity += code.matches("match ").count();
    complexity += code.matches("for ").count();
    complexity += code.matches("while ").count();
    complexity += code.matches("loop ").count();
    complexity
}

fn classify_semantic_category(name: &str) -> SemanticCategory {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        n if n.contains("analyze") || n.contains("generate") || n.contains("extract") => SemanticCategory::Action,
        n if n.contains("signature") || n.contains("block") || n.contains("matrix") => SemanticCategory::Entity,
        n if n.contains("semantic") || n.contains("basic") || n.contains("unknown") => SemanticCategory::Property,
        n if n.contains("rust") || n.contains("cargo") || n.contains("split") => SemanticCategory::System,
        n if n.contains("json") || n.contains("file") || n.contains("string") => SemanticCategory::Data,
        n if n.contains("meta") || n.contains("introspector") => SemanticCategory::Meta,
        _ => SemanticCategory::Entity,
    }
}

fn classify_abstraction_level(name: &str) -> AbstractionLevel {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        n if n.contains("file") || n.contains("string") || n.contains("json") => AbstractionLevel::Concrete,
        n if n.contains("introspector") || n.contains("lattice") || n.contains("meta") => AbstractionLevel::Meta,
        n if n.contains("rust") || n.contains("cargo") || n.contains("compiler") => AbstractionLevel::System,
        _ => AbstractionLevel::Abstract,
    }
}

fn enhance_functions_with_types(analyzer: &mut TypedDomainAnalyzer) {
    // Cross-reference functions with type usage
    for function in &mut analyzer.functions {
        // Add related concepts based on used terms
        for term in &function.used_terms {
            if let Some(concept) = analyzer.concepts.get_mut(term) {
                concept.frequency += 1;
            }
        }
    }
}
