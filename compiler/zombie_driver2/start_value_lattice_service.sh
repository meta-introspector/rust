#!/bin/bash

# Value Lattice Service - Uses 30GB RAM for complete Rust constant indexing
echo "🚀 VALUE LATTICE SERVICE - 30GB RAM MODE"
echo "========================================"

# Set high memory limits
export RUST_MIN_STACK=268435456  # 256MB stack
export RUST_BACKTRACE=1

# Build optimized release version
echo "🔨 Building optimized release version..."
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_driver2
cargo build --release --bin value_lattice_indexer

if [ $? -eq 0 ]; then
    echo "✅ Build successful"
    
    # Create service log directory
    mkdir -p /mnt/data1/meta-introspector/service-logs
    
    # Run with high memory allocation and logging
    echo "🔥 Starting value lattice service with 30GB RAM..."
    echo "📊 Processing ALL Rust files in the ecosystem..."
    echo "⏰ Started at: $(date)"
    
    # Run in background with full logging
    nohup ./target/release/value_lattice_indexer > /mnt/data1/meta-introspector/service-logs/value-lattice-$(date +%Y%m%d-%H%M%S).log 2>&1 &
    
    # Get PID
    SERVICE_PID=$!
    echo $SERVICE_PID > /mnt/data1/meta-introspector/service-logs/value-lattice.pid
    
    echo "🎯 Service started with PID: $SERVICE_PID"
    echo "📋 Log file: /mnt/data1/meta-introspector/service-logs/value-lattice-$(date +%Y%m%d-%H%M%S).log"
    echo "🛑 To stop: kill $SERVICE_PID"
    echo ""
    echo "🔍 Monitor progress with:"
    echo "   tail -f /mnt/data1/meta-introspector/service-logs/value-lattice-*.log"
    echo "   watch 'ls -la /mnt/data1/meta-introspector/value-lattice/'"
    echo ""
    echo "💾 Expected output: Complete lattice of ALL Rust constants"
    echo "📈 Memory usage: Up to 30GB for in-memory processing"
    
else
    echo "❌ Build failed"
    exit 1
fi
