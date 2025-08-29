use std::process::Output;
use std::error::Error;

#[derive(Debug)]
pub struct CommandExecutionResult {
    pub output: Output,
    pub duration: i64, // Duration in nanoseconds
}

pub mod capture_start_time;
pub mod execute_shell_command;
pub mod capture_end_time_and_duration;
pub mod create_command_execution_result;
pub mod run_command; // This is the new one