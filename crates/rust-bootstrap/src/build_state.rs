use std::path::PathBuf;
use crate::bootstrap_stages::cli_parser::create_args_struct::Args;
use crate::bootstrap_stages::stage0_detector::Stage0;

pub struct BuildState {
    pub args: Args,
    pub rust_root: PathBuf,
    pub build_dir: PathBuf,
    pub stage0: Stage0,
    pub clean: bool,
    pub build_triple: String,
    // Add other fields as needed from RustBuild in bootstrap.py
}

impl BuildState {
    pub fn new(args: Args, rust_root: PathBuf, build_dir: PathBuf, stage0: Stage0, build_triple: String) -> Self {
        let clean = args.clean; // Assuming args.clean exists
        BuildState {
            args,
            rust_root,
            build_dir,
            stage0,
            clean,
            build_triple,
        }
    }
}
