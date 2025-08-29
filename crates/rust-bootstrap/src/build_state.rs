use std::path::PathBuf;
use crate::config::args::Args;
//use crate::bootstrap_stages::stage0_detector::Stage0;
//use crate::bootstrap_stages::config_loader;
use crate::bootstrap_stages::stage0_detector::Stage0;
use crate::bootstrap_stages::config_loader;
pub struct BuildState {
    pub args: Args,
    pub rust_root: PathBuf,
    pub build_dir: PathBuf,
    pub stage0: Stage0,
    pub config: config_loader::Config,
    pub clean: bool,
    pub build_triple: String,
    // Add other fields as needed from RustBuild in bootstrap.py
}

impl BuildState {
    pub fn new(args: Args, rust_root: PathBuf, build_dir: PathBuf, stage0: Stage0, config: config_loader::Config, build_triple: String) -> Self {
        let clean = args.clean; // Assuming args.clean exists
        BuildState {
            args,
            rust_root,
            build_dir,
            stage0,
            config,
            clean,
            build_triple,
        }
    }
}
