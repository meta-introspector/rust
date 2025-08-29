use std::process::Output;
use std::error::Error;

pub mod capture_start_time;
pub mod execute_shell_command;
pub mod capture_end_time_and_duration;
pub mod create_command_execution_result;

#[derive(Debug)]
pub struct CommandExecutionResult {
    pub output: Output,
    pub duration: i64, // Duration in nanoseconds
}

pub fn execute_and_report_command(command: &str, args: &[&str]) -> Result<CommandExecutionResult, Box<dyn Error>> {
    let start_time = capture_start_time::capture_start_time();
    let output = execute_shell_command::execute_shell_command(command, args)?;
    let duration = capture_end_time_and_duration::capture_end_time_and_duration(start_time);
    let result = create_command_execution_result::create_command_execution_result(output, duration);
    Ok(result)
}
