use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🌟 STEP3 BOOTSTRAP: Advanced evolution with compiler integration");
    
    let instance_path = Path::new("instances/003");
    fs::create_dir_all(instance_path)?;
    
    // Copy all files from current directory
    copy_dir(".", instance_path)?;
    
    // Add advanced features by copying actual source files
    add_compiler_integration(instance_path)?;
    add_nix_flake_enhancement(instance_path)?;
    
    // Setup git for Step3
    Command::new("git").args(&["init"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["checkout", "-b", "step3-compiler-integration"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["remote", "add", "origin", "https://github.com/meta-introspector/split-decls-rs"]).current_dir(instance_path).status()?;
    
    // Build and validate
    let build = Command::new("cargo").args(&["build"]).current_dir(instance_path).status()?;
    if !build.success() {
        anyhow::bail!("Step3 build failed");
    }
    
    // Commit and push
    Command::new("git").args(&["add", "."]).current_dir(instance_path).status()?;
    Command::new("git").args(&["commit", "-m", "🌟 STEP3: Advanced compiler integration"]).current_dir(instance_path).status()?;
    Command::new("git").args(&["push", "-u", "origin", "step3-compiler-integration"]).current_dir(instance_path).status()?;
    
    println!("🎉 STEP3 COMPLETE: Compiler integration achieved!");
    Ok(())
}

fn add_compiler_integration(instance_path: &Path) -> Result<()> {
    // Copy the actual source files instead of generating strings
    fs::copy("src/airdrop.rs", instance_path.join("src/airdrop.rs"))?;
    fs::copy("src/compiler_macros.rs", instance_path.join("src/compiler_macros.rs"))?;
    fs::copy("src/blockchain_macros.rs", instance_path.join("src/blockchain_macros.rs"))?;
    
    fs::create_dir_all(instance_path.join("src/bin"))?;
    fs::copy("src/bin/test_compiler.rs", instance_path.join("src/bin/test_compiler.rs"))?;
    
    // Create build.rs that generates lib.rs with module declarations (the key manual fix)
    let build_rs = r#"use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    println!("🧬 GENESIS BUILD SYSTEM ACTIVATED");
    
    let system_code = "//! # Split-Decls-Genesis: Pure Macro System\n\npub mod airdrop;\npub mod blockchain_macros;\npub mod compiler_macros;\n\npub use blockchain_macros::*;\npub use compiler_macros::*;\n\n#[macro_export]\nmacro_rules! mknix {\n    () => {\n        // NIX environment setup\n    };\n}\n\n#[macro_export]\nmacro_rules! mkgit {\n    ($repo:expr) => {\n        // Git repository configuration\n    };\n}\n\n#[macro_export]\nmacro_rules! mkdeclfn {\n    ($vis:vis fn $name:ident($($args:tt)*) -> $ret:ty $body:block) => {\n        $vis fn $name($($args)*) -> $ret {\n            println!(\"🔧 EXECUTING: {}\", stringify!($name));\n            $body\n        }\n    };\n}\n\n#[macro_export]\nmacro_rules! mksystem {\n    () => {\n        mknix!();\n        mkgit!(\"split-decls-genesis\");\n        \n        mkdeclfn! {\n            pub fn run_system() -> anyhow::Result<()> {\n                println!(\"🚀 SYSTEM: Complete macro-driven system running!\");\n                Ok(())\n            }\n        }\n    };\n}\n\nmksystem!();";

    fs::write("src/lib.rs", system_code)?;
    
    println!("✨ Generated complete macro-driven system!");
    Ok(())
}"#;
    
    fs::write(instance_path.join("build.rs"), build_rs)?;
    
    Ok(())
}

fn add_nix_flake_enhancement(instance_path: &Path) -> Result<()> {
    let enhanced_flake = r#"{
  description = "Step3 Genesis - Advanced Compiler Integration";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  
  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "genesis-step3";
        version = "0.3.0";
        src = ./.;
        cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        
        buildInputs = with pkgs; [ llvm ];
        
        buildPhase = ''
          echo "🌟 Building Step3 Genesis with compiler integration..."
          cargo build --release
          mkdir -p $out/bin
          cp target/release/* $out/bin/ || true
        '';
      };
      
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          cargo
          git
          nix
          llvm
          clang
        ];
        
        shellHook = ''
          echo "🌟 Step3 Genesis Environment Ready"
          echo "Enhanced with: Full compiler toolchain + LLVM + Advanced macros"
          echo "Run: cargo run --bin step3_bootstrap"
        '';
      };
    };
}"#;
    
    fs::write(instance_path.join("flake.nix"), enhanced_flake)?;
    Ok(())
}

fn copy_dir(src: &str, dst: &Path) -> Result<()> {
    if src == dst.to_str().unwrap_or("") {
        return Ok(()); // Skip self-copy
    }
    
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        // Skip target directories and git
        if let Some(name) = entry.file_name().to_str() {
            if name == "target" || name == ".git" || name == "instances" {
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
