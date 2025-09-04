#!/bin/bash

# Script to attempt building vendored CMake source.
# This script is part of the build attempts log (CRQ-20250904-001).

# Navigate to the project root (assuming this script is run from project root or similar)
# cd /data/data/com.termux/files/home/storage/github/rustc/

# 1. Create build directory and navigate
mkdir -p vendor/cmake/build && cd vendor/cmake/build

# 2. Configure CMake build
cmake ../ -DCMAKE_MAN_DIR=share/man -DCMAKE_DOC_DIR=share/doc/cmake -DCMAKE_USE_SYSTEM_CURL=ON -DCMAKE_USE_SYSTEM_EXPAT=ON -DCMAKE_USE_SYSTEM_FORM=ON -DCMAKE_USE_SYSTEM_JSONCPP=ON -DCMAKE_USE_SYSTEM_LIBARCHIVE=ON -DCMAKE_USE_SYSTEM_LIBRHASH=ON -DCMAKE_USE_SYSTEM_LIBUV=ON -DCMAKE_USE_SYSTEM_ZLIB=ON -DBUILD_CursesDialog=ON

# Note: This script does not include the 'make' step as the CMake configure step was cancelled by the user.
# To continue, run 'make' after successful configuration.
