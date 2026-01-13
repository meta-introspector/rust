use std::collections::HashMap;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RepoSummary {
    canonical_url: String,
    name: String,
    total_files: u32,
    rust_files: u32,
    cargo_files: u32,
    file_sizes: Vec<u64>,
}

fn main() {
    println!("🔍 CODE FINDER - Meta-Introspector Index");
    println!("========================================");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <search_term>", args[0]);
        println!("Examples:");
        println!("  ./code_finder serde          # Find all serde-related repos");
        println!("  ./code_finder rust-lang      # Find all rust-lang repos");
        println!("  ./code_finder tokio          # Find tokio ecosystem");
        return;
    }

    let search_term = &args[1];
    
    // Load master index
    match fs::read_to_string("/mnt/data1/meta-introspector/master_canonical_index.json") {
        Ok(content) => {
            match serde_json::from_str::<HashMap<String, RepoSummary>>(&content) {
                Ok(index) => {
                    search_and_display(&index, search_term);
                }
                Err(e) => println!("❌ Failed to parse index: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to read index: {}", e),
    }
}

fn search_and_display(index: &HashMap<String, RepoSummary>, search_term: &str) {
    let mut matches = Vec::new();
    let mut total_files = 0;
    let mut total_rust_files = 0;

    for (url, repo) in index {
        if repo.name.contains(search_term) || url.contains(search_term) {
            matches.push((url, repo));
            total_files += repo.total_files;
            total_rust_files += repo.rust_files;
        }
    }

    matches.sort_by(|a, b| b.1.rust_files.cmp(&a.1.rust_files));

    println!("🎯 Found {} repositories matching '{}'", matches.len(), search_term);
    println!("📊 Total: {} files ({} Rust files)", total_files, total_rust_files);
    println!();

    for (i, (url, repo)) in matches.iter().take(20).enumerate() {
        let avg_size = if repo.file_sizes.is_empty() { 0 } else { 
            repo.file_sizes.iter().sum::<u64>() / repo.file_sizes.len() as u64 
        };
        
        println!("{}. {} ({} Rust files, avg {}KB)", 
                 i + 1, repo.name, repo.rust_files, avg_size / 1024);
        println!("   📍 {}", url);
        
        // Show canonical form path
        let canonical_path = url.strip_prefix("https://").unwrap_or(url);
        println!("   📂 /mnt/data1/meta-introspector/canonical-forms/{}", canonical_path);
        println!();
    }

    if matches.len() > 20 {
        println!("... and {} more repositories", matches.len() - 20);
    }

    // Show ecosystem stats
    let github_count = matches.iter().filter(|(url, _)| url.contains("github.com")).count();
    let crates_count = matches.iter().filter(|(url, _)| url.contains("crates.io")).count();
    
    println!("🏗️  Ecosystem breakdown:");
    println!("   GitHub: {} repos", github_count);
    println!("   Crates.io: {} repos", crates_count);
}
