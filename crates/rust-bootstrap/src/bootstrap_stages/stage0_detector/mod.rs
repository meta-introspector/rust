
use std::path::PathBuf;

#[derive(Debug)]
pub struct Stage0 {
    pub rustc: PathBuf,
    pub cargo: PathBuf,
}

pub mod detect_rustc_path;
pub mod detect_cargo_path;

impl Stage0 {
    pub fn detect() -> Self {
        let rustc_path = detect_rustc_path::detect_rustc_path();
        let cargo_path = detect_cargo_path::detect_cargo_path();

        Stage0 {
            rustc: rustc_path,
            cargo: cargo_path,
        }
    }
}
