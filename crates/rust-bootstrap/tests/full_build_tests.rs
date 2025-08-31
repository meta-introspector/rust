mod helpers;

#[cfg(test)]
mod tests {
    use super::helpers;
    use rust_bootstrap::bootstrap_stages::build_bootstrap;
    use rust_bootstrap::builder::Builder;
    use rust_bootstrap::BuildState;
    use std::fs;

    #[test]
    fn test_full_bootstrap_build() {
        let build_state = helpers::setup_test_build_state();

        // Call the main build function
        let build_state_for_closure = build_state.clone();
        let result = build_bootstrap::build_bootstrap(&build_state, move |args, rust_root| {
            println!("Mocking cargo command for test environment.");
            // Simulate the creation of the bootstrap binary for testing
            let builder = Builder::new(&build_state_for_closure); // Use the cloned build_state
            let bootstrap_binary_path = builder.bootstrap_binary();
            std::fs::create_dir_all(bootstrap_binary_path.parent().unwrap())?;
            std::fs::File::create(&bootstrap_binary_path)?;
            Ok(())
        });

        // Assert that the build was successful
        assert!(result.is_ok(), "Full bootstrap build should succeed");

        // Verify that the bootstrap binary exists
        let builder = Builder::new(&build_state);
        let bootstrap_binary_path = builder.bootstrap_binary();
        assert!(bootstrap_binary_path.exists(), "Bootstrap binary should exist at {:?}", bootstrap_binary_path);

        // Cleanup
        if build_state.creation_args.rust_root.exists() {
            fs::remove_dir_all(&build_state.creation_args.rust_root).unwrap();
        }
    }
}
