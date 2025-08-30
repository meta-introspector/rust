#[cfg(test)]
mod tests {
    use git2::Repository;
    use crate::git_analyzer::extractors::get_all_refs as get_all_refs_fn;
    use crate::git_analyzer::schemas::git_refs_schema;
    use std::error::Error;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    use git2::{Signature, ReferenceFormat};

    #[test]
    fn test_get_all_refs_with_schema() -> Result<(), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let repo_path = tmp_dir.path();

        let repo = Repository::init(repo_path)?;

        // Create an initial commit to have a HEAD reference
        let mut index = repo.index()?;
        let file_path = repo_path.join("test_ref_file.txt");
        let mut file = File::create(&file_path)?;
        file.write_all(b"Content for ref.")?;
        index.add_path(&std::path::Path::new("test_ref_file.txt"))?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let signature = repo.signature()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit for refs",
            &tree,
            &[],
        )?;

        // Create a new branch to have another reference
        let head = repo.head()?;
        let oid = head.target().unwrap();
        let commit = repo.find_commit(oid)?;
        repo.branch("new-branch", &commit, false)?;

        let refs_batch = get_all_refs_fn::get_all_refs(&repo)?;

        // Assert that the record batch was created successfully and matches the schema
        let expected_schema = git_refs_schema();
        assert_eq!(refs_batch.schema(), expected_schema);
        assert!(refs_batch.num_rows() > 0); // Ensure some refs were extracted

        // Optionally, check for specific reference names
        let ref_names: Vec<String> = refs_batch.column(0).as_any()
            .downcast_ref::<arrow::array::StringArray>().unwrap()
            .iter().filter_map(|s| s.map(|s_val| s_val.to_string())).collect();

        assert!(ref_names.contains(&"refs/heads/main".to_string()) || ref_names.contains(&"refs/heads/master".to_string()));
        assert!(ref_names.contains(&"refs/heads/new-branch".to_string()));

        Ok(())
    }
}
