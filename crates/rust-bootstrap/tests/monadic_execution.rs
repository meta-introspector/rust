#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use rust_bootstrap::Args;
    use rust_bootstrap::bootstrap_stages::command_executor::execute_shell_command::execute_shell_command;
    use clap::Parser;

    #[test]
    #[should_panic(expected = "Shell commands are not allowed in the new system.")]
    fn test_exec_panic_enabled() {
        // Simulate arguments with exec_panic enabled (which is the default)
        let args = Args::parse_from(vec!["rust-bootstrap"]);
        assert!(args.exec_panic, "exec_panic should be true by default");

        // Attempt to execute a shell command with exec_panic true
        // This should cause a panic
        let _ = execute_shell_command("echo", &["hello"], args.exec_panic);
    }

    #[test]
    fn test_exec_panic_disabled() {
        // Simulate arguments with exec_panic disabled
        let mut args = Args::parse_from(vec!["rust-bootstrap"]);
    args.exec_panic = false;
        assert!(!args.exec_panic, "exec_panic should be false when explicitly set");

        // Attempt to execute a shell command with exec_panic false
        // This should NOT cause a panic, but return an Ok result
        let result = execute_shell_command("echo", &["hello"], args.exec_panic);
        assert!(result.is_ok(), "execute_shell_command should return Ok when exec_panic is false");
        let output = result.unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
    }
}
