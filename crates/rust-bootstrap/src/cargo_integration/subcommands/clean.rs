use clap::Command;
use crate::cargo_integration::clap_extensibility::SubcommandProvider;
use cargo::util::command_prelude::CommandExt;

pub struct CleanProvider;

impl SubcommandProvider for CleanProvider {
    fn add_to_command(self: Box<Self>, command: Command) -> Command {
        command.subcommand(
            Command::new("clean")
                .about("Remove build artifacts")
                .arg_manifest_path()
                .arg_target_dir()
                .arg_release("Remove artifacts in release mode")
                .arg_profile("Remove artifacts with the specified profile")
                .arg_target_triple("Remove for the target triple")
                .arg_package_spec(
                    "Package to clean (see `cargo help pkgid`)",
                    "Clean all packages in the workspace",
                    "Exclude packages from the clean",
                )
                .arg_targets_all(
                    "Clean only this package's library",
                    "Clean only the specified binary",
                    "Clean all binaries",
                    "Clean only the specified example",
                    "Clean all examples",
                    "Clean only the specified test target",
                    "Clean all targets that have `test = true` set",
                    "Clean only the specified bench target",
                    "Clean all targets that have `bench = true` set",
                    "Clean all targets",
                )
        )
    }

    fn name(&self) -> &'static str {
        "clean"
    }
}

use cargo::util::interning::InternedString;

// This function will be called by the main dispatch logic
pub fn handle_clean_command(
    gctx: &cargo::GlobalContext,
    ws: &cargo::core::Workspace,
    subcommand_args_str: &[&str],
    rust_root: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("DEBUG: subcommand_args_str: {:?}", subcommand_args_str);
    println!("DEBUG: rust_root: {:?}", rust_root);
    // For clean, we need to create CleanOptions
    let clean_options = cargo::ops::CleanOptions {
        gctx,
        spec: Vec::new(),
        targets: Vec::new(),
        profile_specified: false,
        requested_profile: InternedString::new(""),
        doc: false,
        dry_run: false,
    };
    cargo::ops::clean(&ws, &clean_options)?;
    Ok(())
}
