use std::path::{PathBuf, Path};

pub fn bin_root(build_dir: &Path, build_triple: &str) -> PathBuf {
    build_dir.join(build_triple).join("stage0")
}
