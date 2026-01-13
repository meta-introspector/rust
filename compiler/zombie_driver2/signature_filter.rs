use serde_json;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut target_signature: Option<u128> = None;
    let mut input_file: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--signature" => {
                if i + 1 < args.len() {
                    target_signature = args[i + 1].parse::<u128>().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') => {
                input_file = Some(arg.to_string());
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    if let Some(sig) = target_signature {
        if let Some(file) = input_file {
            println!("🎯 Extracting signature {} from {}", sig, file);
            extract_by_signature(sig, &file);
        }
    } else {
        println!("Usage: spectral-zombie-rustc --signature 70 input.rs");
    }
}

fn extract_by_signature(target_sig: u128, file_path: &str) {
    let signatures_data =
        std::fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json");
    let data: serde_json::Value = serde_json::from_str(&signatures_data).expect("Invalid JSON");
    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    let mut target_paths = Vec::new();
    for (path, sig_val) in signatures {
        let signature = sig_val.as_u64().unwrap_or(0) as u128;
        if signature == target_sig {
            target_paths.push(path.clone());
        }
    }

    println!("// Signature {} extraction from {}", target_sig, file_path);
    println!("// Found {} matching paths", target_paths.len());

    for path in target_paths {
        println!("// Path: {}", path);
    }
}
