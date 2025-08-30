use rust_bootstrap::Args; // Assuming Args struct is public and in lib.rs
use clap::Parser;

#[test]
fn test_basic_arg_parsing() {
    let args = Args::parse_from(&["rust-bootstrap", "--config", "my_config.toml", "--exec-panic"]);
    assert_eq!(args.config, Some("my_config.toml".to_string()));
    assert!(args.exec_panic);
    assert!(args.cargo_args.is_empty());
}

#[test]
fn test_cargo_args_parsing() {
    let args = Args::parse_from(&["rust-bootstrap", "build", "--release", "--", "-j", "4"]);
    assert_eq!(args.config, None);
    assert!(!args.exec_panic);
    assert_eq!(args.cargo_args, vec!["build", "--release", "--", "-j", "4"]);
}

#[test]
fn test_default_values() {
    let args = Args::parse_from(&["rust-bootstrap"]);
    assert_eq!(args.config, None);
    assert!(!args.exec_panic);
    assert!(args.cargo_args.is_empty());
}

#[test]
fn test_mixed_args() {
    let args = Args::parse_from(&["rust-bootstrap", "--exec-panic", "check", "--all-features"]);
    assert_eq!(args.config, None);
    assert!(args.exec_panic);
    assert_eq!(args.cargo_args, vec!["check", "--all-features"]);
}
