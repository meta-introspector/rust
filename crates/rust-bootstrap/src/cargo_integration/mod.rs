use std::error::Error;
use std::path::PathBuf;

pub mod init_global_context;
pub mod init_workspace;
pub mod create_compile_options;
pub mod parse_cargo_args;
pub mod parse_global_args;
pub mod global_args;
pub mod subcommands;
pub mod clap_extensibility;

// The `mock` module was removed as part of simplifying the cargo integration
// and moving towards a more direct mocking approach within the tests themselves.
// #[cfg(test)]
// pub mod mock;

// This function is responsible for running cargo commands.
use cargo::util::command_prelude::ArgMatchesExt;

pub mod dispatch_cargo_command;

pub fn run_cargo_command(args: &[&str], rust_root: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Temporarily change current directory to rust_root
    let original_cwd = std::env::current_dir()?;
    std::env::set_current_dir(rust_root)?;

    let (global_matches, subcommand_args) = match parse_global_args::parse_global_args(args) {
        Ok(result) => result,
        Err(e) => {
            return Err(e.into());
        }
    };

    let mut gctx = init_global_context::init_global_context()?;

    // Configure GlobalContext with global arguments
    gctx.configure(
        global_matches.verbose(),
        global_matches.flag("quiet"),
        global_matches.get_one::<String>("color").map(String::as_str),
        global_matches.flag("frozen"),
        global_matches.flag("locked"),
        global_matches.flag("offline"),
        &None, // target_dir, not handled by global args
        &global_matches.get_many::<String>("unstable-features").unwrap_or_default().cloned().collect::<Vec<String>>(),
        &global_matches.get_many::<String>("config").unwrap_or_default().cloned().collect::<Vec<String>>(),
    )?;

    let ws = init_workspace::init_workspace(&gctx, rust_root)?;

    dispatch_cargo_command::dispatch_cargo_command(
        &global_matches,
        &subcommand_args,
        &gctx,
        &ws,
        rust_root,
    )?;

    // Restore original current directory
    std::env::set_current_dir(original_cwd)?;

    Ok(())
}
