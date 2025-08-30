#[cfg(test)]
mod tests {
    use git2::Repository;
    use crate::git_analyzer::extractors::get_all_tags as get_all_tags_fn;
    use crate::git_analyzer::schemas::git_tags_schema;
    use std::error::Error;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    use git2::{Signature, Time};

    #[test]
    fn test_get_all_tags_with_schema() -> Result<(), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let repo_path = tmp_dir.path();

        let repo = Repository::init(repo_path)?;

        // Create an initial commit to tag
        let mut index = repo.index()?;
        let file_path = repo_path.join("test_tag_file.txt");
        let mut file = File::create(&file_path)?;
        file.write_all(b"Content for tag.")?;
        index.add_path(&std::path::Path::new("test_tag_file.txt"))?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let signature = repo.signature()?;

        let commit_oid = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit for tagging",
            &tree,
            &[],
        )?;
        let commit = repo.find_commit(commit_oid)?;

        // Create a tag
        let tag_signature = Signature::new("Tag Author", "tag.author@example.com", &Time::new(123456789, 0))?;
        repo.tag(
            "v1.0.0",
            commit.as_object(),
            &tag_signature,
            "My first tag",
            true,
        )?;

        let tags_batch = get_all_tags_fn::get_all_tags(&repo)?;

        // Assert that the record batch was created successfully and matches the schema
        let expected_schema = git_tags_schema();
        assert_eq!(tags_batch.schema(), expected_schema);
        assert!(tags_batch.num_rows() > 0); // Ensure some tags were extracted

        Ok(())
    }
}
