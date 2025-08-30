use clap::Command;
use cargo::util::command_prelude::opt;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct ColorOptProvider;

impl ArgumentProvider for ColorOptProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(
            opt("color", "Coloring")
                .value_name("WHEN")
                .global(true)
                .value_parser(["auto", "always", "never"])
                .ignore_case(true),
        )
    }
}
