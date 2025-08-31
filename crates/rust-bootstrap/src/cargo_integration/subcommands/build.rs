use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;
use cargo::util::command_prelude::CommandExt;

pub struct BuildProvider;

impl SubcommandProvider for BuildProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(
            Command::new("build")
                .about("Compile the current package")
                .arg_manifest_path()
                .arg_target_dir()
                .arg_features()
                .arg_release("Build artifacts in release mode, with optimizations")
                .arg_profile("Build artifacts with the specified profile")
                .arg_target_triple("Build for the target triple")
                .arg_build_plan()
                .arg_unit_graph()
                .arg_timings()
                .arg_compile_time_deps()
                .arg_lockfile_path()
                .arg_ignore_rust_version()
                .arg_message_format()
                .arg_silent_suggestion()
                .arg_package_spec(
                    "Package to build (see `cargo help pkgid`)",
                    "Build all packages in the workspace",
                    "Exclude packages from the build",
                )
                .arg_targets_all(
                    "Build only this package's library",
                    "Build only the specified binary",
                    "Build all binaries",
                    "Build only the specified example",
                    "Build all examples",
                    "Build only the specified test target",
                    "Build all targets that have `test = true` set",
                    "Build only the specified bench target",
                    "Build all targets that have `bench = true` set",
                    "Build all targets",
                )
                .arg_parallel()
        )
    }
}

// This function will be called by the main dispatch logic
pub fn handle_build_command(
    gctx: &cargo::GlobalContext,
    ws: &cargo::core::Workspace,
    subcommand_args_str: &[&str],
    rust_root: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let compile_options = crate::cargo_integration::parse_cargo_args::parse_cargo_args(&gctx, &subcommand_args_str, rust_root)?;
    cargo::ops::compile(&ws, &compile_options)?;
    Ok(())
}
