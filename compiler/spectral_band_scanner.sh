#!/bin/bash

echo "🎵 AUTOMATED SPECTRAL BAND SCANNER"
echo "=================================="

RUSTC_DRIVER="../rustc_driver_impl/src/lib.rs"
SUCCESS_COUNT=0
TOTAL_COUNT=0

echo "🔍 Scanning all spectral frequencies..."
echo ""

# Scan through all discovered spectral frequencies
for freq in 0.00 0.05 0.10 0.15 0.20 0.25 0.30 0.35 0.40 0.45 0.50 0.55 0.60 0.65 0.70 0.75 0.80 0.85 0.90 0.95; do
    echo -n "Filter $freq: "
    
    # Extract spectral component
    ../zombie_driver2/target/release/spectral-zombie-rustc --filter $freq $RUSTC_DRIVER >/dev/null 2>&1
    
    # Get the generated file
    SPECTRAL_FILE="../rustc_driver_impl/src/lib.*.rs"
    ACTUAL_FILE=$(ls ../rustc_driver_impl/src/lib.*.rs 2>/dev/null | tail -1)
    
    if [ -f "$ACTUAL_FILE" ]; then
        # Try to compile it
        CRATE_NAME="spectral_$(echo $freq | tr '.' '_')"
        
        if rustc --crate-name "$CRATE_NAME" \
                 --crate-type lib \
                 --edition 2021 \
                 --allow warnings \
                 "$ACTUAL_FILE" \
                 -o "${CRATE_NAME}.rlib" 2>/dev/null; then
            
            ITEMS=$(grep "Generated items:" "$ACTUAL_FILE" | cut -d: -f2 | tr -d ' ')
            SIZE=$(stat -c%s "${CRATE_NAME}.rlib" 2>/dev/null || echo "0")
            echo "✅ SUCCESS ($ITEMS items, ${SIZE}B)"
            SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        else
            echo "❌ FAILED"
        fi
        
        # Clean up
        rm -f "$ACTUAL_FILE"
    else
        echo "❌ NO OUTPUT"
    fi
    
    TOTAL_COUNT=$((TOTAL_COUNT + 1))
done

echo ""
echo "📊 SPECTRAL SCAN RESULTS:"
echo "========================"
echo "Successful bands: $SUCCESS_COUNT/$TOTAL_COUNT"
echo "Success rate: $((SUCCESS_COUNT * 100 / TOTAL_COUNT))%"

if [ $SUCCESS_COUNT -gt 0 ]; then
    echo ""
    echo "🎯 Self-Complete Spectral Bands:"
    ls -la spectral_*.rlib 2>/dev/null | while read line; do
        echo "  $line"
    done
fi
