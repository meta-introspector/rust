use std::process::Command;
use std::error::Error;
use crate::bootstrap_stages::command_executor::CommandExecutionResult;
use crate::bootstrap_stages::command_executor::capture_start_time;
use crate::bootstrap_stages::command_executor::capture_end_time_and_duration;
use crate::bootstrap_stages::command_executor::create_command_execution_result;
use std::io::{self, Write};

pub fn execute_shell_command(
    command_str: &str,
    verbose: bool,
    exception: bool,
) -> Result<CommandExecutionResult, Box<dyn Error>> {
    let start_time = capture_start_time::capture_start_time();

    if verbose {
        println!("running: {}", command_str);
    }
    io::stdout().flush()?;

    let output = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()?;

    let duration = capture_end_time_and_duration::capture_end_time_and_duration(start_time);

    let command_execution_result = create_command_execution_result::create_command_execution_result(output, duration);

    if !command_execution_result.output.status.success() {
        let err_msg = format!(
            "failed to run: {}\nStdout: {}\nStderr: {}",
            command_str,
            String::from_utf8_lossy(&command_execution_result.output.stdout),
            String::from_utf8_lossy(&command_execution_result.output.stderr)
        );
        if exception {
            return Err(err_msg.into());
        } else {
            eprintln!("Error: {}", err_msg);
            return Err(err_msg.into());
        }
    }

    Ok(command_execution_result)
}