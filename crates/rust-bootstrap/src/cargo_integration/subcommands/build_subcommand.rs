use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;

pub struct BuildSubcommandProvider;

impl SubcommandProvider for BuildSubcommandProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(Command::new("build"))
    }
}
