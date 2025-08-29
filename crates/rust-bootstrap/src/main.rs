mod parquet_reporter;
mod bootstrap_stages;
mod main_stages;
mod git_analyzer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = main_stages::parse_and_handle_cli_args::parse_and_handle_cli_args();

    let _config = main_stages::load_and_print_config::load_and_print_config(&args)?;

    let stage0 = main_stages::detect_and_setup_stage0::detect_and_setup_stage0();

    let command_result = main_stages::execute_and_report_command::execute_and_report_command(&stage0)?;

    main_stages::process_build_metrics::process_build_metrics(command_result)?;

    // Initiate Git analysis
    crate::git_analyzer::analyze_git_repository(&args.repo_path)?;

    main_stages::print_final_message::print_final_message();
    Ok(())
}