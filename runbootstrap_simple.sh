#!/bin/bash

# Use g++ wrapper to fix system includes
set -e

echo "Using g++ wrapper to fix system includes..."

# Set cc-rs to use our wrapper
export CXX_x86_64_unknown_linux_gnu="$(pwd)/g++-wrapper"
export CC_x86_64_unknown_linux_gnu="gcc"

echo "Starting Rust bootstrap..."
./x.py build --verbose
