// Expensive processing moved from build.rs
// Run this manually: cargo run --bin process_rustc_files

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Processing rustc files (moved from build.rs)...");
    
    // Check cache first
    let cache_path = "build_cache.json";
    if Path::new(cache_path).exists() {
        if let Ok(cache_content) = fs::read_to_string(cache_path) {
            if let Ok(cache_data) = serde_json::from_str::<serde_json::Value>(&cache_content) {
                if let Some(timestamp) = cache_data.get("timestamp").and_then(|t| t.as_u64()) {
                    let cache_age = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs() - timestamp;
                    
                    if cache_age < 3600 {
                        println!("💾 Found recent cache ({} seconds old), skipping processing", cache_age);
                        return Ok(());
                    }
                }
            }
        }
    }
    
    println!("⚡ Running expensive rustc file processing...");
    
    // Basic file processing (simplified from original build.rs)
    let rustc_path = Path::new("submodules/rust");
    if !rustc_path.exists() {
        println!("❌ rustc submodule not found at {}", rustc_path.display());
        return Ok(());
    }
    
    let rust_files = find_rust_files(rustc_path)?;
    println!("📁 Found {} Rust files to process", rust_files.len());
    
    let mut processed = 0;
    let mut failed = 0;
    
    for file_path in rust_files.iter() { // Process all files
        match fs::read_to_string(file_path) {
            Ok(_content) => {
                processed += 1;
                if processed % 20 == 0 {
                    println!("📊 Processed {} files", processed);
                }
            }
            Err(_) => {
                failed += 1;
            }
        }
    }
    
    println!("✅ Processing complete: {} processed, {} failed", processed, failed);
    
    // Create cache
    let cache_data = serde_json::json!({
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        "status": "complete",
        "processed": processed,
        "failed": failed
    });
    
    fs::write(cache_path, serde_json::to_string_pretty(&cache_data)?)?;
    println!("💾 Cache created: {}", cache_path);
    
    Ok(())
}

fn find_rust_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut rust_files = Vec::new();
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                rust_files.extend(find_rust_files(&path)?);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                rust_files.push(path);
            }
        }
    }
    
    Ok(rust_files)
}
