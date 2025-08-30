use cargo::util::CargoResult;
use clap::ArgMatches;
use tracing;

// Include the generated file that contains get_argument_providers and get_subcommand_providers
include!(concat!(env!("OUT_DIR"), "/generated_clap_providers.rs"));

pub fn parse_global_args(raw_args: &[&str]) -> CargoResult<(ArgMatches, Vec<String>)> {
    tracing::debug!("parse_global_args: raw_args: {:?}", raw_args);
    let mut command = Command::new("cargo");

    // Add global arguments
    for provider in get_argument_providers() {
        command = provider.add_to_command(command);
    }

    // Add subcommands
    for provider in get_subcommand_providers() {
        command = provider.add_to_command(command);
    }

    let mut full_args = vec!["cargo"]; // Prepend "cargo"
    full_args.extend_from_slice(raw_args);
    tracing::debug!("parse_global_args: full_args: {:?}", full_args);
    let matches = command.try_get_matches_from(full_args)?;

    let subcommand_name = matches.subcommand_name();
    tracing::debug!("parse_global_args: matches.subcommand_name(): {:?}", subcommand_name);
    if let Some(name) = subcommand_name {
        tracing::debug!("Parsed subcommand name: {}", name);
    }
    let subcommand_args = matches.subcommand().map(|(_, args)| args.clone());

    let remaining_args: Vec<String> = if let Some(sub_args) = subcommand_args {
        sub_args.get_many::<String>("").unwrap_or_default().cloned().collect()
    } else {
        Vec::new()
    };

    Ok((matches, remaining_args))
}
