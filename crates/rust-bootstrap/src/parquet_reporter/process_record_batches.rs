use arrow_array::{RecordBatchReader, StringArray, TimestampNanosecondArray};
use std::error::Error;


pub fn process_record_batches(arrow_reader: Box<dyn RecordBatchReader>) -> Result<(), Box<dyn Error>> {
    for batch_result in arrow_reader {
        let batch = batch_result?;
        let command_name_array = batch.column_by_name("command_name")
            .ok_or("Column 'command_name' not found")?
            .as_ref()
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'command_name' to StringArray")?;
        let command_status_array = batch.column_by_name("command_status")
            .ok_or("Column 'command_status' not found")?
            .as_ref()
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'command_status' to StringArray")?;
        let command_duration_array = batch.column_by_name("command_duration_ns")
            .ok_or("Column 'command_duration_ns' not found")?
            .as_ref()
            .as_any()
            .downcast_ref::<TimestampNanosecondArray>()
            .ok_or("Failed to downcast 'command_duration_ns' to TimestampNanosecondArray")?;

        for i in 0..batch.num_rows() {
            let command_name = command_name_array.value(i);
            let command_status = command_status_array.value(i);
            let command_duration_ns = command_duration_array.value(i);
            let duration_ms = command_duration_ns as f64 / 1_000_000.0;

            println!("Command: {}", command_name);
            println!("Status: {}", command_status);
            println!("Duration: {:.2} ms", duration_ms);
        }
    }
    Ok(())
}