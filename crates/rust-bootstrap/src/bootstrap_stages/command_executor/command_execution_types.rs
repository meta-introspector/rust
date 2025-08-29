use std::process::Output;
use std::error::Error;

#[derive(Debug)]
pub struct CommandExecutionResult {
    pub output: Output,
    pub duration: i64, // Duration in nanoseconds
}


