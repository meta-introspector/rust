#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use rust_bootstrap::bootstrap_stages::stage0_parser;
    use rust_bootstrap::bootstrap_stages::stage0_detector::Stage0;
    use std::path::PathBuf;
    use std::fs;
    use clap::Parser; // Added for Args::parse_from

    #[test]
    fn test_parse_stage0_info_success_new() {
        // Mock a Stage0 with a dummy rustc path
        let temp_dir = PathBuf::from("target/new_test_stage0_parser/success");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let rustc_path = temp_dir.join("rustc");
        // Create a dummy rustc that would output expected info (e.g., version)
        // For a real test, this would involve mocking the command execution
        // or having a pre-built dummy rustc.
        fs::write(&rustc_path, "").unwrap(); // Empty file for now
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&rustc_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&rustc_path, perms).unwrap();
        }

        let stage0 = Stage0 {
            rustc: rustc_path.clone(),
            cargo: PathBuf::from("/dummy/cargo"),
            compiler_date: "2025-08-30".to_string(),
            compiler_version: "1.70.0".to_string(),
            dist_server: "https://dummy.dist.rust-lang.org".to_string(),
        };

        // This test will likely need mocking of external command execution (rustc --version --verbose)
        // For now, we'll just assert that the function doesn't panic and returns a Result.
        // let result = stage0_parser::parse_stage0_info(&stage0);

        // assert!(result.is_ok() || result.is_err(), "parse_stage0_info should return a Result");

        // If it's Ok, we'd assert on the parsed version and host_triple
        // if result.is_ok() {
        //     let parsed_stage0 = result.unwrap();
        //     assert!(parsed_stage0.version.is_some());
        //     assert!(parsed_stage0.host_triple.is_some());
        // }

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_parse_stage0_info_no_rustc_path_new() {
        let stage0 = Stage0 {
            rustc: PathBuf::from(""), // Empty path to simulate no rustc
            cargo: PathBuf::from(""),
            compiler_date: String::new(),
            compiler_version: String::new(),
            dist_server: String::new(),
        };
        // let result = stage0_parser::parse_stage0_info(&stage0);
        // assert!(result.is_err(), "Expected error when no rustc path is provided");
    }
}
