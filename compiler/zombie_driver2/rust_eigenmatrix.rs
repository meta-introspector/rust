use nalgebra::{DMatrix, DVector};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    println!("🧮 RUST EIGENMATRIX GENERATOR");
    println!("==============================");

    // Collect all JSON analysis files
    let mut all_features = HashMap::new();
    let mut file_vectors = Vec::new();
    let mut file_names = Vec::new();

    // Read analysis file paths from list
    if let Ok(paths_content) = fs::read_to_string("analysis_files.txt") {
        for path in paths_content.lines() {
            let path = std::path::PathBuf::from(path.trim());
            if path.exists() {
                collect_analysis_recursive(
                    &path,
                    &mut all_features,
                    &mut file_vectors,
                    &mut file_names,
                );
            }
        }
    }

    println!("📊 Found {} files with {} unique features", file_names.len(), all_features.len());

    if file_names.is_empty() {
        println!("❌ No analysis files found");
        return;
    }

    // Create feature matrix
    let feature_list: Vec<String> = all_features.keys().cloned().collect();
    let n_files = file_names.len();
    let n_features = feature_list.len();

    println!("🔢 Building {}x{} feature matrix", n_files, n_features);

    let mut matrix_data = vec![0.0; n_files * n_features];

    for (file_idx, file_features) in file_vectors.iter().enumerate() {
        for (feature, count) in file_features {
            if let Some(feature_idx) = feature_list.iter().position(|f| f == feature) {
                matrix_data[file_idx * n_features + feature_idx] = *count as f64;
            }
        }
    }

    let matrix = DMatrix::from_vec(n_files, n_features, matrix_data);

    // Compute covariance matrix and eigenvalues
    println!("🧮 Computing eigendecomposition of Rust codebase...");

    let covariance = &matrix.transpose() * &matrix;
    let eigen = covariance.symmetric_eigen();

    println!("\n🎯 RUST EIGENMATRIX RESULTS");
    println!("============================");
    println!("Matrix dimensions: {}x{}", n_files, n_features);
    println!("Top 10 eigenvalues (Rust's fundamental modes):");

    let eigenvalues = eigen.eigenvalues;
    let mut sorted_indices: Vec<usize> = (0..eigenvalues.len()).collect();
    sorted_indices.sort_by(|&a, &b| eigenvalues[b].partial_cmp(&eigenvalues[a]).unwrap());

    for (i, &idx) in sorted_indices.iter().take(10).enumerate() {
        println!("  λ{}: {:.2e}", i + 1, eigenvalues[idx]);
    }

    // Save eigenmatrix
    let eigen_data = format!(
        r#"{{
  "files": {:?},
  "features": {:?},
  "eigenvalues": {:?},
  "matrix_shape": [{}, {}],
  "total_variance": {},
  "explained_variance_ratio": {:?}
}}"#,
        file_names,
        feature_list,
        eigenvalues.as_slice(),
        n_files,
        n_features,
        eigenvalues.sum(),
        sorted_indices
            .iter()
            .take(10)
            .map(|&i| eigenvalues[i] / eigenvalues.sum())
            .collect::<Vec<_>>()
    );

    fs::write("rust_eigenmatrix.json", eigen_data).unwrap();
    println!("\n💾 Saved Rust eigenmatrix to rust_eigenmatrix.json");
    println!("🎉 Rust codebase mathematically decomposed!");
}

fn collect_analysis_recursive(
    path: &Path,
    all_features: &mut HashMap<String, usize>,
    file_vectors: &mut Vec<HashMap<String, u32>>,
    file_names: &mut Vec<String>,
) {
    if path.is_file() {
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();
            if (name_str.contains("analysis") || name_str.contains("zombie"))
                && name_str.ends_with(".json")
            {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        extract_features(&json, all_features, file_vectors, file_names, path);
                    }
                }
            }
        }
    } else if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                collect_analysis_recursive(&entry.path(), all_features, file_vectors, file_names);
            }
        }
    }
}

fn extract_features(
    json: &Value,
    all_features: &mut HashMap<String, usize>,
    file_vectors: &mut Vec<HashMap<String, u32>>,
    file_names: &mut Vec<String>,
    path: &Path,
) {
    let mut features = HashMap::new();

    // Extract from different analysis formats
    if let Some(obj) = json.as_object() {
        // Character analysis
        if let Some(char_freq) = obj.get("char_frequencies") {
            if let Some(freq_obj) = char_freq.as_object() {
                for (char, count) in freq_obj {
                    let feature = format!("char_{}", char);
                    if let Some(c) = count.as_u64() {
                        features.insert(feature.clone(), c as u32);
                        all_features.insert(feature, all_features.len());
                    }
                }
            }
        }

        // Syn analysis
        if let Some(syn_counts) = obj.get("syn_node_counts") {
            if let Some(counts_obj) = syn_counts.as_object() {
                for (node_type, count) in counts_obj {
                    let feature = format!("syn_{}", node_type);
                    if let Some(c) = count.as_u64() {
                        features.insert(feature.clone(), c as u32);
                        all_features.insert(feature, all_features.len());
                    }
                }
            }
        }

        // Path matrix with normalization and subpaths
        if let Some(path_matrix) = obj.get("path_matrix") {
            if let Some(matrix_obj) = path_matrix.as_object() {
                for (path_pattern, count) in matrix_obj {
                    // Normalize path - remove value assignments
                    let normalized = if path_pattern.contains(" = ") {
                        path_pattern.split(" = ").next().unwrap_or(path_pattern)
                    } else {
                        path_pattern
                    }
                    .replace("items", "path_items");

                    if let Some(c) = count.as_u64() {
                        // Add full path
                        *features.entry(normalized.clone()).or_insert(0) += c as u32;
                        all_features.insert(normalized.clone(), all_features.len());

                        // Add all subpaths
                        let parts: Vec<&str> = normalized.split('.').collect();
                        let mut current_path = String::new();
                        for (i, part) in parts.iter().enumerate() {
                            if i > 0 {
                                current_path.push('.');
                            }
                            current_path.push_str(part);

                            *features.entry(current_path.clone()).or_insert(0) += c as u32;
                            all_features.insert(current_path.clone(), all_features.len());
                        }
                    }
                }
            }
        }
    }

    if !features.is_empty() {
        file_vectors.push(features);
        file_names.push(path.to_string_lossy().to_string());
    }
}
