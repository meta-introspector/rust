use std::process::Command;
use std::error::Error;
use crate::bootstrap_stages::command_executor::CommandExecutionResult;
use crate::bootstrap_stages::command_executor::capture_start_time;
use crate::bootstrap_stages::command_executor::capture_end_time_and_duration;
use crate::bootstrap_stages::command_executor::create_command_execution_result;

pub fn execute_shell_command(command_str: &str) -> Result<CommandExecutionResult, Box<dyn Error>> {
    let start_time = capture_start_time::capture_start_time();

    let output = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()?;

    let duration = capture_end_time_and_duration::capture_end_time_and_duration(start_time);

    let command_execution_result = create_command_execution_result::create_command_execution_result(output, duration);

    Ok(command_execution_result)
}
