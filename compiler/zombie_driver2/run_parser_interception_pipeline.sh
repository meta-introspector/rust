#!/bin/bash
# MASTER PIPELINE: Transparent Parser Function Interception

echo "🎯 RUSTC PARSER INTERCEPTION PIPELINE"
echo "====================================="
echo "1. Capture calls with self-profile/perf"
echo "2. Extract call graph from profile data"  
echo "3. Read target list into program"
echo "4. Install transparent trampolines"
echo ""

cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_driver2

# Make scripts executable
chmod +x step1_capture_calls.sh

# Step 1: Capture parser calls
echo "🚀 STEP 1: Capturing parser calls..."
./step1_capture_calls.sh

# Step 2: Extract call graph
echo ""
echo "🚀 STEP 2: Extracting call graph..."
rustc --extern serde_json step2_extract_callgraph.rs -o step2_extract_callgraph
./step2_extract_callgraph

# Step 3: Prepare targets
echo ""
echo "🚀 STEP 3: Preparing target metadata..."
rustc --extern serde_json step3_prepare_targets.rs -o step3_prepare_targets  
./step3_prepare_targets

# Step 4: Install trampolines
echo ""
echo "🚀 STEP 4: Installing trampolines..."
rustc --extern serde_json --extern goblin step4_install_trampolines.rs -o step4_install_trampolines -L dependency=target/debug/deps
./step4_install_trampolines

echo ""
echo "✅ PIPELINE COMPLETE!"
echo "🎭 Parser function trampolines are now active"
echo "📊 Any rustc parser calls will be intercepted and logged"
