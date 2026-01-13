use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    // Load one analysis file to extract examples
    let analysis_files: Vec<String> = fs::read_to_string("all_analysis_files.txt")
        .expect("Missing all_analysis_files.txt")
        .lines()
        .take(5) // Just first 5 files for examples
        .map(|s| s.to_string())
        .collect();

    let mut class_examples: HashMap<String, String> = HashMap::new();

    for file_path in &analysis_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                extract_examples(&json_data, signatures, &mut class_examples, "");
            }
        }
    }

    // Write one example file per class
    for (class, example) in &class_examples {
        let filename = format!("rustc_driver_impl/src/lib.{}.rs", class);
        fs::write(&filename, example).expect("Failed to write file");
        println!("📝 Created {} with example", filename);
    }
}

fn extract_examples(
    json: &Value,
    signatures: &serde_json::Map<String, Value>,
    class_examples: &mut HashMap<String, String>,
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

                if let Some(sig_val) = signatures.get(&new_path) {
                    let signature = sig_val.as_u64().unwrap_or(0) as u128;
                    let complexity_class = classify_signature(signature);

                    // Only add if we don't have an example for this class yet
                    if !class_examples.contains_key(&complexity_class) {
                        if let Some(code) = find_rust_code_in_subtree(value) {
                            class_examples.insert(complexity_class, code);
                        }
                    }
                }

                extract_examples(value, signatures, class_examples, &new_path);
            }
        }
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", current_path, i);
                extract_examples(item, signatures, class_examples, &new_path);
            }
        }
        _ => {}
    }
}

fn find_rust_code_in_subtree(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => {
            if s.len() > 20
                && (s.contains("fn ")
                    || s.contains("struct ")
                    || s.contains("enum ")
                    || s.contains("impl ")
                    || s.contains("pub ")
                    || s.contains("use "))
            {
                Some(format!("// Example extracted code\n{}", s))
            } else {
                None
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                if let Some(code) = find_rust_code_in_subtree(v) {
                    return Some(code);
                }
            }
            None
        }
        Value::Array(arr) => {
            for item in arr {
                if let Some(code) = find_rust_code_in_subtree(item) {
                    return Some(code);
                }
            }
            None
        }
        _ => None,
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
