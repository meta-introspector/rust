use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    // Load original analysis files to extract actual AST items
    let analysis_files: Vec<String> = fs::read_to_string("all_analysis_files.txt")
        .expect("Missing all_analysis_files.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut class_ast_items: HashMap<String, Vec<String>> = HashMap::new();

    for file_path in &analysis_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                extract_items_by_signature(&json_data, signatures, &mut class_ast_items, "");
            }
        }
    }

    // Write compilable files for each class-ast combination
    for (class_ast, items) in &class_ast_items {
        if items.len() > 5 {
            // Only create files with sufficient content
            let filename = format!("rustc_driver_impl/src/lib.{}.rs", class_ast.replace(" ", "_"));
            let content = items.join("\n\n");
            fs::write(&filename, content).expect("Failed to write file");
            println!("📝 Created {} with {} items", filename, items.len());
        }
    }
}

fn extract_items_by_signature(
    json: &Value,
    signatures: &serde_json::Map<String, Value>,
    class_ast_items: &mut HashMap<String, Vec<String>>,
    current_path: &str,
) {
    match json {
        Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", current_path, key)
                };

                // Check if this path has a signature
                if let Some(sig_val) = signatures.get(&new_path) {
                    let signature = sig_val.as_u64().unwrap_or(0) as u128;
                    let complexity_class = classify_signature(signature);
                    let ast_type = extract_ast_type_from_path(&new_path);
                    let class_key = format!("{} {}", complexity_class, ast_type);

                    // Extract the actual Rust code if this is a leaf node with code
                    if let Some(code) = extract_rust_code(value) {
                        class_ast_items.entry(class_key).or_default().push(code);
                    }
                }

                extract_items_by_signature(value, signatures, class_ast_items, &new_path);
            }
        }
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", current_path, i);
                extract_items_by_signature(item, signatures, class_ast_items, &new_path);
            }
        }
        _ => {}
    }
}

fn extract_rust_code(value: &Value) -> Option<String> {
    if let Some(s) = value.as_str() {
        if s.contains("fn ")
            || s.contains("struct ")
            || s.contains("enum ")
            || s.contains("impl ")
            || s.contains("trait ")
            || s.contains("macro ")
        {
            return Some(s.to_string());
        }
    }
    None
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
    let segments: Vec<&str> = path.split('.').collect();
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
