use git2::{Repository, Error};
use arrow_array::{RecordBatch, StringArray, TimestampNanosecondArray, UInt32Array, UInt64Array, BinaryArray};
use arrow_schema::{Schema, Field, DataType, TimeUnit};
use std::sync::Arc;

// Define Arrow Schemas
pub fn git_commits_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("commit_hash", DataType::Utf8, false),
        Field::new("author_name", DataType::Utf8, false),
        Field::new("author_email", DataType::Utf8, false),
        Field::new("committer_name", DataType::Utf8, false),
        Field::new("committer_email", DataType::Utf8, false),
        Field::new("commit_time", DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into())), false),
        Field::new("message", DataType::Utf8, false),
        Field::new("parent_hashes", DataType::new_list(DataType::Utf8.into(), false), false),
    ]))
}

pub fn git_blobs_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("blob_hash", DataType::Utf8, false),
        Field::new("size", DataType::UInt64, false),
        Field::new("content", DataType::Binary, false),
    ]))
}

pub fn git_trees_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("tree_hash", DataType::Utf8, false),
        Field::new("entry_name", DataType::Utf8, false),
        Field::new("entry_type", DataType::Utf8, false),
        Field::new("entry_id", DataType::Utf8, false),
        Field::new("entry_mode", DataType::UInt32, false),
    ]))
}

pub fn git_tags_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("tag_hash", DataType::Utf8, false),
        Field::new("tag_name", DataType::Utf8, false),
        Field::new("target_id", DataType::Utf8, false),
        Field::new("target_type", DataType::Utf8, false),
        Field::new("tagger_name", DataType::Utf8, false),
        Field::new("tagger_email", DataType::Utf8, false),
        Field::new("tag_time", DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into())), false),
        Field::new("message", DataType::Utf8, false),
    ]))
}





















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

// Main analysis function (placeholder)
pub fn analyze_git_repository(repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_path)?;

    println!("Analyzing Git repository: {}", repo_path);
    println!("Head: {:?}", repo.head()?.peel_to_commit()?.id());

    // TODO: Implement extraction and conversion to RecordBatches
    // For now, just print some info

    Ok(())
}