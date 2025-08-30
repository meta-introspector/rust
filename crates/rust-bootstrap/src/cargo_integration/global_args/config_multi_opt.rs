use clap::Command;
use cargo::util::command_prelude::multi_opt;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct ConfigMultiOptProvider;

impl ArgumentProvider for ConfigMultiOptProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(multi_opt("config", "KEY=VALUE|PATH", "Override a configuration value").global(true))
    }
}
