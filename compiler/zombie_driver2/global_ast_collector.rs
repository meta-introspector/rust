use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("📊 COLLECTING AST FREQUENCIES FROM ALL SUBMODULES");
    println!("=================================================");

    // Find all analysis files in submodules
    let output = std::process::Command::new("find")
        .args(&["../..", "-name", "*.syn_analysis.json"])
        .output()
        .expect("Failed to find analysis files");

    let analysis_files: Vec<String> =
        String::from_utf8_lossy(&output.stdout).lines().map(|s| s.to_string()).collect();

    println!("Found {} analysis files across all submodules", analysis_files.len());

    let mut global_ast_frequencies: HashMap<String, u32> = HashMap::new();
    let mut total_nodes = 0u64;

    // Process each analysis file
    for (i, file_path) in analysis_files.iter().enumerate() {
        if i % 20 == 0 {
            println!("Processing file {}/{}: {}", i + 1, analysis_files.len(), file_path);
        }

        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                // Extract AST node counts
                if let Some(syn_counts) = json_data["syn_node_counts"].as_object() {
                    for (node_type, count_val) in syn_counts {
                        let count = count_val.as_u64().unwrap_or(0) as u32;
                        *global_ast_frequencies.entry(node_type.clone()).or_insert(0) += count;
                        total_nodes += count as u64;
                    }
                }

                // Also count from json_paths
                if let Some(paths) = json_data["json_paths"].as_array() {
                    for path_entry in paths {
                        if let Some(path_str) = path_entry.as_str() {
                            let segments: Vec<&str> = path_str.split('.').collect();
                            for segment in segments {
                                if !segment.chars().all(|c| c.is_numeric() || c == '[' || c == ']')
                                    && !segment.contains(" = ")
                                {
                                    *global_ast_frequencies
                                        .entry(segment.to_string())
                                        .or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort by frequency
    let mut sorted_frequencies: Vec<(String, u32)> = global_ast_frequencies.into_iter().collect();
    sorted_frequencies.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n🎯 GLOBAL AST FREQUENCY ANALYSIS:");
    println!("==================================");
    println!("Total AST nodes analyzed: {}", total_nodes);
    println!("Unique AST types found: {}", sorted_frequencies.len());

    println!("\n📊 TOP 30 AST FREQUENCIES:");
    println!("===========================");
    for (i, (ast_type, count)) in sorted_frequencies.iter().take(30).enumerate() {
        let percentage = (*count as f64 / total_nodes as f64) * 100.0;
        println!("{:2}. {:20} | {:8} occurrences ({:5.2}%)", i + 1, ast_type, count, percentage);
    }

    // Save complete frequency data
    let frequency_json =
        serde_json::to_string_pretty(&sorted_frequencies).expect("Failed to serialize frequencies");
    fs::write("global_ast_frequencies.json", frequency_json).expect("Failed to write frequencies");

    println!("\n💾 Saved complete frequency data to global_ast_frequencies.json");

    // Find the most dominant AST patterns
    println!("\n🧬 DOMINANT AST PATTERNS:");
    println!("=========================");
    let top_5: Vec<&(String, u32)> = sorted_frequencies.iter().take(5).collect();
    for (ast_type, count) in &top_5 {
        println!("🔹 {}: {} occurrences", ast_type, count);
    }

    println!("\n🎭 THE MATHEMATICAL SIGNATURE OF RUST ACROSS ALL SUBMODULES:");
    println!("============================================================");
    if let Some((dominant_type, dominant_count)) = sorted_frequencies.first() {
        println!("Most frequent AST type: {} ({} occurrences)", dominant_type, dominant_count);
        println!("This represents the core mathematical pattern of Rust's syntax tree structure.");
    }
}
