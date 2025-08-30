use std::path::PathBuf;
use crate::config::args::Args;
use crate::bootstrap_stages::stage0_detector::Stage0;
use crate::config::loader::Config;

pub struct BuildStateCreationArgs {
    pub args: Args,
    pub rust_root: PathBuf,
    pub build_dir: PathBuf,
    pub stage0: Stage0,
    pub config: Config,
    pub build_triple: String,
    pub clean: bool,
}
