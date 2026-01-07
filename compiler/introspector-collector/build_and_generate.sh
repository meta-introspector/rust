#!/bin/bash

# Build script for introspector-collector with proper rustc flags
export RUSTFLAGS="-C prefer-dynamic"

echo "Building library..."
cargo build --lib

if [ $? -eq 0 ]; then
    echo "Library built successfully!"
    
    echo "Running enum macro generator..."
    cargo run --bin generate_rustc_macros
    
    if [ $? -eq 0 ]; then
        echo "Enum macros generated successfully!"
        echo "Check src/generated/ for output files"
    else
        echo "Failed to generate enum macros"
    fi
else
    echo "Failed to build library"
fi
