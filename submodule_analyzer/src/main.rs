use std::process::{Command, Output};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const OUTPUT_DIR: &str = "docs/submodule_cargo_trees";

fn run_command(cmd: &str, args: &[&str], current_dir: &Path) -> io::Result<Output> {
    Command::new(cmd)
        .args(args)
        .current_dir(current_dir)
        .output()
}

fn get_repo_root() -> io::Result<PathBuf> {
    let output = run_command("git", &["rev-parse", "--show-toplevel"], Path::new("."))?;
    if output.status.success() {
        Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get git repo root: {}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

fn get_submodule_paths(repo_root: &Path) -> io::Result<Vec<PathBuf>> {
    let output = run_command("git", &["submodule", "status", "--recursive"], repo_root)?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let paths: Vec<PathBuf> = stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    // Submodule path is the second part
                    Some(repo_root.join(parts[1]))
                } else {
                    None
                }
            })
            .collect();
        Ok(paths)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get submodule status: {}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

fn process_submodule(submodule_path: &Path, repo_root: &Path) -> io::Result<()> {
    let submodule_name = submodule_path.file_name().unwrap().to_string_lossy();
    let output_file_path = repo_root.join(OUTPUT_DIR).join(format!("{}_cargo_tree.txt", submodule_name));

    let mut output_file = fs::File::create(&output_file_path)?;

    writeln!(output_file, "--- Cargo Tree for {} ---", submodule_path.display())?;

    let mut success = false;

    // Try cargo tree --workspace
    writeln!(output_file, "\nAttempting `cargo tree --workspace`...")?;
    let output = run_command("cargo", &["tree", "--workspace"], submodule_path)?;
    if output.status.success() {
        output_file.write_all(&output.stdout)?;
        writeln!(output_file, "\nSuccessfully generated cargo tree for workspace.")?;
        success = true;
    } else {
        writeln!(output_file, "\nFailed: {}", String::from_utf8_lossy(&output.stderr))?;
    }

    if !success {
        // Try cargo tree (simple package)
        writeln!(output_file, "\nAttempting `cargo tree` (simple package)...")?;
        let output = run_command("cargo", &["tree"], submodule_path)?;
        if output.status.success() {
            output_file.write_all(&output.stdout)?;
            writeln!(output_file, "\nSuccessfully generated cargo tree for simple package.")?;
            success = true;
        } else {
            writeln!(output_file, "\nFailed: {}", String::from_utf8_lossy(&output.stderr))?;
        }
    }

    if !success {
        // If still no success, try to parse Cargo.toml for workspace members
        let cargo_toml_path = submodule_path.join("Cargo.toml");
        if cargo_toml_path.exists() {
            let cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;
            if cargo_toml_content.contains("[workspace]") {
                writeln!(output_file, "\nAttempting `cargo tree` for workspace members...")?;
                // A very basic attempt to parse members, more robust parsing would need a TOML crate
                for line in cargo_toml_content.lines() {
                    if let Some(stripped) = line.trim_start().strip_prefix("members = [") {
                        let members_str = stripped.split(']').next().unwrap_or("").trim();
                        for member in members_str.split(',').filter(|s| !s.trim().is_empty()) {
                            let member_path_str = member.trim().trim_matches('"');
                            let member_path = submodule_path.join(member_path_str);
                            if member_path.is_dir() && member_path.join("Cargo.toml").exists() {
                                writeln!(output_file, "\n--- Cargo Tree for member: {} ---", member_path.display())?;
                                let member_output = run_command("cargo", &["tree"], &member_path)?;
                                if member_output.status.success() {
                                    output_file.write_all(&member_output.stdout)?;
                                    writeln!(output_file, "\nSuccessfully generated cargo tree for member.")?;
                                    success = true;
                                } else {
                                    writeln!(output_file, "\nFailed for member: {}", String::from_utf8_lossy(&member_output.stderr))?;
                                }
                            }
                        }
                        break; // Only process the first members line
                    }
                }
            }
        }
    }

    if !success {
        writeln!(output_file, "\nCould not generate cargo tree for {}. Please check manually.", submodule_path.display())?;
    }

    writeln!(output_file, "------------------------------------")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let repo_root = get_repo_root()?;
    let output_dir_full_path = repo_root.join(OUTPUT_DIR);

    fs::create_dir_all(&output_dir_full_path)?;
    // Clear previous outputs
    for entry in fs::read_dir(&output_dir_full_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
            fs::remove_file(path)?;
        }
    }

    println!("Generating cargo tree for all git submodules...");

    let submodule_paths = get_submodule_paths(&repo_root)?;

    for path in submodule_paths {
        println!("Processing submodule: {}", path.display());
        if let Err(e) = process_submodule(&path, &repo_root) {
            eprintln!("Error processing submodule {}: {}", path.display(), e);
        }
    }

    println!("All cargo tree outputs generated in {}/", output_dir_full_path.display());

    // Verify total lines generated
    println!("Total lines in generated cargo tree files:");
    let total_lines_output = run_command(
        "bash",
        &[
            "-c",
            &format!("find {} -type f -name \"*_cargo_tree.txt\" -exec cat {{}} + | wc -l", output_dir_full_path.display()),
        ],
        &repo_root,
    )?;
    io::stdout().write_all(&total_lines_output.stdout)?;

    Ok(())
}