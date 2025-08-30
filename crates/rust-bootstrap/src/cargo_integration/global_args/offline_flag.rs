use clap::Command;
use cargo::util::command_prelude::flag;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct OfflineFlagProvider;

impl ArgumentProvider for OfflineFlagProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(
            flag("offline", "Run without accessing the network")
                .help_heading("Manifest Options")
                .global(true),
        )
    }
}
