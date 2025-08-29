// This module will contain the main stages
pub mod parse_and_handle_cli_args;
pub mod load_and_print_config;
pub mod detect_and_setup_stage0;
pub mod execute_and_report_command;
pub mod process_build_metrics;
pub mod download_and_setup_toolchain;
pub mod build_bootstrap;
pub mod run_bootstrap;
pub mod print_final_message;
