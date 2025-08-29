use git2::{Repository, Oid, ObjectType};
use arrow_array::{RecordBatch, StringArray, UInt32Array};
use std::sync::Arc;
use std::collections::HashSet;
use crate::git_analyzer::schemas::git_trees_schema;

pub fn get_all_trees(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let mut tree_hashes = Vec::new();
    let mut entry_names = Vec::new();
    let mut entry_types = Vec::new();
    let mut entry_ids = Vec::new();
    let mut entry_modes = Vec::new();
    let mut visited_trees = HashSet::new();

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let tree = commit.tree()?;

        // Recursively traverse the tree to find all trees and their entries
        fn traverse_tree(
            repo: &Repository,
            tree_oid: Oid,
            tree_hashes: &mut Vec<String>,
            entry_names: &mut Vec<String>,
            entry_types: &mut Vec<String>,
            entry_ids: &mut Vec<String>,
            entry_modes: &mut Vec<u32>,
            visited_trees: &mut HashSet<Oid>,
        ) -> Result<(), Box<dyn std::error::Error>> {
            if !visited_trees.insert(tree_oid) {
                return Ok(()); // Already visited
            }

            let tree = repo.find_tree(tree_oid)?;
            for entry in tree.iter() {
                let object = entry.to_object(repo)?;
                let entry_name = entry.name().unwrap_or("").to_string();
                let entry_id = object.id().to_string();
                let entry_mode = entry.filemode();

                match object.kind() {
                    Some(ObjectType::Tree) => {
                        tree_hashes.push(tree_oid.to_string());
                        entry_names.push(entry_name.clone());
                        entry_types.push("tree".to_string());
                        entry_ids.push(entry_id.clone());
                        entry_modes.push(entry_mode);
                        traverse_tree(
                            repo,
                            object.id(),
                            tree_hashes,
                            entry_names,
                            entry_types,
                            entry_ids,
                            entry_modes,
                            visited_trees,
                        )?;
                    }
                    Some(ObjectType::Blob) => {
                        tree_hashes.push(tree_oid.to_string());
                        entry_names.push(entry_name.clone());
                        entry_types.push("blob".to_string());
                        entry_ids.push(entry_id.clone());
                        entry_modes.push(entry_mode);
                    }
                    Some(ObjectType::Commit) => {
                        tree_hashes.push(tree_oid.to_string());
                        entry_names.push(entry_name.clone());
                        entry_types.push("commit".to_string());
                        entry_ids.push(entry_id.clone());
                        entry_modes.push(entry_mode);
                    }
                    _ => {
                        // Handle other object types if necessary, or ignore
                    }
                }
            }
            Ok(())
        }

        traverse_tree(
            repo,
            tree.id(),
            &mut tree_hashes,
            &mut entry_names,
            &mut entry_types,
            &mut entry_ids,
            &mut entry_modes,
            &mut visited_trees,
        )?;
    }

    let tree_hashes_array = StringArray::from(tree_hashes);
    let entry_names_array = StringArray::from(entry_names);
    let entry_types_array = StringArray::from(entry_types);
    let entry_ids_array = StringArray::from(entry_ids);
    let entry_modes_array = UInt32Array::from(entry_modes);

    let schema = git_trees_schema();
    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(tree_hashes_array),
            Arc::new(entry_names_array),
            Arc::new(entry_types_array),
            Arc::new(entry_ids_array),
            Arc::new(entry_modes_array),
        ],
    )?;

    Ok(record_batch)
}
