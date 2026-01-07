#!/bin/bash
# Zombie Build Script with sccache optimization

set -e

echo "🧟♂️ Zombie Rustc Driver Build System"
echo "======================================"

# Setup sccache
export RUSTC_WRAPPER=~/.cargo/bin/sccache
echo "📦 Using sccache: $(which sccache)"

# Build with optimizations
echo "🔨 Building zombie driver..."
CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu time cargo build -j 20

# Setup runtime environment
echo "🌐 Setting up runtime environment..."
mkdir -p target/debug/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends

# Copy system codegen backends
if [ -d "/nix/store" ]; then
    echo "📋 Copying Nix codegen backends..."
    find /nix/store -name "*codegen*.rmeta" -exec cp {} target/debug/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/ \; 2>/dev/null || true
fi

echo "✅ Zombie build complete!"
echo "🧟 Run with: ./zombie.sh <rustc-args>"
echo "🌐 Network: ./zombie_network.sh"
