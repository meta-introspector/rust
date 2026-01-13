use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🎯 SIGNATURE-AST CLASSIFIER");
    println!("============================");

    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    let mut class_ast_counts: HashMap<(String, String), u32> = HashMap::new();

    for (path, signature_val) in signatures {
        let signature = signature_val.as_u64().unwrap_or(0) as u128;
        let complexity_class = classify_signature(signature);
        let ast_type = extract_ast_type_from_path(path);

        *class_ast_counts.entry((complexity_class, ast_type)).or_insert(0) += 1;
    }

    // Group by complexity class
    let mut by_class: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    for ((class, ast_type), count) in class_ast_counts {
        by_class.entry(class).or_default().push((ast_type, count));
    }

    for class in ["simple", "compound", "complex", "nested", "deep", "ultra", "extreme"] {
        if let Some(ast_types) = by_class.get(class) {
            println!("\n🔹 {} CLASS:", class.to_uppercase());
            let mut sorted = ast_types.clone();
            sorted.sort_by(|a, b| b.1.cmp(&a.1));

            for (ast_type, count) in sorted.iter().take(10) {
                println!("  {:15} | {:5} paths", ast_type, count);
            }
        }
    }
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

fn extract_ast_type_from_path(path: &str) -> String {
    // Extract AST type from path segments mathematically
    let segments: Vec<&str> = path.split('.').collect();

    // Find the most frequent non-numeric segment as AST type
    let mut type_candidates = HashMap::new();
    for segment in segments {
        if !segment.chars().all(|c| c.is_numeric() || c == '[' || c == ']') {
            *type_candidates.entry(segment).or_insert(0) += 1;
        }
    }

    type_candidates
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(ast_type, _)| ast_type.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
