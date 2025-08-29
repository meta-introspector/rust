use arrow_array::{StringArray, TimestampNanosecondArray};
use std::process::Output;

pub struct PreparedBatchData {
    pub command_name: StringArray,
    pub command_status: StringArray,
    pub command_stdout: StringArray,
    pub command_stderr: StringArray,
    pub command_duration: TimestampNanosecondArray,
}

pub fn prepare_record_batch_data(output: &Output, duration: i64) -> PreparedBatchData {
    let command_name = StringArray::from(vec!["cargo --version"]);
    let command_status = StringArray::from(vec![output.status.to_string()]);
    let command_stdout = StringArray::from(vec![String::from_utf8_lossy(&output.stdout).to_string()]);
    let command_stderr = StringArray::from(vec![String::from_utf8_lossy(&output.stderr).to_string()]);
    let command_duration = TimestampNanosecondArray::from(vec![duration]);

    PreparedBatchData {
        command_name,
        command_status,
        command_stdout,
        command_stderr,
        command_duration,
    }
}