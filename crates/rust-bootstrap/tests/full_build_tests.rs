mod helpers;

#[cfg(test)]
mod tests {
    use super::helpers;
    use rust_bootstrap::bootstrap_stages::build_bootstrap;
    use rust_bootstrap::builder::Builder;
    
    use std::fs;

    #[test]
    fn test_full_bootstrap_build() {
        let build_state = helpers::setup_test_build_state();

        // Call the main build function
        let result = build_bootstrap::build_bootstrap(&build_state);
        println!("DEBUG: Result of build_bootstrap: {:?}", result);

        // Assert that the build was successful
        assert!(result.is_ok(), "Full bootstrap build should succeed: {:?}", result);

        // Verify that the bootstrap binary exists
        let builder = Builder::new(&build_state);
        let bootstrap_binary_path = builder.bootstrap_binary();
        println!("DEBUG: Expected bootstrap binary path: {:?}", bootstrap_binary_path);
        assert!(bootstrap_binary_path.exists(), "Bootstrap binary should exist at {:?}", bootstrap_binary_path);

        // Cleanup
        if build_state.creation_args.rust_root.exists() {
            fs::remove_dir_all(&build_state.creation_args.rust_root).unwrap();
        }
    }
}
