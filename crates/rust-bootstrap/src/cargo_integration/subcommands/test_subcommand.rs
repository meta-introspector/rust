use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;

pub struct TestSubcommandProvider;

impl SubcommandProvider for TestSubcommandProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(Command::new("test"))
    }
}
