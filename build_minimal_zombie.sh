#!/bin/bash

echo "Building minimal zombie compiler..."

# Set up environment
export LD_LIBRARY_PATH="/nix/store/swch85kls1srlrji1i0g28wpa4y607hk-llvm-21.1.2-lib/lib:/nix/store/9bzm5g670567swmg6vd1l7l4q5spvc80-gcc-14-20241116-lib/lib:$LD_LIBRARY_PATH"
export RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
export RUSTFLAGS="--cfg bootstrap"

# Build libzombie first
cd compiler/libzombie
cargo build
echo "libzombie built"

# Build zombie wrapper
cd ../zombie_wrapper  
cargo build
echo "zombie_wrapper built"

echo "Minimal zombie compiler ready!"
