#!/bin/bash

echo "🔧 Adding all bin files to Cargo.toml"

# Get all bin files
bin_files=$(find src/bin -name "*.rs" | sort)

# Create the new Cargo.toml content
cat > Cargo.toml << 'EOF'
[package]
name = "introspector-collector"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "clean_collector"
path = "src/clean_collector.rs"

[[bin]]
name = "enhanced_collector"
path = "src/enhanced_collector.rs"

[[bin]]
name = "working_usage_collector"
path = "src/working_usage_collector.rs"

[[bin]]
name = "working_collector"
path = "src/working_collector.rs"

[[bin]]
name = "simple_collector"
path = "src/simple_collector.rs"

[[bin]]
name = "enum_usage_collector"
path = "src/enum_usage_collector.rs"

EOF

# Add all bin files
for file in $bin_files; do
    # Extract filename without extension
    name=$(basename "$file" .rs)
    echo "[[bin]]" >> Cargo.toml
    echo "name = \"$name\"" >> Cargo.toml
    echo "path = \"$file\"" >> Cargo.toml
    echo "" >> Cargo.toml
done

# Add dependencies
cat >> Cargo.toml << 'EOF'
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full"] }
jsonschema = "0.18"
chrono = { version = "0.4", features = ["serde"] }
rustc_index = { workspace = true }
build_common = { workspace = true }
analyzeme = { workspace = true }
EOF

echo "✅ Updated Cargo.toml with $(echo "$bin_files" | wc -l) bin files"
echo "📦 Testing build..."
cargo build --bins
