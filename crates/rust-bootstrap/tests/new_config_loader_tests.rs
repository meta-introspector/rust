#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs;
    use rust_bootstrap::config::loader::Config;

    #[test]
    fn test_load_config_success_new() {
        let temp_dir = PathBuf::from("target/new_test_config_loader/success");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let config_content = r#"
[build]
verbose = 1
build = "x86_64-unknown-linux-gnu"

[rust]
deny-warnings = true
"#;
        let config_path = temp_dir.join("bootstrap.toml");
        fs::write(&config_path, config_content).unwrap();

        let result = config_loader::load_config(&config_path);
        assert!(result.is_ok(), "Failed to load config: {:?}", result.err());
        let config = result.unwrap();

        assert_eq!(config.build.build, "x86_64-unknown-linux-gnu");
        assert_eq!(config.rust.deny_warnings, true);

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_load_config_file_not_found_new() {
        let non_existent_path = PathBuf::from("target/new_test_config_loader/non_existent/bootstrap.toml");
        let result = config_loader::load_config(&non_existent_path);
        assert!(result.is_err(), "Expected error for non-existent config file");
        // Optionally, check the error type or message
    }

    #[test]
    fn test_load_config_invalid_format_new() {
        let temp_dir = PathBuf::from("target/new_test_config_loader/invalid");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let config_content = r#"
[build
  target = "x86_64-unknown-linux-gnu"
"#; // Invalid TOML
        let config_path = temp_dir.join("bootstrap.toml");
        fs::write(&config_path, config_content).unwrap();

        let result = config_loader::load_config(&config_path);
        assert!(result.is_err(), "Expected error for invalid config format");

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
