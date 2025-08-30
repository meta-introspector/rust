use std::process::{Command, Output};
use std::error::Error;

pub fn execute_shell_command(command: &str, args: &[&str], exec_panic: bool) -> Result<Output, Box<dyn Error>> {
    if exec_panic {
        panic!("Shell commands are not allowed in the new system. Attempted to execute: {} with args: {:?}", command, args);
    }
    println!("Executing command: {} {:?}", command, args);
    let output = Command::new(command)
        .args(args)
        .output()?;
    Ok(output)
}
