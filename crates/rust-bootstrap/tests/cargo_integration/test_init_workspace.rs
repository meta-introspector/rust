// crates/rust-bootstrap/tests/cargo_integration/test_init_workspace.rs

use rust_bootstrap::cargo_integration::init_global_context;
use rust_bootstrap::cargo_integration::init_workspace;

use std::path::PathBuf;
use tracing;

#[test]
fn test_init_workspace() {
    let gctx = init_global_context::init_global_context().unwrap();

    // Use the absolute path to the rust repository's Cargo.toml
    let rust_repo_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/rust/");
    let manifest_path = rust_repo_root.join("Cargo.toml");
    tracing::debug!("Manifest path: {:?}", manifest_path);

    let ws = init_workspace::init_workspace(&gctx, &rust_repo_root);
    assert!(ws.is_ok(), "Failed to initialize Workspace: {:?}", ws.err());
}
