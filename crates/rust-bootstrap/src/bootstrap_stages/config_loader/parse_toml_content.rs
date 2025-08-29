use toml;
use crate::bootstrap_stages::config_loader::Config; // Assuming Config is in the parent module

pub fn parse_toml_content(content: &str) -> Result<Config, Box<dyn std::error::Error>> {
    Ok(toml::from_str(content)?)
}