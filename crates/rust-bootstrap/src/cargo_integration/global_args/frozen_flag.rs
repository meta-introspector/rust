use clap::Command;
use cargo::util::command_prelude::flag;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct FrozenFlagProvider;

impl ArgumentProvider for FrozenFlagProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(
            flag("frozen", "Equivalent to specifying both --locked and --offline")
                .help_heading("Manifest Options")
                .global(true),
        )
    }
}
