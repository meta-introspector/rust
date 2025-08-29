

pub mod create_args_struct;
pub mod parse_args_from_cli;

pub use create_args_struct::Args;



pub fn parse_cli_arguments() -> Args {
    parse_args_from_cli::parse_args_from_cli()
}