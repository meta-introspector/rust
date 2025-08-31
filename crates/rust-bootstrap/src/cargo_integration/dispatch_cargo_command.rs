use std::error::Error;
use std::path::PathBuf;

use cargo::core::Workspace;
use cargo::GlobalContext;
 // Added CleanOptions
 // Keep this for global_matches.subcommand_name()

// Import the handle functions from the new subcommand modules
use crate::cargo_integration::subcommands::build::handle_build_command;
use crate::cargo_integration::subcommands::check::handle_check_command;
use crate::cargo_integration::subcommands::run::handle_run_command;
use crate::cargo_integration::subcommands::clean::handle_clean_command;


pub fn dispatch_cargo_command(
    global_matches: &cargo::util::command_prelude::ArgMatches,
    subcommand_args: &Vec<String>,
    gctx: &GlobalContext,
    ws: &Workspace,
    rust_root: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let subcommand_args_str: Vec<&str> = subcommand_args.iter().map(|s| s.as_str()).collect();

    let subcommand_name = global_matches.subcommand_name();
    tracing::debug!("dispatch_cargo_command: subcommand_name: {:?}", subcommand_name);
    tracing::debug!("dispatch_cargo_command: subcommand_args: {:?}", subcommand_args);
    println!("DEBUG: dispatch_cargo_command: subcommand_name: {:?}", subcommand_name);
    println!("DEBUG: dispatch_cargo_command: subcommand_args: {:?}", subcommand_args);

    match subcommand_name {
        Some("build") => handle_build_command(gctx, ws, &subcommand_args_str, rust_root)?,
        Some("check") => handle_check_command(gctx, ws, &subcommand_args_str, rust_root)?,
        Some("run") => handle_run_command(gctx, ws, &subcommand_args_str, rust_root)?,
        Some("clean") => handle_clean_command(gctx, ws, &subcommand_args_str, rust_root)?,
        _ => {
            println!("Unsupported Cargo subcommand: {:?}", subcommand_name);
            return Err(Box::from(format!("Unsupported Cargo subcommand: {:?}", subcommand_name)));
        }
    }

    Ok(())
}
