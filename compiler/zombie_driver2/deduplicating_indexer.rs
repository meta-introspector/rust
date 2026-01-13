use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RepoSummary {
    canonical_url: String,
    name: String,
    total_files: u32,
    rust_files: u32,
    cargo_files: u32,
    file_sizes: Vec<u64>,
    analysis_results: HashMap<String, serde_json::Value>,
}

fn main() {
    println!("🔄 DEDUPLICATING INDEXER -> CANONICAL FORMS");
    println!("==========================================");

    let processed_files = Arc::new(Mutex::new(HashSet::new()));
    let repo_summaries = Arc::new(Mutex::new(HashMap::new()));

    // Load and deduplicate all files
    println!("📂 Reading /mnt/data1/files.txt...");
    match fs::read("/mnt/data1/files.txt") {
        Ok(bytes) => {
            let content = String::from_utf8_lossy(&bytes);
            println!("✅ File loaded, {} total lines", content.lines().count());
            
            let all_files: Vec<_> = content.lines()
                .filter(|line| line.ends_with(".rs") || line.ends_with("Cargo.toml"))
                .map(|line| {
                    if line.starts_with("./") {
                        format!("/mnt/data1{}", &line[1..])
                    } else if line.starts_with("/") {
                        line.to_string()
                    } else {
                        format!("/mnt/data1/{}", line)
                    }
                })
                .collect();

            println!("🦀 Found {} Rust/Cargo files", all_files.len());
            
            if all_files.len() > 0 {
                println!("🔍 Sample files:");
                for sample in all_files.iter().take(3) {
                    println!("  {}", sample);
                }
            }

            // Process the rest of the function inside the Ok block
            // Deduplicate and process in parallel
            let chunk_size = (all_files.len() / 24).max(1000);
            let mut handles = vec![];

            for chunk in all_files.chunks(chunk_size) {
                let processed_clone = Arc::clone(&processed_files);
                let summaries_clone = Arc::clone(&repo_summaries);
                let chunk_owned: Vec<String> = chunk.to_vec();

                let handle = thread::spawn(move || {
                    for file_path in chunk_owned {
                        // Deduplicate
                        {
                            let mut processed = processed_clone.lock().unwrap();
                            if processed.contains(&file_path) {
                                continue;
                            }
                            processed.insert(file_path.clone());
                        }

                        // Process file and flow to canonical repo
                        if let Ok(metadata) = fs::metadata(&file_path) {
                            let repo_name = extract_repo_name(&file_path);
                            let canonical_url = infer_canonical_url(&repo_name);
                            
                            let mut summaries = summaries_clone.lock().unwrap();
                            let summary = summaries.entry(canonical_url.clone()).or_insert_with(|| {
                                RepoSummary {
                                    canonical_url: canonical_url.clone(),
                                    name: repo_name.clone(),
                                    total_files: 0,
                                    rust_files: 0,
                                    cargo_files: 0,
                                    file_sizes: Vec::new(),
                                    analysis_results: HashMap::new(),
                                }
                            });

                            summary.total_files += 1;
                            summary.file_sizes.push(metadata.len());
                            
                            if file_path.ends_with(".rs") {
                                summary.rust_files += 1;
                            } else if file_path.ends_with("Cargo.toml") {
                                summary.cargo_files += 1;
                            }
                        }
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        }
        Err(e) => {
            println!("❌ Failed to read /mnt/data1/files.txt: {}", e);
            return;
        }
    }

    // Flow results to canonical forms
    let final_summaries = repo_summaries.lock().unwrap();
    let processed_count = processed_files.lock().unwrap().len();
    
    println!("✅ Processed {} unique files", processed_count);
    println!("📊 Generated {} repo summaries", final_summaries.len());

    // Create canonical directory structure and flow results
    for (canonical_url, summary) in final_summaries.iter() {
        create_canonical_form(summary);
    }

    // Save master index
    let master_index = serde_json::to_string_pretty(&*final_summaries).unwrap();
    fs::write("/mnt/data1/meta-introspector/master_canonical_index.json", master_index).unwrap();
    
    println!("🏗️  All results flowed to canonical forms in meta-introspector");
}

fn extract_repo_name(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut current = path.parent();
    
    while let Some(dir) = current {
        if dir.join(".git").exists() {
            return dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
        }
        current = dir.parent();
    }
    
    // Fallback: extract from path
    path.components()
        .nth(3) // Skip /mnt/data1/
        .and_then(|c| c.as_os_str().to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn infer_canonical_url(repo_name: &str) -> String {
    // First try to get URL from git config
    if let Some(url) = get_git_url_from_config(repo_name) {
        return url;
    }
    
    // Load git analysis results if available
    if let Ok(content) = fs::read_to_string("git_structure_analysis.json") {
        if let Ok(analysis) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(repos) = analysis.get("repositories").and_then(|r| r.as_array()) {
                for repo in repos {
                    if let Some(name) = repo.get("name").and_then(|n| n.as_str()) {
                        if name == repo_name {
                            if let Some(url) = repo.get("remote_url").and_then(|u| u.as_str()) {
                                return url.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to pattern matching
    match repo_name {
        "cargo2nix" => "https://github.com/cargo2nix/cargo2nix".to_string(),
        "sccache" => "https://github.com/mozilla/sccache".to_string(),
        "rustc" | "rust" => "https://github.com/rust-lang/rust".to_string(),
        name if name.starts_with("rust-") => format!("https://github.com/rust-lang/{}", name),
        _ => format!("https://crates.io/crates/{}", repo_name),
    }
}

fn get_git_url_from_config(repo_name: &str) -> Option<String> {
    // Try common locations for git config
    let possible_paths = vec![
        format!("/mnt/data1/nix/vendor/rust/{}/.git/config", repo_name),
        format!("/mnt/data1/meta-introspector/{}/.git/config", repo_name),
        format!("/mnt/data1/{}/.git/config", repo_name),
    ];
    
    for config_path in possible_paths {
        if let Ok(config) = fs::read_to_string(&config_path) {
            for line in config.lines() {
                if line.trim().starts_with("url = ") {
                    if let Some(url) = line.split("url = ").nth(1) {
                        let clean_url = url.trim().trim_matches('"');
                        if clean_url.contains("github.com") || clean_url.contains("gitlab.com") {
                            return Some(clean_url.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn create_canonical_form(summary: &RepoSummary) {
    // Extract domain path from canonical URL
    let domain_path = summary.canonical_url
        .strip_prefix("https://")
        .unwrap_or(&summary.canonical_url)
        .replace("/", "/");
    
    let canonical_dir = format!("/mnt/data1/meta-introspector/canonical-forms/{}", domain_path);
    fs::create_dir_all(&canonical_dir).ok();

    // Create summary file
    let summary_json = serde_json::to_string_pretty(summary).unwrap();
    fs::write(format!("{}/summary.json", canonical_dir), summary_json).ok();

    // Create analysis results
    let analysis_dir = format!("{}/analysis", canonical_dir);
    fs::create_dir_all(&analysis_dir).ok();
    
    for (analysis_type, results) in &summary.analysis_results {
        let results_json = serde_json::to_string_pretty(results).unwrap();
        fs::write(format!("{}/{}.json", analysis_dir, analysis_type), results_json).ok();
    }

    // Create metrics file
    let metrics = format!(
        "# Repo Metrics: {}\n\
         Total Files: {}\n\
         Rust Files: {}\n\
         Cargo Files: {}\n\
         Total Size: {} bytes\n\
         Average File Size: {} bytes\n",
        summary.name,
        summary.total_files,
        summary.rust_files,
        summary.cargo_files,
        summary.file_sizes.iter().sum::<u64>(),
        if summary.file_sizes.is_empty() { 0 } else { summary.file_sizes.iter().sum::<u64>() / summary.file_sizes.len() as u64 }
    );
    fs::write(format!("{}/metrics.txt", canonical_dir), metrics).ok();

    println!("📋 Created canonical form: {}", domain_path);
}
