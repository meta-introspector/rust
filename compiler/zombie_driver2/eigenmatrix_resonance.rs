#![feature(str_as_str)]

use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🧬 RUST EIGENMATRIX - RESONANCE ANALYSIS");
    println!("========================================");

    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    // Resonance analysis: find patterns that appear across all complexity classes
    let mut type_resonance: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let mut crate_resonance: HashMap<String, HashMap<String, u32>> = HashMap::new();

    // Load analysis files to get crate context
    let analysis_files: Vec<String> = fs::read_to_string("all_analysis_files.txt")
        .expect("Missing all_analysis_files.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    for file_path in &analysis_files {
        let crate_name = extract_crate_name(file_path);

        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                if let Some(paths) = json_data["json_paths"].as_array() {
                    for path_entry in paths {
                        if let Some(path_str) = path_entry.as_str() {
                            let clean_path = path_str.split(" = ").next().unwrap_or(path_str);

                            if let Some(sig_val) = signatures.get(clean_path) {
                                let signature = sig_val.as_u64().unwrap_or(0) as u128;
                                let complexity_class = classify_signature(signature);
                                let ast_type = extract_dominant_type(clean_path);

                                // Track type resonance across complexity classes
                                *type_resonance
                                    .entry(ast_type.clone())
                                    .or_default()
                                    .entry(complexity_class.clone())
                                    .or_insert(0) += 1;

                                // Track crate resonance across complexity classes
                                *crate_resonance
                                    .entry(crate_name.clone())
                                    .or_default()
                                    .entry(complexity_class)
                                    .or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // Find eigenmatrix: types/crates that resonate across ALL complexity classes
    println!("\n🎯 EIGENMATRIX RESONANCE:");
    println!("=========================");

    let complexity_classes =
        ["simple", "compound", "complex", "nested", "deep", "ultra", "extreme"];

    println!("\n📊 AST TYPE EIGENMATRIX:");
    for (ast_type, class_counts) in &type_resonance {
        let resonance_score = complexity_classes
            .iter()
            .map(|class| class_counts.get(*class).unwrap_or(&0))
            .sum::<u32>();

        let class_coverage = complexity_classes
            .iter()
            .filter(|class| class_counts.get(class.as_str()).unwrap_or(&0) > &0)
            .count();

        if class_coverage >= 5 {
            // Resonates across 5+ complexity classes
            println!("🧬 {}: {} total, {} classes", ast_type, resonance_score, class_coverage);
            for class in &complexity_classes {
                if let Some(count) = class_counts.get(class.as_str()) {
                    if *count > 0 {
                        println!("   {}: {}", class, count);
                    }
                }
            }
        }
    }

    println!("\n📦 CRATE EIGENMATRIX:");
    for (crate_name, class_counts) in &crate_resonance {
        let resonance_score = complexity_classes
            .iter()
            .map(|class| class_counts.get(class.as_str()).unwrap_or(&0))
            .sum::<u32>();

        let class_coverage = complexity_classes
            .iter()
            .filter(|class| class_counts.get(class.as_str()).unwrap_or(&0) > &0)
            .count();

        if class_coverage >= 6 {
            // Resonates across 6+ complexity classes
            println!("🧬 {}: {} total, {} classes", crate_name, resonance_score, class_coverage);
        }
    }

    // Find the ultimate eigenmatrix: highest resonance
    let max_type_resonance = type_resonance.iter().max_by_key(|(_, counts)| {
        complexity_classes.iter().filter(|class| counts.get(class.as_str()).unwrap_or(&0) > &0).count()
    });

    if let Some((eigen_type, _)) = max_type_resonance {
        println!("\n🌟 ULTIMATE AST EIGENMATRIX: {}", eigen_type);
    }
}

fn extract_crate_name(file_path: &str) -> String {
    if let Some(build_part) = file_path.split("/build/").nth(1) {
        if let Some(crate_part) = build_part.split('-').next() {
            return crate_part.to_string();
        }
    }
    file_path.split('/').last().unwrap_or("unknown").to_string()
}

fn extract_dominant_type(path: &str) -> String {
    let segments: Vec<&str> = path.split('.').collect();
    let mut type_counts = HashMap::new();

    for segment in segments {
        if !segment.chars().all(|c| c.is_numeric() || c == '[' || c == ']') {
            *type_counts.entry(segment).or_insert(0) += 1;
        }
    }

    type_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(ast_type, _)| ast_type.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn classify_signature(signature: u128) -> String {
    match signature {
        1..=10 => "simple".to_string(),
        11..=20 => "compound".to_string(),
        21..=30 => "complex".to_string(),
        31..=40 => "nested".to_string(),
        41..=50 => "deep".to_string(),
        51..=60 => "ultra".to_string(),
        61..=70 => "extreme".to_string(),
        _ => "overflow".to_string(),
    }
}
