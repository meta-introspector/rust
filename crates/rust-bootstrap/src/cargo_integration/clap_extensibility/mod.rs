use clap::Command;

pub trait ArgumentProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command;
}

pub trait SubcommandProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command;
}
