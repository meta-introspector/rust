use arrow_schema::{Schema, Field, DataType, TimeUnit};
use std::sync::Arc;

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

pub fn git_refs_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("ref_name", DataType::Utf8, false),
        Field::new("target_id", DataType::Utf8, false),
        Field::new("ref_type", DataType::Utf8, false),
    ]))
}