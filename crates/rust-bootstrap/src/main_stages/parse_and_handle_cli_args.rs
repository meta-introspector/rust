use crate::bootstrap_stages::cli_parser;

pub fn parse_and_handle_cli_args() -> cli_parser::Args {
    let args = cli_parser::parse_cli_arguments();

    if args.verbose > 0 {
        println!("Verbose mode is on!");
    }
    args
}