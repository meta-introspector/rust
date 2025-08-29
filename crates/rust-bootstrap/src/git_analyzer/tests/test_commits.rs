#[cfg(test)]
mod tests {
    use git2::Repository;
    use crate::git_analyzer::extractors::get_all_commits as get_all_commits_fn;
    use crate::git_analyzer::schemas::git_commits_schema;
    use std::error::Error;
    use std::path::Path;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_all_commits_with_schema() -> Result<(), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let repo_path = tmp_dir.path();

        let repo = Repository::init(repo_path)?;

        // Create an initial commit
        let mut index = repo.index()?;
        let mut file = File::create(repo_path.join("test_file.txt"))?;
        file.write_all(b"Hello, Git!")?;
        index.add_path(Path::new("test_file.txt"))?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let signature = repo.signature()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;

        let commits_batch = get_all_commits_fn::get_all_commits(&repo)?;

        // Assert that the record batch was created successfully and matches the schema
        let expected_schema = git_commits_schema();
        assert_eq!(commits_batch.schema(), expected_schema);
        assert!(commits_batch.num_rows() > 0); // Ensure some commits were extracted

        Ok(())
    }

    #[test]
    fn test_repo_init_and_commit() -> Result<(), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let repo_path = tmp_dir.path();

        let repo = Repository::init(repo_path)?;

        let mut file = File::create(repo_path.join("test_file.txt"))?;
        file.write_all(b"Hello, Git!")?;

        let mut index = repo.index()?;
        index.add_path(Path::new("test_file.txt"))?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let signature = repo.signature()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;

        // Assert that the commit exists
        let head_commit = repo.head()?.peel_to_commit()?;
        assert_eq!(head_commit.message(), Some("Initial commit"));

        Ok(())
    }
}
