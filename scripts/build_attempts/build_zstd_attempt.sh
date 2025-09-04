#!/bin/bash

# Script to build and install vendored zstd source.
# This script is part of the build attempts log (CRQ-20250904-001).

# Navigate to the zstd source directory
cd vendor/zstd

# Clean previous builds
make clean

# Build zstd library and programs
make allzstd

# Install zstd to Termux prefix
make install PREFIX=/data/data/com.termux/files/usr
