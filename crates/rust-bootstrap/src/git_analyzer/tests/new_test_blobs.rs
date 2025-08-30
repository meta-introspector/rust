#[cfg(test)]
mod tests {
    use git2::Repository;
    use crate::git_analyzer::extractors::get_all_blobs as get_all_blobs_fn;
    use crate::git_analyzer::schemas::git_blobs_schema;
    use std::error::Error;
    use std::path::Path;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_all_blobs_with_schema() -> Result<(), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let repo_path = tmp_dir.path();

        let repo = Repository::init(repo_path)?;

        // Create a file and commit it to generate a blob
        let mut index = repo.index()?;
        let file_path = repo_path.join("test_blob_file.txt");
        let mut file = File::create(&file_path)?;
        file.write_all(b"This is some blob content.")?;
        index.add_path(Path::new("test_blob_file.txt"))?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let signature = repo.signature()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit with blob",
            &tree,
            &[],
        )?;

        let blobs_batch = get_all_blobs_fn::get_all_blobs(&repo)?;

        // Assert that the record batch was created successfully and matches the schema
        let expected_schema = git_blobs_schema();
        assert_eq!(blobs_batch.schema(), expected_schema);
        assert!(blobs_batch.num_rows() > 0); // Ensure some blobs were extracted

        Ok(())
    }
}
