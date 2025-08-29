use std::env;
use std::path::PathBuf;

pub fn set_cargo_stage0(cargo_path: &PathBuf) {
    env::set_var("CARGO_STAGE0", cargo_path);
}