use std::error::Error;
use std::process::{Command, Output};
use std::io::{self, Write};

pub fn run_command(
    cmd: &mut Command,
    verbose: bool,
    exception: bool,
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
        if exception {
            return Err(err_msg.into());
        } else {
            eprintln!("Error: {}", err_msg);
            // In bootstrap.py, it sys.exit(1) or sys.exit(err_msg). For now, just return an error.
            return Err(err_msg.into());
        }
    }

    Ok(output)
}
