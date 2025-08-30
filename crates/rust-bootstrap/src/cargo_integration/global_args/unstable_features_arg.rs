use clap::{Command, ArgAction, Arg};
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct UnstableFeaturesArgProvider;

impl ArgumentProvider for UnstableFeaturesArgProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(Arg::new("unstable-features")
            .help("Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details")
            .short('Z')
            .value_name("FLAG")
            .action(ArgAction::Append)
            .global(true)
        )
    }
}
