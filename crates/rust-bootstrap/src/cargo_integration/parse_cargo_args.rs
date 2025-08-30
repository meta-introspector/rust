use cargo::GlobalContext;
use cargo::ops::CompileOptions;
use cargo::util::CargoResult;
use cargo::core::compiler::UserIntent;
use cargo::core::Workspace;
use cargo::util::command_prelude::{CommandExt, ArgMatchesExt, ProfileChecking, subcommand};
use std::path::PathBuf;

pub fn parse_cargo_args<'gctx>(gctx: &'gctx GlobalContext, raw_args: &[&str], rust_root: &PathBuf) -> CargoResult<CompileOptions> {
    let command = subcommand("build")
        .about("Compile a local package and all of its dependencies")
        .arg_future_incompat_report()
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
        .arg_features()
        .arg_release("Build artifacts in release mode, with optimizations")
        .arg_redundant_default_mode("debug", "build", "release")
        .arg_profile("Build artifacts with the specified profile")
        .arg_parallel()
        .arg_target_triple("Build for the target triple")
        .arg_target_dir()
        .arg_artifact_dir()
        .arg_build_plan()
        .arg_unit_graph()
        .arg_timings()
        .arg_compile_time_deps()
        .arg_manifest_path()
        .arg_lockfile_path()
        .arg_ignore_rust_version();

    let matches = command.try_get_matches_from(raw_args)?;

    let ws = Workspace::new(rust_root, gctx)?;

    let compile_options = matches.compile_options(gctx, UserIntent::Build, Some(&ws), ProfileChecking::Custom)?;

    Ok(compile_options)
}
