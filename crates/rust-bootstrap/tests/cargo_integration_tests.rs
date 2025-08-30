// tests/cargo_integration_tests.rs

use rust_bootstrap::cargo_integration::init_global_context;
use rust_bootstrap::cargo_integration::init_workspace;
use rust_bootstrap::cargo_integration::parse_cargo_args;
use rust_bootstrap::cargo_integration::run_cargo_command;

use std::path::PathBuf;
use std::fs;

#[test]
fn test_init_global_context() {
    let gctx = init_global_context::init_global_context();
    assert!(gctx.is_ok(), "Failed to initialize GlobalContext: {:?}", gctx.err());
}

#[test]
fn test_init_workspace() {
    let gctx = init_global_context::init_global_context().unwrap();

    // Use the absolute path to the rust repository's Cargo.toml
    let rust_repo_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/rust/");
    let manifest_path = rust_repo_root.join("Cargo.toml");

    let ws = init_workspace::init_workspace(&gctx, &rust_repo_root);
    assert!(ws.is_ok(), "Failed to initialize Workspace: {:?}", ws.err());
}

// #[test]
// fn test_parse_cargo_args() {
//     let gctx = init_global_context::init_global_context().unwrap();

//     // Create a dummy Cargo.toml for testing workspace initialization
//     let temp_dir = PathBuf::from("target/test_parse_args_workspace");
//     fs::create_dir_all(&temp_dir).unwrap();
//     let temp_dir_abs = temp_dir.canonicalize().unwrap(); // Convert to absolute path
//     let manifest_path = temp_dir_abs.join("Cargo.toml");
//     fs::write(&manifest_path, "[package]\nname = \"test-package\"\nversion = \"0.1.0\"\nedition = \"2021\"
// ").unwrap();

//     let args = &["build", "--release"];
//     let compile_options = parse_cargo_args::parse_cargo_args(&gctx, args, &temp_dir_abs); // Pass absolute path
//     assert!(compile_options.is_ok(), "Failed to parse cargo args: {:?}", compile_options.err());

//     // TODO: Add assertions to check the parsed CompileOptions fields

//     // Clean up dummy files
//     fs::remove_dir_all(&temp_dir).unwrap();
// }

// #[test]
// fn test_run_cargo_command_version() {
//     // This test will attempt to run a simple 'cargo version' command
//     // using the integrated library calls.
//     let temp_dir = PathBuf::from("target/test_run_cargo_command_version");
//     fs::create_dir_all(&temp_dir).unwrap();
//     let temp_dir_abs = temp_dir.canonicalize().unwrap(); // Convert to absolute path
//     let manifest_path = temp_dir_abs.join("Cargo.toml");
//     fs::write(&manifest_path, "[package]\nname = \"test-package\"\nversion = \"0.1.0\"\nedition = \"2021\"
// ").unwrap();

//     let args = &["version"];
//     let result = run_cargo_command(args, &temp_dir_abs); // Pass absolute path
//     assert!(result.is_ok(), "Failed to run cargo version command: {:?}", result.err());

//     // TODO: Add assertions to check the output of the command

//     // Clean up dummy files
//     fs::remove_dir_all(&temp_dir).unwrap();
// }
