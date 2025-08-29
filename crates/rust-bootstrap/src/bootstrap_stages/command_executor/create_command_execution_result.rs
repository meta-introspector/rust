use std::process::Output;
use crate::bootstrap_stages::command_executor::CommandExecutionResult;

pub fn create_command_execution_result(output: Output, duration: i64) -> CommandExecutionResult {
    CommandExecutionResult { output, duration }
}