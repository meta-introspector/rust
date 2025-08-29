use crate::bootstrap_stages::command_executor;
use crate::bootstrap_stages::stage0_detector::Stage0;
use std::error::Error;
use std::string::String;

pub fn execute_and_report_command(
    stage0: &Stage0,
) -> Result<command_executor::timed_execute::CommandExecutionResult, Box<dyn Error>> {
    println!("Before running cargo --version (via sh -c)..."); // Trace 1

    let command_str = format!("{} --version", stage0.cargo.to_string_lossy());
    let command_result = shell::shell(&command_str, true)?;

    println!("Status: {}", command_result.output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&command_result.output.stdout));
    eprintln!("Stderr: {}", String::from_utf8_lossy(&command_result.output.stderr));

    println!("After cargo --version execution (via sh -c). Duration: {} ns", command_result.duration); // New Trace
    Ok(command_result)
}
