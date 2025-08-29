use git2::{Repository, Oid, ObjectType};
use arrow_array::{RecordBatch, StringArray, UInt64Array, BinaryArray};
use std::sync::Arc;
use std::collections::HashSet;
use crate::git_analyzer::schemas::git_blobs_schema;

pub fn get_all_blobs(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let mut blob_hashes = Vec::new();
    let mut blob_sizes = Vec::new();
    let mut blob_contents: Vec<Vec<u8>> = Vec::new(); // Reverted type
    let mut visited_blobs = HashSet::new();

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let tree = commit.tree()?;

        // Recursively traverse the tree to find all blobs
        fn traverse_tree(
            repo: &Repository,
            tree_oid: Oid,
            blob_hashes: &mut Vec<String>,
            blob_sizes: &mut Vec<u64>,
            blob_contents: &mut Vec<Vec<u8>>,
            visited_blobs: &mut HashSet<Oid>,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let tree = repo.find_tree(tree_oid)?;
            for entry in tree.iter() {
                let object = entry.to_object(repo)?;
                if object.kind() == Some(ObjectType::Blob) {
                    let blob = object.as_blob().unwrap();
                    let blob_id = blob.id();
                    if visited_blobs.insert(blob_id) {
                        // Only add if not visited before
                        blob_hashes.push(blob_id.to_string());
                        blob_sizes.push(blob.size() as u64);
                        blob_contents.push(blob.content().to_vec()); // Reverted to push Vec<u8>
                    }
                } else if object.kind() == Some(ObjectType::Tree) {
                    traverse_tree(
                        repo,
                        object.id(),
                        blob_hashes,
                        blob_sizes,
                        blob_contents,
                        visited_blobs,
                    )?;
                }
            }
            Ok(())
        }

        traverse_tree(
            repo,
            tree.id(),
            &mut blob_hashes,
            &mut blob_sizes,
            &mut blob_contents,
            &mut visited_blobs,
        )?;
    }

    let blob_hashes_array = StringArray::from(blob_hashes);
    let blob_sizes_array = UInt64Array::from(blob_sizes);
    let blob_contents_array = BinaryArray::from_vec(blob_contents.iter().map(|v| v.as_slice()).collect()); // Convert Vec<Vec<u8>> to Vec<&[u8]>

    let schema = git_blobs_schema();
    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(blob_hashes_array),
            Arc::new(blob_sizes_array),
            Arc::new(blob_contents_array),
        ],
    )?;

    Ok(record_batch)
}
