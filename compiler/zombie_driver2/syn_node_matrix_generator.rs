use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🔢 SYN NODE TYPE MATRIX GENERATOR");
    println!("=================================");

    // Load all analysis files
    let analysis_files: Vec<String> = fs::read_to_string("rustc_analysis_files.txt")
        .expect("Need rustc analysis files")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut node_type_counts: HashMap<String, u32> = HashMap::new();
    let mut field_counts: HashMap<String, u32> = HashMap::new();
    let mut path_patterns: HashMap<String, u32> = HashMap::new();

    println!("📊 Processing {} analysis files...", analysis_files.len());

    for file_path in &analysis_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                if let Some(paths) = json_data.get("json_paths").and_then(|p| p.as_array()) {
                    for path in paths {
                        if let Some(path_str) = path.as_str() {
                            // Skip value assignments, focus on structure
                            if path_str.contains(" = ") {
                                continue;
                            }

                            let parts: Vec<&str> = path_str.split('.').collect();

                            // Count each node type in the path
                            for part in &parts {
                                if !part.starts_with("items[") && !part.contains('[') {
                                    *node_type_counts.entry(part.to_string()).or_insert(0) += 1;
                                }
                            }

                            // Count field patterns (node.field combinations)
                            for window in parts.windows(2) {
                                if !window[0].contains('[') && !window[1].contains('[') {
                                    let field_pattern = format!("{}.{}", window[0], window[1]);
                                    *field_counts.entry(field_pattern).or_insert(0) += 1;
                                }
                            }

                            // Count full path patterns (depth-based)
                            let depth = parts.len();
                            let pattern_key = format!("depth_{}", depth);
                            *path_patterns.entry(pattern_key).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    // Generate prime numbers for encoding
    let primes = generate_primes(1000);

    println!("\n🎯 SYN NODE TYPE ENUMERATION:");
    println!("=============================");

    // Sort node types by frequency
    let mut sorted_nodes: Vec<(String, u32)> = node_type_counts.into_iter().collect();
    sorted_nodes.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (node_type, count)) in sorted_nodes.iter().take(50).enumerate() {
        let prime = primes[i];
        let weight = (*count as f64).log2();
        println!(
            "{:2}. {:15} | Prime: {:3} | Count: {:6} | Weight: {:.2}",
            i + 1,
            node_type,
            prime,
            count,
            weight
        );
    }

    println!("\n🔗 TOP FIELD PATTERNS:");
    println!("======================");

    let mut sorted_fields: Vec<(String, u32)> = field_counts.into_iter().collect();
    sorted_fields.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (field_pattern, count)) in sorted_fields.iter().take(20).enumerate() {
        println!("{:2}. {:30} | Count: {:5}", i + 1, field_pattern, count);
    }

    println!("\n📏 PATH DEPTH DISTRIBUTION:");
    println!("===========================");

    let mut sorted_depths: Vec<(String, u32)> = path_patterns.into_iter().collect();
    sorted_depths.sort_by(|a, b| {
        let a_depth: u32 = a.0.strip_prefix("depth_").unwrap().parse().unwrap_or(0);
        let b_depth: u32 = b.0.strip_prefix("depth_").unwrap().parse().unwrap_or(0);
        a_depth.cmp(&b_depth)
    });

    for (depth_key, count) in &sorted_depths {
        println!("{:15} | Count: {:6}", depth_key, count);
    }

    // Save matrix data
    let matrix_data = serde_json::json!({
        "node_types": sorted_nodes.into_iter().take(50).enumerate().map(|(i, (name, count))| {
            serde_json::json!({
                "index": i,
                "name": name,
                "prime": primes[i],
                "count": count,
                "weight": (count as f64).log2()
            })
        }).collect::<Vec<_>>(),
        "field_patterns": sorted_fields.into_iter().take(20).collect::<Vec<_>>(),
        "depth_distribution": sorted_depths.clone()
    });

    fs::write("syn_node_matrix.json", serde_json::to_string_pretty(&matrix_data).unwrap())
        .expect("Failed to save matrix");

    println!("\n💾 Syn node matrix saved to syn_node_matrix.json");
    println!("🎯 Ready for prime-encoded feature extraction!");
}

fn generate_primes(limit: usize) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..limit {
        if is_prime[i] {
            primes.push(i as u32);
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    primes
}
