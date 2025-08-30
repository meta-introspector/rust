use rust_bootstrap::{Args, BuildState, loader, parquet_reporter, builder::Builder};
use rust_bootstrap::bootstrap_stages::stage0_detector::Stage0;
use rust_bootstrap::bootstrap_stages::operational_logger::logger;
use syscall_instrumentation_macro::instrument_syscall;

use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Parsed arguments: {:?}", args);

    let config = loader::load_config(
        args.config.as_deref().unwrap_or("bootstrap.toml")
    )?;

    let stage0 = Stage0::detect();

    let rust_root = std::env::current_dir()?;
    let build_dir = rust_root.join("build"); // Placeholder for now

    let build_state = BuildState::new(
        args,
        rust_root,
        build_dir,
        stage0,
        config,
        String::from("x86_64-unknown-linux-gnu"), // Default build triple
    );

    let builder = Builder::new(&build_state);

    // Placeholder for download_and_setup_toolchain
    // rust_bootstrap::bootstrap_stages::toolchain_downloader::download_and_setup_toolchain(&build_state)?;

    // Placeholder for build_bootstrap
    rust_bootstrap::bootstrap_stages::build_bootstrap::build_bootstrap(&build_state)?;

    // Placeholder for execute_and_report_command
    let command_result = rust_bootstrap::bootstrap_stages::command_executor::execute_and_report_command(
        builder.bootstrap_binary().to_str().unwrap(),
        &std::env::args().skip(1).collect::<Vec<String>>().iter().map(|s| s.as_str()).collect::<Vec<&str>>()
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
