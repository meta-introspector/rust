use git2::{Repository, ReferenceType};
use arrow_array::{RecordBatch, StringArray};
use std::sync::Arc;
use crate::git_analyzer::schemas::git_refs_schema;

pub fn get_all_refs(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error.Error>> {
    let mut ref_names = Vec::new();
    let mut target_ids = Vec::new();
    let mut ref_types = Vec::new();

    for reference in repo.references()? {
        let reference = reference?;
        let name = reference.name().unwrap_or("").to_string();
        ref_names.push(name);

        if let Some(target_id) = reference.target() {
            target_ids.push(target_id.to_string());
        } else {
            target_ids.push("".to_string());
        }

        match reference.kind() {
            Some(ReferenceType::Direct) => ref_types.push("direct".to_string()),
            Some(ReferenceType::Symbolic) => ref_types.push("symbolic".to_string()),
            _ => ref_types.push("unknown".to_string()),
        }
    }

    let ref_names_array = StringArray::from(ref_names);
    let target_ids_array = StringArray::from(target_ids);
    let ref_types_array = StringArray::from(ref_types);

    let schema = git_refs_schema();
    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(ref_names_array),
            Arc::new(target_ids_array),
            Arc::new(ref_types_array),
        ],
    )?;

    Ok(record_batch)
}
