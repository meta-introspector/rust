use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::error::Error;

pub fn close_parquet_writer(writer: ArrowWriter<File>) -> Result<(), Box<dyn Error>> {
    writer.close()?;
    println!("Writer closed successfully."); // Trace 10
    Ok(())
}