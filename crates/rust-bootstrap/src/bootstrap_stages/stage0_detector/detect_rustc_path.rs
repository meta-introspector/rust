use std::env;
use std::path::PathBuf;

pub fn detect_rustc_path() -> PathBuf {
    env::var_os("RUSTC")
        .unwrap_or_else(|| "/data/data/com.termux/files/usr/bin/rustc".into())
        .into()
}