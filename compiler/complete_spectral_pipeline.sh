#!/bin/bash

echo "🎵 COMPLETE SPECTRAL COMPILATION PIPELINE"
echo "========================================="

RUSTC_DRIVER="rustc_driver_impl/src/lib.rs"
SUCCESS_COUNT=0
TOTAL_COUNT=20

echo "🔍 Extracting and compiling all 20 frequency bands..."
echo ""

# Process each frequency band
for i in {1..20}; do
    FREQ=$(echo "scale=2; ($i-1) * 0.05" | bc)
    BAND_NAME="band_$(printf "%02d" $i)_freq_$(echo $FREQ | tr '.' '_')"
    
    echo -n "Band $i (freq $FREQ): "
    
    # Extract spectral component
    zombie_driver2/target/release/spectral-zombie-rustc --filter $FREQ $RUSTC_DRIVER >/dev/null 2>&1
    
    # Find generated file
    SPECTRAL_FILE=$(ls rustc_driver_impl/src/lib.*.rs 2>/dev/null | head -1)
    
    if [ -f "$SPECTRAL_FILE" ]; then
        # Get item count
        ITEMS=$(grep "Generated items:" "$SPECTRAL_FILE" 2>/dev/null | cut -d: -f2 | tr -d ' ' || echo "?")
        
        # Try compilation
        if rustc --crate-name "$BAND_NAME" \
                 --crate-type lib \
                 --edition 2021 \
                 --allow warnings \
                 --allow dead_code \
                 --allow unused_variables \
                 --allow unused_imports \
                 "$SPECTRAL_FILE" \
                 -o "${BAND_NAME}.rlib" 2>/dev/null; then
            
            SIZE=$(stat -c%s "${BAND_NAME}.rlib" 2>/dev/null || echo "0")
            echo "✅ SUCCESS ($ITEMS items, ${SIZE}B)"
            SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        else
            echo "❌ FAILED ($ITEMS items)"
        fi
        
        # Clean up spectral file
        rm -f "$SPECTRAL_FILE"
    else
        echo "❌ NO EXTRACTION"
    fi
done

echo ""
echo "📊 SPECTRAL COMPILATION RESULTS:"
echo "================================"
echo "Successful bands: $SUCCESS_COUNT/$TOTAL_COUNT"
echo "Success rate: $((SUCCESS_COUNT * 100 / TOTAL_COUNT))%"

if [ $SUCCESS_COUNT -gt 0 ]; then
    echo ""
    echo "🎯 Successfully Compiled Spectral Bands:"
    ls -la band_*.rlib 2>/dev/null | while read line; do
        echo "  $line"
    done
    
    echo ""
    echo "📈 Total compiled library size:"
    du -ch band_*.rlib 2>/dev/null | tail -1
fi
