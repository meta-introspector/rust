use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🌟 STEP5 BOOTSTRAP: Macro parameter-driven universe");
    
    let instance_path = Path::new("instances/005");
    fs::create_dir_all(instance_path)?;
    
    // Copy basic files
    copy_dir(".", instance_path)?;
    
    // Create the macro parameter build.rs
    create_build_rs(instance_path)?;
    create_cargo_toml(instance_path)?;
    
    // Setup git
    Command::new("git").args(&["init"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["checkout", "-b", "step5-macro-parameters"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["remote", "add", "origin", "https://github.com/meta-introspector/split-decls-rs"]).current_dir(instance_path).status()?;
    
    // Build and test
    let build = Command::new("cargo").args(&["build"]).current_dir(instance_path).status()?;
    if !build.success() {
        anyhow::bail!("Step5 build failed");
    }
    
    let test = Command::new("cargo").args(&["run", "--bin", "test_universe"]).current_dir(instance_path).status()?;
    if !test.success() {
        anyhow::bail!("Step5 test failed");
    }
    
    // Commit and push
    Command::new("git").args(&["add", "."]).current_dir(instance_path).status()?;
    Command::new("git").args(&["commit", "-m", "🌟 STEP5: Macro parameter universe"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["push", "-u", "origin", "step5-macro-parameters"]).current_dir(instance_path).status()?;
    
    println!("🎉 STEP5 COMPLETE: Macro parameter universe achieved!");
    Ok(())
}

fn create_build_rs(instance_path: &Path) -> Result<()> {
    let build_content = get_build_content();
    fs::write(instance_path.join("build.rs"), build_content)?;
    Ok(())
}

fn get_build_content() -> String {
    r#"use std::fs;
use anyhow::Result;

macro_rules! mkuniverse {
    (
        blockchain: $blockchain:literal,
        compiler: $compiler:literal,
        test: $test:literal,
        lib: $lib:literal
    ) => {
        fn main() -> Result<()> {
            println!("🌟 GENERATING FROM MACRO PARAMETERS");
            
            fs::create_dir_all("src")?;
            fs::create_dir_all("src/bin")?;
            
            fs::write("src/blockchain_macros.rs", $blockchain)?;
            fs::write("src/compiler_macros.rs", $compiler)?;
            fs::write("src/bin/test_universe.rs", $test)?;
            fs::write("src/lib.rs", $lib)?;
            
            println!("🧬 Generated complete system from macro parameters!");
            Ok(())
        }
    };
}

mkuniverse! {
    blockchain: "pub struct Contract { pub address: &'static str }",
    compiler: "pub struct RustCompiler { pub version: &'static str }",
    test: "fn main() { println!(\"Test from macro param!\"); }",
    lib: "// Generated lib from macro parameter"
}
"#.to_string()
}

fn create_cargo_toml(instance_path: &Path) -> Result<()> {
    let content = "[workspace]\n\n[package]\nname = \"split-decls-genesis-step5\"\nversion = \"0.5.0\"\nedition = \"2021\"\n\n[dependencies]\nanyhow = \"1.0\"\n\n[build-dependencies]\nanyhow = \"1.0\"\n";
    fs::write(instance_path.join("Cargo.toml"), content)?;
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
