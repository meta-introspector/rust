#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


if [ $# -ne 2 ]; then
    echo "Usage: $0 <results_file> <scale>"
    echo "Scales: 2, 4, 8, 16, 32, 64, 128, 256"
    exit 1
fi

RESULTS_FILE="$1"
SCALE="$2"

if [ ! -f "$RESULTS_FILE" ]; then
    echo "❌ Results file not found: $RESULTS_FILE"
    exit 1
fi

# Emoji palette based on code characteristics
declare -A CODE_EMOJIS=(
    # Core compiler emojis
    ["rustc"]="🦀"     # Rust crab
    ["ast"]="🌳"       # AST tree
    ["hir"]="🧬"       # High-level IR
    ["mir"]="⚛️"        # Mid-level IR
    ["ty"]="🔢"        # Types
    ["trait"]="💎"     # Traits
    ["impl"]="🔧"      # Implementations
    ["struct"]="🏗️"    # Structures
    ["enum"]="🎭"      # Enums
    ["fn"]="⚡"        # Functions
    ["macro"]="🪄"     # Macros
    ["error"]="🚨"     # Errors
    ["test"]="🧪"      # Tests
    ["lib"]="📚"       # Libraries
    ["main"]="🚀"      # Main files
    
    # Mathematical emojis by LMFDB classification
    ["5.11696"]="🌌"   # Elliptic curves
    ["4.8535"]="💫"    # High degree
    ["3.7374"]="⭐"    # Medium degree
    ["2.6000"]="✨"    # Low degree
    
    # Enum symbols to emojis
    ["Hc"]="🔥"        # Hydrogen-like (fundamental)
    ["Lc"]="💧"        # Lithium-like (reactive)
    ["Cc"]="💨"        # Carbon-like (versatile)
    ["Nc"]="🌪️"        # Nitrogen-like (complex)
    ["Oc"]="🌊"        # Oxygen-like (essential)
    ["Rc"]="⚡"        # Rare elements
    
    # Size-based emojis
    ["tiny"]="🔸"      # < 1KB
    ["small"]="🔹"     # 1-10KB
    ["medium"]="🔶"    # 10-100KB
    ["large"]="🔷"     # > 100KB
)

# Default emoji for unknown patterns
DEFAULT_EMOJI="⬜"

# Function to get emoji for a file entry
get_emoji_for_file() {
    local file="$1"
    local size="$2"
    local lmfdb="$3"
    local enum_symbol="$4"
    
    # Priority order: specific patterns -> LMFDB -> enum -> size
    
    # Check for specific code patterns
    for pattern in rustc ast hir mir ty trait impl struct enum fn macro error test lib main; do
        if [[ "$file" == *"$pattern"* ]]; then
            echo "${CODE_EMOJIS[$pattern]}"
            return
        fi
    done
    
    # Check LMFDB classification
    for lmfdb_pattern in "5.11696" "4.8535" "3.7374" "2.6000"; do
        if [[ "$lmfdb" == *"$lmfdb_pattern"* ]]; then
            echo "${CODE_EMOJIS[$lmfdb_pattern]}"
            return
        fi
    done
    
    # Check enum symbol
    if [[ -n "${CODE_EMOJIS[$enum_symbol]}" ]]; then
        echo "${CODE_EMOJIS[$enum_symbol]}"
        return
    fi
    
    # Check size
    if [ "$size" -lt 1024 ]; then
        echo "${CODE_EMOJIS[tiny]}"
    elif [ "$size" -lt 10240 ]; then
        echo "${CODE_EMOJIS[small]}"
    elif [ "$size" -lt 102400 ]; then
        echo "${CODE_EMOJIS[medium]}"
    else
        echo "${CODE_EMOJIS[large]}"
    fi
}

# Extract file data and generate tapestry
echo "🎨 Generating ${SCALE}x${SCALE} emoji tapestry from $RESULTS_FILE"
echo ""

TOTAL_CELLS=$((SCALE * SCALE))
CONTRIBUTOR=$(jq -r '.contributor' "$RESULTS_FILE")
CHUNK=$(jq -r '.chunk' "$RESULTS_FILE")
TOTAL_FILES=$(jq -r '.total_files' "$RESULTS_FILE")

echo "👤 Contributor: $CONTRIBUTOR"
echo "📦 Chunk: $CHUNK"
echo "📊 Files: $TOTAL_FILES"
echo "🎯 Tapestry: ${SCALE}x${SCALE} = $TOTAL_CELLS cells"
echo ""

# Create tapestry array
declare -a TAPESTRY

# Process files and map to tapestry cells
for i in $(seq 0 $((TOTAL_CELLS - 1))); do
    # Map cell to file index (with wrapping for smaller file counts)
    FILE_IDX=$((i % TOTAL_FILES))
    
    # Extract file data
    FILE=$(jq -r ".processed_files[$FILE_IDX].file" "$RESULTS_FILE")
    SIZE=$(jq -r ".processed_files[$FILE_IDX].size" "$RESULTS_FILE")
    LMFDB=$(jq -r ".processed_files[$FILE_IDX].lmfdb_label" "$RESULTS_FILE")
    ENUM=$(jq -r ".processed_files[$FILE_IDX].enum_symbol" "$RESULTS_FILE")
    
    # Get emoji for this file
    EMOJI=$(get_emoji_for_file "$FILE" "$SIZE" "$LMFDB" "$ENUM")
    TAPESTRY[$i]="$EMOJI"
done

# Generate tapestry output
OUTPUT_FILE="tapestry_${SCALE}x${SCALE}_${CONTRIBUTOR}_$(basename $CHUNK).txt"

echo "🎨 EMOJI TAPESTRY - ${SCALE}x${SCALE}" > "$OUTPUT_FILE"
echo "====================================" >> "$OUTPUT_FILE"
echo "Contributor: $CONTRIBUTOR" >> "$OUTPUT_FILE"
echo "Chunk: $CHUNK" >> "$OUTPUT_FILE"
echo "Files: $TOTAL_FILES" >> "$OUTPUT_FILE"
echo "Generated: $(date)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Print tapestry grid
for row in $(seq 0 $((SCALE - 1))); do
    ROW_STR=""
    for col in $(seq 0 $((SCALE - 1))); do
        CELL_IDX=$((row * SCALE + col))
        ROW_STR="${ROW_STR}${TAPESTRY[$CELL_IDX]}"
    done
    echo "$ROW_STR" >> "$OUTPUT_FILE"
done

echo "" >> "$OUTPUT_FILE"
echo "🎨 LEGEND:" >> "$OUTPUT_FILE"
echo "=========" >> "$OUTPUT_FILE"
echo "🦀 Rust compiler core    🌳 AST structures" >> "$OUTPUT_FILE"
echo "🧬 HIR (High-level IR)   ⚛️ MIR (Mid-level IR)" >> "$OUTPUT_FILE"
echo "🔢 Type system          💎 Traits" >> "$OUTPUT_FILE"
echo "🔧 Implementations      🏗️ Structures" >> "$OUTPUT_FILE"
echo "🎭 Enums               ⚡ Functions" >> "$OUTPUT_FILE"
echo "🪄 Macros              🚨 Error handling" >> "$OUTPUT_FILE"
echo "🧪 Tests               📚 Libraries" >> "$OUTPUT_FILE"
echo "🚀 Main files          🌌 Elliptic curves" >> "$OUTPUT_FILE"
echo "💫 High complexity     ⭐ Medium complexity" >> "$OUTPUT_FILE"
echo "✨ Low complexity      🔥 Fundamental types" >> "$OUTPUT_FILE"
echo "💧 Reactive code       💨 Versatile code" >> "$OUTPUT_FILE"
echo "🌪️ Complex patterns    🌊 Essential code" >> "$OUTPUT_FILE"
echo "🔸 Tiny files          🔹 Small files" >> "$OUTPUT_FILE"
echo "🔶 Medium files        🔷 Large files" >> "$OUTPUT_FILE"

# Also display on screen
echo "🎨 EMOJI TAPESTRY - ${SCALE}x${SCALE}"
echo "===================================="
for row in $(seq 0 $((SCALE - 1))); do
    ROW_STR=""
    for col in $(seq 0 $((SCALE - 1))); do
        CELL_IDX=$((row * SCALE + col))
        ROW_STR="${ROW_STR}${TAPESTRY[$CELL_IDX]}"
    done
    echo "$ROW_STR"
done

echo ""
echo "✅ Tapestry saved to: $OUTPUT_FILE"
echo "🎨 Visual representation of $TOTAL_FILES Rust files complete!"
