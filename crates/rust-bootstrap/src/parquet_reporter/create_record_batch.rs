use arrow_array::RecordBatch;
use arrow_schema::Schema;
use std::sync::Arc;
use std::error::Error;
use crate::parquet_reporter::prepare_record_batch_data::PreparedBatchData;

pub fn create_record_batch(
    schema: Arc<Schema>,
    data: PreparedBatchData,
) -> Result<RecordBatch, Box<dyn Error>> {
    Ok(RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(data.command_name),
            Arc::new(data.command_status),
            Arc::new(data.command_stdout),
            Arc::new(data.command_stderr),
            Arc::new(data.command_duration),
        ],
    )?)
}