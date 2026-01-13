use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanonicalRepo {
    canonical_url: String,  // https://github.com/cargo2nix/cargo2nix
    name: String,
    forks: Vec<RepoFork>,
    crates_io: Option<String>,  // crates.io name if different
    last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RepoFork {
    fork_path: String,
    fork_type: String,  // "local", "fork", "vendor"
    commit_hash: Option<String>,
    modifications: Vec<String>,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanonicalStructure {
    repos: HashMap<String, CanonicalRepo>,  // keyed by canonical URL
    total_repos: u32,
    total_forks: u32,
}

// Known canonical mappings
fn get_canonical_mappings() -> HashMap<String, String> {
    let mut mappings = HashMap::new();
    mappings.insert("cargo2nix".to_string(), "https://github.com/cargo2nix/cargo2nix".to_string());
    mappings.insert("rust-overlay".to_string(), "https://github.com/oxalica/rust-overlay".to_string());
    mappings.insert("sccache".to_string(), "https://github.com/mozilla/sccache".to_string());
    mappings.insert("just".to_string(), "https://github.com/casey/just".to_string());
    mappings
}

fn main() {
    println!("🏗️  CANONICAL STRUCTURE BUILDER (TLD-based)");
    println!("==========================================");

    let mut structure = CanonicalStructure {
        repos: HashMap::new(),
        total_repos: 0,
        total_forks: 0,
    };

    let canonical_mappings = get_canonical_mappings();
    
    // Scan our local sources and map to canonical URLs
    let sources = vec![
        "/mnt/data1/nix/vendor/rust",
        "/mnt/data1/meta-introspector",
    ];

    for source in sources {
        scan_and_canonicalize(&mut structure, source, &canonical_mappings);
    }

    // Create TLD-based canonical structure
    create_tld_structure(&structure);
    
    // Save structure
    save_structure(&structure);
    
    println!("\n📊 CANONICAL STRUCTURE COMPLETE");
    println!("Canonical repos: {}, Total forks: {}", 
             structure.total_repos, structure.total_forks);
}

fn scan_and_canonicalize(
    structure: &mut CanonicalStructure, 
    source_path: &str, 
    mappings: &HashMap<String, String>
) {
    println!("🔍 Scanning and canonicalizing: {}", source_path);
    
    if let Ok(entries) = fs::read_dir(source_path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if !matches!(name, "target" | ".git" | "node_modules") {
                        let fork_path = entry.path().to_string_lossy().to_string();
                        
                        // Get canonical URL
                        let canonical_url = mappings.get(name)
                            .cloned()
                            .unwrap_or_else(|| infer_canonical_url(name));

                        let repo = structure.repos.entry(canonical_url.clone()).or_insert_with(|| {
                            CanonicalRepo {
                                canonical_url: canonical_url.clone(),
                                name: name.to_string(),
                                forks: Vec::new(),
                                crates_io: check_crates_io(name),
                                last_updated: chrono::Utc::now().to_rfc3339(),
                            }
                        });

                        // Add this as a fork/local copy
                        repo.forks.push(RepoFork {
                            fork_path: fork_path.clone(),
                            fork_type: determine_fork_type(&fork_path),
                            commit_hash: get_git_commit(&entry.path()),
                            modifications: detect_modifications(&entry.path()),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                }
            }
        }
    }
}

fn infer_canonical_url(name: &str) -> String {
    // Try to infer from common patterns
    if name.contains("rust") || name.contains("cargo") {
        format!("https://github.com/rust-lang/{}", name)
    } else {
        format!("https://github.com/unknown/{}", name)
    }
}

fn check_crates_io(name: &str) -> Option<String> {
    // In a real implementation, we'd check crates.io API
    // For now, assume same name if it looks like a crate
    if name.contains("-") || name.len() < 20 {
        Some(name.to_string())
    } else {
        None
    }
}

fn determine_fork_type(path: &str) -> String {
    if path.contains("vendor") {
        "vendor".to_string()
    } else if path.contains("fork") {
        "fork".to_string()
    } else {
        "local".to_string()
    }
}

fn get_git_commit(path: &Path) -> Option<String> {
    let git_head = path.join(".git/HEAD");
    if let Ok(content) = fs::read_to_string(git_head) {
        if content.starts_with("ref: ") {
            let ref_path = content.trim().strip_prefix("ref: ")?;
            let ref_file = path.join(".git").join(ref_path);
            fs::read_to_string(ref_file).ok().map(|s| s.trim().to_string())
        } else {
            Some(content.trim().to_string())
        }
    } else {
        None
    }
}

fn detect_modifications(path: &Path) -> Vec<String> {
    let mut mods = Vec::new();
    
    // Check for common modification indicators
    if path.join("zombie_driver2").exists() {
        mods.push("zombie_driver2_analysis".to_string());
    }
    if path.join("split-decls").exists() {
        mods.push("split_decls_canonical".to_string());
    }
    if path.join("meta-introspector").exists() {
        mods.push("meta_introspector_integration".to_string());
    }
    
    mods
}

fn create_tld_structure(structure: &CanonicalStructure) {
    println!("🏗️  Creating TLD-based canonical structure...");
    
    let base_path = "/mnt/data1/meta-introspector/canonical-tld";
    fs::create_dir_all(&base_path).ok();

    for (canonical_url, repo) in &structure.repos {
        // Extract domain and path from URL
        if let Some(domain_path) = extract_domain_path(canonical_url) {
            let repo_dir = format!("{}/{}", base_path, domain_path);
            fs::create_dir_all(&repo_dir).ok();
            
            // Create forks directory
            let forks_dir = format!("{}/forks", repo_dir);
            fs::create_dir_all(&forks_dir).ok();
            
            // Link each fork
            for (i, fork) in repo.forks.iter().enumerate() {
                let link_name = format!("{}/{}-{}", forks_dir, fork.fork_type, i);
                std::os::unix::fs::symlink(&fork.fork_path, link_name).ok();
            }

            // Create metadata
            let metadata = serde_json::to_string_pretty(repo).unwrap();
            fs::write(format!("{}/canonical.json", repo_dir), metadata).ok();
        }
    }
}

fn extract_domain_path(url: &str) -> Option<String> {
    if let Some(stripped) = url.strip_prefix("https://") {
        Some(stripped.replace("/", "/"))
    } else {
        None
    }
}

fn save_structure(structure: &CanonicalStructure) {
    let mut final_structure = structure.clone();
    final_structure.total_repos = structure.repos.len() as u32;
    final_structure.total_forks = structure.repos.values()
        .map(|r| r.forks.len() as u32).sum();

    let json = serde_json::to_string_pretty(&final_structure).unwrap();
    fs::write("/mnt/data1/meta-introspector/canonical_tld_structure.json", json).unwrap();
    
    println!("✅ TLD-based canonical structure saved");
}
