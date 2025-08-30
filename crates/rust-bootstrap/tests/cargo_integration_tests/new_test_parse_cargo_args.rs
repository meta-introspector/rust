#[cfg(test)]
mod tests {
    use crate::GlobalContext;
    use crate::cargo_integration::parse_cargo_args;
    use std::error::Error;
    use tempfile::tempdir;
    use std::path::PathBuf;
    use cargo::core::{Workspace, compiler::{CompileMode, MessageFormat}};
    use cargo::ops::CompileOptions;
    use cargo::util::command_prelude::ProfileChecking;

    fn setup_mock_env() -> Result<(GlobalContext, PathBuf), Box<dyn Error>> {
        let tmp_dir = tempdir()?;
        let rust_root = tmp_dir.path().to_path_buf();

        // Create a dummy Cargo.toml for the workspace
        std::fs::write(rust_root.join("Cargo.toml"), """
[package]
name = "test-package"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
""")?;
        std::fs::create_dir_all(rust_root.join("src"))?;
        std::fs::write(rust_root.join("src/lib.rs"), """
pub fn hello() { println!("Hello"); }
""")?;

        let gctx = GlobalContext::new()?; // Assuming GlobalContext can be created without complex setup
        Ok((gctx, rust_root))
    }

    #[test]
    fn test_parse_cargo_args_build() -> Result<(), Box<dyn Error>> {
        let (gctx, rust_root) = setup_mock_env()?;
        let raw_args = &["build"];

        let compile_options = parse_cargo_args(&gctx, raw_args, &rust_root)?;

        assert_eq!(compile_options.mode, CompileMode::Build);
        assert_eq!(compile_options.message_format, MessageFormat::Human);
        assert_eq!(compile_options.profile_checking, ProfileChecking::Custom);
        // Add more assertions for default build options

        Ok(())
    }

    #[test]
    fn test_parse_cargo_args_check() -> Result<(), Box<dyn Error>> {
        let (gctx, rust_root) = setup_mock_env()?;
        let raw_args = &["check"];

        let compile_options = parse_cargo_args(&gctx, raw_args, &rust_root)?;

        assert_eq!(compile_options.mode, CompileMode::Check);
        Ok(())
    }

    #[test]
    fn test_parse_cargo_args_release() -> Result<(), Box<dyn Error>> {
        let (gctx, rust_root) = setup_mock_env()?;
        let raw_args = &["build", "--release"];

        let compile_options = parse_cargo_args(&gctx, raw_args, &rust_root)?;

        assert!(compile_options.build_config.release);
        Ok(())
    }

    #[test]
    fn test_parse_cargo_args_verbose() -> Result<(), Box<dyn Error>> {
        let (gctx, rust_root) = setup_mock_env()?;
        let raw_args = &["build", "-v"];

        let compile_options = parse_cargo_args(&gctx, raw_args, &rust_root)?;

        assert_eq!(compile_options.build_config.log_level, Some(cargo::util::config::LogLevel::Verbose));
        Ok(())
    }

    // Add more tests for other arguments and error cases
}
