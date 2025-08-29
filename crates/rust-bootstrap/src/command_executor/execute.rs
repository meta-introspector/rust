use std::error::Error;
use std::process::{Command, Output};
use std::io::{self, Write};

pub fn execute(
    cmd: &mut Command,
    verbose: bool,
) -> Result<Output, Box<dyn Error>> {
    if verbose {
        println!("running: {:?}", cmd);
    }
    io::stdout().flush()?;

    let output = cmd.output()?;

    if !output.status.success() {
        let err_msg = format!(
            "failed to run: {:?}\nStdout: {}\nStderr: {}",
            cmd,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(err_msg.into());
    }

    Ok(output)
}