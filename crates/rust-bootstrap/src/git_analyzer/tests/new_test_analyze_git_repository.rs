#[cfg(test)]
mod tests {
    

    // fn create_test_repo_with_history(repo_path: &Path) -> Result<Repository, Box<dyn Error>> {
    //     let repo = Repository::init(repo_path)?;
    //     let mut index = repo.index()?;
    //     let signature = repo.signature()?;

    //     // Initial commit
    //     let file1_path = repo_path.join("file1.txt");
    //     let mut file1 = File::create(&file1_path)?;
    //     file1.write_all(b"content of file1")?;
    //     index.add_path(&std::path::Path::new("file1.txt"))?;
    //     let oid = index.write_tree()?;
    //     let tree = repo.find_tree(oid)?;
    //     let commit_oid = repo.commit(
    //         Some("HEAD"),
    //         &signature,
    //         &signature,
    //         "Initial commit",
    //         &tree,
    //         &[],
    //     )?;
    //     let commit = repo.find_commit(commit_oid)?;

    //     // Second commit with a new file
    //     let file2_path = repo_path.join("file2.txt");
    //     let mut file2 = File::create(&file2_path)?;
    //     file2.write_all(b"content of file2")?;
    //     index.add_path(&std::path::Path::new("file2.txt"))?;
    //     let oid = index.write_tree()?;
    //     let tree = repo.find_tree(oid)?;
    //     repo.commit(
    //         Some("HEAD"),
    //         &signature,
    //         &signature,
    //         "Second commit",
    //         &tree,
    //         &[&commit],
    //     )?;

    //     // Create a tag
    //     let tag_signature = Signature::new("Tag Author", "tag.author@example.com", &Time::new(123456789, 0))?;
    //     repo.tag(
    //         "v1.0.0",
    //         commit.as_object(),
    //         &tag_signature,
    //         "My first tag",
    //         false,
    //     )?;

    //     // Create a branch
    //     let head = repo.head()?;
    //     let oid = head.target().unwrap();
    //     let head_commit = repo.find_commit(oid)?;
    //     repo.branch("feature-branch", &head_commit, false)?;

    //     drop(head_commit);
    //     drop(head);

    //     Ok(repo)
    // }

    // fn read_parquet_file(path: &Path) -> Result<RecordBatch, Box<dyn Error>> {
    //     let file = File::open(path)?;
    //     let reader = ParquetFileReader::new(file)?;
    //     let arrow_reader = reader.get_record_reader(2048)?; // 2048 is a reasonable batch size
    //     let mut batches: Vec<RecordBatch> = Vec::new();
    //     for batch_result in arrow_reader {
    //         batches.push(batch_result?);
    //     }
    //     // For simplicity, return the first batch or an error if no batches
    //     batches.into_iter().next().ok_or_else(|| "No record batch found".into())
    // }

    // #[test]
    // fn test_analyze_git_repository_produces_parquet_files() -> Result<(), Box<dyn Error>> {
    //     let tmp_dir = tempdir()?;
    //     let repo_path = tmp_dir.path().join("test_repo");
    //     fs::create_dir(&repo_path)?;

    //     let _repo = create_test_repo_with_history(&repo_path)?;

    //     let output_dir = tmp_dir.path().join("output");
    //     fs::create_dir(&output_dir)?;

    //     // Change current directory to output_dir before calling analyze_git_repository_fn
    //     let original_cwd = std::env::current_dir()?;
    //     std::env::set_current_dir(&output_dir)?;

    //     // analyze_git_repository_fn(repo_path.to_str().unwrap())?;

    //     // Restore original current directory
    //     std::env::set_current_dir(&original_cwd)?;

    //     // Verify that the expected Parquet files are created
    //     // assert!(output_dir.join("git_blobs.parquet").exists());
    //     // assert!(output_dir.join("git_trees.parquet").exists());
    //     // assert!(output_dir.join("git_commits.parquet").exists());
    //     // assert!(output_dir.join("git_tags.parquet").exists());
    //     // assert!(output_dir.join("git_refs.parquet").exists());
    //     // assert!(output_dir.join("git_errata.parquet").exists());

    //     // Optionally, read and assert some content from the files
    //     // let blobs_batch = read_parquet_file(&output_dir.join("git_blobs.parquet"))?;
    //     // assert!(blobs_batch.num_rows() > 0);

    //     // let commits_batch = read_parquet_file(&output_dir.join("git_commits.parquet"))?;
    //     // assert!(commits_batch.num_rows() > 0);

    //     // Test errata file (should be empty for a healthy repo)
    //     // let errata_file = File::open(output_dir.join("git_errata.parquet"))?;
    //     // let errata_reader = SerializedFileReader::new(errata_file)?;
    //     // let errata_metadata = errata_reader.metadata().file_metadata();
    //     // assert_eq!(errata_metadata.num_rows(), 0); // Expect 0 rows for a healthy repo

    //     Ok(())
    // }
}
