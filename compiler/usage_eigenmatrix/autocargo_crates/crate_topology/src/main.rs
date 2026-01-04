use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("🔬 Crate-level topology analysis");
    
    // Step 1: Find all crates and their dependencies
    let crate_deps = find_crate_dependencies();
    println!("✅ Found {} crates", crate_deps.len());
    
    // Step 2: Topological sort of crates
    let ordered_crates = topological_sort_crates(&crate_deps);
    println!("✅ Ordered {} crates topologically", ordered_crates.len());
    
    // Step 3: Identify independent crates (no dependencies)
    let independent = find_independent_crates(&crate_deps);
    println!("✅ Found {} independent crates", independent.len());
    
    output_crate_topology(&ordered_crates, &independent, &crate_deps);
}

fn find_crate_dependencies() -> HashMap<String, HashSet<String>> {
    let mut crate_deps = HashMap::new();
    let compiler_dir = "submodules/rust/compiler";
    
    if let Ok(entries) = fs::read_dir(compiler_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let crate_name = entry.file_name().to_string_lossy().to_string();
                let deps = extract_crate_dependencies(&entry.path());
                crate_deps.insert(crate_name, deps);
            }
        }
    }
    
    crate_deps
}

fn extract_crate_dependencies(crate_path: &std::path::Path) -> HashSet<String> {
    let mut deps = HashSet::new();
    
    // Check Cargo.toml
    let cargo_toml = crate_path.join("Cargo.toml");
    if let Ok(content) = fs::read_to_string(cargo_toml) {
        for line in content.lines() {
            if line.starts_with("rustc_") && line.contains("=") {
                if let Some(dep) = line.split('=').next() {
                    let dep_name = dep.trim().replace("\"", "");
                    if dep_name.starts_with("rustc_") {
                        deps.insert(dep_name);
                    }
                }
            }
        }
    }
    
    // Check use statements in lib.rs
    let lib_rs = crate_path.join("src/lib.rs");
    if let Ok(content) = fs::read_to_string(lib_rs) {
        for line in content.lines() {
            if line.trim().starts_with("extern crate rustc_") {
                if let Some(crate_name) = line.split_whitespace().nth(2) {
                    let clean_name = crate_name.replace(";", "");
                    deps.insert(clean_name);
                }
            }
            if line.trim().starts_with("use rustc_") {
                if let Some(first_part) = line.split("::").next() {
                    if let Some(crate_name) = first_part.split_whitespace().nth(1) {
                        deps.insert(crate_name.to_string());
                    }
                }
            }
        }
    }
    
    deps
}

fn topological_sort_crates(crate_deps: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut ordered = Vec::new();
    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();
    
    for crate_name in crate_deps.keys() {
        if !visited.contains(crate_name) {
            visit_crate(crate_name, crate_deps, &mut visited, &mut visiting, &mut ordered);
        }
    }
    
    ordered
}

fn visit_crate(
    crate_name: &str,
    crate_deps: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
    ordered: &mut Vec<String>
) {
    if visiting.contains(crate_name) {
        return; // Cycle detected, skip
    }
    if visited.contains(crate_name) {
        return;
    }
    
    visiting.insert(crate_name.to_string());
    
    if let Some(deps) = crate_deps.get(crate_name) {
        for dep in deps {
            if crate_deps.contains_key(dep) {
                visit_crate(dep, crate_deps, visited, visiting, ordered);
            }
        }
    }
    
    visiting.remove(crate_name);
    visited.insert(crate_name.to_string());
    ordered.push(crate_name.to_string());
}

fn find_independent_crates(crate_deps: &HashMap<String, HashSet<String>>) -> Vec<String> {
    crate_deps.iter()
        .filter(|(_, deps)| deps.is_empty())
        .map(|(name, _)| name.clone())
        .collect()
}

fn output_crate_topology(
    ordered: &[String], 
    independent: &[String], 
    crate_deps: &HashMap<String, HashSet<String>>
) {
    let mut output = String::new();
    output.push_str("# Rustc Crate Topology\n\n");
    
    output.push_str(&format!("## Independent Crates ({} total)\n", independent.len()));
    output.push_str("These crates have no rustc dependencies and can be built first:\n\n");
    for crate_name in independent {
        output.push_str(&format!("- **{}**\n", crate_name));
    }
    
    output.push_str("\n## Topological Order\n");
    output.push_str("Build order from least to most dependent:\n\n");
    for (i, crate_name) in ordered.iter().enumerate() {
        let dep_count = crate_deps.get(crate_name).map_or(0, |deps| deps.len());
        output.push_str(&format!("{}. **{}** ({} dependencies)\n", i + 1, crate_name, dep_count));
    }
    
    output.push_str("\n## Dependency Details\n\n");
    for crate_name in ordered {
        if let Some(deps) = crate_deps.get(crate_name) {
            if !deps.is_empty() {
                output.push_str(&format!("### {}\nDepends on: {}\n\n", 
                    crate_name, 
                    deps.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
            }
        }
    }
    
    fs::write("crate_topology.md", output).unwrap();
    println!("✅ Saved crate topology to crate_topology.md");
    
    // Output build script
    let mut build_script = String::new();
    build_script.push_str("#!/bin/bash\n# Auto-generated crate build order\n\n");
    for crate_name in ordered {
        build_script.push_str(&format!("echo \"Building {}...\"\n", crate_name));
        build_script.push_str(&format!("cd submodules/rust/compiler/{}\n", crate_name));
        build_script.push_str("cargo build\ncd ../../../..\n\n");
    }
    
    fs::write("build_crates_in_order.sh", build_script).unwrap();
    println!("✅ Generated build script: build_crates_in_order.sh");
}
