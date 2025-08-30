use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;

pub struct CheckSubcommandProvider;

impl SubcommandProvider for CheckSubcommandProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(Command::new("check"))
    }
}
