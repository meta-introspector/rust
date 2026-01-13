#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


if [ $# -ne 1 ]; then
    echo "Usage: $0 <results_file>"
    exit 1
fi

RESULTS_FILE="$1"
SCALES=(2 4 8 16 32 64 128 256)

echo "🎨 GENERATING MULTI-SCALE EMOJI TAPESTRIES"
echo "=========================================="
echo "Input: $RESULTS_FILE"
echo ""

for scale in "${SCALES[@]}"; do
    echo "🎯 Generating ${scale}x${scale} tapestry..."
    ./generate_emoji_tapestry.sh "$RESULTS_FILE" "$scale"
    echo ""
done

echo "✅ All tapestries generated!"
echo "📁 Files created:"
ls -la tapestry_*x*_*.txt
