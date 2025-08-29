use std::error::Error;
use std::fs;
use std::path::Path;

pub fn should_fix_bins_and_dylibs(build_state: &crate::build_state::BuildState) -> Result<bool, Box<dyn Error>> {
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

    // TODO: Implement check for `build.patch-binaries-for-nix` in bootstrap.toml (from build_state.config)
    // For now, default to false if not NixOS
    Ok(false)
}
