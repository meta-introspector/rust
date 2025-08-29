use git2::Repository;
use arrow_array::{RecordBatch, StringArray, TimestampNanosecondArray};
use std::sync::Arc;
use crate::git_analyzer::schemas::git_commits_schema;

pub fn get_all_commits(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let mut commit_hashes = Vec::new();
    let mut author_names = Vec::new();
    let mut author_emails = Vec::new();
    let mut committer_names = Vec::new();
    let mut committer_emails = Vec::new();
    let mut commit_times = Vec::new();
    let mut messages = Vec::new();
    let mut parent_hashes_list = Vec::new(); // For ListArray builder

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        commit_hashes.push(commit.id().to_string());
        author_names.push(commit.author().name().unwrap_or(""));
        author_emails.push(commit.author().email().unwrap_or(""));
        committer_names.push(commit.committer().name().unwrap_or(""));
        committer_emails.push(commit.committer().email().unwrap_or(""));
        commit_times.push(commit.time().seconds() * 1_000_000_000); // Convert to nanoseconds
        messages.push(commit.message().unwrap_or(""));

        let parents: Vec<String> = commit.parents().map(|p| p.id().to_string()).collect();
        parent_hashes_list.push(parents);
    }

    let commit_hashes_array = StringArray::from(commit_hashes);
    let author_names_array = StringArray::from(author_names);
    let author_emails_array = StringArray::from(author_emails);
    let committer_names_array = StringArray::from(committer_names);
    let committer_emails_array = StringArray::from(committer_emails);
    let commit_times_array = TimestampNanosecondArray::from(commit_times);
    let messages_array = StringArray::from(messages);

    // Build ListArray for parent_hashes
    let mut parent_hashes_builder = arrow_array::builder::ListBuilder::new(
        arrow_array::builder::StringArrayBuilder::new(),
    );
    for parents in parent_hashes_list {
        parent_hashes_builder.append_value(parents);
    }
    let parent_hashes_array = parent_hashes_builder.finish();

    let schema = git_commits_schema();
    let record_batch = RecordBatch::try_new(
        schema,
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

    Ok(record_batch)
}