use std::fs::File;
use std::error::Error;

pub fn open_parquet_file(file_path: &str) -> Result<File, Box<dyn Error>> {
    Ok(File::open(file_path)?)
}