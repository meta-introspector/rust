use std::env;
use std::path::PathBuf;

pub fn detect_cargo_path() -> PathBuf {
    env::var_os("CARGO")
        .unwrap_or_else(|| "/data/data/com.termux/files/usr/bin/cargo".into())
        .into()
}