use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;
use cargo::util::command_prelude::CommandExt;

pub struct CheckProvider;

impl SubcommandProvider for CheckProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(
            Command::new("check")
                .about("Check the current package for errors")
                .arg_manifest_path()
                .arg_target_dir()
                .arg_features()
                .arg_release("Check artifacts in release mode")
                .arg_profile("Check artifacts with the specified profile")
                .arg_target_triple("Check for the target triple")
                .arg_build_plan()
                .arg_unit_graph()
                .arg_timings()
                .arg_compile_time_deps()
                .arg_lockfile_path()
                .arg_ignore_rust_version()
                .arg_message_format()
                .arg_silent_suggestion()
                .arg_package_spec(
                    "Package to check (see `cargo help pkgid`)",
                    "Check all packages in the workspace",
                    "Exclude packages from the check",
                )
                .arg_targets_all(
                    "Check only this package's library",
                    "Check only the specified binary",
                    "Check all binaries",
                    "Check only the specified example",
                    "Check all examples",
                    "Check only the specified test target",
                    "Check all targets that have `test = true` set",
                    "Check only the specified bench target",
                    "Check all targets that have `bench = true` set",
                    "Check all targets",
                )
                .arg_parallel()
        )
    }

    fn name(&self) -> &'static str {
        "check"
    }
}

// This function will be called by the main dispatch logic
pub fn handle_check_command(
    gctx: &cargo::GlobalContext,
    ws: &cargo::core::Workspace,
    subcommand_args_str: &[&str],
    rust_root: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let compile_options = crate::cargo_integration::parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;
    cargo::ops::compile(&ws, &compile_options)?; // 'check' also uses compile op
    Ok(())
}
