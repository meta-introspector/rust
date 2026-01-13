use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    let mut class_examples: HashMap<String, Vec<String>> = HashMap::new();

    // Process first analysis file to get examples
    let first_file = "./target/release/build/serde-b041d4b6aa60a94f/build.syn_analysis.json";
    if let Ok(content) = fs::read_to_string(first_file) {
        if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
            if let Some(paths) = json_data["json_paths"].as_array() {
                for path_entry in paths {
                    if let Some(path_str) = path_entry.as_str() {
                        if let Some(sig_val) = signatures.get(path_str) {
                            let signature = sig_val.as_u64().unwrap_or(0) as u128;
                            let complexity_class = classify_signature(signature);

                            // Extract code from path if it contains assignment
                            if path_str.contains(" = ") {
                                let code =
                                    path_str.split(" = ").nth(1).unwrap_or("").trim_matches('"');
                                if !code.is_empty() && code.len() > 3 {
                                    class_examples
                                        .entry(complexity_class)
                                        .or_default()
                                        .push(code.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Create one example file per class
    for (class, examples) in &class_examples {
        if !examples.is_empty() {
            let filename = format!("rustc_driver_impl/src/lib.{}.rs", class);
            let content = format!(
                "// Example {} class code\n// Extracted from AST analysis\n\n{}",
                class, examples[0]
            );
            fs::write(&filename, content).expect("Failed to write file");
            println!(
                "📝 Created {} with example: {}",
                filename,
                &examples[0][..examples[0].len().min(50)]
            );
        }
    }

    println!("\n🎯 EXTRACTED EXAMPLES BY CLASS:");
    for (class, examples) in &class_examples {
        println!("  {}: {} examples", class, examples.len());
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
