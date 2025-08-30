use clap::Command;
use cargo::util::command_prelude::flag;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct LockedFlagProvider;

impl ArgumentProvider for LockedFlagProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(
            flag("locked", "Assert that `Cargo.lock` will remain unchanged")
                .help_heading("Manifest Options")
                .global(true),
        )
    }
}
