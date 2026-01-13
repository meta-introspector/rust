use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🧬 CRATE EIGENVALUE ANALYSIS");
    println!("============================");

    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    // Load analysis files
    let analysis_files: Vec<String> = fs::read_to_string("all_analysis_files.txt")
        .expect("Missing all_analysis_files.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut crate_eigenvalues: HashMap<String, f64> = HashMap::new();
    let mut crate_signatures: HashMap<String, Vec<u128>> = HashMap::new();

    // Calculate eigenvalue for each crate
    for file_path in &analysis_files {
        let crate_name = extract_crate_name(file_path);

        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                if let Some(paths) = json_data["json_paths"].as_array() {
                    let mut crate_sigs = Vec::new();

                    for path_entry in paths {
                        if let Some(path_str) = path_entry.as_str() {
                            let clean_path = path_str.split(" = ").next().unwrap_or(path_str);

                            if let Some(sig_val) = signatures.get(clean_path) {
                                let signature = sig_val.as_u64().unwrap_or(0) as u128;
                                crate_sigs.push(signature);
                            }
                        }
                    }

                    if !crate_sigs.is_empty() {
                        // Calculate eigenvalue: weighted average with soul resonance
                        let soul_signature = 38u128;
                        let eigenvalue = crate_sigs
                            .iter()
                            .map(|&sig| {
                                let distance_from_soul =
                                    ((sig as f64) - (soul_signature as f64)).abs();
                                let resonance = 1.0 / (1.0 + distance_from_soul / 10.0);
                                (sig as f64) * resonance
                            })
                            .sum::<f64>()
                            / crate_sigs.len() as f64;

                        crate_eigenvalues.insert(crate_name.clone(), eigenvalue);
                        crate_signatures.insert(crate_name, crate_sigs);
                    }
                }
            }
        }
    }

    // Sort by eigenvalue (closest to soul)
    let mut sorted_crates: Vec<(String, f64)> = crate_eigenvalues.clone().into_iter().collect();
    sorted_crates.sort_by(|a, b| {
        let soul_distance_a = (a.1 - 38.0).abs();
        let soul_distance_b = (b.1 - 38.0).abs();
        soul_distance_a.partial_cmp(&soul_distance_b).unwrap()
    });

    println!("\n🎯 CRATE EIGENVALUES (sorted by soul resonance):");
    println!("================================================");

    for (i, (crate_name, eigenvalue)) in sorted_crates.iter().take(20).enumerate() {
        let soul_distance = (eigenvalue - 38.0).abs();
        let resonance_strength = 1.0 / (1.0 + soul_distance);

        println!(
            "{:2}. {:25} | Eigenvalue: {:6.2} | Soul distance: {:5.2} | Resonance: {:.4}",
            i + 1,
            crate_name,
            eigenvalue,
            soul_distance,
            resonance_strength
        );
    }

    // Find most soul-resonant crate
    if let Some((soul_crate, soul_eigenvalue)) = sorted_crates.first() {
        println!("\n🌟 MOST SOUL-RESONANT CRATE:");
        println!("=============================");
        println!("Crate: {}", soul_crate);
        println!("Eigenvalue: {:.6}", soul_eigenvalue);
        println!("Soul distance: {:.6}", (soul_eigenvalue - 38.0).abs());

        if let Some(sigs) = crate_signatures.get(soul_crate) {
            println!("Signature count: {}", sigs.len());
            println!(
                "Signature range: {} - {}",
                sigs.iter().min().unwrap_or(&0),
                sigs.iter().max().unwrap_or(&0)
            );
        }
    }

    // Now analyze our custom scripts
    println!("\n🔧 CUSTOM SCRIPT ANALYSIS:");
    println!("==========================");

    let custom_scripts = [
        "rust_soul_finder.rs",
        "eigenmatrix_resonance.rs",
        "crate_signature_analysis.rs",
        "signature_ast_classifier.rs",
        "positional_prime_encoder.rs",
        "spectral_zombie_rustc.rs",
        "signature_filter.rs",
    ];

    for script in &custom_scripts {
        if let Some(eigenvalue) = crate_eigenvalues.get(&script.replace(".rs", "")) {
            let soul_distance = (eigenvalue - 38.0).abs();
            println!(
                "📜 {:25} | Eigenvalue: {:6.2} | Soul distance: {:5.2}",
                script, eigenvalue, soul_distance
            );
        }
    }
}

fn extract_crate_name(file_path: &str) -> String {
    if let Some(build_part) = file_path.split("/build/").nth(1) {
        if let Some(crate_part) = build_part.split('-').next() {
            return crate_part.to_string();
        }
    }

    // Extract from filename for our custom scripts
    if let Some(filename) = file_path.split('/').last() {
        if filename.ends_with(".syn_analysis.json") {
            return filename.replace(".syn_analysis.json", "");
        }
    }

    file_path.split('/').last().unwrap_or("unknown").to_string()
}
