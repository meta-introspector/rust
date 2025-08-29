use git2::Repository;
use std::error::Error;
use crate::git_analyzer::extractors::get_all_commits;

pub fn analyze_git_repository(repo_path: &str) -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;

    println!("Analyzing Git repository: {}", repo_path);
    println!("Head: {:?}", repo.head()?.peel_to_commit()?.id());

    let commits_batch = get_all_commits::get_all_commits(&repo)?;
    println!("Extracted {} commits.", commits_batch.num_rows());

    // TODO: Implement extraction and conversion for other Git objects (blobs, trees, tags, refs)
    // TODO: Write RecordBatches to Parquet files

    Ok(())
}