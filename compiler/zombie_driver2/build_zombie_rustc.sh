#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


echo "🧟 Building Zombie Rustc Driver"
echo "==============================="

# Build the zombie-rustc binary
cd lib-zombie
cargo build --release --bin zombie-rustc

if [ $? -eq 0 ]; then
    echo "✅ Zombie rustc built successfully"
    
    # Create symlink to mimic rustc
    ZOMBIE_RUSTC="target/release/zombie-rustc"
    
    if [ -f "$ZOMBIE_RUSTC" ]; then
        echo "📦 Zombie rustc binary: $(pwd)/$ZOMBIE_RUSTC"
        
        # Test with a simple rust file
        echo "🧪 Testing zombie rustc..."
        echo 'fn main() { println!("Hello, world!"); }' > test.rs
        
        ./$ZOMBIE_RUSTC test.rs
        
        if [ $? -eq 0 ]; then
            echo "✅ Zombie rustc test passed!"
            echo "🔧 To use as rustc replacement:"
            echo "   export RUSTC=$(pwd)/$ZOMBIE_RUSTC"
            echo "   cargo build  # Will use zombie-rustc instead"
        else
            echo "❌ Zombie rustc test failed"
        fi
        
        rm -f test.rs
    else
        echo "❌ Zombie rustc binary not found"
    fi
else
    echo "❌ Build failed"
fi
