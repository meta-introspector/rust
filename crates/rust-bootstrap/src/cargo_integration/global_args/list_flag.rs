use clap::Command;
use cargo::util::command_prelude::flag;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct ListFlagProvider;

impl ArgumentProvider for ListFlagProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(flag("list", "List installed commands"))
    }
}
