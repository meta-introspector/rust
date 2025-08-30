use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;

pub struct CheckProvider;

impl SubcommandProvider for CheckProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(
            Command::new("check")
                .about("Check the current package for errors")
                // Add specific arguments for 'check' if needed
        )
    }
}

// This function will be called by the main dispatch logic
pub fn handle_check_command(
    gctx: &cargo::GlobalContext,
    ws: &cargo::core::Workspace,
    subcommand_args_str: &[&str],
    rust_root: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let compile_options = crate::cargo_integration::parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;
    cargo::ops::compile(&ws, &compile_options)?; // 'check' also uses compile op
    Ok(())
}
