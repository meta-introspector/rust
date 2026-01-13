use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("📊 RUST AST OBJECT COUNTER");
    println!("==========================");

    // Find all analysis files
    let analysis_files: Vec<String> = fs::read_to_string("rustc_analysis_files.txt")
        .unwrap_or_else(|_| {
            println!("Creating file list...");
            let output = std::process::Command::new("find")
                .args(&[".", "-name", "*.syn_analysis.json"])
                .output()
                .expect("Failed to find analysis files");
            String::from_utf8_lossy(&output.stdout).to_string()
        })
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|s| s.trim_start_matches("./").to_string())
        .collect();

    println!("Found {} analysis files", analysis_files.len());

    let mut total_objects = 0;
    let mut total_paths = 0;
    let mut file_stats = Vec::new();
    let mut global_path_counts: HashMap<String, u32> = HashMap::new();

    for file_path in &analysis_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&content) {
                let mut file_objects = 0;
                let mut file_paths = 0;

                // Count from json_paths array
                if let Some(paths) = json_data.get("json_paths").and_then(|p| p.as_array()) {
                    file_paths = paths.len();
                    total_paths += file_paths;

                    // Count unique objects (normalize paths)
                    for path in paths {
                        if let Some(path_str) = path.as_str() {
                            let normalized = if path_str.contains(" = ") {
                                path_str.split(" = ").next().unwrap_or(path_str)
                            } else {
                                path_str
                            }
                            .replace("items", "path_items");

                            *global_path_counts.entry(normalized).or_insert(0) += 1;
                        }
                    }
                }

                // Count from path_matrix
                if let Some(path_matrix) = json_data.get("path_matrix").and_then(|p| p.as_object())
                {
                    for (path, count) in path_matrix {
                        if let Some(c) = count.as_u64() {
                            file_objects += c as usize;
                            total_objects += c as usize;

                            let normalized = if path.contains(" = ") {
                                path.split(" = ").next().unwrap_or(path)
                            } else {
                                path
                            }
                            .replace("items", "path_items");

                            *global_path_counts.entry(normalized).or_insert(0) += c as u32;
                        }
                    }
                }

                let file_name = file_path.split('/').last().unwrap_or(file_path);
                file_stats.push((file_name.to_string(), file_objects, file_paths));
            }
        }
    }

    // Sort files by object count
    file_stats.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n📈 SUMMARY STATISTICS:");
    println!("======================");
    println!("Total files analyzed: {}", analysis_files.len());
    println!("Total AST objects: {}", total_objects);
    println!("Total unique paths: {}", total_paths);
    println!("Unique path patterns: {}", global_path_counts.len());

    println!("\n🏆 TOP 15 FILES BY OBJECT COUNT:");
    println!("=================================");
    for (i, (file, objects, paths)) in file_stats.iter().take(15).enumerate() {
        println!(
            "{:2}. {:40} | Objects: {:6} | Paths: {:4}",
            i + 1,
            truncate_filename(file, 40),
            objects,
            paths
        );
    }

    // Sort global paths by frequency
    let mut sorted_paths: Vec<(String, u32)> = global_path_counts.into_iter().collect();
    sorted_paths.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n🎯 TOP 20 MOST FREQUENT AST PATTERNS:");
    println!("======================================");
    for (i, (path, count)) in sorted_paths.iter().take(20).enumerate() {
        println!("{:2}. {:50} | Count: {:4}", i + 1, truncate_path(path, 50), count);
    }

    // Analyze by AST node type
    let mut node_type_counts: HashMap<String, u32> = HashMap::new();
    for (path, count) in &sorted_paths {
        let parts: Vec<&str> = path.split('.').collect();
        for part in parts {
            if !part.starts_with("path_items[") && !part.contains('[') {
                *node_type_counts.entry(part.to_string()).or_insert(0) += count;
            }
        }
    }

    let mut sorted_node_types: Vec<(String, u32)> = node_type_counts.into_iter().collect();
    sorted_node_types.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n🌳 TOP 15 AST NODE TYPES:");
    println!("=========================");
    for (i, (node_type, count)) in sorted_node_types.iter().take(15).enumerate() {
        println!("{:2}. {:20} | Total Count: {:6}", i + 1, node_type, count);
    }

    // Distribution analysis
    let avg_objects_per_file = total_objects as f64 / analysis_files.len() as f64;
    let avg_paths_per_file = total_paths as f64 / analysis_files.len() as f64;

    println!("\n📊 DISTRIBUTION ANALYSIS:");
    println!("=========================");
    println!("Average objects per file: {:.1}", avg_objects_per_file);
    println!("Average paths per file: {:.1}", avg_paths_per_file);

    let large_files = file_stats
        .iter()
        .filter(|(_, objects, _)| *objects > avg_objects_per_file as usize * 2)
        .count();
    let small_files = file_stats
        .iter()
        .filter(|(_, objects, _)| *objects < avg_objects_per_file as usize / 2)
        .count();

    println!("Large files (>2x avg): {}", large_files);
    println!("Small files (<0.5x avg): {}", small_files);
    println!("Medium files: {}", analysis_files.len() - large_files - small_files);
}

fn truncate_filename(filename: &str, max_len: usize) -> String {
    if filename.len() <= max_len {
        filename.to_string()
    } else {
        format!("...{}", &filename[filename.len() - max_len + 3..])
    }
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - max_len + 3..])
    }
}
