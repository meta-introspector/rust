use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
use std::error::Error;
use arrow_array::RecordBatchReader;

pub fn get_record_batch_reader(file: File) -> Result<Box<dyn RecordBatchReader>, Box<dyn Error>> {
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    Ok(Box::new(builder.build()?))
}