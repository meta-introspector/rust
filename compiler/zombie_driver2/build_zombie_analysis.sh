#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


echo "🧟 Zombie Rustc Build - Character Analysis of Entire Compiler"
echo "============================================================="

set -e

# Setup zombie-rustc environment
export RUSTC="$(pwd)/target/release/zombie-rustc"
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/local/bin
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu

echo "🧟 Using zombie-rustc: $RUSTC"
echo "📊 This will analyze character patterns in the entire rustc codebase"

# Build with zombie-rustc (will do character analysis instead of compilation)
echo "🔍 Starting character analysis of rustc..."
time cargo build --workspace --all -j 20 2>&1 | tee zombie_build.log

echo "✅ Zombie analysis complete!"
echo "📁 Analysis files saved to target/debug/*.zombie_analysis.json"

# Count analysis files
ANALYSIS_COUNT=$(ls target/debug/*.zombie_analysis.json 2>/dev/null | wc -l)
echo "📊 Generated $ANALYSIS_COUNT analysis files"

# Merge all analyses
echo "🔗 Merging all analysis files..."
./target/release/merge-analysis target/debug

echo "🎯 Complete rustc character analysis finished!"
echo "   - Individual files: target/debug/*.zombie_analysis.json"
echo "   - Merged analysis: target/debug/merged_rustc_analysis.json"
echo "   - Build log: zombie_build.log"
