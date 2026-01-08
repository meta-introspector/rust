#!/bin/bash

echo "🧟♂️ Optimized Zombie Build (Skip Rustc Recompilation)"
echo "====================================================="

set -e

# Setup environment (use normal sccache)
# export RUSTC_WRAPPER="$HOME/.cargo/bin/sccache_wrapper.sh"
export RUSTC_WRAPPER="$HOME/.cargo/bin/sccache"
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/local/bin
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu

echo "📦 Using normal sccache: $RUSTC_WRAPPER"

# Check if we have cached rustc libraries
DEPS_DIR="target/debug/deps"
if [ -d "$DEPS_DIR" ] && [ -f "$DEPS_DIR/librustc_driver-379b3e9d757fb052.so" ]; then
    echo "✅ Found cached rustc libraries, skipping full rebuild"
    
    # Just build our zombie binary directly
    echo "🧟 Building zombie binary only..."
    if ! time cargo build --bin zombie_rustc_driver 2>&1 | tee cargo_optimized.log; then
        echo "❌ Build failed!"
        echo "🔍 First 2 errors:"
        echo "=================="
        grep -A5 "error\[" cargo_optimized.log | head -20
        exit 1
    fi
else
    echo "⚠️  No cached libraries found, doing full build..."
    
    # Build dependencies first (need full compilation for .rlib files)
    echo "📦 Building rustc dependencies..."
    if ! time cargo build --lib -j 20 2>&1 | tee build_deps.log; then
        echo "❌ Failed to build rustc dependencies"
        echo "🔍 Last 10 lines of output:"
        tail -10 build_deps.log
        echo "🔍 Sccache errors:"
        grep -E "(sccache.*error|Failed to open file)" build_deps.log | head -5 || echo "No sccache errors found"
        echo "🔍 Missing files:"
        grep -E "No such file or directory" build_deps.log | head -3 || echo "No missing file errors found"
        exit 1
    fi
    
    # Create symlinks after dependencies
    echo "🔗 Creating library symlinks..."
    if [ -d "$DEPS_DIR" ]; then
        cd "$DEPS_DIR"
        [ -f "librustc_driver-379b3e9d757fb052.so" ] && ln -sf librustc_driver-379b3e9d757fb052.so librustc_driver.so
        [ -f "librustc_interface-bb7c76ac1ed5097e.rlib" ] && ln -sf librustc_interface-bb7c76ac1ed5097e.rlib librustc_interface-0baff863143ebe73.rlib
        [ -f "librustc_middle-b882c09d2b6fedab.rlib" ] && ln -sf librustc_middle-b882c09d2b6fedab.rlib librustc_middle-245c3150683558be.rlib
        cd - > /dev/null
    fi
    
    # Build zombie binary
    echo "🧟 Building zombie binary..."
    if ! time cargo build --bin zombie_rustc_driver -j 20 2>&1 | tee cargo_optimized.log; then
        echo "❌ Build failed!"
        echo "🔍 First 2 errors:"
        echo "=================="
        grep -A5 "error\[" cargo_optimized.log | head -20
        exit 1
    fi
fi

# Check build success
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Optimized build complete!"
echo "📊 Build stats:"
echo "   - Zombie binary: $(ls -lh target/debug/zombie_rustc_driver 2>/dev/null | awk '{print $5}' || echo 'Not found')"
echo "   - Rustc libraries: $(ls -1 target/debug/deps/librustc_*.so 2>/dev/null | wc -l) shared objects"
echo "🧟 Run with: ./zombie.sh <rustc-args>"
