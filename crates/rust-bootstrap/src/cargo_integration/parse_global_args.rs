use cargo::util::CargoResult;
use clap::{Command, ArgMatches, ArgAction, Arg};
use cargo::util::command_prelude::{flag, opt, multi_opt};
use tracing;

pub fn parse_global_args(raw_args: &[&str]) -> CargoResult<(ArgMatches, Vec<String>)> {
    let command = Command::new("cargo")
        .arg(flag("version", "Print version info and exit").short('V'))
        .arg(flag("list", "List installed commands"))
        .arg(
            opt(
                "explain",
                "Provide a detailed explanation of a rustc error message",
            )
            .value_name("CODE"),
        )
        .arg(
            opt(
                "verbose",
                "Use verbose output (-vv very verbose/build.rs output)",
            )
            .short('v')
            .action(ArgAction::Count)
            .global(true),
        )
        .arg(flag("quiet", "Do not print cargo log messages").short('q').global(true))
        .arg(
            opt("color", "Coloring")
                .value_name("WHEN")
                .global(true)
                .value_parser(["auto", "always", "never"])
                .ignore_case(true),
        )
        .arg(
            flag("locked", "Assert that `Cargo.lock` will remain unchanged")
                .help_heading("Manifest Options")
                .global(true),
        )
        .arg(
            flag("offline", "Run without accessing the network")
                .help_heading("Manifest Options")
                .global(true),
        )
        .arg(
            flag("frozen", "Equivalent to specifying both --locked and --offline")
                .help_heading("Manifest Options")
                .global(true),
        )
        .arg(multi_opt("config", "KEY=VALUE|PATH", "Override a configuration value").global(true))
        .arg(Arg::new("unstable-features")
            .help("Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details")
            .short('Z')
            .value_name("FLAG")
            .action(ArgAction::Append)
            .global(true)
        );

    let matches = command.try_get_matches_from(raw_args)?;

    let subcommand_name = matches.subcommand_name();
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
