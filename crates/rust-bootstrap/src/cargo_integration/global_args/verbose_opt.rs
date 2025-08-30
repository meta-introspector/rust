use clap::{Command, ArgAction};
use cargo::util::command_prelude::opt;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct VerboseOptProvider;

impl ArgumentProvider for VerboseOptProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(
            opt(
                "verbose",
                "Use verbose output (-vv very verbose/build.rs output)",
            )
            .short('v')
            .action(ArgAction::Count)
            .global(true),
        )
    }
}
