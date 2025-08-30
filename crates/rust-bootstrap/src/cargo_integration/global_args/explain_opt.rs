use clap::Command;
use cargo::util::command_prelude::opt;
use crate::cargo_integration::clap_extensibility::ArgumentProvider;

pub struct ExplainOptProvider;

impl ArgumentProvider for ExplainOptProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.arg(
            opt(
                "explain",
                "Provide a detailed explanation of a rustc error message",
            )
            .value_name("CODE"),
        )
    }
}
