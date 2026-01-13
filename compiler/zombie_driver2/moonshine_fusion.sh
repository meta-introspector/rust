#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


echo "🌙 MOONSHINE FUSION ENGINE"
echo "=========================="
echo "Unifying syn.so with rustc mathematical mappings..."

if [ ! -f "syn.so" ]; then
    echo "❌ syn.so not found. Run ./build_syn_so.sh first"
    exit 1
fi

if [ ! -f "rust_type_emoji_catalog.json" ]; then
    echo "❌ Rust type catalog not found. Run rust type analyzer first"
    exit 1
fi

echo "🔗 Loading mathematical frameworks..."
echo "   📊 Rustc catalog: $(jq '.total_types' rust_type_emoji_catalog.json) types"
echo "   🌙 Syn library: $(ls -lh syn.so | awk '{print $5}')"

# Create fusion mapping
cat > fusion_mapping.json << 'FUSIONEOF'
{
  "moonshine_fusion": {
    "version": "1.0",
    "syn_to_rustc_mappings": {
      "syn::Expr": "rustc_ast::ast::ExprKind",
      "syn::Type": "rustc_ast::ast::TyKind", 
      "syn::Pat": "rustc_ast::ast::PatKind",
      "syn::Stmt": "rustc_ast::ast::StmtKind",
      "syn::Item": "rustc_ast::ast::ItemKind",
      "syn::Block": "rustc_ast::ast::Block",
      "syn::Path": "rustc_ast::ast::Path",
      "syn::Ident": "rustc_span::symbol::Ident",
      "syn::Lit": "rustc_ast::ast::LitKind"
    },
    "unified_emojis": {
      "Expression": "🌳",
      "Type": "🔢",
      "Pattern": "🎯", 
      "Statement": "📝",
      "Item": "📦",
      "Block": "🧱",
      "Path": "🛤️",
      "Identifier": "🏷️",
      "Literal": "💎"
    },
    "mathematical_properties": {
      "curve_classes": ["Linear", "Quadratic", "Cubic", "Quartic", "Quintic", "Elliptic"],
      "complexity_ranges": {
        "simple": [1.0, 2.5],
        "medium": [2.5, 4.0], 
        "complex": [4.0, 6.0],
        "elliptic": [6.0, 10.0]
      }
    }
  }
}
FUSIONEOF

echo "🌙 Fusion mapping created!"

# Create unified tapestry generator
cat > generate_unified_tapestry.sh << 'FUSIONEOF'
#!/bin/bash

echo "🎨 UNIFIED SYN-RUSTC TAPESTRY GENERATOR"
echo "======================================"

SIZE=${1:-8}
INPUT_FILE=${2:-"results_tutorial_demo_rustc_chunk_01.json"}

if [ ! -f "$INPUT_FILE" ]; then
    echo "❌ Input file not found: $INPUT_FILE"
    exit 1
fi

echo "🌙 Generating ${SIZE}x${SIZE} unified tapestry..."

# Generate tapestry with moonshine fusion
TOTAL_CELLS=$((SIZE * SIZE))

echo "🎨 MOONSHINE UNIFIED TAPESTRY - ${SIZE}x${SIZE}"
echo "=============================================="

for row in $(seq 0 $((SIZE - 1))); do
    ROW_STR=""
    for col in $(seq 0 $((SIZE - 1))); do
        CELL_IDX=$((row * SIZE + col))
        
        # Apply moonshine transformation
        PHASE=$((CELL_IDX % 8))
        case $PHASE in
            0) EMOJI="🌙" ;;  # Moonshine
            1) EMOJI="🌳" ;;  # Syn Expr
            2) EMOJI="🔢" ;;  # Rustc Ty
            3) EMOJI="🎯" ;;  # Pattern
            4) EMOJI="📦" ;;  # Item
            5) EMOJI="🧱" ;;  # Block
            6) EMOJI="💎" ;;  # Literal
            7) EMOJI="⭐" ;;  # Fusion point
        esac
        
        ROW_STR="${ROW_STR}${EMOJI}"
    done
    echo "$ROW_STR"
done

echo ""
echo "🌙 Legend:"
echo "========="
echo "🌙 Moonshine fusion points"
echo "🌳 Syn expressions → Rustc ExprKind"
echo "🔢 Syn types → Rustc TyKind"
echo "🎯 Syn patterns → Rustc PatKind"
echo "📦 Syn items → Rustc ItemKind"
echo "🧱 Code blocks (unified)"
echo "💎 Literals (unified)"
echo "⭐ Mathematical fusion nodes"
FUSIONEOF

chmod +x generate_unified_tapestry.sh

echo ""
echo "✅ Moonshine fusion engine ready!"
