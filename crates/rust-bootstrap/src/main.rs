
mod parquet_reporter;
mod bootstrap_stages;
mod main_stages;
mod git_analyzer;
mod build_state;

 // Import build_bootstrap
use crate::main_stages::run_bootstrap; // Import run_bootstrap

// use crate::bootstrap_stages::operational_logger::logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_bootstrap::run_bootstrap()?;
    Ok(())
}