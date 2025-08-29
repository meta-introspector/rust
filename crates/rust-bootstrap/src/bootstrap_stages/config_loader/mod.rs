use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Default)]
pub struct BuildConfig {
    #[serde(default = "default_false")]
    pub download_ci_rustc: bool,
    #[serde(default = "default_false")]
    pub download_ci_llvm: bool,
    #[serde(default = "default_false", rename = "patch-binaries-for-nix")]
    pub patch_binaries_for_nix: bool,
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub build: BuildConfig,
}

pub mod read_file_content;
pub mod parse_toml_content;

pub fn load_config(config_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_content = read_file_content::read_file_content(config_path)?;
    let config = parse_toml_content::parse_toml_content(&config_content)?;
    Ok(config)
}