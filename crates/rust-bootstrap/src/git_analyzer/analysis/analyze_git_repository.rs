use git2::Repository;
use std::error::Error;
use std::io::Write;
use crate::git_analyzer::extractors::{get_all_commits, get_all_blobs, get_all_trees, get_all_tags, get_all_refs};
use crate::parquet_reporter;
use crate::git_analyzer::analysis::git_analysis_summary::GitAnalysisSummary;

pub fn analyze_git_repository(repo_path: &str) -> Result<GitAnalysisSummary, Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;

    println!("Analyzing Git repository: {}", repo_path);
    std::io::stdout().flush().unwrap();
    println!("Head: {:?}", repo.head()?.peel_to_commit()?.id());
    std::io::stdout().flush().unwrap();

    let commits_batch = get_all_commits::get_all_commits(&repo)?;
    println!("Extracted {} commits.", commits_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let commits_parquet_path = "git_commits.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(commits_batch.clone(), &commits_parquet_path)?;

    let blobs_batch = get_all_blobs::get_all_blobs(&repo)?;
    println!("Extracted {} blobs.", blobs_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let blobs_parquet_path = "git_blobs.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(blobs_batch.clone(), &blobs_parquet_path)?;

    let trees_batch = get_all_trees::get_all_trees(&repo)?;
    println!("Extracted {} trees.", trees_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let trees_parquet_path = "git_trees.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(trees_batch.clone(), &trees_parquet_path)?;

    let tags_batch = get_all_tags::get_all_tags(&repo)?;
    println!("Extracted {} tags.", tags_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let tags_parquet_path = "git_tags.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(tags_batch.clone(), &tags_parquet_path)?;

    let refs_batch = get_all_refs::get_all_refs(&repo)?;
    println!("Extracted {} refs.", refs_batch.num_rows());
    std::io::stdout().flush().unwrap();
    let refs_parquet_path = "git_refs.parquet".to_string();
    parquet_reporter::write_record_batch_to_parquet(refs_batch.clone(), &refs_parquet_path)?;

    Ok(GitAnalysisSummary {
        commits_count: commits_batch.num_rows(),
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