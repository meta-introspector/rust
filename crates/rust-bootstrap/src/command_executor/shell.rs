use std::error::Error;
use std::process::Command;
use super::timed_execute::{self, CommandExecutionResult};

pub fn shell(
    command_str: &str,
    verbose: bool,
) -> Result<CommandExecutionResult, Box<dyn Error>> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command_str);
    timed_execute::timed_execute(&mut cmd, verbose)
}