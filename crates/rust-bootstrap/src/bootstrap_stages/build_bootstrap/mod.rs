use std::error::Error;
use std::env;
//use std::path::PathBuf;
use crate::BuildState;

pub fn build_bootstrap(build_state: &BuildState) -> Result<(), Box<dyn Error>> {
    println!("Building bootstrap");

    let bootstrap_dir = build_state.creation_args.build_dir.join("bootstrap");
    if build_state.creation_args.args.clean && bootstrap_dir.exists() {
        std::fs::remove_dir_all(&bootstrap_dir)?;
    }

    env::set_var("CARGO_TARGET_DIR", &bootstrap_dir);
    env::set_var("RUSTC", build_state.creation_args.stage0.rustc.to_str().unwrap());

    let bin_root = build_state.creation_args.build_dir.join(&build_state.creation_args.build_triple).join("stage0");
    let lib_path = format!("{}/lib", bin_root.to_str().unwrap());

    // Set library paths
    if let Ok(current_ld_library_path) = env::var("LD_LIBRARY_PATH") {
        env::set_var("LD_LIBRARY_PATH", format!("{}{}{}", lib_path, std::path::MAIN_SEPARATOR, current_ld_library_path));
    } else {
        env::set_var("LD_LIBRARY_PATH", &lib_path);
    }
    if let Ok(current_dyld_library_path) = env::var("DYLD_LIBRARY_PATH") {
        env::set_var("DYLD_LIBRARY_PATH", format!("{}{}{}", lib_path, std::path::MAIN_SEPARATOR, current_dyld_library_path));
    } else {
        env::set_var("DYLD_LIBRARY_PATH", &lib_path);
    }
    if let Ok(current_library_path) = env::var("LIBRARY_PATH") {
        env::set_var("LIBRARY_PATH", format!("{}{}{}", lib_path, std::path::MAIN_SEPARATOR, current_library_path));
    } else {
        env::set_var("LIBRARY_PATH", &lib_path);
    }
    if let Ok(current_libpath) = env::var("LIBPATH") {
        env::set_var("LIBPATH", format!("{}{}{}", lib_path, std::path::MAIN_SEPARATOR, current_libpath));
    } else {
        env::set_var("LIBPATH", &lib_path);
    }

    env::set_var("RUSTC_BOOTSTRAP", "1");

    let mut rustflags = env::var("RUSTFLAGS").unwrap_or_else(|_| String::new());

    if rustflags.is_empty() {
        rustflags.push_str("-Zallow-features=");
    }

    rustflags.push_str(" -Wrust_2018_idioms -Wunused_lifetimes");

    // TODO: Implement deny-warnings logic from config.toml
    let deny_warnings = if build_state.creation_args.args.warnings == "default" {
        build_state.creation_args.config.rust.deny_warnings
    } else {
        build_state.creation_args.args.warnings == "deny"
    };

    if deny_warnings {
        rustflags.push_str(" -Dwarnings");
    }

    env::set_var("RUSTFLAGS", rustflags);

    let bootstrap_cargo_toml = build_state.creation_args.rust_root.join("src/bootstrap/Cargo.toml");
    let root_dir_arg = format!("-Zroot-dir={}", build_state.creation_args.rust_root.to_str().unwrap());
    let mut args = vec![
        "build",
        "--manifest-path",
        bootstrap_cargo_toml.to_str().unwrap(),
        &root_dir_arg,
    ];

    if build_state.creation_args.args.verbose > 0 {
        for _ in 0..build_state.creation_args.args.verbose {
            args.push("--verbose");
        }
    }

    if build_state.creation_args.config.build.locked_deps {
        args.push("--locked");
    }
    if build_state.creation_args.config.build.vendor {
        args.push("--frozen");
    }
    if build_state.creation_args.config.build.metrics {
        args.push("--features");
        args.push("build-metrics");
    }
    if build_state.creation_args.args.json_output {
        args.push("--message-format=json");
    }
    match build_state.creation_args.args.color.as_str() {
        "always" => args.push("--color=always"),
        "never" => args.push("--color=never"),
        _ => { /* auto, do nothing */ }
    }

    if env::var("BOOTSTRAP_TRACING").is_ok() {
        args.push("--features");
        args.push("tracing");
    }

    let mut additional_cargo_flags: Vec<String> = Vec::new();
    if let Ok(cargo_flags) = env::var("CARGOFLAGS") {
        for flag in cargo_flags.split_whitespace() {
            additional_cargo_flags.push(flag.to_string());
        }
    }
    let additional_cargo_flags_str: Vec<&str> = additional_cargo_flags.iter().map(|s| s.as_str()).collect();
    args.extend_from_slice(&additional_cargo_flags_str);

    // crate::cargo_integration::run_cargo_command(&["version"], &build_state.creation_args.rust_root)?;

    crate::cargo_integration::run_cargo_command(&args, &build_state.creation_args.rust_root)?;

    Ok(())
}
