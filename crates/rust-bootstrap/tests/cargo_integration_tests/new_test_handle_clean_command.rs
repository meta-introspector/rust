#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs;
    use cargo::GlobalContext;
    use cargo::core::Workspace;
    use crate::cargo_integration::subcommands::clean::handle_clean_command;
    use crate::cargo_integration::subcommands::build::handle_build_command;

    // Helper function to create a temporary Cargo project (duplicated for self-containment)
    fn setup_temp_cargo_project(test_name: &str) -> (PathBuf, GlobalContext, Workspace) {
        let temp_dir = PathBuf::from(format!("target/test_cargo_projects/{}", test_name));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let cargo_toml_content = r#"
[package]
name = "test-package"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[[bin]]
name = "test-binary"
path = "src/main.rs"
"#;

        let lib_rs_content = r#"
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
"#;

        let main_rs_content = r#"
fn main() {
    println!("Hello from test-binary!");
    for arg in std::env::args().skip(1) {
        println!("Arg: {}", arg);
    }
}
"#;

        fs::write(temp_dir.join("Cargo.toml"), cargo_toml_content).unwrap();
        fs::create_dir_all(temp_dir.join("src")).unwrap();
        fs::write(temp_dir.join("src/lib.rs"), lib_rs_content).unwrap();
        fs::write(temp_dir.join("src/main.rs"), main_rs_content).unwrap();

        let gctx = GlobalContext::new().unwrap();
        let ws = Workspace::new(&temp_dir.join("Cargo.toml"), &gctx).unwrap();

        (temp_dir, gctx, ws)
    }

    #[test]
    fn test_handle_clean_command() {
        let (temp_dir, gctx, ws) = setup_temp_cargo_project("clean_test");
        let subcommand_args_str: Vec<&str> = vec![];
        let rust_root = PathBuf::from(".");

        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // First, build something to clean
        let build_result = handle_build_command(&gctx, &ws, &vec![], &rust_root);
        assert!(build_result.is_ok(), "Build failed before clean test: {:?}", build_result.err());

        // Verify build artifacts exist before cleaning
        assert!(temp_dir.join("target/debug/test-package.lib").exists());

        let result = handle_clean_command(&gctx, &ws, &subcommand_args_str, &rust_root);
        assert!(result.is_ok(), "handle_clean_command failed: {:?}", result.err());

        // Verify build artifacts are removed after cleaning
        assert!(!temp_dir.join("target/debug/test-package.lib").exists());

        std::env::set_current_dir(&original_cwd).unwrap();
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
