use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;

pub struct RunProvider;

impl SubcommandProvider for RunProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(
            Command::new("run")
                .about("Run a binary or example of the current package")
                .trailing_var_arg(true) // Allow passing args to the binary
                .allow_hyphen_values(true)
                // Add specific arguments for 'run' if needed
        )
    }
}

// This function will be called by the main dispatch logic
pub fn handle_run_command(
    gctx: &cargo::GlobalContext,
    ws: &cargo::core::Workspace,
    subcommand_args_str: &[&str],
    rust_root: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // For run, the subcommand_args_str are the args to pass to the binary
    // cargo::ops::run expects a CompileOptions and then the args for the binary
    let run_options = crate::cargo_integration::parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;
    cargo::ops::run(&ws, &run_options, &subcommand_args_str.iter().map(|s| s.to_string().into()).collect::<Vec<_>>())?;
    Ok(())
}
