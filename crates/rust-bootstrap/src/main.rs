mod config;
mod parquet_reporter;
mod git_analyzer;
mod build_state;
mod command_executor;
mod bootstrap_stages;

use crate::config::args::Args;
use crate::bootstrap_stages::config_loader as loader; // Use the correct loader
use crate::build_state::BuildState;
use clap::Parser;
use std::error::Error;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Parsed arguments: {:?}", args);

    let config = loader::load_config(
        args.config.as_deref().unwrap_or("bootstrap.toml")
    )?;

    let stage0 = crate::bootstrap_stages::stage0_detector::Stage0::detect();

    let rust_root = std::env::current_dir()?;
    let build_dir = rust_root.join("build"); // Placeholder for now

    let build_state = BuildState::new(
        args,
        rust_root,
        build_dir,
        stage0,
        config,
        String::from("x86_64-unknown-linux-gnu"), // Default build triple
    );

    // Placeholder for download_and_setup_toolchain
    // bootstrap_stages::toolchain_downloader::download_and_setup_toolchain(&build_state)?;

    // Placeholder for build_bootstrap
    // bootstrap_stages::build_bootstrap::build_bootstrap(&build_state)?;

    // Placeholder for execute_and_report_command
    // let command_result = bootstrap_stages::command_executor::execute_and_report_command(&build_state.stage0)?;
    // bootstrap_stages::process_build_metrics::process_build_metrics(command_result)?;

    // Initiate Git analysis
    let repo_path = build_state.args.repo_path.clone().unwrap_or_else(|| {
        "/data/data/com.termux/files/home/storage/github/rust".to_string()
    });
    let git_analysis_summary = crate::git_analyzer::analysis::analyze_git_repository::analyze_git_repository(&repo_path)?;
    crate::parquet_reporter::write_git_analysis_summary_to_parquet(git_analysis_summary, "git_analysis_summary.parquet")?;
    crate::parquet_reporter::read_and_summarize_git_analysis_metrics("git_analysis_summary.parquet")?;

    // Write build config to parquet
    crate::parquet_reporter::write_build_config_to_parquet(&build_state, "build_config.parquet")?;
    crate::parquet_reporter::read_and_summarize_build_config_metrics("build_config.parquet")?;

    let operational_log_batch = crate::bootstrap_stages::operational_logger::logger::get_logged_events_as_record_batch()?;
    crate::parquet_reporter::write_record_batch_to_parquet(operational_log_batch, "operational_log.parquet")?;

    println!("Git analysis and reporting complete.");

    Ok(())
}
