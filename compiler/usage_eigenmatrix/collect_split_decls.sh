#!/bin/bash

echo "🔍 Running collector on split-decls projects"

# Run autocargo first to create crates from standalone scripts
echo "🚀 Running autocargo to create crates from standalone scripts..."
cd "$(dirname "$0")"
./autocargo.sh

PROJECTS=("split-decls-clean" "split-decls-genesis" "split-decls-rs")

# Add projects from current directory level
LOCAL_PROJECTS=("split-decls-clean" "split-decls-genesis" "split-decls-rs" "incremental-rust-compiler")

# Add projects from meta-introspector
META_PROJECTS=("rust-bootstrap-nix" "git-submodules-rs-nix" "git-submodule-tools-rs" "introspector-llc" "lattice-introspector" "minizinc-introspector")

# Function to process a project
process_project() {
    local project_path="$1"
    local project_name=$(basename "$project_path")
    
    echo ""
    echo "📦 Processing $project_name..."
    echo "Path: $project_path"
    echo "=================="
    
    cd "$project_path"
    
    # Check if it's a Rust project
    if [ -f "Cargo.toml" ]; then
        echo "🦀 Found Rust project, running collector..."
        
        # Copy our collector script
        cp "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver/build_with_collector.sh" .
        
        # Make it executable
        chmod +x build_with_collector.sh
        
        # Run with timeout and error handling
        USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data" timeout 300 bash ./build_with_collector.sh 2>/dev/null || echo "⚠️ Timeout or error, continuing..."
        
        echo "✅ Collection complete for $project_name"
    else
        echo "⚠️ Not a Rust project, skipping..."
    fi
    
    # Return to original directory
    cd - > /dev/null
}

for project in "${PROJECTS[@]}"; do
    project_path="../../../$project"
    if [ -d "$project_path" ]; then
        process_project "$project_path"
    else
        echo "❌ Project not found: $project_path"
    fi
done

# Process local projects at ../../ level
for project in "${LOCAL_PROJECTS[@]}"; do
    project_path="../../../$project"
    if [ -d "$project_path" ]; then
        process_project "$project_path"
    else
        echo "❌ Project not found: $project_path"
    fi
done

# Process meta-introspector projects - find all Cargo.toml files
echo ""
echo "🔍 Finding all Rust projects in meta-introspector..."
mapfile -t CARGO_TOMLS < <(find /mnt/data1/nix/source/github/meta-introspector -name "Cargo.toml" | head -20)

for cargo_toml in "${CARGO_TOMLS[@]}"; do
    project_dir=$(dirname "$cargo_toml")
    process_project "$project_dir"
done

# Process autocargo-generated crates
echo ""
echo "📦 Processing autocargo-generated crates..."
if [ -d "./autocargo_crates" ]; then
    for crate_dir in ./autocargo_crates/*/; do
        if [ -d "$crate_dir" ] && [ -f "$crate_dir/Cargo.toml" ]; then
            process_project "$crate_dir"
        fi
    done
fi

echo ""
echo "🎯 All projects processed!"
echo "Usage data should now be in ../rust/usage_data/"
