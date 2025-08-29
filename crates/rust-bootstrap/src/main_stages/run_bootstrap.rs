use std::error::Error;
use crate::build_state::BuildState;
use crate::main_stages;
use crate::main_stages::download_and_setup_toolchain::download_and_setup_toolchain;
use crate::main_stages::build_bootstrap::build_bootstrap;

pub fn run_bootstrap() -> Result<(), Box<dyn Error>> {
    let args = main_stages::parse_and_handle_cli_args::parse_and_handle_cli_args();

    let config = main_stages::load_and_print_config::load_and_print_config(&args)?;

    let stage0 = main_stages::detect_and_setup_stage0::detect_and_setup_stage0();

    let rust_root = std::env::current_dir()?;
    let build_dir = rust_root.join("build"); // Placeholder for now

    let build_state = BuildState::new(args, rust_root, build_dir, stage0, config, String::from("x86_64-unknown-linux-gnu"));

    download_and_setup_toolchain(&build_state)?;
    build_bootstrap::build_bootstrap(&build_state)?;
    build::build(&build_state)?; // Call build_bootstrap

    let command_result = main_stages::execute_and_report_command::execute_and_report_command(&build_state.stage0)?;

    main_stages::process_build_metrics::process_build_metrics(command_result)?;

    // Write build config to parquet
    crate::parquet_reporter::write_build_config_to_parquet(&build_state, "build_config.parquet")?;
    crate::parquet_reporter::read_and_summarize_build_config_metrics("build_config.parquet")?;

    // Initiate Git analysis
    let git_analysis_summary = crate::git_analyzer::analysis::analyze_git_repository::analyze_git_repository(&build_state.args.repo_path)?;
    crate::parquet_reporter::write_git_analysis_summary_to_parquet(git_analysis_summary, "git_analysis_summary.parquet")?;
    crate::parquet_reporter::read_and_summarize_git_analysis_metrics("git_analysis_summary.parquet")?;

    main_stages::print_final_message::print_final_message();

    let operational_log_batch = crate::bootstrap_stages::operational_logger::get_logged_events_as_record_batch()?;
    crate::parquet_reporter::write_record_batch_to_parquet(operational_log_batch, "operational_log.parquet")?;

    Ok(())
}

