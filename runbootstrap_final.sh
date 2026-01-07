#!/bin/bash

echo "Starting final Rust bootstrap with proper library paths..."

# Set up environment with all necessary library paths
export CXX="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/g++-wrapper"
export CC_ENABLE_DEBUG_OUTPUT=1
export LD_LIBRARY_PATH="/nix/store/swch85kls1srlrji1i0g28wpa4y607hk-llvm-21.1.2-lib/lib:/nix/store/9bzm5g670567swmg6vd1l7l4q5spvc80-gcc-14-20241116-lib/lib:$LD_LIBRARY_PATH"

echo "Environment set:"
echo "CXX=$CXX"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

# Run bootstrap
python3 ./x.py build --verbose
