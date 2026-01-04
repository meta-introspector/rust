#!/bin/bash
# 🚀 AUTOCARGO - Convert standalone Rust scripts to Cargo crates

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/autocargo_crates"

# Create target directory
mkdir -p "$TARGET_DIR"

# Function to create Cargo.toml for a script
create_cargo_toml() {
    local crate_name="$1"
    local crate_dir="$2"
    
    cat > "$crate_dir/Cargo.toml" << EOF
[package]
name = "$crate_name"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
EOF
}

# Function to process a single Rust file
process_rust_file() {
    local rust_file="$1"
    local base_name=$(basename "$rust_file" .rs)
    local crate_name=$(echo "$base_name" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9_]/_/g')
    local crate_dir="$TARGET_DIR/$crate_name"
    
    echo "📦 Creating crate: $crate_name"
    
    # Create crate structure
    mkdir -p "$crate_dir/src"
    
    # Copy source file
    cp "$rust_file" "$crate_dir/src/main.rs"
    
    # Create Cargo.toml
    create_cargo_toml "$crate_name" "$crate_dir"
    
    echo "✅ Created: $crate_dir"
}

# Process split-decls-genesis binaries
echo "🎭 Processing Monster Group binaries..."
if [ -d "../../../split-decls-genesis/src/bin" ]; then
    for rust_file in ../../../split-decls-genesis/src/bin/*.rs; do
        if [ -f "$rust_file" ]; then
            process_rust_file "$rust_file"
        fi
    done
fi

# Process any other standalone Rust files
echo "🔍 Scanning for other Rust files..."
find ../../../ -name "*.rs" -type f | grep -E "(bin/|examples/)" | while read rust_file; do
    # Skip if already in a proper cargo project
    if [ ! -f "$(dirname "$rust_file")/../Cargo.toml" ] && [ ! -f "$(dirname "$rust_file")/../../Cargo.toml" ]; then
        process_rust_file "$rust_file"
    fi
done

echo "🎯 Autocargo complete! Created crates in: $TARGET_DIR"
echo "📊 Total crates: $(ls -1 "$TARGET_DIR" | wc -l)"
