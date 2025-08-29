use std::path::{PathBuf, Path};
use super::Stage0;
use super::detect_rustc_path;
use super::detect_cargo_path;
use crate::bootstrap_stages::stage0_parser; // Import the new parser

impl Stage0 {
    pub fn detect() -> Self {
        let rustc_path = detect_rustc_path::detect_rustc_path();
        let cargo_path = detect_cargo_path::detect_cargo_path();

        // Parse src/stage0 file
        let stage0_file_path = Path::new("src/stage0"); // Assuming relative to rust_root
        let stage0_data = stage0_parser::parse_stage0_file::parse_stage0_file(
            stage0_file_path.to_str().expect("Invalid path")
        ).expect("Failed to parse src/stage0 file");

        let compiler_date = stage0_data.get("compiler_date")
            .expect("compiler_date not found in src/stage0")
            .clone();
        let compiler_version = stage0_data.get("compiler_version")
            .expect("compiler_version not found in src/stage0")
            .clone();
        let dist_server = stage0_data.get("dist_server")
            .expect("dist_server not found in src/stage0")
            .clone();

        Stage0 {
            rustc: rustc_path,
            cargo: cargo_path,
            compiler_date,
            compiler_version,
            dist_server,
        }
    }
}
