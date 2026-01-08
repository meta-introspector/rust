#!/bin/bash

echo "Building zombie driver using bootstrap system..."

# Set up environment with all necessary library paths
export CXX="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/g++-wrapper"
export CC_ENABLE_DEBUG_OUTPUT=1
export LD_LIBRARY_PATH="/nix/store/swch85kls1srlrji1i0g28wpa4y607hk-llvm-21.1.2-lib/lib:/nix/store/9bzm5g670567swmg6vd1l7l4q5spvc80-gcc-14-20241116-lib/lib:$LD_LIBRARY_PATH"
export RUSTC_WRAPPER="$HOME/.cargo/bin/sccache"
export CFG_RELEASE_CHANNEL="dev"
export RUSTC_INSTALL_BINDIR="$HOME/.cargo/bin/"

# Build zombie driver directly with cargo
cd compiler/zombie_driver/rustc_driver_impl
export RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
export RUSTC_SYSROOT="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/build/x86_64-unknown-linux-gnu/stage1"
cargo build

echo "Zombie driver built successfully!"
