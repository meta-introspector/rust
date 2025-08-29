use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::env;

use crate::build_state::BuildState;

pub fn build_bootstrap_cmd(build_state: &BuildState) -> Result<Command, Box<dyn Error>> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    // Set CARGO_TARGET_DIR
    let build_dir = build_state.build_dir.join("bootstrap");
    cmd.env("CARGO_TARGET_DIR", &build_dir);

    // Set RUSTC
    // For now, use the detected rustc path from stage0
    cmd.env("RUSTC", &build_state.stage0.rustc);

    // Set LD_LIBRARY_PATH, DYLD_LIBRARY_PATH, LIBRARY_PATH, LIBPATH
    // This is complex in bootstrap.py, so for now, just set a basic one
    let lib_path = build_state.stage0.rustc.parent().unwrap().join("lib");
    if let Some(path_str) = lib_path.to_str() {
        cmd.env("LD_LIBRARY_PATH", path_str);
        cmd.env("DYLD_LIBRARY_PATH", path_str);
        cmd.env("LIBRARY_PATH", path_str);
        cmd.env("LIBPATH", path_str);
    }

    // Set RUSTC_BOOTSTRAP
    cmd.env("RUSTC_BOOTSTRAP", "1");

    // Set RUSTFLAGS (simplified)
    cmd.env("RUSTFLAGS", "-Zallow-features=");

    // Add --manifest-path
    let manifest_path = build_state.rust_root.join("src").join("bootstrap").join("Cargo.toml");
    cmd.arg("--manifest-path").arg(&manifest_path);

    // Add verbose flags
    for _ in 0..build_state.args.verbose {
        cmd.arg("--verbose");
    }

    // TODO: Add other flags like --locked, --frozen, --features, --message-format, --color, CARGOFLAGS

    Ok(cmd)
}
