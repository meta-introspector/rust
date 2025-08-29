use std::error::Error;
use std::process::{Command, Output};
use std::time::Instant;
use super::execute;

pub struct CommandExecutionResult {
    pub output: Output,
    pub duration: u128,
}

pub fn timed_execute(
    cmd: &mut Command,
    verbose: bool,
) -> Result<CommandExecutionResult, Box<dyn Error>> {
    let start_time = Instant::now();

    let output = execute::execute(cmd, verbose)?;

    let duration = start_time.elapsed().as_nanos();

    Ok(CommandExecutionResult { output, duration })
}