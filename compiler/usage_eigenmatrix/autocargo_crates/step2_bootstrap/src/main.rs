use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🚀 STEP2 BOOTSTRAP: Automated evolution");
    
    let instance_path = Path::new("instances/002");
    fs::create_dir_all(instance_path)?;
    
    // Copy files
    for file in &["Cargo.toml", "build.rs", "flake.nix", ".gitignore"] {
        if Path::new(file).exists() {
            fs::copy(file, instance_path.join(file))?;
        }
    }
    
    // Copy src directory
    if Path::new("src").exists() {
        copy_dir("src", &instance_path.join("src"))?;
    }
    
    // Setup git
    Command::new("git").args(&["init"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["checkout", "-b", "step2-automated-evolution"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["remote", "add", "origin", "https://github.com/meta-introspector/split-decls-rs"]).current_dir(instance_path).status()?;
    
    // Build and test
    let build = Command::new("cargo").args(&["build"]).current_dir(instance_path).status()?;
    if !build.success() {
        anyhow::bail!("Build failed");
    }
    
    // Commit and push
    Command::new("git").args(&["add", "."]).current_dir(instance_path).status()?;
    Command::new("git").args(&["commit", "-m", "🚀 STEP2: Automated evolution"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["push", "-u", "origin", "step2-automated-evolution"]).current_dir(instance_path).status()?;
    
    println!("🎉 STEP2 COMPLETE!");
    Ok(())
}

fn copy_dir(src: &str, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path.to_string_lossy(), &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
