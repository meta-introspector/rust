use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::json;

#[derive(Debug, Clone)]
struct DomainConcept {
    name: String,
    frequency: usize,
    related_functions: Vec<String>,
    semantic_category: SemanticCategory,
    abstraction_level: AbstractionLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SemanticCategory {
    Action,        // analyze, generate, extract, process
    Entity,        // signature, block, matrix, lattice
    Property,      // semantic, basic, unknown, novel
    System,        // rust, cargo, split, decls
    Data,          // json, file, string, vec
    Meta,          // meta, introspector, zombie
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AbstractionLevel {
    Concrete,      // file, string, json
    Abstract,      // signature, analysis, pattern
    Meta,          // introspector, lattice, semantic
    System,        // rust, cargo, compiler
}

#[derive(Debug, Clone)]
struct DomainRelation {
    from_concept: String,
    to_concept: String,
    relation_type: RelationType,
    strength: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RelationType {
    IsA,           // signature is_a pattern
    PartOf,        // block part_of signature
    Uses,          // analyzer uses extractor
    Creates,       // generator creates signature
    Processes,     // analyzer processes data
}

struct DomainAnalyzer {
    concepts: HashMap<String, DomainConcept>,
    relations: Vec<DomainRelation>,
    function_names: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  DOMAIN MODEL ANALYZER");
    println!("=========================");
    
    // Load function names from lattice analysis
    let lattice_data = fs::read_to_string("function_lattice_analysis.json")?;
    let lattice_json: serde_json::Value = serde_json::from_str(&lattice_data)?;
    
    let mut function_names = Vec::new();
    
    // Extract function names from lattice classes
    if let Some(classes) = lattice_json["lattice_classes"].as_array() {
        for class in classes {
            if let Some(functions) = class["functions"].as_array() {
                for function in functions {
                    if let Some(name) = function.as_str() {
                        function_names.push(name.to_string());
                    }
                }
            }
        }
    }
    
    // Also add function names from our analysis tools
    let additional_functions = vec![
        "semantic_signature_generator", "basic_block_analyzer", "io_matrix_analyzer",
        "unknown_type_classifier", "code_snippet_extractor", "split_decls_applicator",
        "function_lattice_analyzer", "personal_repo_identifier", "master_tld_analyzer",
        "massive_rust_analyzer", "git_structure_analyzer", "tld_directory_organizer",
        "rust_signature_extractor", "duplicate_block_detector", "self_analyzer",
    ];
    
    for func in additional_functions {
        function_names.push(func.to_string());
    }
    
    println!("📊 Analyzing {} function names", function_names.len());
    
    let mut analyzer = DomainAnalyzer {
        concepts: HashMap::new(),
        relations: Vec::new(),
        function_names: function_names.clone(),
    };
    
    // Extract keywords and build concepts
    extract_domain_concepts(&mut analyzer);
    
    // Build semantic relationships
    build_domain_relations(&mut analyzer);
    
    // Analyze domain structure
    println!("\n🏗️  DOMAIN MODEL ANALYSIS:");
    println!("==========================");
    println!("Total concepts: {}", analyzer.concepts.len());
    println!("Total relations: {}", analyzer.relations.len());
    
    // Top concepts by frequency
    println!("\n📊 TOP DOMAIN CONCEPTS:");
    let mut sorted_concepts: Vec<_> = analyzer.concepts.values().collect();
    sorted_concepts.sort_by(|a, b| b.frequency.cmp(&a.frequency));
    
    for (i, concept) in sorted_concepts.iter().take(20).enumerate() {
        println!("{}. {} ({} occurrences, {:?})", 
                 i + 1, 
                 concept.name, 
                 concept.frequency,
                 concept.semantic_category);
        println!("   Level: {:?}, Functions: {}", 
                 concept.abstraction_level,
                 concept.related_functions.len());
    }
    
    // Analyze by semantic category
    println!("\n🏷️  SEMANTIC CATEGORIES:");
    let mut category_counts: HashMap<SemanticCategory, usize> = HashMap::new();
    for concept in analyzer.concepts.values() {
        *category_counts.entry(concept.semantic_category.clone()).or_insert(0) += 1;
    }
    
    let mut sorted_categories: Vec<_> = category_counts.iter().collect();
    sorted_categories.sort_by(|a, b| b.1.cmp(a.1));
    
    for (category, count) in &sorted_categories {
        println!("   {:?}: {} concepts", category, count);
    }
    
    // Analyze by abstraction level
    println!("\n📐 ABSTRACTION LEVELS:");
    let mut level_counts: HashMap<AbstractionLevel, usize> = HashMap::new();
    for concept in analyzer.concepts.values() {
        *level_counts.entry(concept.abstraction_level.clone()).or_insert(0) += 1;
    }
    
    let mut sorted_levels: Vec<_> = level_counts.iter().collect();
    sorted_levels.sort_by(|a, b| b.1.cmp(a.1));
    
    for (level, count) in &sorted_levels {
        println!("   {:?}: {} concepts", level, count);
    }
    
    // Analyze domain relationships
    println!("\n🔗 DOMAIN RELATIONSHIPS:");
    let mut relation_counts: HashMap<RelationType, usize> = HashMap::new();
    for relation in &analyzer.relations {
        *relation_counts.entry(relation.relation_type.clone()).or_insert(0) += 1;
    }
    
    for (relation_type, count) in &relation_counts {
        println!("   {:?}: {} relationships", relation_type, count);
    }
    
    // Show strongest relationships
    println!("\n💪 STRONGEST RELATIONSHIPS:");
    let mut sorted_relations = analyzer.relations.clone();
    sorted_relations.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
    
    for relation in sorted_relations.iter().take(10) {
        println!("   {} {:?} {} (strength: {:.2})", 
                 relation.from_concept,
                 relation.relation_type,
                 relation.to_concept,
                 relation.strength);
    }
    
    // Generate domain ontology
    println!("\n🧠 DOMAIN ONTOLOGY:");
    generate_domain_ontology(&analyzer);
    
    // Generate comprehensive report
    let report = json!({
        "analysis_type": "domain_model",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "total_concepts": analyzer.concepts.len(),
            "total_relations": analyzer.relations.len(),
            "functions_analyzed": function_names.len()
        },
        "concepts": sorted_concepts.iter().take(30).map(|concept| json!({
            "name": concept.name,
            "frequency": concept.frequency,
            "semantic_category": format!("{:?}", concept.semantic_category),
            "abstraction_level": format!("{:?}", concept.abstraction_level),
            "related_functions": concept.related_functions.len()
        })).collect::<Vec<_>>(),
        "semantic_categories": sorted_categories.iter().map(|(cat, count)| json!({
            "category": format!("{:?}", cat),
            "concept_count": count
        })).collect::<Vec<_>>(),
        "abstraction_levels": sorted_levels.iter().map(|(level, count)| json!({
            "level": format!("{:?}", level),
            "concept_count": count
        })).collect::<Vec<_>>(),
        "relationships": sorted_relations.iter().take(20).map(|rel| json!({
            "from": rel.from_concept,
            "relation": format!("{:?}", rel.relation_type),
            "to": rel.to_concept,
            "strength": rel.strength
        })).collect::<Vec<_>>()
    });
    
    fs::write("domain_model_analysis.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Domain model analysis saved to: domain_model_analysis.json");
    
    Ok(())
}

fn extract_domain_concepts(analyzer: &mut DomainAnalyzer) {
    let mut keyword_counts: HashMap<String, Vec<String>> = HashMap::new();
    
    // Extract keywords from function names
    for function_name in &analyzer.function_names {
        let keywords = extract_keywords(function_name);
        
        for keyword in keywords {
            keyword_counts.entry(keyword).or_default().push(function_name.clone());
        }
    }
    
    // Create domain concepts
    for (keyword, functions) in keyword_counts {
        if functions.len() >= 1 { // Include all keywords
            let concept = DomainConcept {
                name: keyword.clone(),
                frequency: functions.len(),
                related_functions: functions,
                semantic_category: classify_semantic_category(&keyword),
                abstraction_level: classify_abstraction_level(&keyword),
            };
            
            analyzer.concepts.insert(keyword, concept);
        }
    }
}

fn extract_keywords(function_name: &str) -> Vec<String> {
    let mut keywords = Vec::new();
    
    // Split on common separators
    let parts: Vec<&str> = function_name
        .split(&['_', '-', '.', ':'][..])
        .filter(|s| !s.is_empty())
        .collect();
    
    for part in parts {
        // Split camelCase
        let camel_parts = split_camel_case(part);
        for camel_part in camel_parts {
            if camel_part.len() > 2 { // Filter out very short words
                keywords.push(camel_part.to_lowercase());
            }
        }
    }
    
    keywords
}

fn split_camel_case(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    
    for ch in s.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            result.push(current);
            current = String::new();
        }
        current.push(ch);
    }
    
    if !current.is_empty() {
        result.push(current);
    }
    
    result
}

fn classify_semantic_category(keyword: &str) -> SemanticCategory {
    match keyword {
        // Actions
        "analyze" | "generate" | "extract" | "process" | "build" | "create" | "find" | "detect" | "classify" => SemanticCategory::Action,
        
        // Entities
        "signature" | "block" | "matrix" | "lattice" | "node" | "graph" | "tree" | "pattern" => SemanticCategory::Entity,
        
        // Properties
        "semantic" | "basic" | "unknown" | "novel" | "complex" | "simple" | "unique" => SemanticCategory::Property,
        
        // System
        "rust" | "cargo" | "split" | "decls" | "compiler" | "driver" | "zombie" => SemanticCategory::System,
        
        // Data
        "json" | "file" | "string" | "vec" | "data" | "report" | "output" => SemanticCategory::Data,
        
        // Meta
        "meta" | "introspector" | "analysis" | "model" | "framework" => SemanticCategory::Meta,
        
        _ => SemanticCategory::Entity, // Default
    }
}

fn classify_abstraction_level(keyword: &str) -> AbstractionLevel {
    match keyword {
        // Concrete
        "file" | "string" | "json" | "data" | "output" | "input" | "path" => AbstractionLevel::Concrete,
        
        // Abstract
        "signature" | "analysis" | "pattern" | "model" | "structure" | "algorithm" => AbstractionLevel::Abstract,
        
        // Meta
        "introspector" | "lattice" | "semantic" | "meta" | "ontology" | "framework" => AbstractionLevel::Meta,
        
        // System
        "rust" | "cargo" | "compiler" | "driver" | "system" | "platform" => AbstractionLevel::System,
        
        _ => AbstractionLevel::Abstract, // Default
    }
}

fn build_domain_relations(analyzer: &mut DomainAnalyzer) {
    let concept_names: Vec<String> = analyzer.concepts.keys().cloned().collect();
    
    for i in 0..concept_names.len() {
        for j in 0..concept_names.len() {
            if i != j {
                let from_concept = &concept_names[i];
                let to_concept = &concept_names[j];
                
                // Calculate relationship strength based on co-occurrence
                let strength = calculate_relationship_strength(
                    &analyzer.concepts[from_concept],
                    &analyzer.concepts[to_concept]
                );
                
                if strength > 0.1 {
                    let relation_type = infer_relation_type(from_concept, to_concept);
                    
                    analyzer.relations.push(DomainRelation {
                        from_concept: from_concept.clone(),
                        to_concept: to_concept.clone(),
                        relation_type,
                        strength,
                    });
                }
            }
        }
    }
}

fn calculate_relationship_strength(concept1: &DomainConcept, concept2: &DomainConcept) -> f64 {
    let shared_functions: HashSet<_> = concept1.related_functions.iter()
        .filter(|f| concept2.related_functions.contains(f))
        .collect();
    
    let total_functions = concept1.related_functions.len() + concept2.related_functions.len();
    
    if total_functions > 0 {
        (shared_functions.len() * 2) as f64 / total_functions as f64
    } else {
        0.0
    }
}

fn infer_relation_type(from_concept: &str, to_concept: &str) -> RelationType {
    // Simple heuristics for relation type inference
    if from_concept.contains("analyzer") && to_concept.contains("signature") {
        RelationType::Creates
    } else if from_concept.contains("block") && to_concept.contains("signature") {
        RelationType::PartOf
    } else if from_concept.contains("generator") {
        RelationType::Creates
    } else if from_concept.contains("extractor") {
        RelationType::Processes
    } else {
        RelationType::Uses
    }
}

fn generate_domain_ontology(analyzer: &DomainAnalyzer) {
    println!("Core Domain Areas:");
    
    // Group concepts by semantic category
    let mut category_groups: HashMap<SemanticCategory, Vec<&DomainConcept>> = HashMap::new();
    for concept in analyzer.concepts.values() {
        category_groups.entry(concept.semantic_category.clone()).or_default().push(concept);
    }
    
    for (category, concepts) in category_groups {
        println!("\n{:?} Domain:", category);
        let mut sorted_concepts = concepts;
        sorted_concepts.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        
        for concept in sorted_concepts.iter().take(5) {
            println!("  - {} ({})", concept.name, concept.frequency);
        }
    }
}
