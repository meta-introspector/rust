use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🧪 RUST CRATE SIGNATURE ANALYSIS");
    println!("=================================");

    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    // Load analysis files to get crate info
    let analysis_files: Vec<String> = fs::read_to_string("all_analysis_files.txt")
        .expect("Missing all_analysis_files.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut crate_signatures: HashMap<String, HashMap<u128, u32>> = HashMap::new();
    let mut crate_paths: HashMap<String, Vec<String>> = HashMap::new();

    // Process each analysis file
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

                                *crate_signatures
                                    .entry(crate_name.clone())
                                    .or_default()
                                    .entry(signature)
                                    .or_insert(0) += 1;

                                crate_paths
                                    .entry(crate_name.clone())
                                    .or_default()
                                    .push(clean_path.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Display crate rollup
    println!("📦 CRATE COVERAGE:");
    println!("==================");

    for (crate_name, sig_map) in &crate_signatures {
        let total_paths = crate_paths.get(crate_name).map(|v| v.len()).unwrap_or(0);
        let unique_sigs = sig_map.len();

        println!("\n🔹 {}", crate_name);
        println!("   Total paths: {}", total_paths);
        println!("   Unique signatures: {}", unique_sigs);

        // Show top 5 signatures for this crate
        let mut sorted_sigs: Vec<(u128, u32)> = sig_map.iter().map(|(&k, &v)| (k, v)).collect();
        sorted_sigs.sort_by(|a, b| b.1.cmp(&a.1));

        println!("   Top signatures:");
        for (sig, count) in sorted_sigs.iter().take(5) {
            let class = classify_signature(*sig);
            println!("     Sig {:2} ({:9}): {:3} paths", sig, class, count);
        }
    }

    // Overall signature distribution
    println!("\n🎯 OVERALL SIGNATURE DISTRIBUTION:");
    println!("===================================");

    let mut global_sigs: HashMap<u128, u32> = HashMap::new();
    for sig_map in crate_signatures.values() {
        for (&sig, &count) in sig_map {
            *global_sigs.entry(sig).or_insert(0) += count;
        }
    }

    let mut sorted_global: Vec<(u128, u32)> = global_sigs.into_iter().collect();
    sorted_global.sort_by_key(|(sig, _)| *sig);

    println!("┌─────┬───────────┬───────┐");
    println!("│ Sig │   Class   │ Count │");
    println!("├─────┼───────────┼───────┤");

    for (signature, count) in &sorted_global {
        let class = classify_signature(*signature);
        println!("│ {:3} │ {:9} │ {:5} │", signature, class, count);
    }

    println!("└─────┴───────────┴───────┘");

    println!("\nTotal crates analyzed: {}", crate_signatures.len());
}

fn extract_crate_name(file_path: &str) -> String {
    // Extract crate name from path like "./target/release/build/serde-b041d4b6aa60a94f/build.syn_analysis.json"
    if let Some(build_part) = file_path.split("/build/").nth(1) {
        if let Some(crate_part) = build_part.split('-').next() {
            return crate_part.to_string();
        }
    }

    // Fallback to filename
    file_path.split('/').last().unwrap_or("unknown").to_string()
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
