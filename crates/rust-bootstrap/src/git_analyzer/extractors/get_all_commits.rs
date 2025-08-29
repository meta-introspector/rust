use git2::Repository;
use arrow_array::{RecordBatch, StringArray, PrimitiveArray, TimestampNanosecondArray};
use arrow_array::types::TimestampNanosecondType;
use std::sync::Arc;
use crate::git_analyzer::schemas::git_commits_schema;
use arrow_array::Array; // Import Array trait
use arrow_data::ArrayData; // Import ArrayData
use crate::tracer::record_trace_event;
use crate::git_analyzer::errata::GitErrata;

pub struct GitAnalysisResult {
    pub record_batch: RecordBatch,
    pub errata: Vec<GitErrata>,
}

pub fn get_all_commits(repo: &Repository) -> Result<GitAnalysisResult, Box<dyn std::error::Error>> {
    record_trace_event("GitExtractor", "Starting commit extraction", None);
    let mut commit_hashes = Vec::new();
    let mut author_names = Vec::new();
    let mut author_emails = Vec::new();
    let mut committer_names = Vec::new();
    let mut committer_emails = Vec::new();
    let mut commit_times = Vec::new();
    let mut messages = Vec::new();
    let mut parent_hashes_list = Vec::new(); // For ListArray builder
    let mut errata_list: Vec<GitErrata> = Vec::new(); // Collect errata here

    let mut revwalk = repo.revwalk()?;
    record_trace_event("GitExtractor", "Initialized revwalk", None);
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        record_trace_event("GitExtractor", "Processing OID", Some(&format!("oid: {}", oid)));
        let commit_result = repo.find_commit(oid);
        let commit = match commit_result {
            Ok(c) => {
                record_trace_event("GitExtractor", "Found commit", Some(&format!("oid: {}", oid)));
                c
            },
            Err(e) => {
                record_trace_event("GitExtractor", "Failed to find commit", Some(&format!("oid: {}, error: {}", oid, e)));
                errata_list.push(GitErrata::new(oid.to_string(), e.to_string()));
                continue; // Skip this OID and continue with the next one
            }
        };

        commit_hashes.push(commit.id().to_string());
        author_names.push(commit.author().name().unwrap_or("").to_string());
        author_emails.push(commit.author().email().unwrap_or("").to_string());
        committer_names.push(commit.committer().name().unwrap_or("").to_string());
        committer_emails.push(commit.committer().email().unwrap_or("").to_string());
        commit_times.push(commit.time().seconds() * 1_000_000_000); // Convert to nanoseconds
        messages.push(commit.message().unwrap_or("").to_string());

        let parents: Vec<String> = commit.parents().map(|p| p.id().to_string()).collect();
        parent_hashes_list.push(parents);
    }

    let commit_hashes_array = StringArray::from(commit_hashes);
    let author_names_array = StringArray::from(author_names);
    let author_emails_array = StringArray::from(author_emails);
    let committer_names_array = StringArray::from(committer_names);
    let committer_emails_array = StringArray::from(committer_emails);

    let schema = git_commits_schema(); // Get schema first
    let commit_time_field = schema.field_with_name("commit_time")?;

    // Create a PrimitiveArray of i64 for the timestamps
    let primitive_times_array: PrimitiveArray<TimestampNanosecondType> = PrimitiveArray::from(commit_times);

    // Create ArrayData with the correct DataType and buffers from PrimitiveArray
    let commit_times_array_data = ArrayData::builder(commit_time_field.data_type().clone())
        .len(primitive_times_array.len())
        .add_buffer(primitive_times_array.to_data().buffers()[0].clone())
        .build()?;

    let commit_times_array = TimestampNanosecondArray::from(commit_times_array_data);

    let messages_array = StringArray::from(messages);

    // Build ListArray for parent_hashes
    let mut parent_hashes_builder = arrow_array::builder::ListBuilder::new(
        arrow_array::builder::StringBuilder::new(),
    );
    for parents in parent_hashes_list {
        parent_hashes_builder.append_value(parents.into_iter().map(Some));
    }
    let parent_hashes_array = parent_hashes_builder.finish();

    let record_batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(commit_hashes_array),
            Arc::new(author_names_array),
            Arc::new(author_emails_array),
            Arc::new(committer_names_array),
            Arc::new(committer_emails_array),
            Arc::new(commit_times_array),
            Arc::new(messages_array),
            Arc::new(parent_hashes_array),
        ],
    )?;

    println!("Actual RecordBatch Schema: {:#?}", record_batch.schema());
    record_trace_event("GitExtractor", "Finished commit extraction", Some(&format!("extracted_count: {}", record_batch.num_rows())));

    Ok(GitAnalysisResult { record_batch, errata: errata_list })
}