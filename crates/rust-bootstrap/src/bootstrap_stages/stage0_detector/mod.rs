


#[derive(Debug, Clone)]
pub struct Stage0 {
    pub rustc: PathBuf,
    pub cargo: PathBuf,
    pub compiler_date: String,
    pub compiler_version: String,
    pub dist_server: String,
}

pub mod detect_rustc_path;
pub mod detect_cargo_path;

use std::path::PathBuf;

pub mod detect_stage0;
