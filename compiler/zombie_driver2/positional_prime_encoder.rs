use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🔢 POSITIONAL PRIME PATH ENCODER");
    println!("=================================");

    // Load the syn node matrix
    let matrix_content =
        fs::read_to_string("syn_node_matrix.json").expect("Need syn_node_matrix.json");
    let matrix: Value = serde_json::from_str(&matrix_content).unwrap();

    // Build prime lookup table
    let mut prime_map: HashMap<String, u32> = HashMap::new();
    if let Some(node_types) = matrix.get("node_types").and_then(|n| n.as_array()) {
        for node in node_types {
            if let (Some(name), Some(prime)) = (
                node.get("name").and_then(|n| n.as_str()),
                node.get("prime").and_then(|p| p.as_u64()),
            ) {
                prime_map.insert(name.to_string(), prime as u32);
            }
        }
    }

    println!("📊 Loaded {} prime mappings", prime_map.len());

    // Load analysis files
    let analysis_files: Vec<String> = fs::read_to_string("rustc_analysis_files.txt")
        .expect("Need rustc analysis files")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut path_signatures: HashMap<String, u128> = HashMap::new();
    let mut signature_counts: HashMap<u128, u32> = HashMap::new();

    println!("🔍 Processing {} files for positional encoding...", analysis_files.len());

    for file_path in analysis_files.iter().take(10) {
        // Sample first 10 files
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                if let Some(paths) = json_data.get("json_paths").and_then(|p| p.as_array()) {
                    for path in paths {
                        if let Some(path_str) = path.as_str() {
                            // Skip value assignments
                            if path_str.contains(" = ") {
                                continue;
                            }

                            let signature = encode_path_signature(path_str, &prime_map);
                            if signature > 0 {
                                path_signatures.insert(path_str.to_string(), signature);
                                *signature_counts.entry(signature).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n🎯 POSITIONAL PRIME SIGNATURES (mod 71):");
    println!("=========================================");

    // Sort by signature frequency
    let mut sorted_sigs: Vec<(u128, u32)> = signature_counts.into_iter().collect();
    sorted_sigs.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (signature, count)) in sorted_sigs.iter().take(20).enumerate() {
        // Find example path for this signature
        let example_path = path_signatures
            .iter()
            .find(|(_, &sig)| sig == *signature)
            .map(|(path, _)| path.as_str())
            .unwrap_or("unknown");

        // Decode signature back to prime factors (for verification)
        let signature_class = classify_signature(*signature);

        println!(
            "{:2}. Sig: {:2} | Count: {:4} | Class: {:10} | Path: {}",
            i + 1,
            signature,
            count,
            signature_class,
            truncate_path(example_path, 40)
        );
    }

    println!("\n🔬 MODULAR SIGNATURE ANALYSIS:");
    println!("==============================");

    // Analyze signature distribution in mod 71 space
    let mut mod_distribution = vec![0u32; 71];
    for (signature, count) in &sorted_sigs {
        mod_distribution[*signature as usize] += count;
    }

    println!("Signature distribution in Z/71Z:");
    for (i, count) in mod_distribution.iter().enumerate() {
        if *count > 0 {
            println!("  Sig {:2}: {:5} paths", i, count);
        }
    }

    // Find most common signature classes
    let mut class_counts: HashMap<String, u32> = HashMap::new();
    for (signature, count) in &sorted_sigs {
        let class = classify_signature(*signature);
        *class_counts.entry(class).or_insert(0) += count;
    }

    println!("\n🎵 SPECTRAL SIGNATURE CLASSES:");
    println!("==============================");
    let mut sorted_classes: Vec<(String, u32)> = class_counts.into_iter().collect();
    sorted_classes.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (class, count)) in sorted_classes.iter().take(15).enumerate() {
        println!("{:2}. {:15} | Total paths: {:6}", i + 1, class, count);
    }

    println!("\n🔬 SIGNATURE ANALYSIS:");
    println!("======================");

    // Analyze signature distribution
    let total_unique = path_signatures.len();
    let total_paths = sorted_sigs.iter().map(|(_, count)| count).sum::<u32>();
    let unique_signatures = sorted_sigs.len();

    println!("Total paths processed: {}", total_paths);
    println!("Unique path strings: {}", total_unique);
    println!("Unique signatures: {}", unique_signatures);
    println!("Compression ratio: {:.2}%", (unique_signatures as f64 / total_unique as f64) * 100.0);

    // Save signature mapping
    let signature_data = serde_json::json!({
        "path_signatures": path_signatures.iter().take(100).collect::<HashMap<_, _>>(),
        "signature_frequencies": sorted_sigs.into_iter().take(50).collect::<Vec<_>>(),
        "statistics": {
            "total_paths": total_paths,
            "unique_paths": total_unique,
            "unique_signatures": unique_signatures,
            "compression_ratio": (unique_signatures as f64 / total_unique as f64) * 100.0
        }
    });

    fs::write("path_signatures.json", serde_json::to_string_pretty(&signature_data).unwrap())
        .expect("Failed to save signatures");

    println!("\n💾 Path signatures saved to path_signatures.json");
    println!("🎯 Ready for signature-based spectral filtering!");
}

fn encode_path_signature(path: &str, prime_map: &HashMap<String, u32>) -> u128 {
    let parts: Vec<&str> = path.split('.').collect();
    let mut signature: u128 = 1;
    const MOD_VALUE: u128 = 71; // Fun modular arithmetic!

    for (index, part) in parts.iter().enumerate() {
        // Skip array indices and value assignments
        if part.contains('[') || part.contains('=') {
            continue;
        }

        if let Some(&prime) = prime_map.get(*part) {
            // Positional encoding: prime^(position+1) mod 71
            let positional_factor = mod_pow(prime as u128, (index + 1) as u128, MOD_VALUE);
            signature = (signature * positional_factor) % MOD_VALUE;
        }
    }

    signature
}

// Fast modular exponentiation
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - max_len + 3..])
    }
}

fn classify_signature(signature: u128) -> String {
    // Classify signatures based on their mod 71 value
    match signature {
        0 => "null".to_string(),
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
