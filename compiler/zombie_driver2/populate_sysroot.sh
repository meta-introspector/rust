#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Populate sysroot script

set -e

echo "🔧 Populating zombie sysroot..."

# Base sysroot path
SYSROOT="target/debug/glibc-hwcaps"

# Create all needed directories
echo "📁 Creating directory structure..."
mkdir -p "$SYSROOT/lib/rustlib/src/rust/library/std/src"
mkdir -p "$SYSROOT/lib/rustlib/rustc-src/rust/compiler/rustc/src"
mkdir -p "$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends"
mkdir -p "$SYSROOT/lib64/rustlib"

# Copy source files
echo "📋 Copying source files..."
if [ -f "../../library/sysroot/src/lib.rs" ]; then
    cp "../../library/sysroot/src/lib.rs" "$SYSROOT/lib/rustlib/src/rust/library/std/src/lib.rs"
    echo "✅ Copied std lib.rs"
else
    echo "⚠️  std lib.rs not found, creating dummy"
    echo "// Dummy std lib.rs" > "$SYSROOT/lib/rustlib/src/rust/library/std/src/lib.rs"
fi

# Create rustc main.rs
echo "📋 Creating rustc main.rs..."
echo "// Dummy rustc main.rs" > "$SYSROOT/lib/rustlib/rustc-src/rust/compiler/rustc/src/main.rs"

# Copy codegen backends
echo "📋 Copying codegen backends..."
if [ -d "target/debug/codegen-backends" ]; then
    cp target/debug/codegen-backends/* "$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/"
    echo "✅ Copied codegen backends"
else
    echo "⚠️  No codegen backends found"
fi

# Find and copy LLVM codegen .so if it exists
echo "🔍 Looking for LLVM codegen .so..."
llvm_so=$(find /nix/store -name "*codegen_llvm*.so" 2>/dev/null | head -1)
if [ -n "$llvm_so" ] && [ -f "$llvm_so" ]; then
    cp "$llvm_so" "$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/"
    echo "✅ Copied LLVM codegen: $(basename "$llvm_so")"
else
    echo "⚠️  LLVM codegen .so not found"
fi

echo "✅ Sysroot population complete!"
echo "📊 Sysroot structure:"
find "$SYSROOT" -type f | head -10
