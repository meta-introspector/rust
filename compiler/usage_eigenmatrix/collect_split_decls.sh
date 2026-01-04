#!/bin/bash

echo "🔍 Running collector on split-decls projects"

PROJECTS=("split-decls-clean" "split-decls-genesis" "split-decls-rs")

# Add projects from current directory level
LOCAL_PROJECTS=("split-decls-clean" "split-decls-genesis" "split-decls-rs" "incremental-rust-compiler")

# Add projects from meta-introspector
META_PROJECTS=("rust-bootstrap-nix" "git-submodules-rs-nix" "git-submodule-tools-rs" "introspector-llc" "lattice-introspector" "minizinc-introspector")

for project in "${PROJECTS[@]}"; do
    project_path="../../../$project"
    
    if [ -d "$project_path" ]; then
        echo ""
        echo "📦 Processing $project..."
        echo "=================="
        
        cd "$project_path"
        
        # Check if it's a Rust project
        if [ -f "Cargo.toml" ]; then
            echo "🦀 Found Rust project, running collector..."
            
            # Copy our collector script
            cp "../rust/compiler/rustc_driver/build_with_collector.sh" .
            
            # Make it executable
            chmod +x build_with_collector.sh
            
            # Run with debug output and file tracking
            echo "=== Before collection ==="
            ls -la ../rust/usage_data/ | wc -l
            
            USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data" bash -x ./build_with_collector.sh
            
            echo "=== After collection ==="
            ls -la ../rust/usage_data/ | wc -l
            
            echo "=== Files created in this run ==="
            find ../rust/usage_data/ -name "*.json" -newer ./build_with_collector.sh | head -10
            
            echo "=== Collector trace summary ==="
            grep -E "(usage_data|\.json)" /tmp/collector_trace_$project.log | tail -5
            
            echo "✅ Collection complete for $project"
        else
            echo "⚠️  Not a Rust project, skipping..."
        fi
        
        # Return to original directory
        cd - > /dev/null
    else
        echo "❌ Project not found: $project_path"
    fi
done

# Process local projects at ../../ level
for project in "${LOCAL_PROJECTS[@]}"; do
    project_path="../../../$project"
    
    if [ -d "$project_path" ]; then
        echo ""
        echo "📦 Processing local ../../../$project..."
        echo "=================="
        
        cd "$project_path"
        
        # Check if it's a Rust project
        if [ -f "Cargo.toml" ]; then
            echo "🦀 Found Rust project, running collector..."
            
            # Copy our collector script
            cp "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver/build_with_collector.sh" .
            
            # Make it executable
            chmod +x build_with_collector.sh
            
            # Run with debug output and file tracking
            echo "=== Before collection ==="
            ls -la /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data/ | wc -l
            
            USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data" bash -x ./build_with_collector.sh
            
            echo "=== After collection ==="
            ls -la /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data/ | wc -l
            
            echo "=== Files created in this run ==="
            find /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data/ -name "*.json" -newer ./build_with_collector.sh | head -10
            
            echo "✅ Collection complete for local ../../../$project"
        else
            echo "⚠️  Not a Rust project, skipping..."
        fi
        
        # Return to original directory
        cd - > /dev/null
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
    project_name=$(basename "$project_dir")
    
    echo ""
    echo "📦 Processing meta-introspector Rust project: $project_name..."
    echo "Path: $project_dir"
    echo "=================="
    
    cd "$project_dir"
    
    echo "🦀 Found Rust project, running collector..."
    
    # Copy our collector script
    cp "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver/build_with_collector.sh" .
    
    # Make it executable
    chmod +x build_with_collector.sh
    
    # Run with debug output and file tracking
    echo "=== Before collection ==="
    ls -la /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data/ | wc -l
    
    USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data" timeout 300 bash -x ./build_with_collector.sh 2>/dev/null || echo "Timeout or error, continuing..."
    
    echo "=== After collection ==="
    ls -la /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data/ | wc -l
    
    echo "=== Files created in this run ==="
    find /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data/ -name "*.json" -newer ./build_with_collector.sh 2>/dev/null | head -5
    
    echo "✅ Collection complete for $project_name"
    
    # Return to original directory
    cd - > /dev/null
done

echo ""
echo "🎯 All projects processed!"
echo "Usage data should now be in ../rust/usage_data/"
