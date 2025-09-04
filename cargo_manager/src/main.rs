use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use toml::Table;
use walkdir::WalkDir;

const OUTPUT_DIR: &str = "docs/submodule_cargo_trees";
const INDEX_DIR: &str = "index";

#[derive(Debug, Deserialize, Serialize)]
struct Workspace {
    members: Option<Vec<String>>,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    resolver: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Package {
    name: Option<String>,
    version: Option<String>,
    edition: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
struct Dependency {
    version: Option<String>,
    path: Option<String>,
    git: Option<String>,
    branch: Option<String>,
    features: Option<Vec<String>>,
    #[serde(flatten)]
    other: BTreeMap<String, toml::Value>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Dependencies {
    #[serde(flatten)]
    map: BTreeMap<String, Dependency>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Features {
    #[serde(flatten)]
    map: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CargoToml {
    #[serde(default)]
    package: Option<Package>,
    #[serde(default)]
    workspace: Option<Workspace>,
    #[serde(default)]
    dependencies: Option<Dependencies>,
    #[serde(rename = "dev-dependencies", default)]
    dev_dependencies: Option<Dependencies>,
    #[serde(rename = "build-dependencies", default)]
    build_dependencies: Option<Dependencies>,
    #[serde(default)]
    features: Option<Features>,
    #[serde(flatten)]
    other: toml::Value,
}

fn parse_cargo_toml(path: &Path) -> io::Result<CargoToml> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse {}: {}", path.display(), e)))
}

fn main() -> io::Result<()> {
    let repo_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()))
        .parent()
        .unwrap()
        .to_path_buf();

    let old_cargo_toml_path = repo_root.join("old_cargo_toml/Cargo.toml");
    let new_cargo_toml_path = repo_root.join("Cargo.toml");
    let index_dir_path = repo_root.join(INDEX_DIR);

    println!("Generating new top-level Cargo.toml...");

    let mut all_members: BTreeSet<String> = BTreeSet::new();
    let mut all_excludes: BTreeSet<String> = BTreeSet::new();
    let mut all_dependencies: BTreeMap<String, Dependency> = BTreeMap::new();
    let mut all_dev_dependencies: BTreeMap<String, Dependency> = BTreeMap::new();
    let mut all_build_dependencies: BTreeMap<String, Dependency> = BTreeMap::new();
    let mut all_features: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    // 1. Read existing members and excludes from old_cargo_toml/Cargo.toml
    let mut old_resolver: Option<String> = None;
    if old_cargo_toml_path.exists() {
        let old_cargo_toml = parse_cargo_toml(&old_cargo_toml_path)?;
        if let Some(workspace) = old_cargo_toml.workspace {
            if let Some(members) = workspace.members {
                for member in members {
                    all_members.insert(member);
                }
            }
            for exclude in workspace.exclude {
                all_excludes.insert(exclude);
            }
            old_resolver = workspace.resolver;
        }

        // Also extract dependencies from the old top-level Cargo.toml if it had any
        if let Some(deps) = old_cargo_toml.dependencies {
            for (name, dep) in deps.map {
                all_dependencies.insert(name, dep);
            }
        }
        if let Some(deps) = old_cargo_toml.dev_dependencies {
            for (name, dep) in deps.map {
                all_dev_dependencies.insert(name, dep);
            }
        }
        if let Some(deps) = old_cargo_toml.build_dependencies {
            for (name, dep) in deps.map {
                all_build_dependencies.insert(name, dep);
            }
        }
        if let Some(features) = old_cargo_toml.features {
            for (feature_name, feature_items) in features.map {
                let entry = all_features.entry(feature_name).or_insert_with(BTreeSet::new);
                for item in feature_items {
                    entry.insert(item);
                }
            }
        }
    }

    // Add cargo_manager and submodule_analyzer to members
    all_members.insert("cargo_manager".to_string());
    all_members.insert("submodule_analyzer".to_string());

    // Add the new lattice submodules as members
    all_members.insert("vendor/lattirust".to_string());
    all_members.insert("vendor/latticefold-rs".to_string());
    all_members.insert("vendor/Lazarus".to_string());
    all_members.insert("vendor/qfall-crypto".to_string());

    // Add other known top-level crates from the original Cargo.toml
    // This list should ideally be dynamically discovered or managed.
    let known_members = vec![
        "compiler/rustc".to_string(),
        "crates/introspector/lattice/construction".to_string(),
        "crates/introspector/lattice/lattice-introspector".to_string(),
        "crates/introspector/lattice/lattice-macros".to_string(),
        "crates/introspector/lattice/lattice-macros-test".to_string(),
        "crates/introspector/lattice/lattice-types".to_string(),
        "crates/introspector/fixed_point_experiments".to_string(),
        "crates/vendored/solfunmeme-banner".to_string(),
        "crates/introspector/lattice/lattice-analyzer".to_string(),
        "crates/rust-bootstrap".to_string(),
        "crates/syscall_instrumentation_macro".to_string(),
        "src/build_helper".to_string(),
        "src/rustc-std-workspace/rustc-std-workspace-alloc".to_string(),
        "src/rustc-std-workspace/rustc-std-workspace-core".to_string(),
        "src/rustc-std-workspace/rustc-std-workspace-std".to_string(),
        "src/rustdoc-json-types".to_string(),
        "src/tools/build-manifest".to_string(),
        "src/tools/bump-stage0".to_string(),
        "src/tools/cargotest".to_string(),
        "src/tools/clippy".to_string(),
        "src/tools/clippy/clippy_dev".to_string(),
        "src/tools/collect-license-metadata".to_string(),
        "src/tools/compiletest".to_string(),
        "src/tools/coverage-dump".to_string(),
        "src/tools/features-status-dump".to_string(),
        "src/tools/generate-copyright".to_string(),
        "src/tools/generate-windows-sys".to_string(),
        "src/tools/html-checker".to_string(),
        "src/tools/jsondocck".to_string(),
        "src/tools/jsondoclint".to_string(),
        "src/tools/linkchecker".to_string(),
        "src/tools/lint-docs".to_string(),
        "src/tools/lld-wrapper".to_string(),
        "src/tools/llvm-bitcode-linker".to_string(),
        "src/tools/miri/cargo-miri".to_string(),
        "src/tools/opt-dist".to_string(),
        "src/tools/remote-test-client".to_string(),
        "src/tools/remote-test-server".to_string(),
        "src/tools/replace-version-placeholder".to_string(),
        "src/tools/run-make-support".to_string(),
        "src/tools/rust-installer".to_string(),
        "src/tools/rustdoc".to_string(),
        "src/tools/rustdoc-gui-test".to_string(),
        "src/tools/rustdoc-themes".to_string(),
        "src/tools/rustfmt".to_string(),
        "src/tools/test-float-parse".to_string(),
        "src/tools/tidy".to_string(),
        "src/tools/tier-check".to_string(),
        "src/tools/unicode-table-generator".to_string(),
        "src/tools/unstable-book-gen".to_string(),
        "src/tools/wasm-component-ld".to_string(),
        "src/tools/x".to_string(),
        "crates/introspector/crates/resonance_analyzer".to_string(),
        "crates/introspector/crates/resonance_core".to_string(),
        "crates/introspector/crates/ultimate_blinkenlights_simulation".to_string(),
        "crates/introspector/crates/introspector_profiler_macros".to_string(),
    ];

    for member in known_members {
        all_members.insert(member);
    }

    // 2. Iterate through index/*.txt files to find Cargo.toml paths and extract info
    for entry in WalkDir::new(&index_dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
            let content = fs::read_to_string(&path)?;
            for line in content.lines() {
                let cargo_toml_path = repo_root.join(line.trim());
                if cargo_toml_path.exists() {
                    match parse_cargo_toml(&cargo_toml_path) {
                        Ok(parsed_toml) => {
                            // Add the package itself as a member if it's not already there
                            if let Some(package) = parsed_toml.package {
                                if let Some(_name) = package.name {
                                    let relative_path = cargo_toml_path.parent().unwrap().strip_prefix(&repo_root).unwrap().display().to_string();
                                    if !relative_path.is_empty() {
                                        all_members.insert(relative_path);
                                    }
                                }
                            }

                            // Aggregate dependencies
                            if let Some(deps) = parsed_toml.dependencies {
                                for (name, dep) in deps.map {
                                    all_dependencies.insert(name, dep);
                                }
                            }
                            if let Some(deps) = parsed_toml.dev_dependencies {
                                for (name, dep) in deps.map {
                                    all_dev_dependencies.insert(name, dep);
                                }
                            }
                            if let Some(deps) = parsed_toml.build_dependencies {
                                for (name, dep) in deps.map {
                                    all_build_dependencies.insert(name, dep);
                                }
                            }
                            // Aggregate features
                            if let Some(features) = parsed_toml.features {
                                for (feature_name, feature_items) in features.map {
                                    let entry = all_features.entry(feature_name).or_insert_with(BTreeSet::new);
                                    for item in feature_items {
                                        entry.insert(item);
                                    }
                                }
                            }
                        }, 
                        Err(e) => eprintln!("Warning: Could not parse {}: {}", cargo_toml_path.display(), e),
                    }
                }
            }
        }
    }

    // Construct the new top-level Cargo.toml content
    let mut new_workspace_members: Vec<String> = all_members.into_iter().collect();
    new_workspace_members.sort();

    let mut new_cargo_toml_content = String::new();
    new_cargo_toml_content.push_str("[workspace]\n");
    new_cargo_toml_content.push_str("resolver = \"2\"\n"); // Set resolver to 2
    new_cargo_toml_content.push_str("members = [\n");
    for member in new_workspace_members {
        new_cargo_toml_content.push_str(&format!("    \"{}\"", member));
        new_cargo_toml_content.push_str(",\n");
    }
    new_cargo_toml_content.push_str(