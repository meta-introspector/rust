#[cfg(test)]
mod tests {
    use git2::Repository;
    use std::error::Error;
    use tempfile::tempdir;

    #[test]
    fn test_clone_non_existent_repo_error_handling() -> Result<(), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let repo_path = tmp_dir.path().join("non_existent_repo");
        let non_existent_url = "https://github.com/rust-lang/non-existent-repo-for-test.git";

        let result = Repository::clone(non_existent_url, &repo_path);

        assert!(result.is_err());
        if let Err(err) = result { // Correctly handle the error
            let error_message = err.message();
            assert!(
                error_message.contains("repository not found") ||
                error_message.contains("authentication required") ||
                error_message.contains("failed to authenticate") ||
                error_message.contains("could not read username") ||
                error_message.contains("network") ||
                error_message.contains("unresolvable hostname") ||
                error_message.contains("failed to connect")
            );
        } else {
            panic!("Expected an error but got Ok"); // Should not happen
        }

        Ok(())
    }

    #[test]
    fn test_read_non_existent_commit_error_handling() -> Result<(), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let repo_path = tmp_dir.path();

        let repo = Repository::init(repo_path)?;

        // Attempt to find a non-existent commit ID
        let non_existent_oid = git2::Oid::from_str("3e9dc46aa563ca0c53ec826c41b05f10c5915925")?; // Use the problematic OID

        let result = repo.find_commit(non_existent_oid);

        assert!(result.is_err());
        if let Err(err) = result {
            let error_message = err.message();
            assert!(error_message.contains("object not found") || error_message.contains("No such file or directory"));
        } else {
            panic!("Expected an error but got Ok");
        }

        Ok(())
    }
}
