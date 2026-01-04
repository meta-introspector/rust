use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🌌 STEP4 BOOTSTRAP: Single macro-driven universe generation");
    
    let instance_path = Path::new("instances/004");
    fs::create_dir_all(instance_path)?;
    
    // Copy basic files
    copy_dir(".", instance_path)?;
    
    // Create the ultimate build.rs that generates everything
    create_ultimate_build_rs(instance_path)?;
    
    // Create minimal Cargo.toml
    let cargo_toml = "[workspace]\n\n[package]\nname = \"split-decls-genesis-step4\"\nversion = \"0.4.0\"\nedition = \"2021\"\n\n[dependencies]\nanyhow = \"1.0\"\n\n[build-dependencies]\nanyhow = \"1.0\"\n";
    fs::write(instance_path.join("Cargo.toml"), cargo_toml)?;
    
    // Setup git for Step4
    Command::new("git").args(&["init"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["checkout", "-b", "step4-single-macro-universe"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["remote", "add", "origin", "https://github.com/meta-introspector/split-decls-rs"]).current_dir(instance_path).status()?;
    
    // Build and test
    let build = Command::new("cargo").args(&["build"]).current_dir(instance_path).status()?;
    if !build.success() {
        anyhow::bail!("Step4 build failed");
    }
    
    // Test the universe
    let test = Command::new("cargo").args(&["run", "--bin", "test_universe"]).current_dir(instance_path).status()?;
    if !test.success() {
        anyhow::bail!("Step4 universe test failed");
    }
    
    // Commit and push
    Command::new("git").args(&["add", "."]).current_dir(instance_path).status()?;
    Command::new("git").args(&["commit", "-m", "🌌 STEP4: Single macro universe generation"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["push", "-u", "origin", "step4-single-macro-universe"]).current_dir(instance_path).status()?;
    
    println!("🎉 STEP4 COMPLETE: Single macro universe generation achieved!");
    Ok(())
}

fn create_ultimate_build_rs(instance_path: &Path) -> Result<()> {
    let build_rs_content = "use std::fs;\nuse anyhow::Result;\n\nfn main() -> Result<()> {\n    println!(\"🌌 GENERATING UNIVERSE: Split-Decls-Genesis-Step4\");\n    \n    // Create all modules and binaries from single function\n    generate_complete_system()?;\n    \n    Ok(())\n}\n\nfn generate_complete_system() -> Result<()> {\n    fs::create_dir_all(\"src\")?;\n    fs::create_dir_all(\"src/bin\")?;\n    \n    // Generate blockchain_macros\n    fs::write(\"src/blockchain_macros.rs\", \"#[macro_export]\\nmacro_rules! mkcontract {\\n    ($addr:literal) => {\\n        pub struct Contract { pub address: &'static str }\\n    };\\n}\")?;\n    \n    // Generate compiler_macros\n    fs::write(\"src/compiler_macros.rs\", \"#[macro_export]\\nmacro_rules! mkrust {\\n    () => {\\n        pub struct RustCompiler { pub version: &'static str }\\n        pub fn create_rust_universe() -> RustCompiler {\\n            RustCompiler { version: \\\"1.84.0-step4\\\" }\\n        }\\n    };\\n}\")?;\n    \n    // Generate test binary\n    fs::write(\"src/bin/test_universe.rs\", \"use split_decls_genesis_step4::mkrust;\\n\\nfn main() {\\n    println!(\\\"🌌 Testing universe...\\\");\\n    mkrust!();\\n    let compiler = create_rust_universe();\\n    println!(\\\"✅ Compiler: {}\\\", compiler.version);\\n    println!(\\\"🎉 Universe test complete!\\\");\\n}\")?;\n    \n    // Generate lib.rs\n    fs::write(\"src/lib.rs\", \"pub mod blockchain_macros;\\npub mod compiler_macros;\\npub use blockchain_macros::*;\\npub use compiler_macros::*;\")?;\n    \n    println!(\"🧬 Generated complete system from single function!\");\n    Ok(())\n}\n";
    
    fs::write(instance_path.join("build.rs"), build_rs_content)?;
    Ok(())
}

fn copy_dir(src: &str, dst: &Path) -> Result<()> {
    if src == dst.to_str().unwrap_or("") {
        return Ok(());
    }
    
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if let Some(name) = entry.file_name().to_str() {
            if name == "target" || name == ".git" || name == "instances" || name == "main.rs" {
                continue;
            }
        }
        
        if src_path.is_dir() {
            copy_dir(&src_path.to_string_lossy(), &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
