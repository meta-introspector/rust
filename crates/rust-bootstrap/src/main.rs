mod config;
mod parquet_reporter;
mod git_analyzer;
mod build_state;
mod command_executor;
mod bootstrap_stages;
use crate::config::args::Args;
use crate::config::loader::load_config;
use clap::Parser;
use std::path::Path;
use crate::command_executor::shell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Parsed arguments: {:?}", args);

    let config_path = args.config.as_deref().unwrap_or("bootstrap.toml");
    if Path::new(config_path).exists() {
        let config = load_config(Path::new(config_path))?;
        println!("Loaded config: {:?}", config);
    } else {
        println!("No config file found.");
    }

    // Test the consolidated command execution function
    let _ = shell::shell("echo Hello from rust-bootstrap!", true)?;

    Ok(())
}
