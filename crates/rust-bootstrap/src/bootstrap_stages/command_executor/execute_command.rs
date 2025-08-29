use std::error::Error;
use std::process::Command;
use super::CommandExecutionResult; // Import the struct
use super::capture_start_time;
use super::capture_end_time_and_duration;
use super::create_command_execution_result;
use super::run_command; // Import run_command

pub fn execute_command(
    command_str: &str,
) -> Result<CommandExecutionResult, Box<dyn Error>> {
    let start_time = capture_start_time::capture_start_time();

    // Construct the command
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command_str);

    // Use run_command to execute
    let output = run_command::run_command(&mut cmd, false, true)?; // verbose=false, exception=true

    let duration = capture_end_time_and_duration::capture_end_time_and_duration(start_time);

    Ok(create_command_execution_result::create_command_execution_result(output, duration))
}