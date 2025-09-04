#!/bin/bash

# Directory to store cargo tree outputs
mkdir -p docs/submodule_cargo_trees

# Get the root of the git repository
REPO_ROOT=$(git rev-parse --show-toplevel)

echo "Generating cargo tree for all git submodules..."

# Iterate over each submodule
git submodule foreach --quiet --recursive '(
    echo "Processing submodule: $sm_path"
    SUBMODULE_PATH="$sm_path"
    SUBMODULE_NAME=$(basename "$SUBMODULE_PATH")
    OUTPUT_FILE="$REPO_ROOT/docs/submodule_cargo_trees/${SUBMODULE_NAME}_cargo_tree.txt"

    # Navigate into the submodule directory
    cd "$SUBMODULE_PATH" || {
        echo "Error: Could not change directory to $SUBMODULE_PATH" >> "$OUTPUT_FILE"
        exit 1
    }

    echo "--- Cargo Tree for $SUBMODULE_PATH ---" > "$OUTPUT_FILE"

    # Attempt to run cargo tree
    # Try with --workspace first, then without, then with --package if a workspace
    if cargo tree --workspace >> "$OUTPUT_FILE" 2>&1; then
        echo "Successfully generated cargo tree for workspace: $SUBMODULE_PATH"
    elif cargo tree >> "$OUTPUT_FILE" 2>&1; then
        echo "Successfully generated cargo tree for package: $SUBMODULE_PATH"
    else
        # If it's a workspace and the above failed, try to find a package to run cargo tree on
        # This part needs to be careful about parsing Cargo.toml for members
        # A more robust way would be to use `cargo metadata` and parse its JSON output
        # However, given the previous issues with `cargo metadata` not recognizing workspaces,
        # we'll stick to a simpler grep-based approach for member detection.
        if grep -q "^\[workspace\]" Cargo.toml; then
            # Extract member paths, handling potential multiple lines and comments
            MEMBER_CRATES=$(grep -E '^[[:space:]]*"[^"]+"' Cargo.toml | sed 's/^[[:space:]]*"\(.*\)".*/\1/')
            if [ -n "$MEMBER_CRATES" ]; then
                echo "Attempting cargo tree for member crates in $SUBMODULE_PATH:"
                for CRATE in $MEMBER_CRATES; do
                    # Check if the member path is a directory and contains a Cargo.toml
                    if [ -d "$CRATE" ] && [ -f "$CRATE/Cargo.toml" ]; then
                        echo "--- Cargo Tree for $SUBMODULE_PATH/$CRATE ---" >> "$OUTPUT_FILE"
                        (cd "$CRATE" && cargo tree) >> "$OUTPUT_FILE" 2>&1
                        if [ $? -eq 0 ]; then
                            echo "Successfully generated cargo tree for member crate: $SUBMODULE_PATH/$CRATE"
                        else
                            echo "Failed to generate cargo tree for member crate: $SUBMODULE_PATH/$CRATE" >> "$OUTPUT_FILE"
                        fi
                    else
                        echo "Skipping non-existent or invalid member crate path: $CRATE" >> "$OUTPUT_FILE"
                    fi
                done
            else
                echo "Failed to generate cargo tree for $SUBMODULE_PATH: Workspace but no valid members found." >> "$OUTPUT_FILE"
            fi
        else
            echo "Failed to generate cargo tree for $SUBMODULE_PATH: Not a simple package or recognized workspace." >> "$OUTPUT_FILE"
        fi
    fi

    echo "------------------------------------" >> "$OUTPUT_FILE"
)'

echo "All cargo tree outputs generated in docs/submodule_cargo_trees/"

# Verify total lines generated
echo "Total lines in generated cargo tree files:"
find docs/submodule_cargo_trees -type f -name "*_cargo_tree.txt" -exec cat {} + | wc -l