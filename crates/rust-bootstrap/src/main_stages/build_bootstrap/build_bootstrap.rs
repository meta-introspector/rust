use std::error::Error;
use std::process::Command;
use crate::build_state::BuildState;
use super::build_bootstrap_cmd; // Import the function to build the command

pub fn build_bootstrap(build_state: &BuildState) -> Result<(), Box<dyn Error>> {
    println!("Building bootstrap...");

    let mut cmd = build_bootstrap_cmd::build_bootstrap_cmd(build_state)?;

    // Execute the command
    let output = cmd.output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to build bootstrap:\nStdout: {}\nStderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }

    println!("Bootstrap built successfully.");
    Ok(())
}
