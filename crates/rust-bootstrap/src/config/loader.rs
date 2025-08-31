use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub build: Build,
    #[serde(default)]
    pub rust: Rust,
    #[serde(default)]
    pub target: BTreeMap<String, Target>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Build {
    #[serde(default)]
    pub verbose: u8,
    #[serde(default)]
    pub vendor: bool,
    #[serde(default, rename = "locked-deps")]
    pub locked_deps: bool,
    #[serde(default, rename = "build-dir")]
    pub build_dir: String,
    #[serde(default, rename = "bootstrap-cache-path")]
    pub bootstrap_cache_path: String,
    #[serde(default, rename = "patch-binaries-for-nix")]
    pub patch_binaries_for_nix: bool,
    #[serde(default)]
    pub metrics: bool,
    #[serde(default)]
    pub build: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Rust {
    #[serde(default, rename = "deny-warnings")]
    pub deny_warnings: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Target {
    #[serde(default)]
    pub cc: String,
    #[serde(default)]
    pub cxx: String,
    #[serde(default)]
    pub linker: String,
    #[serde(default)]
    pub ar: String,
    #[serde(default)]
    pub ranlib: String,
    #[serde(default, rename = "crt-static")]
    pub crt_static: bool,
}

pub fn load_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}