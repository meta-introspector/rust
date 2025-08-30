use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;

pub struct BuildProvider;

impl SubcommandProvider for BuildProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(
            Command::new("build")
                .about("Compile the current package")
                // Add specific arguments for 'build' if needed
        )
    }
}

// This function will be called by the main dispatch logic
pub fn handle_build_command(
    gctx: &cargo::GlobalContext,
    ws: &cargo::core::Workspace,
    subcommand_args_str: &[&str],
    rust_root: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let compile_options = crate::cargo_integration::parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;
    cargo::ops::compile(&ws, &compile_options)?;
    Ok(())
}
