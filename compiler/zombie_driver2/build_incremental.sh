#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


echo "🧟♂️ Zombie Incremental Build"
echo "============================"

set -e

# Setup environment (same as build_optimized.sh)
export RUSTC_WRAPPER="$HOME/.cargo/bin/sccache_wrapper.sh"
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/local/bin
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu

echo "📦 Using sccache wrapper: $RUSTC_WRAPPER"

# Just build the zombie binary (dependencies should be cached)
echo "🧟 Building zombie binary (incremental)..."
time cargo build --bin zombie_rustc_driver

echo "✅ Incremental build complete!"
echo "📊 Binary size: $(ls -lh target/debug/zombie_rustc_driver 2>/dev/null | awk '{print $5}' || echo 'Not found')"
