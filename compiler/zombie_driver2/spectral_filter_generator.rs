use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🎛️ SPECTRAL FILTER GENERATOR");
    println!("============================");

    // Load frequency analysis from rustc data
    let analysis_files: Vec<String> = fs::read_to_string("rustc_analysis_files.txt")
        .expect("Need rustc analysis files")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut global_node_counts: HashMap<String, u32> = HashMap::new();

    // Collect all node type frequencies
    for file_path in &analysis_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                if let Some(paths) = json_data.get("json_paths").and_then(|p| p.as_array()) {
                    for path in paths {
                        if let Some(path_str) = path.as_str() {
                            // Extract node type from path
                            let parts: Vec<&str> = path_str.split('.').collect();
                            for part in parts {
                                if !part.starts_with("path_items[")
                                    && !part.contains('[')
                                    && !part.contains('=')
                                {
                                    *global_node_counts.entry(part.to_string()).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort by frequency (descending)
    let mut sorted_classes: Vec<(String, u32)> = global_node_counts.into_iter().collect();
    sorted_classes.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n🔍 DISCOVERED SPECTRAL CLASSES:");
    println!("===============================");

    // Generate spectral filter mapping
    let mut filter_map = HashMap::new();
    for (i, (class_name, count)) in sorted_classes.iter().take(20).enumerate() {
        let frequency = i as f64 * 0.05; // 0.0, 0.05, 0.10, 0.15, ...
        filter_map.insert(class_name.clone(), frequency);

        println!("{:2}. {:15} | Count: {:8} | Filter: {:.2}", i + 1, class_name, count, frequency);
    }

    // Generate filter commands
    println!("\n🎵 SPECTRAL COMPILATION COMMANDS:");
    println!("=================================");
    for (i, (class_name, _)) in sorted_classes.iter().take(10).enumerate() {
        let frequency = i as f64 * 0.05;
        println!(
            "zombie-rustc --filter={:.2} input.rs  # {} filter ({} objects)",
            frequency, class_name, sorted_classes[i].1
        );
    }

    // Save filter mapping
    let filter_json = serde_json::to_string_pretty(&filter_map).unwrap();
    fs::write("spectral_filters.json", filter_json).expect("Failed to save filters");

    println!("\n💾 Spectral filter mapping saved to spectral_filters.json");
    println!("🎯 Ready for frequency-driven compilation!");
}
