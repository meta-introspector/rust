// Semantic Monster Group Mapping of Demangled Rustc Names
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug, Serialize, Deserialize)]
struct SemanticMonsterMapping {
    demangled_name: String,
    mangled_name: String,
    type_components: Vec<TypeComponent>,
    monster_signature: Vec<u64>,
    semantic_category: String,
    ngram_analysis: HashMap<String, u32>,
    monster_type_correlation: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TypeComponent {
    component_type: String, // "crate", "module", "function", "type", "impl"
    name: String,
    monster_score: f64,
    type_semantics: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 SEMANTIC MONSTER GROUP MAPPING OF RUSTC NAMES");
    println!("================================================");

    // Load topology with mangled names
    let topology = load_topology()?;
    let nodes = topology["nodes"].as_array().unwrap();
    println!("📦 Loaded {} function nodes", nodes.len());

    // Demangle all names and analyze
    let mut semantic_mappings = Vec::new();

    for node in nodes {
        if let Some(mangled_name) = node["symbol_name"].as_str() {
            if let Some(mapping) = analyze_demangled_name(mangled_name, node) {
                semantic_mappings.push(mapping);
            }
        }
    }

    println!("🔍 Analyzed {} demangled names", semantic_mappings.len());

    // Create type system mapping
    let type_system_map = create_type_system_mapping(&semantic_mappings);

    // Display results
    display_semantic_analysis(&semantic_mappings, &type_system_map);

    // Save results
    save_semantic_mapping(&semantic_mappings, &type_system_map)?;

    Ok(())
}

fn load_topology() -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("unified_monster_topology.json")?;
    Ok(serde_json::from_str(&content)?)
}

fn analyze_demangled_name(mangled_name: &str, node: &Value) -> Option<SemanticMonsterMapping> {
    // Demangle the name
    let demangled = demangle_rust_name(mangled_name);

    // Parse type components
    let type_components = parse_type_components(&demangled);

    // Generate n-grams from demangled name
    let ngram_analysis = generate_name_ngrams(&demangled);

    // Calculate Monster signature
    let monster_signature = calculate_name_monster_signature(&demangled);

    // Determine semantic category
    let semantic_category = determine_semantic_category(&demangled, &type_components);

    // Calculate type correlation
    let monster_type_correlation = calculate_type_monster_correlation(&type_components);

    Some(SemanticMonsterMapping {
        demangled_name: demangled,
        mangled_name: mangled_name.to_string(),
        type_components,
        monster_signature,
        semantic_category,
        ngram_analysis,
        monster_type_correlation,
    })
}

fn demangle_rust_name(mangled: &str) -> String {
    // Try using rustfilt for demangling
    if let Ok(output) = Command::new("rustfilt").arg(mangled).output() {
        if let Ok(demangled) = String::from_utf8(output.stdout) {
            return demangled.trim().to_string();
        }
    }

    // Fallback: simple parsing of Rust mangled names
    simple_rust_demangle(mangled)
}

fn simple_rust_demangle(mangled: &str) -> String {
    if !mangled.starts_with("_ZN") {
        return mangled.to_string();
    }

    let mut result = Vec::new();
    let mut chars = mangled[3..].chars().peekable(); // Skip "_ZN"

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            // Parse length-prefixed identifier
            let mut len_str = String::new();
            len_str.push(ch);

            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    len_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            if let Ok(len) = len_str.parse::<usize>() {
                let identifier: String = chars.by_ref().take(len).collect();
                result.push(identifier);
            }
        }
    }

    if result.is_empty() { mangled.to_string() } else { result.join("::") }
}

fn parse_type_components(demangled: &str) -> Vec<TypeComponent> {
    let mut components = Vec::new();
    let parts: Vec<&str> = demangled.split("::").collect();

    for (i, part) in parts.iter().enumerate() {
        let component_type = match i {
            0 => "crate",
            _ if part.contains("impl") => "impl",
            _ if part.starts_with(char::is_uppercase) => "type",
            _ if part.contains("_") || part.chars().all(|c| c.is_lowercase() || c == '_') => {
                "function"
            }
            _ => "module",
        };

        let monster_score = calculate_component_monster_score(part);
        let type_semantics = determine_type_semantics(part, component_type);

        components.push(TypeComponent {
            component_type: component_type.to_string(),
            name: part.to_string(),
            monster_score,
            type_semantics,
        });
    }

    components
}

fn calculate_component_monster_score(component: &str) -> f64 {
    let mut score = 0.0;

    for ch in component.chars() {
        let ascii_val = ch as u32 as u64;
        for &prime in &MONSTER_PRIMES {
            if ascii_val % prime == 0 {
                score += 1.0 / prime as f64;
            }
        }
    }

    score / component.len() as f64
}

fn determine_type_semantics(component: &str, component_type: &str) -> String {
    let mut semantics = Vec::new();

    // Semantic analysis based on name patterns
    if component.contains("driver") {
        semantics.push("compiler_core");
    }
    if component.contains("parse") {
        semantics.push("parsing");
    }
    if component.contains("compile") {
        semantics.push("compilation");
    }
    if component.contains("error") {
        semantics.push("error_handling");
    }
    if component.contains("debug") {
        semantics.push("debugging");
    }
    if component.contains("alloc") {
        semantics.push("memory");
    }
    if component.contains("impl") {
        semantics.push("implementation");
    }
    if component.contains("trait") {
        semantics.push("trait_system");
    }
    if component.contains("type") {
        semantics.push("type_system");
    }

    // Type-based semantics
    match component_type {
        "crate" => semantics.push("namespace"),
        "module" => semantics.push("organization"),
        "function" => semantics.push("behavior"),
        "type" => semantics.push("data_structure"),
        "impl" => semantics.push("method_implementation"),
        _ => semantics.push("unknown"),
    }

    if semantics.is_empty() { "generic".to_string() } else { semantics.join("_") }
}

fn generate_name_ngrams(name: &str) -> HashMap<String, u32> {
    let mut ngrams = HashMap::new();

    // Generate 2-grams, 3-grams, 4-grams from the demangled name
    for n in 2..=4 {
        if name.len() >= n {
            for i in 0..=name.len() - n {
                let ngram = name[i..i + n].to_string();
                *ngrams.entry(ngram).or_insert(0) += 1;
            }
        }
    }

    // Also generate n-grams from individual components
    for component in name.split("::") {
        for n in 2..=3 {
            if component.len() >= n {
                for i in 0..=component.len() - n {
                    let ngram = component[i..i + n].to_string();
                    *ngrams.entry(ngram).or_insert(0) += 1;
                }
            }
        }
    }

    ngrams
}

fn calculate_name_monster_signature(name: &str) -> Vec<u64> {
    let mut signature = Vec::new();

    for &prime in &MONSTER_PRIMES {
        let mut count = 0;

        for ch in name.chars() {
            let ascii_val = ch as u32 as u64;
            if ascii_val % prime == 0 {
                count += 1;
            }
        }

        signature.push(count);
    }

    signature
}

fn determine_semantic_category(name: &str, components: &[TypeComponent]) -> String {
    // Hierarchical semantic categorization
    if name.contains("rustc_driver") {
        "compiler_driver"
    } else if name.contains("rustc_") {
        "compiler_component"
    } else if components.iter().any(|c| c.component_type == "impl") {
        "implementation"
    } else if components.iter().any(|c| c.type_semantics.contains("parsing")) {
        "parser"
    } else if components.iter().any(|c| c.type_semantics.contains("error")) {
        "error_system"
    } else if components.iter().any(|c| c.type_semantics.contains("memory")) {
        "memory_management"
    } else {
        "utility"
    }
    .to_string()
}

fn calculate_type_monster_correlation(components: &[TypeComponent]) -> f64 {
    if components.is_empty() {
        return 0.0;
    }

    let total_score: f64 = components.iter().map(|c| c.monster_score).sum();
    total_score / components.len() as f64
}

fn create_type_system_mapping(mappings: &[SemanticMonsterMapping]) -> HashMap<String, Vec<String>> {
    let mut type_map = HashMap::new();

    for mapping in mappings {
        // Group by semantic category
        type_map
            .entry(mapping.semantic_category.clone())
            .or_insert_with(Vec::new)
            .push(mapping.demangled_name.clone());

        // Group by type components
        for component in &mapping.type_components {
            let key = format!("{}_{}", component.component_type, component.type_semantics);
            type_map.entry(key).or_insert_with(Vec::new).push(component.name.clone());
        }
    }

    type_map
}

fn display_semantic_analysis(
    mappings: &[SemanticMonsterMapping],
    type_map: &HashMap<String, Vec<String>>,
) {
    println!("\n🔤 SEMANTIC MONSTER ANALYSIS:");
    println!("=============================");

    // Top demangled names by Monster correlation
    let mut by_correlation: Vec<_> =
        mappings.iter().map(|m| (m.demangled_name.clone(), m.monster_type_correlation)).collect();
    by_correlation.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("   🧬 Top Monster-Correlated Names:");
    for (i, (name, correlation)) in by_correlation.iter().take(10).enumerate() {
        let display_name = if name.len() > 60 { &name[..60] } else { name };
        println!("     {}: {} ({:.4})", i + 1, display_name, correlation);
    }

    // Semantic categories
    println!("\n   📂 Semantic Categories:");
    let mut category_counts = HashMap::new();
    for mapping in mappings {
        *category_counts.entry(mapping.semantic_category.clone()).or_insert(0) += 1;
    }

    let mut sorted_categories: Vec<_> = category_counts.into_iter().collect();
    sorted_categories.sort_by(|a, b| b.1.cmp(&a.1));

    for (category, count) in sorted_categories {
        println!("     {}: {} functions", category, count);
    }

    // Type component analysis
    println!("\n   🏗️ Type Component Distribution:");
    let mut component_counts = HashMap::new();
    for mapping in mappings {
        for component in &mapping.type_components {
            *component_counts.entry(component.component_type.clone()).or_insert(0) += 1;
        }
    }

    for (comp_type, count) in component_counts {
        println!("     {}: {} components", comp_type, count);
    }

    // Most frequent n-grams
    println!("\n   🔤 Most Frequent N-grams:");
    let mut all_ngrams = HashMap::new();
    for mapping in mappings {
        for (ngram, &count) in &mapping.ngram_analysis {
            *all_ngrams.entry(ngram.clone()).or_insert(0) += count;
        }
    }

    let mut sorted_ngrams: Vec<_> = all_ngrams.into_iter().collect();
    sorted_ngrams.sort_by(|a, b| b.1.cmp(&a.1));

    for (ngram, count) in sorted_ngrams.iter().take(10) {
        let monster_score = calculate_component_monster_score(ngram);
        println!("     '{}': {} occurrences (Monster: {:.3})", ngram, count, monster_score);
    }
}

fn save_semantic_mapping(
    mappings: &[SemanticMonsterMapping],
    type_map: &HashMap<String, Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete semantic mapping
    let json = serde_json::to_string_pretty(mappings)?;
    fs::write("rustc_semantic_monster_mapping.json", json)?;

    // Save type system mapping
    let type_json = serde_json::to_string_pretty(type_map)?;
    fs::write("rustc_type_system_mapping.json", type_json)?;

    // Save summary CSV
    let mut csv_content = String::from(
        "demangled_name,semantic_category,monster_correlation,component_count,top_ngram\n",
    );
    for mapping in mappings.iter().take(1000) {
        let top_ngram = mapping
            .ngram_analysis
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(ngram, _)| ngram.clone())
            .unwrap_or_else(|| "none".to_string());

        csv_content.push_str(&format!(
            "{},{},{:.4},{},{}\n",
            mapping.demangled_name.replace(',', ";"),
            mapping.semantic_category,
            mapping.monster_type_correlation,
            mapping.type_components.len(),
            top_ngram
        ));
    }
    fs::write("rustc_semantic_analysis.csv", csv_content)?;

    println!("\n💾 SEMANTIC MONSTER MAPPING SAVED:");
    println!("==================================");
    println!("   Complete mapping: rustc_semantic_monster_mapping.json");
    println!("   Type system: rustc_type_system_mapping.json");
    println!("   Analysis CSV: rustc_semantic_analysis.csv");
    println!("   Functions analyzed: {}", mappings.len());

    Ok(())
}
