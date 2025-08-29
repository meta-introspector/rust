use std::error::Error;
use crate::build_state::BuildState;
use crate::bootstrap_stages::toolchain_downloader::{bin_root, rustc_stamp, program_out_of_date, download_component, unpack_component};
use crate::bootstrap_stages::nix_patcher::{should_fix_bins_and_dylibs, fix_bin_or_dylib}; // Import nix_patcher functions
use std::fs;
use std::path::PathBuf;

pub fn download_and_setup_toolchain(build_state: &BuildState) -> Result<(), Box<dyn Error>> {
    let bin_root_path = bin_root::bin_root(&build_state.build_dir, &build_state.build_triple);
    let rustc_stamp_path = rustc_stamp::rustc_stamp(&bin_root_path);

    let key = format!("{}-{}", build_state.stage0.compiler_version, build_state.stage0.compiler_date);

    let is_outdated = program_out_of_date::program_out_of_date(&rustc_stamp_path, &key, build_state.clean);

    if is_outdated {
        println!("Toolchain is outdated or clean build requested. Download/setup needed.");

        // Remove existing bin_root_path if it exists
        if bin_root_path.exists() {
            fs::remove_dir_all(&bin_root_path)?;
        }
        fs::create_dir_all(&bin_root_path)?;

        let tarball_suffix = ".tar.xz"; // Assuming .tar.xz for now

        let toolchain_filename = format!("rustc-{}-{}{}", build_state.stage0.compiler_version, build_state.build_triple, tarball_suffix);
        let download_url = format!("{}/dist/{}/{}/{}", build_state.stage0.dist_server, build_state.stage0.compiler_date, build_state.build_triple, toolchain_filename);
        
        let tarball_path = build_state.build_dir.join(&toolchain_filename);

        println!("Downloading {} to {:?}\n", download_url, tarball_path);
        download_component::download_component(&download_url, &tarball_path)?;
        println!("Download complete. Unpacking...\n");

        unpack_component::unpack_component(&tarball_path, &bin_root_path, tarball_suffix)?;
        println!("Unpacking complete.\n");

        // Nix patching logic
        if should_fix_bins_and_dylibs::should_fix_bins_and_dylibs(build_state)? {
            println!("Applying Nix patches...");
            fix_bin_or_dylib::fix_bin_or_dylib(&bin_root_path.join("bin").join("rustc"))?;
            fix_bin_or_dylib::fix_bin_or_dylib(&bin_root_path.join("bin").join("cargo"))?;
            // Add other binaries as needed, similar to x.py
            println!("Nix patches applied.\n");
        }

        // Write the stamp file
        fs::write(&rustc_stamp_path, &key)?;
        println!("Stamp file written to {:?}\n", rustc_stamp_path);

    } else {
        println!("Toolchain is up to date. No download/setup needed.\n");
    }

    Ok(())
}
