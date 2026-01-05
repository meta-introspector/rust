//! Integration tests for the Usage Eigenmatrix project
//! 
//! This module contains tests for the experimental Rust programs exploring
//! mathematical concepts, prime number patterns, and quantum mechanics metaphors.

#[cfg(test)]
mod integration_tests {
    use std::process::Command;

    #[test]
    fn test_observe_bits_runs() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "observe_bits"])
            .output()
            .expect("Failed to execute observe_bits");
        
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("OBSERVE - Reading the Bits"));
        assert!(stdout.contains("Quantum Collapse"));
        assert!(stdout.contains("UR-VECTOR DETECTED"));
    }

    #[test]
    fn test_prime_sieve_table_runs() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "prime_sieve_table"])
            .output()
            .expect("Failed to execute prime_sieve_table");
        
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Prime Sieve Structure Table"));
        assert!(stdout.contains("Complexity Distribution"));
        assert!(stdout.contains("256 total combinations"));
    }

    #[test]
    fn test_all_tests_pass() {
        let output = Command::new("cargo")
            .args(&["test", "--bins"])
            .output()
            .expect("Failed to run tests");
        
        assert!(output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should contain test results
        assert!(stderr.contains("test result: ok"));
    }

    #[test]
    fn test_project_compiles() {
        let output = Command::new("cargo")
            .args(&["check", "--all-targets"])
            .output()
            .expect("Failed to check compilation");
        
        assert!(output.status.success());
    }
}

#[cfg(test)]
mod unit_tests {
    /// Test prime number utilities
    #[test]
    fn test_first_eight_primes() {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(primes.len(), 8);
        
        // Verify they are actually prime
        for &p in &primes {
            assert!(is_prime(p));
        }
    }

    #[test]
    fn test_binary_representations() {
        assert_eq!(format!("{:08b}", 2), "00000010");
        assert_eq!(format!("{:08b}", 3), "00000011");
        assert_eq!(format!("{:08b}", 19), "00010011");
    }

    #[test]
    fn test_bit_operations() {
        let encoding = 0b00000011u8; // First two bits set
        assert_eq!(encoding.count_ones(), 2);
        assert_eq!((encoding >> 0) & 1, 1);
        assert_eq!((encoding >> 1) & 1, 1);
        assert_eq!((encoding >> 2) & 1, 0);
    }

    fn is_prime(n: u32) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        
        let sqrt_n = (n as f64).sqrt() as u32;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
}

#[cfg(test)]
mod quantum_tests {
    /// Tests for quantum mechanics metaphors
    #[test]
    fn test_quantum_state_collapse() {
        let superposition = vec![0, 1, 0, 1, 1, 0, 1, 1];
        let collapsed = observe_state(&superposition);
        
        // After observation, state should be deterministic
        assert_eq!(collapsed.len(), superposition.len());
        assert!(collapsed.iter().all(|&x| x == 0 || x == 1));
    }

    #[test]
    fn test_observer_effect() {
        let quantum_bits = [1, 2, 3, 5, 7, 11, 13, 17, 19];
        
        // Each observation should yield a definite value
        for &bit in &quantum_bits {
            assert!(bit > 0);
            assert!(bit < 256); // Within u8 range
        }
    }

    fn observe_state(state: &[u8]) -> Vec<u8> {
        // Simulate quantum state collapse
        state.iter().map(|&x| if x > 0 { 1 } else { 0 }).collect()
    }
}

#[cfg(test)]
mod graph_tests {
    use std::collections::HashMap;

    #[test]
    fn test_call_graph_structure() {
        let mut graph = HashMap::new();
        graph.insert("main".to_string(), vec!["func1", "func2"]);
        graph.insert("func1".to_string(), vec!["func3"]);
        graph.insert("func2".to_string(), vec![]);
        
        assert_eq!(graph.len(), 3);
        assert_eq!(graph["main"].len(), 2);
        assert_eq!(graph["func2"].len(), 0);
    }

    #[test]
    fn test_symbol_table() {
        let mut symbols = HashMap::new();
        symbols.insert("main".to_string(), "entry_point");
        symbols.insert("func1".to_string(), "helper_function");
        
        assert!(symbols.contains_key("main"));
        assert_eq!(symbols["main"], "entry_point");
    }
}
