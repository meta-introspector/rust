use std::fs;
use std::path::Path;
use anyhow::Result;

/// Self-replication binary - Genesis system creates itself using only Rust
fn main() -> Result<()> {
    // Check if we're already in an instance to prevent infinite loops
    let current_dir = std::env::current_dir()?;
    if current_dir.to_string_lossy().contains("/instances/") {
        println!("🔄 Already in instance directory - skipping recursive self-replication");
        return Ok(());
    }
    
    println!("🧬 GENESIS SELF-REPLICATION: Creating instance-001...");
    
    // Create instance directory
    let instance_path = Path::new("instances/001");
    if instance_path.exists() {
        println!("✅ Instance-001 already exists - skipping creation");
        return Ok(());
    }
    
    fs::create_dir_all(instance_path)?;
    
    // Self-replicate all source files
    replicate_file("Cargo.toml", &instance_path.join("Cargo.toml"))?;
    replicate_file("build.rs", &instance_path.join("build.rs"))?;
    replicate_file("flake.nix", &instance_path.join("flake.nix"))?;
    replicate_file(".gitignore", &instance_path.join(".gitignore"))?;
    replicate_file("Makefile", &instance_path.join("Makefile"))?;
    replicate_file("PLAN.md", &instance_path.join("PLAN.md"))?;
    replicate_file("GENESIS_SUCCESS.md", &instance_path.join("GENESIS_SUCCESS.md"))?;
    
    // Replicate source directory
    replicate_directory("src", &instance_path.join("src"))?;
    replicate_directory("templates", &instance_path.join("templates"))?;
    
    println!("🔧 Testing instance-001 can build itself...");
    
    // Test build in instance directory
    let build_result = std::process::Command::new("cargo")
        .args(&["build"])
        .current_dir(instance_path)
        .status()?;
    
    if !build_result.success() {
        anyhow::bail!("❌ Instance-001 failed to build itself");
    }
    
    // Test run in instance directory
    let run_result = std::process::Command::new("cargo")
        .args(&["run", "--bin", "split-decls-genesis"])
        .current_dir(instance_path)
        .status()?;
    
    if !run_result.success() {
        anyhow::bail!("❌ Instance-001 failed to run itself");
    }
    
    println!("🎉 SELF-REPLICATION SUCCESSFUL!");
    println!("✅ Instance-001 can build and run itself");
    println!("🧬 Genesis system has successfully replicated itself!");
    
    Ok(())
}

/// Replicate a single file
fn replicate_file(source: &str, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(source, dest)?;
    println!("📄 Replicated: {}", source);
    Ok(())
}

/// Replicate an entire directory
fn replicate_directory(source: &str, dest: &Path) -> Result<()> {
    if !Path::new(source).exists() {
        return Ok(());
    }
    
    fs::create_dir_all(dest)?;
    
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        
        if source_path.is_dir() {
            replicate_directory(&source_path.to_string_lossy(), &dest_path)?;
        } else {
            fs::copy(&source_path, &dest_path)?;
        }
    }
    
    println!("📁 Replicated directory: {}", source);
    Ok(())
}
