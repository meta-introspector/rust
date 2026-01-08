#!/bin/bash

echo "🧟♂️ Zombie Rustc Driver Build System (with strace)"
echo "=================================================="

export RUSTC_WRAPPER=~/.cargo/bin/sccache_wrapper.sh
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/local/bin
export RUSTC_BOOTSTRAP=1

echo "📦 Using sccache wrapper: $RUSTC_WRAPPER"
echo "🔍 Tracing build with strace..."

strace -e trace=openat -f cargo build 2>&1 | grep -E "(rustc_driver|rustc_interface|rustc_middle)" | grep "ENOENT"
