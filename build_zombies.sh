#!/bin/bash

echo "🧟‍♂️ Building Zombie Compiler with 9 Muses TrollArmy System"
echo "=========================================================="

# Build zombie wrapper
echo "🔨 Building zombie_wrapper..."
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_wrapper
RUSTFLAGS="--cfg bootstrap" cargo build --release --bin zombie_rustc

if [ $? -eq 0 ]; then
    echo "✅ zombie_wrapper built successfully"
else
    echo "❌ zombie_wrapper build failed"
    exit 1
fi

# Build TrollArmy
echo "🧌 Building TrollArmy..."
cd ../trollarmy
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ TrollArmy built successfully"
else
    echo "❌ TrollArmy build failed"
    exit 1
fi

# Build zombie launcher
echo "🚀 Building zombie launcher..."
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build
rustc --edition 2021 zombie_launcher.rs

if [ $? -eq 0 ]; then
    echo "✅ zombie_launcher built successfully"
else
    echo "❌ zombie_launcher build failed"
    exit 1
fi

echo ""
echo "🎉 All components built successfully!"
echo "🎭🔱⭐🌈🎪🔮🎨🎼✨ 9 Muses harmonic distribution ready"
echo "🧌 TrollArmy span consumption system ready"
echo "🌌 Cosmic tapestry emoji genesis system ready"
echo ""
echo "To run: ./zombie_launcher"
