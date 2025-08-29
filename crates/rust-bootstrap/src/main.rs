mod parquet_reporter;
mod bootstrap_stages;
mod main_stages;
mod git_analyzer;
mod build_state;

use crate::build_state::BuildState;
use crate::main_stages::download_and_setup_toolchain::download_and_setup_toolchain;
use crate::main_stages::build_bootstrap::build_bootstrap; // Import build_bootstrap
use crate::main_stages::run_bootstrap; // Import run_bootstrap

// use crate::bootstrap_stages::operational_logger::logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_bootstrap::run_bootstrap()?;
    Ok(())
}