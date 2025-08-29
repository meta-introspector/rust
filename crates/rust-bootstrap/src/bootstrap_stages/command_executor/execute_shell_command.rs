use std::process::{Command, Output};
use std::error::Error;

pub fn execute_shell_command(command_str: &str) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()?)
}