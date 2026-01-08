#!/bin/bash
# Zombie Analysis Script

set -e

echo "🧟 Zombie Driver Analysis"
echo "========================"

# Check if zombie is built
if [ ! -f "target/debug/zombie_rustc_driver" ]; then
    echo "❌ Zombie not built. Run ./build_zombie.sh first"
    exit 1
fi

# Analyze binary
echo "📊 Binary Analysis:"
echo "   Size: $(du -h target/debug/zombie_rustc_driver | cut -f1)"
echo "   Type: $(file target/debug/zombie_rustc_driver)"

# Check dependencies
echo ""
echo "🔗 Dependencies:"
ldd target/debug/zombie_rustc_driver | head -10

# Network capabilities
echo ""
echo "🌐 Network Analysis:"
if netstat -tuln | grep -q ":0"; then
    echo "   Active listeners detected"
else
    echo "   No active listeners"
fi

echo ""
echo "✅ Analysis complete"
