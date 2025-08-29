
use std::process::Output;

pub struct CommandExecutionResult {
    pub output: Output,
    pub duration: i64, // Duration in nanoseconds
}

pub mod capture_start_time;
pub mod execute_shell_command;
pub mod capture_end_time_and_duration;
pub mod create_command_execution_result;

use std::error::Error;

pub fn execute_command(
    command_str: &str,
) -> Result<CommandExecutionResult, Box<dyn Error>> {
    let start_time = capture_start_time::capture_start_time();

    let output = execute_shell_command::execute_shell_command(command_str)?;

    let duration = capture_end_time_and_duration::capture_end_time_and_duration(start_time);

    Ok(create_command_execution_result::create_command_execution_result(output, duration))
}