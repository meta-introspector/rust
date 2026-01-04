use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🌌 STEP6 BOOTSTRAP: Clean standalone functions");
    
    let instance_path = Path::new("instances/006");
    fs::create_dir_all(instance_path)?;
    
    // Create build.rs with clean separation
    create_clean_build_rs(instance_path)?;
    create_cargo_toml(instance_path)?;
    
    // Setup git
    Command::new("git").args(&["init"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["checkout", "-b", "step6-clean-functions"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["remote", "add", "origin", "https://github.com/meta-introspector/split-decls-rs"]).current_dir(instance_path).status()?;
    
    // Build and test
    let build = Command::new("cargo").args(&["build"]).current_dir(instance_path).status()?;
    if !build.success() {
        anyhow::bail!("Step6 build failed");
    }
    
    let test = Command::new("cargo").args(&["run", "--bin", "test_universe"]).current_dir(instance_path).status()?;
    if !test.success() {
        anyhow::bail!("Step6 test failed");
    }
    
    // Commit and push
    Command::new("git").args(&["add", "."]).current_dir(instance_path).status()?;
    Command::new("git").args(&["commit", "-m", "🌌 STEP6: Clean standalone functions"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["push", "-u", "origin", "step6-clean-functions"]).current_dir(instance_path).status()?;
    
    println!("🎉 STEP6 COMPLETE: Clean function universe achieved!");
    Ok(())
}

fn create_clean_build_rs(instance_path: &Path) -> Result<()> {
    // Write each content function separately to avoid escaping issues
    write_build_rs_header(instance_path)?;
    write_content_functions(instance_path)?;
    write_macro_and_main(instance_path)?;
    Ok(())
}

fn write_build_rs_header(instance_path: &Path) -> Result<()> {
    let header = "use std::fs;\nuse anyhow::Result;\n\n";
    fs::write(instance_path.join("build.rs"), header)?;
    Ok(())
}

fn write_content_functions(instance_path: &Path) -> Result<()> {
    let functions = r#"// Standalone content functions
fn get_blockchain_content() -> &'static str {
    "pub struct Contract { pub address: &'static str }"
}

fn get_compiler_content() -> &'static str {
    "pub struct RustCompiler { pub version: &'static str }"
}

fn get_test_content() -> &'static str {
    "fn main() { println!(\"Test from clean functions!\"); }"
}

fn get_lib_content() -> &'static str {
    "pub mod blockchain_macros;\npub mod compiler_macros;"
}

"#;
    
    let mut current = fs::read_to_string(instance_path.join("build.rs"))?;
    current.push_str(functions);
    fs::write(instance_path.join("build.rs"), current)?;
    Ok(())
}

fn write_macro_and_main(instance_path: &Path) -> Result<()> {
    let macro_and_main = r#"// Macro that uses the standalone functions
macro_rules! mkuniverse {
    (
        blockchain: $blockchain_fn:ident,
        compiler: $compiler_fn:ident,
        test: $test_fn:ident,
        lib: $lib_fn:ident
    ) => {
        fn main() -> Result<()> {
            println!("🌌 GENERATING FROM CLEAN FUNCTIONS");
            
            fs::create_dir_all("src")?;
            fs::create_dir_all("src/bin")?;
            
            fs::write("src/blockchain_macros.rs", $blockchain_fn())?;
            fs::write("src/compiler_macros.rs", $compiler_fn())?;
            fs::write("src/bin/test_universe.rs", $test_fn())?;
            fs::write("src/lib.rs", $lib_fn())?;
            
            println!("🧬 Generated from clean standalone functions!");
            Ok(())
        }
    };
}

// Generate universe using clean functions
mkuniverse! {
    blockchain: get_blockchain_content,
    compiler: get_compiler_content,
    test: get_test_content,
    lib: get_lib_content
}
"#;
    
    let mut current = fs::read_to_string(instance_path.join("build.rs"))?;
    current.push_str(macro_and_main);
    fs::write(instance_path.join("build.rs"), current)?;
    Ok(())
}

fn create_cargo_toml(instance_path: &Path) -> Result<()> {
    let content = "[workspace]\n\n[package]\nname = \"split-decls-genesis-step6\"\nversion = \"0.6.0\"\nedition = \"2021\"\n\n[build-dependencies]\nanyhow = \"1.0\"\n";
    fs::write(instance_path.join("Cargo.toml"), content)?;
    Ok(())
}
