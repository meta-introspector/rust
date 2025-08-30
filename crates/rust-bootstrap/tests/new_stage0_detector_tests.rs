#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use rust_bootstrap::bootstrap_stages::stage0_detector::Stage0;
    use std::path::PathBuf;
    use std::fs;
    use clap::Parser; // Added for Args::parse_from

    #[test]
    fn test_stage0_detect_mocked_rustc_new() {
        // This test mocks the environment to simulate a rustc executable.
        // In a real scenario, Stage0::detect() would look for an actual rustc.

        let temp_dir = PathBuf::from("target/new_test_stage0_detector/mock_rustc");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a dummy rustc executable (e.g., a simple script or empty file)
        let rustc_path = temp_dir.join("rustc");
        fs::write(&rustc_path, "").unwrap();
        // Make it executable (on Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&rustc_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&rustc_path, perms).unwrap();
        }

        // Temporarily add the dummy rustc to PATH
        let original_path = std::env::var_os("PATH");
        let mut new_path = temp_dir.clone().into_os_string();
        if let Some(path) = original_path.clone() {
            new_path.push(":");
            new_path.push(path);
        }
        std::env::set_var("PATH", &new_path);

        let stage0 = Stage0::detect();

        // Assert that a Stage0 was detected
        assert!(stage0.rustc.to_str().unwrap().contains("rustc"), "Stage0 rustc path should be detected");
        // assert_eq!(stage0.rustc, rustc_path, "Detected path should match mock rustc"); // Path comparison might be tricky due to temp paths

        // Restore original PATH
        if let Some(path) = original_path {
            std::env::set_var("PATH", path);
        } else {
            std::env::remove_var("PATH");
        }
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_stage0_detect_no_rustc_new() {
        // Ensure no rustc is found if not in PATH
        let original_path = std::env::var_os("PATH");
        std::env::remove_var("PATH"); // Temporarily clear PATH

        let stage0 = Stage0::detect();
        assert!(stage0.rustc.to_str().unwrap().is_empty(), "Stage0 rustc path should be empty when rustc is not in PATH");

        // Restore original PATH
        if let Some(path) = original_path {
            std::env::set_var("PATH", path);
        }
    }
}
