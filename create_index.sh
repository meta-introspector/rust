#!/bin/bash

# Create the index directory if it doesn't exist
mkdir -p index

# Clear previous index files
rm -f index/*.txt

CRATES_FILE="/data/data/com.termux/files/home/storage/github/rustc/crates.txt"
TEMP_UNASSIGNED_CRATES="$(mktemp)"
cp "$CRATES_FILE" "$TEMP_UNASSIGNED_CRATES"

echo "Splitting crates.txt into logical smaller files (max 7 items per file) with unique assignment..."

# Function to paginate results into chunks of 7 and remove from unassigned list
paginate_and_save_unique() {
    local grep_pattern="$1"
    local base_filename="$2"
    local count=0
    local chunk_num=1
    local temp_matched_crates="$(mktemp)"

    # Grep from the current unassigned list
    grep -E "$grep_pattern" "$TEMP_UNASSIGNED_CRATES" > "$temp_matched_crates"

    local total_matched_lines=$(wc -l < "$temp_matched_crates")

    if [ "$total_matched_lines" -eq 0 ]; then
        rm "$temp_matched_crates"
        return
    fi

    while IFS= read -r line;
    do
        echo "$line" >> "index/${base_filename}_$(printf "%02d" $chunk_num).txt"
        count=$((count + 1))
        if [ "$count" -ge 7 ]; then
            count=0
            chunk_num=$((chunk_num + 1))
        fi
    done < "$temp_matched_crates"

    # Remove matched crates from the unassigned list
    grep -v -Ff "$temp_matched_crates" "$TEMP_UNASSIGNED_CRATES" > "${TEMP_UNASSIGNED_CRATES}.tmp"
    mv "${TEMP_UNASSIGNED_CRATES}.tmp" "$TEMP_UNASSIGNED_CRATES"

    rm "$temp_matched_crates"
}

# Define all patterns to be used for exclusion later
# IMPORTANT: Ensure these patterns are specific enough to avoid matching unintended crates
ALL_SPECIFIC_PATTERNS=(
    "compiler/rustc_"
    "library/"
    "crates/"
    "src/bootstrap|src/tools/"
    "tests/"
    "vendor/aichat/vendor/google-apis-rs|vendor/aichat/vendor/google-cloud-rust"
    "vendor/libminizinc|vendor/asciicast_processor|vendor/gemini_cli_manager|vendor/kantspel_lib|vendor/minizinc_introspector|vendor/poem_|vendor/zos-"
    "vendor/ragit|vendor/layer[0-9]|vendor/ragit-command-"
    "vendor/hugging-face-dataset-validator-rust|vendor/intel-mkl-src|vendor/scirs"
    "vendor/addr2line|vendor/anyhow|vendor/bitflags|vendor/chrono|vendor/clap|vendor/serde|vendor/tokio|vendor/regex|vendor/git2|vendor/libc|vendor/log|vendor/num|vendor/rand|vendor/tempfile|vendor/toml|vendor/url|vendor/uuid|vendor/walkdir|vendor/zstd"
    "vendor/flate2|vendor/crossbeam|vendor/parking_lot|vendor/proc-macro2|vendor/syn|vendor/itertools|vendor/memchr|vendor/nix|vendor/openssl|vendor/reqwest|vendor/ring|vendor/tracing|vendor/hyper|vendor/futures|vendor/backtrace|vendor/atoi|vendor/arc-swap|vendor/ahash|vendor/anstream|vendor/anstyle|vendor/askama|vendor/pest|vendor/arrow"
)

# Prioritized Categories (more specific to less specific)

# 1. Compiler Components
paginate_and_save_unique "compiler/rustc_" "compiler_components"

# 2. Standard Library Components
paginate_and_save_unique "library/" "std_lib_components"

# 3. Project-Specific Utilities (crates/)
paginate_and_save_unique "crates/" "project_utilities"

# 4. Build System & Tools
paginate_and_save_unique "src/bootstrap|src/tools/" "build_system_tools"

# 5. Testing Infrastructure
paginate_and_save_unique "tests/" "testing_infrastructure"

# 6. Vendored Dependencies - Google APIs (more specific)
paginate_and_save_unique "vendor/aichat/vendor/google-apis-rs|vendor/aichat/vendor/google-cloud-rust" "vendored_google_apis"

# 7. Vendored Dependencies - MiniZinc related (more specific)
paginate_and_save_unique "vendor/libminizinc|vendor/asciicast_processor|vendor/gemini_cli_manager|vendor/kantspel_lib|vendor/minizinc_introspector|vendor/poem_|vendor/zos-" "vendored_minizinc_related"

# 8. Vendored Dependencies - Ragit related (more specific)
paginate_and_save_unique "vendor/ragit|vendor/layer[0-9]|vendor/ragit-command-" "vendored_ragit_related"

# 9. Vendored Dependencies - AI/ML Related (more specific)
paginate_and_save_unique "vendor/hugging-face-dataset-validator-rust|vendor/intel-mkl-src|vendor/scirs" "vendored_ai_ml"

# 10. Vendored Dependencies - General Purpose (Chunk 1 - example common libs)
paginate_and_save_unique "vendor/addr2line|vendor/anyhow|vendor/bitflags|vendor/chrono|vendor/clap|vendor/serde|vendor/tokio|vendor/regex|vendor/git2|vendor/libc|vendor/log|vendor/num|vendor/rand|vendor/tempfile|vendor/toml|vendor/url|vendor/uuid|vendor/walkdir|vendor/zstd" "vendored_general_1"

# 11. Vendored Dependencies - General Purpose (Chunk 2 - more example common libs)
paginate_and_save_unique "vendor/flate2|vendor/crossbeam|vendor/parking_lot|vendor/proc-macro2|vendor/syn|vendor/itertools|vendor/memchr|vendor/nix|vendor/openssl|vendor/reqwest|vendor/ring|vendor/tracing|vendor/hyper|vendor/futures|vendor/backtrace|vendor/atoi|vendor/arc-swap|vendor/ahash|vendor/anstream|vendor/anstyle|vendor/askama|vendor/pest|vendor/arrow" "vendored_general_2"

# 12. Catch-all for remaining unassigned crates
# Construct a single, comprehensive exclude pattern from all specific patterns
EXCLUDE_ALL_SPECIFIC_PATTERNS=$(IFS='|'; echo "${ALL_SPECIFIC_PATTERNS[*]}")

paginate_and_save_unique "." "vendored_other"

# Clean up temporary file
rm "$TEMP_UNASSIGNED_CRATES"

echo "File counts:"
wc -l index/*.txt

echo "Total lines in index files:"
find index -type f -name "*.txt" -exec cat {} + | wc -l