use crate::bootstrap_stages::cli_parser;
use crate::bootstrap_stages::config_loader;
use std::error::Error;

pub fn load_and_print_config(args: &cli_parser::Args) -> Result<config_loader::Config, Box<dyn Error>> {
    let config_path_str = args.config.clone().unwrap_or_else(|| {
        "/data/data/com.termux/files/home/storage/github/rust/config.toml".to_string()
    });
    let config = config_loader::load_config(&config_path_str)?;

    println!("Config: {:?}\n", config);
    Ok(config)
}
