use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use rust_bootstrap::git_analyzer::extractors::get_all_commits::{get_all_commits, GitAnalysisResult};
use rust_bootstrap::parquet_reporter::errata_reporter::write_errata_to_parquet;
use rust_bootstrap::config::args::Args; // Assuming Args is defined in config/args.rs and can be reused

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let repo_path = args.repo_path.clone().unwrap_or_else(|| {
        "/data/data/com.termux/files/home/storage/github/rust".to_string()
    });

    // Set current directory to the repository root before analysis
    std::env::set_current_dir(&PathBuf::from(repo_path.clone()))?;

    let git_analysis_result = get_all_commits(&git2::Repository::open(&repo_path)?)?;
        rust_bootstrap::parquet_reporter::write_record_batch_to_parquet(git_analysis_result.record_batch.clone(), "git_commits.parquet")?; // Write record_batch
        write_errata_to_parquet(git_analysis_result.errata.clone(), "git_errata.parquet")?; // Write errata

    let git_analysis_summary = rust_bootstrap::git_analyzer::analysis::analyze_git_repository(&git2::Repository::open(&repo_path)?, git_analysis_result, "git_commits.parquet".to_string())?;
    rust_bootstrap::parquet_reporter::write_git_analysis_summary_to_parquet(git_analysis_summary, "git_analysis_summary.parquet")?;
    rust_bootstrap::parquet_reporter::read_and_summarize_git_analysis_metrics("git_analysis_summary.parquet")?;

    println!("Git analysis and reporting complete.");

    Ok(())
}
