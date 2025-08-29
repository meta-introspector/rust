use git2::Repository;
use std::error::Error;
use crate::git_analyzer::extractors::{get_all_commits, get_all_blobs, get_all_trees, get_all_tags, get_all_refs};
use crate::parquet_reporter;

pub fn analyze_git_repository(repo_path: &str) -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;

    println!("Analyzing Git repository: {}", repo_path);
    println!("Head: {:?}", repo.head()?.peel_to_commit()?.id());

    let commits_batch = get_all_commits::get_all_commits(&repo)?;
    println!("Extracted {} commits.", commits_batch.num_rows());
    parquet_reporter::write_record_batch_to_parquet(commits_batch, "git_commits.parquet")?;

    let blobs_batch = get_all_blobs::get_all_blobs(&repo)?;
    println!("Extracted {} blobs.", blobs_batch.num_rows());
    parquet_reporter::write_record_batch_to_parquet(blobs_batch, "git_blobs.parquet")?;

    let trees_batch = get_all_trees::get_all_trees(&repo)?;
    println!("Extracted {} trees.", trees_batch.num_rows());
    parquet_reporter::write_record_batch_to_parquet(trees_batch, "git_trees.parquet")?;

    let tags_batch = get_all_tags::get_all_tags(&repo)?;
    println!("Extracted {} tags.", tags_batch.num_rows());
    parquet_reporter::write_record_batch_to_parquet(tags_batch, "git_tags.parquet")?;

    let refs_batch = get_all_refs::get_all_refs(&repo)?;
    println!("Extracted {} refs.", refs_batch.num_rows());
    parquet_reporter::write_record_batch_to_parquet(refs_batch, "git_refs.parquet")?;

    Ok(())
}