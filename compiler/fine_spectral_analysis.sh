#!/bin/bash

echo "🎵 FINE-GRAINED SPECTRAL ANALYSIS"
echo "================================="

RUSTC_DRIVER="../rustc_driver_impl/src/lib.rs"

echo "🔍 Testing micro-frequency bands around 0.10..."
echo ""

# Test frequencies around 0.10 with smaller increments
for freq in 0.08 0.09 0.095 0.10 0.105 0.11 0.12; do
    echo -n "Micro-band $freq: "
    
    # Extract spectral component
    ../zombie_driver2/target/release/spectral-zombie-rustc --filter $freq $RUSTC_DRIVER >/dev/null 2>&1
    
    # Get the generated file
    ACTUAL_FILE=$(ls ../rustc_driver_impl/src/lib.*.rs 2>/dev/null | tail -1)
    
    if [ -f "$ACTUAL_FILE" ]; then
        ITEMS=$(grep "Generated items:" "$ACTUAL_FILE" | cut -d: -f2 | tr -d ' ')
        echo "$ITEMS items"
        
        # Clean up
        rm -f "$ACTUAL_FILE"
    else
        echo "NO OUTPUT"
    fi
done

echo ""
echo "🎯 Testing even finer increments around successful bands..."

# Test very fine increments
for freq in 0.098 0.099 0.100 0.101 0.102; do
    echo -n "Ultra-fine $freq: "
    
    ../zombie_driver2/target/release/spectral-zombie-rustc --filter $freq $RUSTC_DRIVER >/dev/null 2>&1
    ACTUAL_FILE=$(ls ../rustc_driver_impl/src/lib.*.rs 2>/dev/null | tail -1)
    
    if [ -f "$ACTUAL_FILE" ]; then
        ITEMS=$(grep "Generated items:" "$ACTUAL_FILE" | cut -d: -f2 | tr -d ' ')
        echo "$ITEMS items"
        rm -f "$ACTUAL_FILE"
    else
        echo "NO OUTPUT"
    fi
done

echo ""
echo "📊 Analysis: Need to find frequency bands with different item counts"
echo "    to separate mixed content in the 'path' band."
