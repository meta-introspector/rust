use clap::Command;
use cargo::util::command_prelude::flag;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct VersionFlagProvider;

impl ArgumentProvider for VersionFlagProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(flag("version", "Print version info and exit").short('V'))
    }
}
