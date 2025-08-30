use clap::Command;
use cargo::util::command_prelude::flag;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct QuietFlagProvider;

impl ArgumentProvider for QuietFlagProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(flag("quiet", "Do not print cargo log messages").short('q').global(true))
    }
}
