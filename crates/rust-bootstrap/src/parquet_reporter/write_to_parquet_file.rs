use std::fs::File;
use std::sync::Arc;
use std::error::Error;
use arrow_array::RecordBatch;
use arrow_schema::Schema;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

pub fn write_to_parquet_file(
    file_path: &str,
    schema: Arc<Schema>,
    batch: RecordBatch,
) -> Result<ArrowWriter<File>, Box<dyn Error>> {
    let file = File::create(file_path)?;
    println!("File created successfully."); // Trace 7

    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
    println!("ArrowWriter created successfully."); // Trace 8

    writer.write(&batch)?;
    println!("Batch written successfully."); // Trace 9

    Ok(writer)
}