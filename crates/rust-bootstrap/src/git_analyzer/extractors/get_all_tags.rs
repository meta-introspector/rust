use git2::{Repository, ObjectType};
use arrow_array::{RecordBatch, StringArray, TimestampNanosecondArray};
use std::sync::Arc;
use crate::git_analyzer::schemas::git_tags_schema;
use crate::git_analyzer::extractors::tag_target_id_to_string;

pub fn get_all_tags(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let mut tag_hashes = Vec::new();
    let mut tag_names = Vec::new();
    let mut target_ids = Vec::new();
    let mut target_types = Vec::new();
    let mut tagger_names = Vec::new();
    let mut tagger_emails = Vec::new();
    let mut tag_times = Vec::new();
    let mut messages = Vec::new();

    repo.tag_foreach(|_id, name_bytes| {
        let name = String::from_utf8_lossy(name_bytes);
        println!("Processing tag: {}", name); // Debug print
        match repo.find_reference(&name) {
            Ok(tag) => {
                println!("  Found reference for tag: {}", name); // Debug print
                match tag.peel_to_tag() {
                    Ok(tag_obj) => {
                        println!("    Peeled to tag object for tag: {}", name); // Debug print
                        tag_hashes.push(tag_obj.id().to_string());
                        tag_names.push(name.to_string());
                        target_ids.push(tag_target_id_to_string::tag_target_id_to_string(&tag_obj));
                        target_types.push(tag_obj.target_type().map_or("".to_string(), |t| t.to_string()));
                        if let Some(tagger) = tag_obj.tagger() {
                            tagger_names.push(tagger.name().unwrap_or("").to_string());
                            tagger_emails.push(tagger.email().unwrap_or("").to_string());
                            tag_times.push(tagger.when().seconds() * 1_000_000_000); // Convert to nanoseconds
                        } else {
                            tagger_names.push("".to_string());
                            tagger_emails.push("".to_string());
                            tag_times.push(0); // Default or error value
                        }
                        messages.push(tag_obj.message().unwrap_or("").to_string());
                    },
                    Err(e) => {
                        println!("    Failed to peel to tag object for tag {}: {:?}", name, e); // Debug print
                    }
                }
            },
            Err(e) => {
                println!("  Failed to find reference for tag {}: {:?}", name, e); // Debug print
            }
        }
        true
    })?;

    let tag_hashes_array = StringArray::from(tag_hashes);
    let tag_names_array = StringArray::from(tag_names);
    let target_ids_array = StringArray::from(target_ids);
    let target_types_array = StringArray::from(target_types);
    let tagger_names_array = StringArray::from(tagger_names);
    let tagger_emails_array = StringArray::from(tagger_emails);
    let tag_times_array = TimestampNanosecondArray::from(tag_times);
    let messages_array = StringArray::from(messages);

    let schema = git_tags_schema();
    let record_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(tag_hashes_array),
            Arc::new(tag_names_array),
            Arc::new(target_ids_array),
            Arc::new(target_types_array),
            Arc::new(tagger_names_array),
            Arc::new(tagger_emails_array),
            Arc::new(tag_times_array),
            Arc::new(messages_array),
        ],
    )?;

    Ok(record_batch)
}