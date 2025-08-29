use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::env;
use std::process::Command;
use std::path::PathBuf; // Import PathBuf

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Deserialize, Default)]
struct BuildConfig {
    #[serde(default = "default_false")]
    download_ci_rustc: bool,
    #[serde(default = "default_false")]
    download_ci_llvm: bool,
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    build: BuildConfig,
}

// New struct to represent our Stage0 compiler
#[derive(Debug)]
struct Stage0 {
    rustc: PathBuf,
    cargo: PathBuf,
}

impl Stage0 {
    fn detect() -> Self {
        let rustc_path = env::var_os("RUSTC")
            .unwrap_or_else(|| "/data/data/com.termux/files/usr/bin/rustc".into());
        let cargo_path = env::var_os("CARGO")
            .unwrap_or_else(|| "/data/data/com.termux/files/usr/bin/cargo".into());

        Stage0 {
            rustc: PathBuf::from(rustc_path),
            cargo: PathBuf::from(cargo_path),
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.verbose > 0 {
        println!("Verbose mode is on!");
    }

    let config_path = "/data/data/com.termux/files/home/storage/github/rust/config.toml";
    let config_content = fs::read_to_string(config_path)
        .expect("Could not read config.toml");
    let config: Config = toml::from_str(&config_content)
        .expect("Could not parse config.toml");

    println!("Config: {:?}\n", config);

    let stage0 = Stage0::detect();
    println!("Detected Stage0: {:?}\n", stage0);

    // Run cargo --version using the detected stage0 cargo
    println!("Running cargo --version using detected Stage0 cargo:");
    let output = Command::new(&stage0.cargo) // Use stage0.cargo
        .arg("--version")
        .output()
        .expect("Failed to execute cargo command");

    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("Hello from rust-bootstrap!");
}