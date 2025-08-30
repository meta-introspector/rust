use rust_bootstrap::{Args, BuildState, parquet_reporter, builder::Builder, BuildStateCreationArgs};
use rust_bootstrap::bootstrap_stages::stage0_detector::Stage0;
use rust_bootstrap::bootstrap_stages::operational_logger::logger;
use syscall_instrumentation_macro::instrument_syscall;
use tracing;

use clap::Parser;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cargo_args_for_command = args.cargo_args.clone();
    let exec_panic_for_command = args.exec_panic;

    tracing::info!("Tracing initialized. exec_panic_for_command: {}", exec_panic_for_command);

    println!("Parsed arguments: {:?}", args);

    let config = rust_bootstrap::config::loader::load_config(
        Path::new(args.config.as_deref().unwrap_or("bootstrap.toml"))
    )?;

    let stage0 = Stage0::detect();

    let rust_root = std::env::current_dir()?;
    let build_dir = rust_root.join("build"); // Placeholder for now

    let build_state_creation_args = BuildStateCreationArgs {
        args: args.clone(),
        rust_root,
        build_dir,
        stage0,
        config,
        build_triple: String::from("aarch64-unknown-linux-gnu"),
        clean: args.clean,
    };

    let build_state = BuildState::new(build_state_creation_args);

    let builder = Builder::new(&build_state);

    // Placeholder for download_and_setup_toolchain
    // rust_bootstrap::bootstrap_stages::toolchain_downloader::download_and_setup_toolchain(&build_state)?;

    // Placeholder for build_bootstrap
    rust_bootstrap::bootstrap_stages::build_bootstrap::build_bootstrap(&build_state)?;

    // Execute Cargo command via integration
    rust_bootstrap::cargo_integration::run_cargo_command(
        &cargo_args_for_command.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
        &builder.bootstrap_binary(), // Pass rust_root
    )?;
    // rust_bootstrap::bootstrap_stages::process_build_metrics::process_build_metrics(command_result)?;

    // Write build config to parquet
    parquet_reporter::write_build_config_to_parquet(&build_state, "build_config.parquet")?;
    parquet_reporter::read_and_summarize_build_config_metrics("build_config.parquet")?;

    let operational_log_batch = logger::get_logged_events_as_record_batch()?;
    parquet_reporter::write_record_batch_to_parquet(operational_log_batch, "operational_log.parquet")?;

    // Test the syscall instrumentation macro
    my_dummy_function(10, 20);

    Ok(())
}

#[instrument_syscall]
fn my_dummy_function(a: i32, b: i32) -> i32 {
    a + b
}
