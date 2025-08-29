pub mod prepare_record_batch_data;
pub mod create_arrow_schema;
pub mod create_record_batch;
pub mod write_to_parquet_file;
pub mod close_parquet_writer;
pub mod print_summary_header;
pub mod open_parquet_file;
pub mod read_parquet_metadata;
pub mod print_total_records;
pub mod get_record_batch_reader;
pub mod process_record_batches;
pub mod print_summary_footer;
pub mod errata_reporter;
use arrow_array::{RecordBatch, UInt64Array, StringArray, BooleanArray};
use std::sync::Arc;
use crate::build_state::BuildState;
use crate::git_analyzer::analysis::git_analysis_summary::GitAnalysisSummary;
use crate::git_analyzer::schemas::{git_analysis_summary_schema, build_config_schema};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;

pub fn write_build_metrics_to_parquet(
    output: &std::process::Output,
    duration: i64,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let prepared_data = prepare_record_batch_data::prepare_record_batch_data(output, duration);
    let schema = create_arrow_schema::create_arrow_schema();
    let batch = create_record_batch::create_record_batch(schema.clone(), prepared_data)?;

    println!("Before writing to Parquet file..."); // Trace 6
    let writer = write_to_parquet_file::write_to_parquet_file(file_path, schema, batch)?;

    close_parquet_writer::close_parquet_writer(writer)?;

    println!("\nBuild metrics written to {}", file_path);
    Ok(())
}



pub fn write_record_batch_to_parquet(
    record_batch: RecordBatch,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = record_batch.schema();
    let writer = write_to_parquet_file::write_to_parquet_file(file_path, schema, record_batch)?;
    close_parquet_writer::close_parquet_writer(writer)?;
    println!("RecordBatch written to {}", file_path);
    Ok(())
}

pub fn write_build_config_to_parquet(
    build_state: &BuildState,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = build_config_schema();
    let download_ci_rustc_array = BooleanArray::from(vec![build_state.config.build.download_ci_rustc]);
    let download_ci_llvm_array = BooleanArray::from(vec![build_state.config.build.download_ci_llvm]);
    let patch_binaries_for_nix_array = BooleanArray::from(vec![build_state.config.build.patch_binaries_for_nix]);
    let rustc_path_array = StringArray::from(vec![build_state.stage0.rustc.to_str().unwrap_or_default()]);
    let cargo_path_array = StringArray::from(vec![build_state.stage0.cargo.to_str().unwrap_or_default()]);
    let compiler_date_array = StringArray::from(vec![build_state.stage0.compiler_date.clone()]);
    let compiler_version_array = StringArray::from(vec![build_state.stage0.compiler_version.clone()]);
    let dist_server_array = StringArray::from(vec![build_state.stage0.dist_server.clone()]);

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(download_ci_rustc_array),
            Arc::new(download_ci_llvm_array),
            Arc::new(patch_binaries_for_nix_array),
            Arc::new(rustc_path_array),
            Arc::new(cargo_path_array),
            Arc::new(compiler_date_array),
            Arc::new(compiler_version_array),
            Arc::new(dist_server_array),
        ],
    )?;

    let writer = write_to_parquet_file::write_to_parquet_file(file_path, schema, batch)?;
    close_parquet_writer::close_parquet_writer(writer)?;
    println!("Build config written to {}", file_path);
    Ok(())
}

pub fn write_git_analysis_summary_to_parquet(
    summary: GitAnalysisSummary,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = git_analysis_summary_schema();

    let commits_count_array = UInt64Array::from(vec![summary.commits_count as u64]);
    let blobs_count_array = UInt64Array::from(vec![summary.blobs_count as u64]);
    let trees_count_array = UInt64Array::from(vec![summary.trees_count as u64]);
    let tags_count_array = UInt64Array::from(vec![summary.tags_count as u64]);
    let refs_count_array = UInt64Array::from(vec![summary.refs_count as u64]);
    let commits_parquet_path_array = StringArray::from(vec![summary.commits_parquet_path]);
    let blobs_parquet_path_array = StringArray::from(vec![summary.blobs_parquet_path]);
    let trees_parquet_path_array = StringArray::from(vec![summary.trees_parquet_path]);
    let tags_parquet_path_array = StringArray::from(vec![summary.tags_parquet_path]);
    let refs_parquet_path_array = StringArray::from(vec![summary.refs_parquet_path]);

    let record_batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(commits_count_array),
            Arc::new(blobs_count_array),
            Arc::new(trees_count_array),
            Arc::new(tags_count_array),
            Arc::new(refs_count_array),
            Arc::new(commits_parquet_path_array),
            Arc::new(blobs_parquet_path_array),
            Arc::new(trees_parquet_path_array),
            Arc::new(tags_parquet_path_array),
            Arc::new(refs_parquet_path_array),
        ],
    )?;

    let writer = write_to_parquet_file::write_to_parquet_file(file_path, schema, record_batch)?;
    close_parquet_writer::close_parquet_writer(writer)?;
    println!("Git analysis summary written to {}", file_path);
    Ok(())
}

pub fn read_and_summarize_parquet_metrics(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_summary_header::print_summary_header();

    let file = open_parquet_file::open_parquet_file(file_path)?;
    let metadata = read_parquet_metadata::read_parquet_metadata(file)?;
    print_total_records::print_total_records(metadata.file_metadata().num_rows());

    let file = open_parquet_file::open_parquet_file(file_path)?;
    let arrow_reader = get_record_batch_reader::get_record_batch_reader(file)?;
    process_record_batches::process_record_batches(arrow_reader)?;

    print_summary_footer::print_summary_footer();
    Ok(())
}

pub fn read_and_summarize_build_config_metrics(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Build Config Summary ---");

    let file = File::open(file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;

    if let Some(record_batch) = reader.next() {
        let record_batch = record_batch?;

        let download_ci_rustc_array = record_batch.column(0).as_any().downcast_ref::<BooleanArray>().ok_or("Failed to downcast download_ci_rustc_array")?;
        let download_ci_llvm_array = record_batch.column(1).as_any().downcast_ref::<BooleanArray>().ok_or("Failed to downcast download_ci_llvm_array")?;
        let patch_binaries_for_nix_array = record_batch.column(2).as_any().downcast_ref::<BooleanArray>().ok_or("Failed to downcast patch_binaries_for_nix_array")?;
        let rustc_path_array = record_batch.column(3).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast rustc_path_array")?;
        let cargo_path_array = record_batch.column(4).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast cargo_path_array")?;
        let compiler_date_array = record_batch.column(5).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast compiler_date_array")?;
        let compiler_version_array = record_batch.column(6).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast compiler_version_array")?;
        let dist_server_array = record_batch.column(7).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast dist_server_array")?;

        println!("Download CI Rustc: {}", download_ci_rustc_array.value(0));
        println!("Download CI LLVM: {}", download_ci_llvm_array.value(0));
        println!("Patch Binaries for Nix: {}", patch_binaries_for_nix_array.value(0));
        println!("Rustc Path: {}", rustc_path_array.value(0));
        println!("Cargo Path: {}", cargo_path_array.value(0));
        println!("Compiler Date: {}", compiler_date_array.value(0));
        println!("Compiler Version: {}", compiler_version_array.value(0));
        println!("Dist Server: {}", dist_server_array.value(0));
    } else {
        println!("No build config summary found in {}", file_path);
    }

    println!("----------------------------");
    Ok(())
}


pub fn read_and_summarize_git_analysis_metrics(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Git Analysis Summary ---");

    let file = File::open(file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;

    if let Some(record_batch) = reader.next() {
        let record_batch = record_batch?;

        let commits_count_array = record_batch.column(0).as_any().downcast_ref::<UInt64Array>().ok_or("Failed to downcast commits_count_array")?;
        let blobs_count_array = record_batch.column(1).as_any().downcast_ref::<UInt64Array>().ok_or("Failed to downcast blobs_count_array")?;
        let trees_count_array = record_batch.column(2).as_any().downcast_ref::<UInt64Array>().ok_or("Failed to downcast trees_count_array")?;
        let tags_count_array = record_batch.column(3).as_any().downcast_ref::<UInt64Array>().ok_or("Failed to downcast tags_count_array")?;
        let refs_count_array = record_batch.column(4).as_any().downcast_ref::<UInt64Array>().ok_or("Failed to downcast refs_count_array")?;
        let commits_parquet_path_array = record_batch.column(5).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast commits_parquet_path_array")?;
        let blobs_parquet_path_array = record_batch.column(6).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast blobs_parquet_path_array")?;
        let trees_parquet_path_array = record_batch.column(7).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast trees_parquet_path_array")?;
        let tags_parquet_path_array = record_batch.column(8).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast tags_parquet_path_array")?;
        let refs_parquet_path_array = record_batch.column(9).as_any().downcast_ref::<StringArray>().ok_or("Failed to downcast refs_parquet_path_array")?;

        println!("Commits: {}", commits_count_array.value(0));
        println!("Blobs: {}", blobs_count_array.value(0));
        println!("Trees: {}", trees_count_array.value(0));
        println!("Tags: {}", tags_count_array.value(0));
        println!("Refs: {}", refs_count_array.value(0));
        println!("Commits Parquet: {}", commits_parquet_path_array.value(0));
        println!("Blobs Parquet: {}", blobs_parquet_path_array.value(0));
        println!("Trees Parquet: {}", trees_parquet_path_array.value(0));
        println!("Tags Parquet: {}", tags_parquet_path_array.value(0));
        println!("Refs Parquet: {}", refs_parquet_path_array.value(0));
    } else {
        println!("No git analysis summary found in {}", file_path);
    }

    println!("----------------------------");
    Ok(())
}

