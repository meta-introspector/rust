use std::env;
use std::process::Command;

fn main() {
    // Auto-discover nix store paths for OpenSSL
    if let Ok(output) = Command::new("nix-store")
        .args(&["--query", "--requisites", "/run/current-system"])
        .output() 
    {
        let paths = String::from_utf8_lossy(&output.stdout);
        for path in paths.lines() {
            if path.contains("openssl") && path.contains("/lib") {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
    }
    
    // Try nix-locate for OpenSSL libraries
    if let Ok(output) = Command::new("nix-locate")
        .args(&["--type", "f", "libssl.so"])
        .output()
    {
        let paths = String::from_utf8_lossy(&output.stdout);
        for line in paths.lines() {
            if let Some(path) = line.split_whitespace().last() {
                if let Some(lib_dir) = std::path::Path::new(path).parent() {
                    println!("cargo:rustc-link-search=native={}", lib_dir.display());
                }
            }
        }
    }
    
    // Fallback: check common nix paths
    let nix_paths = [
        "/nix/store",
        "/run/current-system/sw/lib",
    ];
    
    for base_path in &nix_paths {
        if let Ok(entries) = std::fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.file_name()
                    .and_then(|n| n.to_str())
                    .map_or(false, |s| s.contains("openssl"))
                {
                    let lib_path = path.join("lib");
                    if lib_path.exists() {
                        println!("cargo:rustc-link-search=native={}", lib_path.display());
                    }
                }
            }
        }
    }
    
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
}
