use std::collections::HashSet;
use std::fs;
use std::path::Path;
use serde_json::Value;

fn read_manifest_tree(manifest_path: &str, visited: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    if visited.contains(manifest_path) {
        return Ok(());
    }
    visited.insert(manifest_path.to_string());
    
    let content = fs::read_to_string(manifest_path)?;
    let manifest: Value = serde_json::from_str(&content)?;
    
    let crate_name = manifest["crate_name"].as_str().unwrap_or("unknown");
    let total_usages = manifest["total_usages"].as_u64().unwrap_or(0);
    let files_count = manifest["generated_files"].as_array().map(|a| a.len()).unwrap_or(0);
    
    println!("📦 {}: {} usages, {} files", crate_name, total_usages, files_count);
    
    // Read dependency manifests
    if let Some(deps) = manifest["dependency_manifests"].as_array() {
        for dep in deps {
            if let Some(dep_path) = dep.as_str() {
                let base_dir = Path::new(manifest_path).parent().unwrap();
                let full_path = base_dir.join(dep_path);
                if full_path.exists() {
                    read_manifest_tree(full_path.to_str().unwrap(), visited)?;
                }
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_manifest = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/test_usage_data/working_usage_collector_manifest.json";
    
    if !Path::new(root_manifest).exists() {
        eprintln!("❌ Root manifest not found: {}", root_manifest);
        return Ok(());
    }
    
    println!("🔍 Reading manifest dependency tree...");
    let mut visited = HashSet::new();
    read_manifest_tree(root_manifest, &mut visited)?;
    
    println!("\n📊 Summary: {} total manifests processed", visited.len());
    Ok(())
}
