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
        Field::new("parent_hashes", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), false),
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

pub fn git_analysis_summary_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("commits_count", DataType::UInt64, false),
        Field::new("blobs_count", DataType::UInt64, false),
        Field::new("trees_count", DataType::UInt64, false),
        Field::new("tags_count", DataType::UInt64, false),
        Field::new("refs_count", DataType::UInt64, false),
        Field::new("commits_parquet_path", DataType::Utf8, false),
        Field::new("blobs_parquet_path", DataType::Utf8, false),
        Field::new("trees_parquet_path", DataType::Utf8, false),
        Field::new("tags_parquet_path", DataType::Utf8, false),
        Field::new("refs_parquet_path", DataType::Utf8, false),
    ]))
}

pub fn build_config_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("download_ci_rustc", DataType::Boolean, false),
        Field::new("download_ci_llvm", DataType::Boolean, false),
        Field::new("patch_binaries_for_nix", DataType::Boolean, false),
        Field::new("rustc_path", DataType::Utf8, false),
        Field::new("cargo_path", DataType::Utf8, false),
        Field::new("compiler_date", DataType::Utf8, false),
        Field::new("compiler_version", DataType::Utf8, false),
        Field::new("dist_server", DataType::Utf8, false),
    ]))
}
