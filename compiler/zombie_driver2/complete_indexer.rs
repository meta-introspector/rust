use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompleteIndex {
    repos: HashMap<String, CanonicalRepo>,
    files: HashMap<String, FileEntry>,
    total_repos: u32,
    total_files: u32,
    rust_files: u32,
    cargo_projects: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanonicalRepo {
    canonical_url: String,
    name: String,
    forks: Vec<String>,
    files: Vec<String>,
    cargo_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileEntry {
    path: String,
    repo: String,
    file_type: String,
    size: u64,
}

fn main() {
    println!("🗂️  COMPLETE META-INTROSPECTOR INDEX (24 CPU)");
    println!("==============================================");

    let index = Arc::new(Mutex::new(CompleteIndex {
        repos: HashMap::new(),
        files: HashMap::new(),
        total_repos: 0,
        total_files: 0,
        rust_files: 0,
        cargo_projects: 0,
    }));

    let sources = vec![
        "/mnt/data1/nix/vendor/rust",
        "/mnt/data1/meta-introspector",
    ];

    // Load the massive files list (33.9M files)
    if let Ok(content) = fs::read_to_string("/mnt/data1/files.txt") {
        println!("📂 Processing 33.9M files from massive list...");
        let all_lines: Vec<_> = content.lines().collect();
        
        // Filter for Rust files and nested file lists
        let mut rust_files = Vec::new();
        let mut nested_lists = Vec::new();
        
        for line in all_lines {
            if line.ends_with(".rs") {
                rust_files.push(format!("/mnt/data1/{}", line.trim_start_matches("./")));
            } else if line.ends_with("Cargo.toml") {
                rust_files.push(format!("/mnt/data1/{}", line.trim_start_matches("./")));
            } else if line.ends_with(".txt") && (line.contains("files") || line.contains("rust")) {
                nested_lists.push(format!("/mnt/data1/{}", line.trim_start_matches("./")));
            }
        }
        
        println!("🦀 Found {} direct Rust/Cargo files", rust_files.len());
        println!("📋 Found {} nested file lists to process", nested_lists.len());
        
        // Process nested lists for more Rust files
        for list_file in nested_lists {
            if let Ok(nested_content) = fs::read_to_string(&list_file) {
                for nested_line in nested_content.lines() {
                    if nested_line.ends_with(".rs") || nested_line.ends_with("Cargo.toml") {
                        rust_files.push(nested_line.to_string());
                    }
                }
            }
        }
        
        println!("🔥 Total Rust files to process: {}", rust_files.len());
        
        // Process in parallel chunks across 24 CPUs
        let chunk_size = (rust_files.len() / 24).max(1000);
        let mut handles = vec![];
        
        for chunk in rust_files.chunks(chunk_size) {
            let index_clone = Arc::clone(&index);
            let chunk_owned: Vec<String> = chunk.iter().map(|s| s.to_string()).collect();
            
            let handle = thread::spawn(move || {
                for file_path in chunk_owned {
                    add_file_to_index_safe(&index_clone, &file_path);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }

    let final_index = index.lock().unwrap().clone();
    save_complete_index(&final_index);
    create_file_manifest(&final_index);
    
    println!("\n📊 COMPLETE INDEX STATS:");
    println!("Repos: {}", final_index.total_repos);
    println!("Total files: {}", final_index.total_files);
    println!("Rust files: {}", final_index.rust_files);
    println!("Cargo projects: {}", final_index.cargo_projects);
}

fn index_source_parallel(index: &Arc<Mutex<CompleteIndex>>, source_path: &str) {
    println!("🔍 Parallel indexing: {}", source_path);
    
    if let Ok(entries) = fs::read_dir(source_path) {
        let entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        let chunk_size = (entries.len() / 24).max(1);
        let mut handles = vec![];

        for chunk in entries.chunks(chunk_size) {
            let index_clone = Arc::clone(index);
            let paths: Vec<_> = chunk.iter().map(|e| e.path()).collect();
            
            let handle = thread::spawn(move || {
                for path in paths {
                    
                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if !matches!(name, "target" | ".git" | "node_modules" | "build" | "dist" | "common") {
                                let path_str = path.to_string_lossy();
                                if path_str.len() < 150 && !path_str.contains("/common/common") {
                                    if is_repo_root(&path) {
                                        add_repo_to_index_safe(&index_clone, &path);
                                    }
                                    // Recurse one level only to prevent loops
                                    if let Ok(sub_entries) = fs::read_dir(&path) {
                                        for sub_entry in sub_entries.flatten() {
                                            if sub_entry.path().extension().map_or(false, |ext| ext == "rs") {
                                                add_file_to_index_safe(&index_clone, &sub_entry.path().to_string_lossy());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if path.extension().map_or(false, |ext| ext == "rs" || ext == "toml") {
                        add_file_to_index_safe(&index_clone, &path.to_string_lossy());
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn add_repo_to_index_safe(index: &Arc<Mutex<CompleteIndex>>, repo_path: &Path) {
    if let Some(name) = repo_path.file_name().and_then(|n| n.to_str()) {
        let canonical_url = infer_canonical_url(name, repo_path);
        let path_str = repo_path.to_string_lossy().to_string();
        let has_cargo = repo_path.join("Cargo.toml").exists();
        
        let mut index = index.lock().unwrap();
        
        let mut new_repo = false;
        let mut new_fork = false;
        let mut new_cargo = false;
        
        {
            let repo = index.repos.entry(canonical_url.clone()).or_insert_with(|| {
                new_repo = true;
                CanonicalRepo {
                    canonical_url: canonical_url.clone(),
                    name: name.to_string(),
                    forks: Vec::new(),
                    files: Vec::new(),
                    cargo_projects: Vec::new(),
                }
            });

            if !repo.forks.contains(&path_str) {
                repo.forks.push(path_str.clone());
                new_fork = true;
            }

            if has_cargo && !repo.cargo_projects.contains(&path_str) {
                repo.cargo_projects.push(path_str);
                new_cargo = true;
            }
        }
        
        if new_repo || new_fork {
            index.total_repos += 1;
        }
        if new_cargo {
            index.cargo_projects += 1;
        }
    }
}

fn add_file_to_index_safe(index: &Arc<Mutex<CompleteIndex>>, file_path: &str) {
    if let Ok(metadata) = fs::metadata(file_path) {
        let mut index = index.lock().unwrap();
        
        let file_type = if file_path.ends_with(".rs") {
            index.rust_files += 1;
            "rust".to_string()
        } else if file_path.ends_with(".toml") {
            "toml".to_string()
        } else {
            "other".to_string()
        };

        let repo = find_repo_for_file(file_path);
        
        index.files.insert(file_path.to_string(), FileEntry {
            path: file_path.to_string(),
            repo: repo.unwrap_or_else(|| "unknown".to_string()),
            file_type,
            size: metadata.len(),
        });
        
        index.total_files += 1;
    }
}

fn is_repo_root(path: &Path) -> bool {
    path.join(".git").exists() || 
    path.join("Cargo.toml").exists() ||
    path.join("README.md").exists()
}

fn infer_canonical_url(name: &str, path: &Path) -> String {
    if let Ok(config) = fs::read_to_string(path.join(".git/config")) {
        let mut all_urls = Vec::new();
        
        // Collect all remote URLs
        for line in config.lines() {
            if line.contains("url = ") {
                if let Some(url) = line.split("url = ").nth(1) {
                    all_urls.push(url.trim().to_string());
                }
            }
        }
        
        // Prefer GitHub URLs over local paths
        for url in &all_urls {
            if url.contains("github.com") {
                return url.clone();
            }
        }
        
        // If no GitHub URL, return the first non-local URL
        for url in &all_urls {
            if url.starts_with("http") {
                return url.clone();
            }
        }
    }
    
    match name {
        "cargo2nix" => "https://github.com/cargo2nix/cargo2nix".to_string(),
        "rust-overlay" => "https://github.com/oxalica/rust-overlay".to_string(),
        "sccache" => "https://github.com/mozilla/sccache".to_string(),
        _ => format!("https://github.com/unknown/{}", name),
    }
}

fn find_repo_for_file(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    let mut current = path.parent();
    
    while let Some(dir) = current {
        if is_repo_root(dir) {
            return dir.file_name().and_then(|n| n.to_str()).map(|s| s.to_string());
        }
        current = dir.parent();
    }
    None
}

fn save_complete_index(index: &CompleteIndex) {
    let json = serde_json::to_string_pretty(index).unwrap();
    fs::write("/mnt/data1/meta-introspector/complete_index.json", json).unwrap();
    println!("✅ Complete index saved");
}

fn create_file_manifest(index: &CompleteIndex) {
    let mut manifest = String::new();
    manifest.push_str(&format!("# Meta-Introspector Complete File Manifest\n"));
    manifest.push_str(&format!("# Generated: {}\n", chrono::Utc::now().to_rfc3339()));
    manifest.push_str(&format!("# Total repos: {}\n", index.total_repos));
    manifest.push_str(&format!("# Total files: {}\n", index.total_files));
    manifest.push_str(&format!("# Rust files: {}\n", index.rust_files));
    manifest.push_str(&format!("# Cargo projects: {}\n\n", index.cargo_projects));

    for (path, file) in &index.files {
        manifest.push_str(&format!("{}|{}|{}|{}\n", path, file.repo, file.file_type, file.size));
    }

    fs::write("/mnt/data1/meta-introspector/file_manifest.txt", manifest).unwrap();
    println!("✅ File manifest created");
}
