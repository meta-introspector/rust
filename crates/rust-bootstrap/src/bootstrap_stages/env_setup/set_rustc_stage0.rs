use std::env;
use std::path::PathBuf;

pub fn set_rustc_stage0(rustc_path: &PathBuf) {
    env::set_var("RUSTC_STAGE0", rustc_path);
}