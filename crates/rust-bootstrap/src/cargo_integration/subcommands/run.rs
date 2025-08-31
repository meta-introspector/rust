use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;
use cargo::util::command_prelude::CommandExt;

pub struct RunProvider;

impl SubcommandProvider for RunProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(
            Command::new("run")
                .about("Run a binary or example of the current package")
                .trailing_var_arg(true) // Allow passing args to the binary
                .allow_hyphen_values(true)
                .arg_manifest_path()
                .arg_target_dir()
                .arg_features()
                .arg_release("Run artifacts in release mode, with optimizations")
                .arg_profile("Run artifacts with the specified profile")
                .arg_target_triple("Run for the target triple")
                .arg_build_plan()
                .arg_unit_graph()
                .arg_timings()
                .arg_compile_time_deps()
                .arg_lockfile_path()
                .arg_ignore_rust_version()
                .arg_message_format()
                .arg_silent_suggestion()
                .arg_package_spec(
                    "Package to run (see `cargo help pkgid`)",
                    "Run all packages in the workspace",
                    "Exclude packages from the run",
                )
                .arg_targets_all(
                    "Run only this package's library",
                    "Run only the specified binary",
                    "Run all binaries",
                    "Run only the specified example",
                    "Run all examples",
                    "Run only the specified test target",
                    "Run all targets that have `test = true` set",
                    "Run only the specified bench target",
                    "Run all targets that have `bench = true` set",
                    "Run all targets",
                )
                .arg_parallel()
        )
    }

    fn name(&self) -> &'static str {
        "run"
    }
}

// This function will be called by the main dispatch logic
pub fn handle_run_command(
    gctx: &cargo::GlobalContext,
    ws: &cargo::core::Workspace,
    subcommand_args_str: &[&str],
    rust_root: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // For run, the subcommand_args_str are the args to pass to the binary
    // cargo::ops::run expects a CompileOptions and then the args for the binary
    let run_options = crate::cargo_integration::parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;
    cargo::ops::run(&ws, &run_options, &subcommand_args_str.iter().map(|s| s.to_string().into()).collect::<Vec<_>>())?;
    Ok(())
}
