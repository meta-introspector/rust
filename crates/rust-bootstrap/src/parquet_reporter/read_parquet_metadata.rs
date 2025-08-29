use parquet::file::reader::SerializedFileReader;
use parquet::file::metadata::ParquetMetaData;
use std::fs::File;
use std::error::Error;
use parquet::file::reader::FileReader;

pub fn read_parquet_metadata(file: File) -> Result<ParquetMetaData, Box<dyn Error>> {
    let reader = SerializedFileReader::new(file)?;
    Ok(reader.metadata().clone())
}