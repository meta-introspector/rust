

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