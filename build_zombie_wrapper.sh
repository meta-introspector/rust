#!/bin/bash

LOG_FILE="zombie_build.log"
echo "🧟‍♂️ Starting zombie_wrapper build at $(date)" | tee $LOG_FILE
echo "=========================================" | tee -a $LOG_FILE

cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_wrapper

echo "📍 Current directory: $(pwd)" | tee -a $LOG_FILE
echo "🔨 Running: RUSTFLAGS=\"--cfg bootstrap\" cargo build --release --bin zombie_rustc" | tee -a $LOG_FILE

RUSTFLAGS="--cfg bootstrap" cargo build --release --bin zombie_rustc 2>&1 | tee -a $LOG_FILE

BUILD_EXIT_CODE=${PIPESTATUS[0]}

if [ $BUILD_EXIT_CODE -eq 0 ]; then
    echo "✅ Build completed successfully!" | tee -a $LOG_FILE
    
    # Check if binary exists
    if [ -f "target/release/zombie_rustc" ]; then
        echo "🎉 Binary found: target/release/zombie_rustc" | tee -a $LOG_FILE
        ls -la target/release/zombie_rustc | tee -a $LOG_FILE
    else
        echo "⚠️  Binary not found in expected location" | tee -a $LOG_FILE
        echo "🔍 Searching for zombie_rustc..." | tee -a $LOG_FILE
        find target -name "zombie_rustc" -type f 2>/dev/null | tee -a $LOG_FILE
    fi
else
    echo "❌ Build failed with exit code: $BUILD_EXIT_CODE" | tee -a $LOG_FILE
fi

echo "📝 Full build log saved to: $LOG_FILE" | tee -a $LOG_FILE
echo "🧟‍♂️ Build completed at $(date)" | tee -a $LOG_FILE
