#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use rust_bootstrap::bootstrap_stages::stage0_parser::parse_stage0_file::parse_stage0_file;
    use std::path::PathBuf;
    use std::fs;
    use std::collections::HashMap;

    #[test]
    fn test_parse_stage0_file_success_v2() {
        let temp_dir = PathBuf::from("target/new_test_stage0_parser_v2/success");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let stage0_file_content = r#"
rustc=/path/to/rustc
cargo=/path/to/cargo
compiler_date=2025-08-30
compiler_version=1.70.0
dist_server=https://example.com
"#;
        let file_path = temp_dir.join("stage0_info.txt");
        fs::write(&file_path, stage0_file_content).unwrap();

        let result = parse_stage0_file(file_path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to parse stage0 file: {:?}", result.err());
        let parsed_info = result.unwrap();

        assert_eq!(parsed_info.get("rustc"), Some(&"/path/to/rustc".to_string()));
        assert_eq!(parsed_info.get("cargo"), Some(&"/path/to/cargo".to_string()));
        assert_eq!(parsed_info.get("compiler_date"), Some(&"2025-08-30".to_string()));
        assert_eq!(parsed_info.get("compiler_version"), Some(&"1.70.0".to_string()));
        assert_eq!(parsed_info.get("dist_server"), Some(&"https://example.com".to_string()));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_parse_stage0_file_not_found_v2() {
        let non_existent_path = PathBuf::from("target/new_test_stage0_parser_v2/non_existent/stage0_info.txt");
        let result = parse_stage0_file(non_existent_path.to_str().unwrap());
        assert!(result.is_err(), "Expected error for non-existent stage0 file");
    }

    #[test]
    fn test_parse_stage0_file_empty_content_v2() {
        let temp_dir = PathBuf::from("target/new_test_stage0_parser_v2/empty");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let file_path = temp_dir.join("stage0_info.txt");
        fs::write(&file_path, "").unwrap();

        let result = parse_stage0_file(file_path.to_str().unwrap());
        assert!(result.is_ok(), "Expected success for empty file");
        assert!(result.unwrap().is_empty(), "Expected empty HashMap for empty file");

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
