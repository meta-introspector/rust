// crates/rust-bootstrap/tests/cargo_integration/test_run_cargo_command_dispatch.rs

use rust_bootstrap::cargo_integration::run_cargo_command;

use std::fs;
use std::path::PathBuf;

#[test]
fn test_run_cargo_command_dispatch() {
    // This test will attempt to run a simple 'cargo check' command
    // using the integrated library calls and verify subcommand dispatch.
    let temp_dir = PathBuf::from("target/test_run_cargo_command_dispatch");
    let _ = fs::remove_dir_all(&temp_dir); // Clean up previous run
    fs::create_dir_all(&temp_dir).unwrap();
    let temp_dir_abs = temp_dir.canonicalize().unwrap(); // Convert to absolute path
    let manifest_path = temp_dir_abs.join("Cargo.toml");
    fs::write(&manifest_path, "[package]\nname = \"test-package\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n\n[workspace]\n").unwrap(); // Added [dependencies] to make it a valid Cargo.toml

    // Create a dummy src/main.rs for the test package
    fs::create_dir_all(temp_dir_abs.join("src")).unwrap();
    fs::write(temp_dir_abs.join("src/main.rs"), "fn main() { println!(\"Hello, world!\"); }").unwrap();

    let args = &["check"]; // Test 'cargo check'
    let result = run_cargo_command(args, &temp_dir_abs); // Pass absolute path
    assert!(result.is_ok(), "Failed to run cargo check command: {:?}", result.err());

    // Clean up dummy files
    fs::remove_dir_all(&temp_dir).unwrap();
}
