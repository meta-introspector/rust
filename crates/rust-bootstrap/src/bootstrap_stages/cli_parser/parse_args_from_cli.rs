use crate::bootstrap_stages::cli_parser::create_args_struct::Args;
use clap::Parser;

pub fn parse_args_from_cli() -> Args {
    Args::parse()
}