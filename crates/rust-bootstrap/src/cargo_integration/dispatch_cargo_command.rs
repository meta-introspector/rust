use std::error::Error;
use std::path::PathBuf;

use cargo::ops::compile;
use cargo::util::command_prelude::ArgMatchesExt;
use cargo::core::Workspace;
use cargo::GlobalContext;

use crate::cargo_integration::parse_cargo_args;

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

    match subcommand_name {
        Some("build") | Some("check") => {
            let compile_options = parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;
            compile(&ws, &compile_options)?;
        },
        // TODO: Add placeholders for other common Cargo subcommands (e.g., test, run, clean)
        // They will require their own `cargo::ops` functions and potentially different option structs.
        _ => {
            println!("Unsupported Cargo subcommand: {:?}", subcommand_name);
            return Err(Box::from(format!("Unsupported Cargo subcommand: {:?}", subcommand_name)));
        }
    }

    Ok(())
}
