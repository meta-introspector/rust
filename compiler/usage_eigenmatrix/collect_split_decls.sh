#!/bin/bash

echo "🔍 Running collector on split-decls projects"

PROJECTS=("split-decls-clean" "split-decls-genesis" "split-decls-rs")

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

echo ""
echo "🎯 All projects processed!"
echo "Usage data should now be in ../rust/usage_data/"
