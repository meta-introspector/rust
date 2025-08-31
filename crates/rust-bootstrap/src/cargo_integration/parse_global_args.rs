use cargo::util::CargoResult;
use clap::ArgMatches;
use tracing;
use cargo::util::command_prelude::CommandExt;

// Include the generated file that contains get_argument_providers and get_subcommand_providers
include!(concat!(env!("OUT_DIR"), "/generated_clap_providers.rs"));

pub fn parse_global_args(raw_args: &[&str]) -> CargoResult<(ArgMatches, Vec<String>)> {
    println!("DEBUG: parse_global_args: raw_args: {:?}", raw_args);
    tracing::debug!("parse_global_args: raw_args: {:?}", raw_args);
    let mut command = Command::new("cargo");

    // Add global arguments
    for provider in get_argument_providers() {
        command = provider.add_to_command(command);
    }

    // Explicitly add build subcommand with its arguments for debugging
    command = command.subcommand(
        Command::new("build")
            .about("Compile the current package")
            .arg_manifest_path()
            .arg_target_dir()
            .arg_features()
            .arg_release("Build artifacts in release mode, with optimizations")
            .arg_profile("Build artifacts with the specified profile")
            .arg_target_triple("Build for the target triple")
            .arg_build_plan()
            .arg_unit_graph()
            .arg_timings()
            .arg_compile_time_deps()
            .arg_lockfile_path()
            .arg_ignore_rust_version()
            .arg_message_format()
            .arg_silent_suggestion()
            .arg_package_spec(
                "Package to build (see `cargo help pkgid`)",
                "Build all packages in the workspace",
                "Exclude packages from the build",
            )
            .arg_targets_all(
                "Build only this package's library",
                "Build only the specified binary",
                "Build all binaries",
                "Build only the specified example",
                "Build all examples",
                "Build only the specified test target",
                "Build all targets that have `test = true` set",
                "Build only the specified bench target",
                "Build all targets that have `bench = true` set",
                "Build all targets",
            )
            .arg_parallel()
    );

    // Add other subcommands (excluding build, as it's added explicitly above)
    for provider in get_subcommand_providers() {
        if provider.name() != "build" {
            command = provider.add_to_command(command);
        }
    }

    let mut full_args = vec!["cargo"]; // Prepend "cargo"
    full_args.extend_from_slice(raw_args);
    println!("DEBUG: parse_global_args: full_args: {:?}", full_args);
    tracing::debug!("parse_global_args: full_args: {:?}", full_args);
    let matches = command.try_get_matches_from(full_args)?;

    let subcommand_name = matches.subcommand_name();
    println!("DEBUG: parse_global_args: subcommand_name: {:?}", subcommand_name);
    tracing::debug!("parse_global_args: matches.subcommand_name(): {:?}", subcommand_name);
    if let Some(name) = subcommand_name {
        println!("DEBUG: parse_global_args: Parsed subcommand name: {}", name);
        tracing::debug!("Parsed subcommand name: {}", name);
    }
    let subcommand_args = matches.subcommand().map(|(_, args)| args.clone());

    let remaining_args: Vec<String> = if let Some(sub_args) = subcommand_args {
        let collected_args = sub_args.get_many::<String>("").unwrap_or_default().cloned().collect();
        println!("DEBUG: parse_global_args: remaining_args (from subcommand): {:?}", collected_args);
        collected_args
    } else {
        let collected_args = Vec::new();
        println!("DEBUG: parse_global_args: remaining_args (no subcommand): {:?}", collected_args);
        collected_args
    };

    println!("DEBUG: parse_global_args: Final remaining_args: {:?}", remaining_args);

    Ok((matches, remaining_args))
}
