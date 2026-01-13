#!/bin/bash

echo "🚀 AUTOMATED MODEL EXPANSION"
echo "============================"

# Build all tools first
echo "🔨 Building analysis suite..."
cargo build --release

# Run continuous expansion
while true; do
    echo "📅 $(date): Running expansion cycle..."
    ./target/release/continuous_expander
    
    # Check model size
    if [ -f "expanding_model.json" ]; then
        CONCEPTS=$(jq '.concepts | length' expanding_model.json 2>/dev/null || echo "unknown")
        echo "📊 Current model size: $CONCEPTS concepts"
    fi
    
    # Wait 1 hour before next expansion
    echo "⏳ Waiting 1 hour for next expansion..."
    sleep 3600
done
