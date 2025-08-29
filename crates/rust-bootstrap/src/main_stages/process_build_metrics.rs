use crate::parquet_reporter;
use crate::bootstrap_stages::command_executor;
use std::error::Error;

pub fn process_build_metrics(
    command_result: command_executor::command_execution_types::CommandExecutionResult,
) -> Result<(), Box<dyn Error>> {
    let file_path = "build_metrics.parquet";
    parquet_reporter::write_build_metrics_to_parquet(&command_result.output, command_result.duration, file_path)?;
    parquet_reporter::read_and_summarize_parquet_metrics(file_path)?;
    Ok(())
}