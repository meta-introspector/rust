
use rust_bootstrap::{Args, BuildState, BuildStateCreationArgs};
use rust_bootstrap::config::loader::Config;
use rust_bootstrap::bootstrap_stages::stage0_detector::Stage0;
use std::path::PathBuf;
use std::fs;
use clap::Parser;

pub fn setup_test_build_state() -> BuildState {
    let rust_root = PathBuf::from("target/test_build_state");
    let build_dir = rust_root.join("build");
    fs::create_dir_all(&build_dir).unwrap();
    let args = Args::parse_from(vec!["rust-bootstrap", "--config", "/path/to/your/bootstrap.toml", "--build-dir", build_dir.to_str().unwrap()]);
    let stage0 = Stage0 {
        rustc: PathBuf::from("/path/to/rustc"),
        cargo: PathBuf::from("/path/to/cargo"),
        compiler_date: "2025-08-30".to_string(), // Added this line
        compiler_version: "1.70.0".to_string(),
        dist_server: "https://dummy.dist.rust-lang.org".to_string(),
    };
    let mut config = Config::default();
    config.build.build = String::from("aarch64-unknown-linux-gnu");

    let build_state_creation_args = BuildStateCreationArgs {
        args: args.clone(),
        rust_root,
        build_dir,
        stage0,
        config,
        build_triple: String::from("aarch64-unknown-linux-gnu"),
        clean: args.clean,
    };

    BuildState::new(build_state_creation_args)
}
