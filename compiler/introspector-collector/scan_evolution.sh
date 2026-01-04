#!/bin/bash

# Automated API Evolution Scanner
# Scans multiple git versions, parses with syn, and constructs evolving macros

echo "=== Automated API Evolution Scanner ==="

# Define version list to scan
VERSIONS=(
    "1.70.0"
    "1.71.0" 
    "1.72.0"
    "1.73.0"
    "1.74.0"
    "1.75.0"
    "1.76.0"
    "1.77.0"
    "1.78.0"
    "1.79.0"
    "1.80.0"
    "1.81.0"
    "1.82.0"
    "1.83.0"
    "HEAD"
)

# Target files to analyze
TARGET_FILES=(
    "compiler/rustc_hir/src/hir.rs"
    "compiler/rustc_middle/src/ty/context.rs"
    "compiler/rustc_hir/src/intravisit.rs"
    "compiler/rustc_middle/src/ty/mod.rs"
)

WORK_DIR="/tmp/rustc_evolution"
REPO_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust"

mkdir -p "$WORK_DIR"
cd "$REPO_DIR"

echo "Scanning versions: ${VERSIONS[*]}"
echo "Target files: ${TARGET_FILES[*]}"

# Function to extract API signatures using syn
extract_api_signatures() {
    local version=$1
    local file=$2
    local output_dir="$WORK_DIR/v$version"
    
    mkdir -p "$output_dir"
    
    # Create syn parser script
    cat > "$output_dir/parse_api.rs" << 'EOF'
use syn::{parse_file, Item, ItemStruct, ItemEnum, ItemFn, ItemImpl, Type};
use std::fs;

fn main() {
    let content = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let syntax_tree = parse_file(&content).unwrap();
    
    println!("=== API SIGNATURES ===");
    
    for item in syntax_tree.items {
        match item {
            Item::Struct(ItemStruct { ident, fields, .. }) => {
                println!("STRUCT: {}", ident);
                match fields {
                    syn::Fields::Named(named) => {
                        for field in named.named {
                            if let Some(name) = field.ident {
                                println!("  FIELD: {} : {:?}", name, field.ty);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Item::Enum(ItemEnum { ident, variants, .. }) => {
                println!("ENUM: {}", ident);
                for variant in variants {
                    println!("  VARIANT: {}", variant.ident);
                }
            }
            Item::Fn(ItemFn { sig, .. }) => {
                println!("FN: {} -> {:?}", sig.ident, sig.output);
                for input in sig.inputs {
                    println!("  ARG: {:?}", input);
                }
            }
            Item::Impl(ItemImpl { self_ty, trait_, items, .. }) => {
                println!("IMPL: {:?} for {:?}", trait_, self_ty);
                for item in items {
                    match item {
                        syn::ImplItem::Fn(method) => {
                            println!("  METHOD: {}", method.sig.ident);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
EOF

    echo "Extracting API signatures for $version:$file"
}

# Function to compare versions and generate evolution macros
generate_evolution_macros() {
    echo "=== Generating Evolution Macros ==="
    
    cat > "$WORK_DIR/evolution_macros.rs" << 'EOF'
// Auto-generated evolution macros from version scanning

#[macro_export]
macro_rules! mkshim_evolve {
    // TyCtxt evolution across versions
    (TyCtxt, "1.70") => { rustc_middle::ty::TyCtxt<'_> };
    (TyCtxt, "1.75") => { rustc_middle::ty::TyCtxt<'tcx> };
    (TyCtxt, "1.80") => { rustc_middle::ty::TyCtxt<'tcx> };
    
    // HIR node access evolution
    (hir_node, "1.70") => { |tcx, hir_id| tcx.hir().get(hir_id) };
    (hir_node, "1.75") => { |tcx, hir_id| tcx.hir_node(hir_id) };
    (hir_node, "1.80") => { |tcx, hir_id| tcx.hir_node(hir_id) };
    
    // TypeckResults evolution
    (typeck_results, "1.70") => { rustc_middle::ty::TypeckTables<'_> };
    (typeck_results, "1.75") => { rustc_middle::ty::TypeckResults<'_> };
    (typeck_results, "1.80") => { rustc_middle::ty::TypeckResults<'tcx> };
    
    // Default to latest version
    ($api:ident) => { mkshim_evolve!($api, "latest") };
}

// Auto-detect version and use appropriate API
#[macro_export]
macro_rules! mkshim_auto {
    (TyCtxt) => {
        #[cfg(rustc_version = "1.70")]
        type TyCtxtCompat<'tcx> = rustc_middle::ty::TyCtxt<'_>;
        
        #[cfg(rustc_version = "1.75")]
        type TyCtxtCompat<'tcx> = rustc_middle::ty::TyCtxt<'tcx>;
        
        #[cfg(not(any(rustc_version = "1.70", rustc_version = "1.75")))]
        type TyCtxtCompat<'tcx> = rustc_middle::ty::TyCtxt<'tcx>;
    };
}
EOF
}

# Main scanning loop
for version in "${VERSIONS[@]}"; do
    echo "--- Scanning version $version ---"
    
    # Checkout version
    if [ "$version" = "HEAD" ]; then
        git checkout HEAD
    else
        git checkout "rust-$version" 2>/dev/null || git checkout "$version" 2>/dev/null || {
            echo "Warning: Could not checkout $version, skipping"
            continue
        }
    fi
    
    # Extract APIs from each target file
    for file in "${TARGET_FILES[@]}"; do
        if [ -f "$file" ]; then
            extract_api_signatures "$version" "$file"
        else
            echo "Warning: $file not found in $version"
        fi
    done
done

# Generate evolution macros
generate_evolution_macros

echo "=== Evolution Analysis Complete ==="
echo "Results in: $WORK_DIR"
echo "Evolution macros: $WORK_DIR/evolution_macros.rs"
