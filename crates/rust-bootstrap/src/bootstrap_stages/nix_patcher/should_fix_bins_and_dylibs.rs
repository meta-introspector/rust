use std::error::Error;
use std::fs;
use std::path::Path;

pub fn should_fix_bins_and_dylibs(build_state: &crate::build_state::BuildState) -> Result<bool, Box<dyn Error>> {
    if build_state.config.build.patch_binaries_for_nix {
        return Ok(true);
    }
    // Check if on Linux
    if cfg!(target_os = "linux") {
        // Check for NixOS by looking at /etc/os-release
        let os_release_path = Path::new("/etc/os-release");
        if os_release_path.exists() {
            let content = fs::read_to_string(os_release_path)?;
            if content.lines().any(|line| line.trim() == "ID=nixos" || line.trim() == "ID=\"nixos\"" || line.trim() == "ID='nixos'") {
                println!("INFO: You seem to be using NixOS.");
                return Ok(true);
            }
        }
    }

    Ok(false)
}
