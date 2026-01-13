#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Test script to build and test each wrapper individually

echo "🧟 Testing Zombie Driver Components"
echo "=================================="

# Test span wrapper
echo "📍 Testing Span Wrapper..."
cd span_wrapper
cargo build --release
if [ $? -eq 0 ]; then
    echo "✅ Span wrapper built successfully"
else
    echo "❌ Span wrapper build failed"
    exit 1
fi
cd ..

# Test ty wrapper  
echo "🔧 Testing Ty Wrapper..."
cd ty_wrapper
cargo build --release
if [ $? -eq 0 ]; then
    echo "✅ Ty wrapper built successfully"
else
    echo "❌ Ty wrapper build failed"
    exit 1
fi
cd ..

# Test hir wrapper
echo "🏗️ Testing HIR Wrapper..."
cd hir_wrapper
cargo build --release
if [ $? -eq 0 ]; then
    echo "✅ HIR wrapper built successfully"
else
    echo "❌ HIR wrapper build failed"
    exit 1
fi
cd ..

# Test plugin system
echo "🔌 Testing Plugin System..."
rustc --edition 2021 test_plugins.rs -o test_plugins -L span_wrapper/target/release/deps -L ty_wrapper/target/release/deps -L hir_wrapper/target/release/deps --extern libloading
if [ $? -eq 0 ]; then
    echo "✅ Plugin test compiled"
    ./test_plugins
else
    echo "❌ Plugin test compilation failed"
fi

echo "🎉 All tests completed!"
