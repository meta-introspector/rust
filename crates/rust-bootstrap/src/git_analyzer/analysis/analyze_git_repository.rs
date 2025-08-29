use git2::Repository;
use std::error::Error;
use std::io::Write;
use crate::git_analyzer::extractors::{get_all_commits, get_all_blobs, get_all_trees, get_all_tags, get_all_refs};
use crate::parquet_reporter;
use crate::git_analyzer::analysis::git_analysis_summary::GitAnalysisSummary;
use crate::tracer::record_trace_event;
use crate::parquet_reporter::errata_reporter::write_errata_to_parquet;

use std::path::Path;

pub fn analyze_git_repository(repo_path: &str) -> Result<GitAnalysisSummary, Box<dyn Error>> {
    record_trace_event("GitAnalysis", "Starting repository analysis", Some(&format!("repo_path: {}", repo_path)));
    println!("DEBUG: Attempting to analyze repository at path: {}", repo_path);
    std::io::stdout().flush().unwrap();

    let path = Path::new(repo_path);
    record_trace_event("GitAnalysis", "Checking if path exists", Some(&format!("path: {}, exists: {}", repo_path, path.exists())));
    println!("DEBUG: Checking if path exists: {}", path.exists());
    std::io::stdout().flush().unwrap();
    if !path.exists() {
        record_trace_event("GitAnalysis", "Repository path does not exist", Some(&format!("path: {}", repo_path)));
        return Err(format!("Repository path does not exist: {}", repo_path).into());
    }
    record_trace_event("GitAnalysis", "Checking if path is a directory", Some(&format!("path: {}, is_dir: {}", repo_path, path.is_dir())));
    println!("DEBUG: Checking if path is a directory: {}", path.is_dir());
    std::io::stdout().flush().unwrap();
    if !path.is_dir() {
        record_trace_event("GitAnalysis", "Repository path is not a directory", Some(&format!("path: {}", repo_path)));
        return Err(format!("Repository path is not a directory: {}", repo_path).into());
    }

    record_trace_event("GitAnalysis", "Attempting to open repository", Some(&format!("repo_path: {}", repo_path)));
    println!("DEBUG: Attempting Repository::open({})", repo_path);
    std::io::stdout().flush().unwrap();
    let repo = Repository::open(repo_path)?;
    record_trace_event("GitAnalysis", "Successfully opened repository", Some(&format!("repo_path: {}", repo_path)));
    println!("DEBUG: Successfully opened repository.");
    std::io::stdout().flush().unwrap();

    println!("Analyzing Git repository: {}", repo_path);
    std::io::stdout().flush().unwrap();
    let head_id = repo.head()?.peel_to_commit()?.id();
    record_trace_event("GitAnalysis", "Head commit ID", Some(&format!("head_id: {}", head_id)));
    println!("Head: {:?}", head_id);
    std::io::stdout().flush().unwrap();

    let commits_analysis_result = get_all_commits::get_all_commits(&repo)?;
    record_trace_event("GitAnalysis", "Extracted commits", Some(&format!("count: {}", commits_analysis_result.record_batch.num_rows())));
    println!("Extracted {} commits.", commits_analysis_result.record_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let commits_parquet_path = "git_commits.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(commits_analysis_result.record_batch.clone(), &commits_parquet_path)?;
    write_errata_to_parquet(commits_analysis_result.errata, "git_commits_errata.parquet")?;

    let blobs_batch = get_all_blobs::get_all_blobs(&repo)?;
    record_trace_event("GitAnalysis", "Extracted blobs", Some(&format!("count: {}", blobs_batch.num_rows())));
    println!("Extracted {} blobs.", blobs_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let blobs_parquet_path = "git_blobs.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(blobs_batch.clone(), &blobs_parquet_path)?;

    let trees_batch = get_all_trees::get_all_trees(&repo)?;
    record_trace_event("GitAnalysis", "Extracted trees", Some(&format!("count: {}", trees_batch.num_rows())));
    println!("Extracted {} trees.", trees_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let trees_parquet_path = "git_trees.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(trees_batch.clone(), &trees_parquet_path)?;

    let tags_batch = get_all_tags::get_all_tags(&repo)?;
    record_trace_event("GitAnalysis", "Extracted tags", Some(&format!("count: {}", tags_batch.num_rows())));
    println!("Extracted {} tags.", tags_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let tags_parquet_path = "git_tags.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(tags_batch.clone(), &tags_parquet_path)?;

    let refs_batch = get_all_refs::get_all_refs(&repo)?;
    record_trace_event("GitAnalysis", "Extracted refs", Some(&format!("count: {}", refs_batch.num_rows())));
    println!("Extracted {} refs.", refs_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let refs_parquet_path = "git_refs.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(refs_batch.clone(), &refs_parquet_path)?;

    record_trace_event("GitAnalysis", "Finished repository analysis", None);
    Ok(GitAnalysisSummary {
        commits_count: commits_analysis_result.record_batch.num_rows(),
        blobs_count: blobs_batch.num_rows(),
        trees_count: trees_batch.num_rows(),
        tags_count: tags_batch.num_rows(),
        refs_count: refs_batch.num_rows(),
        commits_parquet_path,
        blobs_parquet_path,
        trees_parquet_path,
        tags_parquet_path,
        refs_parquet_path,
    })
}