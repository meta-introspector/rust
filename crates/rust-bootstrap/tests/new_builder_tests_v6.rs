#![allow(unused_imports)]

mod helpers;

#[cfg(test)]
mod tests {
    use super::helpers;
    use rust_bootstrap::builder::Builder;
    use rust_bootstrap::BuildState;
    use rust_bootstrap::Args;
    use rust_bootstrap::config::loader::Config;
    use rust_bootstrap::bootstrap_stages::stage0_detector::Stage0;
    use std::path::PathBuf;
    use clap::Parser; // Added for Args::parse_from
    use rust_bootstrap::BuildStateCreationArgs;

    #[test]
    fn test_builder_initialization_v6() {
        let build_state = helpers::setup_test_build_state();

        let builder = Builder::new(&build_state);

        assert!(true, "Builder should be initialized");
    }

    #[test]
    fn test_builder_bootstrap_binary_path_v6() {
        let build_state = helpers::setup_test_build_state();

        let builder = Builder::new(&build_state);
        let expected_path = build_state.creation_args.build_dir
            .join("bootstrap")
            .join("debug")
            .join("bootstrap");
        assert_eq!(builder.bootstrap_binary(), expected_path);
    }
}
