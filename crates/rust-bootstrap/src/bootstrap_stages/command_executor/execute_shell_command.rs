use std::process::{Command, Output};
use std::error::Error;

pub fn execute_shell_command(command: &str, args: &[&str]) -> Result<Output, Box<dyn Error>> {
    println!("Executing command: {} {:?}", command, args);
    let output = Command::new(command)
        .args(args)
        .output()?;
    Ok(output)
}
