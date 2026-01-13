#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Build LLVM codegen backend with sccache

set -e

echo "🔨 Building LLVM codegen backend..."

cd ../rustc_codegen_llvm

# Setup sccache
export RUSTC_WRAPPER=~/.cargo/bin/sccache
echo "📦 Using sccache: $(which sccache)"

# Build with LLVM environment and sccache
nix-shell -p llvm libxml2 --run "cargo build --lib"

echo "✅ LLVM codegen backend build complete!"

# Show built files
echo "📋 Built files:"
find ../../target -name "*rustc_codegen_llvm*" -type f | head -5
