use serde_json::Value;
use std::collections::{HashMap, BTreeSet};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FeatureBitmap {
    bits: BTreeSet<String>,
}

#[derive(Debug)]
struct TypeFunctionMapping {
    name: String,
    feature_bitmap: FeatureBitmap,
    complexity_level: usize,
}

fn main() {
    println!("🔬 Rust Feature Lattice: Types & Functions → Feature Bitmaps");
    
    let usage_data_dir = "../../usage_data";
    let mut type_function_mappings = Vec::new();
    let mut feature_lattice_levels = HashMap::new();
    
    // Extract all types/functions and their feature requirements
    extract_type_function_features(usage_data_dir, &mut type_function_mappings);
    
    // Build feature lattice levels
    build_feature_lattice(&type_function_mappings, &mut feature_lattice_levels);
    
    // Display lattice structure
    display_feature_lattice(&feature_lattice_levels);
    
    // Show examples of types/functions at each complexity level
    show_complexity_examples(&type_function_mappings);
    
    // Save the complete mapping
    save_feature_mapping(&type_function_mappings, &feature_lattice_levels);
}

fn extract_type_function_features(usage_data_dir: &str, mappings: &mut Vec<TypeFunctionMapping>) {
    let mut processed_files = 0;
    
    if let Ok(entries) = fs::read_dir(usage_data_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    processed_files += 1;
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            extract_from_file(&json, mappings);
                        }
                    }
                    
                    if processed_files % 1000 == 0 {
                        println!("   Processed {} files...", processed_files);
                    }
                }
            }
        }
    }
    
    println!("📊 Processed {} files", processed_files);
    println!("📊 Found {} type/function mappings", mappings.len());
}

fn extract_from_file(json: &Value, mappings: &mut Vec<TypeFunctionMapping>) {
    if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
        for usage in usages {
            if let (Some(usage_str), Some(used_def_id)) = (
                usage.get("usage").and_then(|u| u.as_str()),
                usage.get("used_def_id").and_then(|u| u.as_str())
            ) {
                let feature_bitmap = analyze_feature_requirements(usage_str, used_def_id);
                let name = extract_type_function_name(used_def_id);
                
                if !feature_bitmap.bits.is_empty() && !name.is_empty() {
                    mappings.push(TypeFunctionMapping {
                        name,
                        complexity_level: feature_bitmap.bits.len(),
                        feature_bitmap,
                    });
                }
            }
        }
    }
}

fn analyze_feature_requirements(usage: &str, def_id: &str) -> FeatureBitmap {
    let mut bits = BTreeSet::new();
    
    // Base language (always present)
    bits.insert("base".to_string());
    
    // Visibility features
    if usage.contains("pub") || def_id.contains("pub") || usage.contains("private") {
        bits.insert("visibility".to_string());
    }
    
    // Generic features
    if usage.contains("<") || usage.contains(">") || def_id.contains("<") {
        bits.insert("generics".to_string());
    }
    
    // Lifetime features
    if usage.contains("'") && (usage.contains("'a") || usage.contains("'static")) {
        bits.insert("lifetimes".to_string());
    }
    
    // Trait features
    if usage.contains("impl") || usage.contains("trait") || def_id.contains("impl") {
        bits.insert("traits".to_string());
    }
    
    // Const features
    if usage.contains("const") || def_id.contains("const") {
        bits.insert("const".to_string());
    }
    
    // Async features
    if usage.contains("async") || usage.contains("await") {
        bits.insert("async".to_string());
    }
    
    // Unsafe features
    if usage.contains("unsafe") || def_id.contains("unsafe") {
        bits.insert("unsafe".to_string());
    }
    
    // Macro features
    if usage.contains("macro") || usage.contains("!") {
        bits.insert("macros".to_string());
    }
    
    FeatureBitmap { bits }
}

fn extract_type_function_name(def_id: &str) -> String {
    // Extract meaningful name from DefId
    if let Some(tilde_pos) = def_id.find(" ~ ") {
        if let Some(end_pos) = def_id.rfind(")") {
            let path = &def_id[tilde_pos + 3..end_pos];
            if let Some(last_colon) = path.rfind("::") {
                return path[last_colon + 2..].to_string();
            }
            return path.to_string();
        }
    }
    
    // Fallback
    if def_id.len() > 50 {
        def_id[..50].to_string()
    } else {
        def_id.to_string()
    }
}

fn build_feature_lattice(mappings: &[TypeFunctionMapping], lattice: &mut HashMap<usize, Vec<String>>) {
    for mapping in mappings {
        lattice.entry(mapping.complexity_level)
            .or_insert_with(Vec::new)
            .push(mapping.name.clone());
    }
    
    // Remove duplicates and sort
    for (_, names) in lattice.iter_mut() {
        names.sort();
        names.dedup();
    }
}

fn display_feature_lattice(lattice: &HashMap<usize, Vec<String>>) {
    println!("\n🏗️  Feature Lattice Structure:");
    println!("   Level 0: ∅ (empty language)");
    
    let mut levels: Vec<_> = lattice.keys().collect();
    levels.sort();
    
    for &level in &levels {
        let count = lattice[level].len();
        println!("   Level {}: {} types/functions", level, count);
        
        // Show feature combinations for this level
        match level {
            1 => println!("      Features: {{base}}"),
            2 => println!("      Features: {{base, X}} where X ∈ {{visibility, generics, traits, ...}}"),
            3 => println!("      Features: {{base, X, Y}} - binary combinations"),
            4 => println!("      Features: {{base, X, Y, Z}} - ternary combinations"),
            _ => println!("      Features: {}-ary combinations", level - 1),
        }
    }
    
    let total_types = lattice.values().map(|v| v.len()).sum::<usize>();
    println!("\n📊 Total: {} unique types/functions across {} complexity levels", total_types, levels.len());
}

fn show_complexity_examples(mappings: &[TypeFunctionMapping]) {
    println!("\n🎯 Examples by Complexity Level:");
    
    // Group by complexity level
    let mut by_level: HashMap<usize, Vec<&TypeFunctionMapping>> = HashMap::new();
    for mapping in mappings {
        by_level.entry(mapping.complexity_level)
            .or_insert_with(Vec::new)
            .push(mapping);
    }
    
    let mut levels: Vec<_> = by_level.keys().collect();
    levels.sort();
    
    for &level in levels.iter().take(6) {  // Show first 6 levels
        println!("\n   Level {} examples:", level);
        let examples = &by_level[level];
        
        for example in examples.iter().take(3) {  // Show 3 examples per level
            let features: Vec<_> = example.feature_bitmap.bits.iter().map(|s| s.as_str()).collect();
            println!("      {} → {{{}}}", 
                example.name, 
                features.join(", ")
            );
        }
        
        if examples.len() > 3 {
            println!("      ... and {} more", examples.len() - 3);
        }
    }
}

fn save_feature_mapping(mappings: &[TypeFunctionMapping], lattice: &HashMap<usize, Vec<String>>) {
    let output = serde_json::json!({
        "analysis_type": "rust_feature_lattice_mapping",
        "lattice_levels": lattice.iter().map(|(level, names)| {
            serde_json::json!({
                "complexity_level": level,
                "type_function_count": names.len(),
                "examples": names.iter().take(10).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>(),
        "sample_mappings": mappings.iter().take(100).map(|m| {
            serde_json::json!({
                "name": m.name,
                "complexity_level": m.complexity_level,
                "feature_bitmap": m.feature_bitmap.bits.iter().collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>(),
        "statistics": {
            "total_mappings": mappings.len(),
            "max_complexity": lattice.keys().max().unwrap_or(&0),
            "lattice_levels": lattice.len()
        }
    });
    
    fs::write("feature_lattice_mapping.json", serde_json::to_string_pretty(&output).unwrap())
        .expect("Failed to write mapping");
    
    println!("\n💾 Saved feature_lattice_mapping.json");
}
