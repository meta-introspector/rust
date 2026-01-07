#!/bin/bash

# Fix C++ header paths for Rust bootstrap
# Based on strace analysis of missing files

set -e

echo "Setting up C++ compiler flags for Nix environment..."

# Find the correct GCC C++ headers
GCC_CXX_HEADERS="/nix/store/kzq78n13l8w24jn8bx4djj79k5j717f1-gcc-14.3.0/include/c++/14.3.0"
GCC_CXX_HEADERS_ARCH="/nix/store/kzq78n13l8w24jn8bx4djj79k5j717f1-gcc-14.3.0/include/c++/14.3.0/x86_64-unknown-linux-gnu"

# Verify the paths exist
if [ ! -d "$GCC_CXX_HEADERS" ]; then
    echo "ERROR: GCC C++ headers not found at $GCC_CXX_HEADERS"
    exit 1
fi

if [ ! -f "$GCC_CXX_HEADERS/cstdlib" ]; then
    echo "ERROR: cstdlib not found at $GCC_CXX_HEADERS/cstdlib"
    exit 1
fi

echo "Found GCC C++ headers at: $GCC_CXX_HEADERS"
echo "Found cstdlib at: $GCC_CXX_HEADERS/cstdlib"

# Set up the environment with proper include paths
# Put glibc headers BEFORE C++ headers so stdlib.h is found correctly
export CXXFLAGS="-O2 -g -I${NIX_GLIBC_DEV}/include -I$GCC_CXX_HEADERS -I$GCC_CXX_HEADERS_ARCH"
export CFLAGS="-O2 -g -I${NIX_GLIBC_DEV}/include"

# Also set the compiler environment variables that cc-rs uses
export CXX="g++ -I${NIX_GLIBC_DEV}/include -I$GCC_CXX_HEADERS -I$GCC_CXX_HEADERS_ARCH"
export CC="gcc -I${NIX_GLIBC_DEV}/include"

echo "Environment configured:"
echo "CXXFLAGS: $CXXFLAGS"
echo "CFLAGS: $CFLAGS"
echo "CXX: $CXX"
echo "CC: $CC"

# Test that the compiler can find the headers
echo "Testing C++ compiler..."
echo '#include <cstdlib>' | g++ -x c++ -E - -I${NIX_GLIBC_DEV}/include -I$GCC_CXX_HEADERS -I$GCC_CXX_HEADERS_ARCH > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ C++ compiler can find cstdlib"
else
    echo "✗ C++ compiler cannot find cstdlib"
    exit 1
fi

echo "Testing C compiler..."
echo '#include <stdlib.h>' | gcc -x c -E - -I${NIX_GLIBC_DEV}/include > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ C compiler can find stdlib.h"
else
    echo "✗ C compiler cannot find stdlib.h"
    exit 1
fi

echo "Compiler configuration successful!"

# Now run the bootstrap
echo "Starting Rust bootstrap with fixed compiler flags..."
./x.py build --verbose
