#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


echo "🚀 EXECUTING SYN MOONSHINE UNIFICATION"
echo "======================================"
echo ""

echo "Step 1: Building syn.so..."
./build_syn_so.sh

echo ""
echo "Step 2: Analyzing syn AST..."
./analyze_syn_ast.sh

echo ""
echo "Step 3: Moonshine fusion..."
./moonshine_fusion.sh

echo ""
echo "Step 4: Generating unified tapestry..."
./generate_unified_tapestry.sh 8

echo ""
echo "🌙 SYN MOONSHINE UNIFICATION COMPLETE!"
echo "====================================="
echo ""
echo "📁 Created files:"
echo "   🔧 syn.so - Dynamic syn library"
echo "   🌙 fusion_mapping.json - Syn-Rustc mappings"
echo "   🎨 Unified tapestry system"
echo ""
echo "🎯 Next steps:"
echo "   - Apply to P2P compilation"
echo "   - Integrate with rustc mathematical framework"
echo "   - Generate moonshine-enhanced emoji tapestries"
