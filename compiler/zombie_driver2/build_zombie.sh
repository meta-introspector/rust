#!/bin/bash
# Zombie Build Script with sccache optimization

set -e

echo "🧟♂️ Zombie Rustc Driver Build System"
echo "======================================"

# Setup sccache
# export RUSTC_WRAPPER="$HOME/.cargo/bin/sccache_wrapper.sh"
# echo "📦 Using sccache wrapper: $RUSTC_WRAPPER"
echo "📦 Building without cache to fix version mismatch"

# Build with optimizations
echo "🔨 Building zombie driver..."
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/local/bin

# Build dependencies first, then create symlinks
echo "📦 Building dependencies..."
CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu cargo build --lib -j 20 2>&1 | tee build_deps.log

# Create required symlinks after dependencies are built
echo "🔗 Creating library symlinks..."
DEPS_DIR="target/debug/deps"
if [ -d "$DEPS_DIR" ]; then
    cd "$DEPS_DIR"
    [ -f "librustc_driver-379b3e9d757fb052.so" ] && ln -sf librustc_driver-379b3e9d757fb052.so librustc_driver.so
    [ -f "librustc_interface-bb7c76ac1ed5097e.rlib" ] && ln -sf librustc_interface-bb7c76ac1ed5097e.rlib librustc_interface-0baff863143ebe73.rlib
    [ -f "librustc_middle-b882c09d2b6fedab.rlib" ] && ln -sf librustc_middle-b882c09d2b6fedab.rlib librustc_middle-245c3150683558be.rlib
    cd - > /dev/null
else
    echo "⚠️  Dependencies directory not found, skipping symlinks"
fi

# Now build the binary
echo "🧟 Building zombie binary..."
CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu time cargo build --bin zombie_rustc_driver -j 20 2>&1 | tee build.log

# Check for build errors
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "❌ Build failed! Checking for extern location errors..."
    grep -n "extern location.*does not exist" build.log || echo "No extern location errors found"
    grep -n "could not compile" build.log || echo "No compilation errors found"
    exit 1
fi

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
