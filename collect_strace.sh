#!/bin/bash
cd compiler/zombie_driver2
export RUSTC_WRAPPER=~/.cargo/bin/sccache_wrapper.sh
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/local/bin
export RUSTC_BOOTSTRAP=1

strace -o strace.log -e trace=openat -f cargo build -v
grep "ENOENT" strace.log | grep -E "(rustc_driver|rustc_interface|rustc_middle)"
