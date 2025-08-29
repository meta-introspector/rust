use std::error::Error;
use std::path::PathBuf;

pub mod init_global_context;
pub mod init_workspace;
pub mod create_compile_options;
pub mod parse_cargo_args;
pub mod parse_global_args;

use cargo::ops::compile;
use cargo::util::command_prelude::ArgMatchesExt;

pub fn run_cargo_command(args: &[&str], rust_root: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Running cargo command via integration: {:?}", args);
    println!("rust_root: {:?}", rust_root);

    // Temporarily change current directory to rust_root
    let original_cwd = std::env::current_dir()?;
    std::env::set_current_dir(rust_root)?;

    let (global_matches, subcommand_args) = parse_global_args::parse_global_args(args)?;

    let mut gctx = init_global_context::init_global_context()?;
    println!("gctx.cwd(): {:?}", gctx.cwd());

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
    println!("ws.root(): {:?}", ws.root());

    // Convert Vec<String> to Vec<&str> for parse_cargo_args
    let subcommand_args_str: Vec<&str> = subcommand_args.iter().map(|s| s.as_str()).collect();
    let compile_options = parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;

    compile(&ws, &compile_options)?;

    // Restore original current directory
    std::env::set_current_dir(original_cwd)?;

    Ok(())
}
