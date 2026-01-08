#!/bin/bash

echo "🧟♂️ Manual Rustc Build (Build Required Libraries)"
echo "================================================="

set -e

# Setup environment
export RUSTC_WRAPPER="$HOME/.cargo/bin/sccache_wrapper.sh"
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/local/bin
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu

echo "📦 Using sccache wrapper: $RUSTC_WRAPPER"

# List of rustc crates we need to build (excluding zombie_rustc_driver)
RUSTC_CRATES=(
    "rustc_session"
    "rustc_query_system" 
    "rustc_parse"
    "rustc_middle"
    "rustc_attr_parsing"
    "rustc_infer"
    "rustc_mir_dataflow"
    "rustc_query_impl"
    "rustc_public_bridge"
    "rustc_symbol_mangling"
    "rustc_pattern_analysis"
    "rustc_transmute"
    "rustc_monomorphize"
    "rustc_incremental"
    "rustc_public"
    "rustc_ast_passes"
    "rustc_ast_lowering"
    "rustc_trait_selection"
    "rustc_expand"
    "rustc_lint"
    "rustc_ty_utils"
    "rustc_traits"
    "rustc_hir_analysis"
    "rustc_const_eval"
    "rustc_codegen_ssa"
    "rustc_resolve"
    "rustc_metadata"
    "rustc_builtin_macros"
    "rustc_hir_typeck"
    "rustc_mir_build"
    "rustc_mir_transform"
    "rustc_passes"
    "rustc_privacy"
    "rustc_borrowck"
    "rustc_driver_impl"
    "rustc_interface"
    "rustc_driver"
)

echo "📦 Building ${#RUSTC_CRATES[@]} rustc crates..."

for crate in "${RUSTC_CRATES[@]}"; do
    echo "🔨 Building $crate..."
    if ! cargo build -p "$crate" --lib 2>&1 | tee "build_${crate}.log"; then
        echo "❌ Failed to build $crate"
        echo "🔍 Last 5 lines of output:"
        tail -5 "build_${crate}.log"
        exit 1
    fi
done

echo "🔗 Creating library symlinks..."
DEPS_DIR="target/debug/deps"
if [ -d "$DEPS_DIR" ]; then
    cd "$DEPS_DIR"
    
    # Find the actual rustc_driver .so file and create symlink
    DRIVER_SO=$(ls -t librustc_driver-*.so 2>/dev/null | head -1)
    if [ -n "$DRIVER_SO" ]; then
        ln -sf "$DRIVER_SO" librustc_driver.so
        echo "✅ Created librustc_driver.so -> $DRIVER_SO"
    fi
    
    # Find the actual rustc_interface .rlib file and create symlink
    INTERFACE_RLIB=$(ls -t librustc_interface-*.rlib 2>/dev/null | head -1)
    if [ -n "$INTERFACE_RLIB" ]; then
        ln -sf "$INTERFACE_RLIB" librustc_interface-0baff863143ebe73.rlib
        echo "✅ Created librustc_interface-0baff863143ebe73.rlib -> $INTERFACE_RLIB"
    fi
    
    # Find the actual rustc_middle .rlib file and create symlink
    MIDDLE_RLIB=$(ls -t librustc_middle-*.rlib 2>/dev/null | head -1)
    if [ -n "$MIDDLE_RLIB" ]; then
        ln -sf "$MIDDLE_RLIB" librustc_middle-245c3150683558be.rlib
        echo "✅ Created librustc_middle-245c3150683558be.rlib -> $MIDDLE_RLIB"
    fi
    
    cd - > /dev/null
fi

echo "🧟 Building zombie binary..."
if ! cargo build --bin zombie_rustc_driver -v 2>&1 | tee zombie_build.log; then
    echo "❌ Failed to build zombie binary"
    echo "🔍 Last 10 lines of output:"
    tail -10 zombie_build.log
    exit 1
fi

echo "✅ Manual build complete!"
echo "📊 Build stats:"
echo "   - Zombie binary: $(ls -lh target/debug/zombie_rustc_driver 2>/dev/null | awk '{print $5}' || echo 'Not found')"
echo "   - Rustc libraries: $(ls -1 target/debug/deps/librustc_*.so target/debug/deps/librustc_*.rlib 2>/dev/null | wc -l) files"
echo "🧟 Run with: ./zombie.sh <rustc-args>"
