use std::path::{PathBuf, Path};

pub fn rustc_stamp(bin_root: &Path) -> PathBuf {
    bin_root.join(".rustc-stamp")
}
