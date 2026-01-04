#!/bin/bash

# Tool to find and match older vs newer API versions for mkshim compatibility

echo "=== API Version Matcher Tool ==="
echo "Finding older and newer versions to create mkshim mappings..."

# Find all TyCtxt usage patterns
echo "1. Finding TyCtxt patterns..."
grep -r "TyCtxt" /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/ --include="*.rs" | head -10

echo -e "\n2. Finding mkbuild usage patterns..."
grep -r "mkbuild" /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/ --include="*.rs" | head -10

echo -e "\n3. Finding def_id patterns..."
grep -r "def_id" /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/ --include="*.rs" | head -10

echo -e "\n4. Finding hir_node patterns..."
grep -r "hir_node\|hir().node" /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/ --include="*.rs" | head -10

echo -e "\n5. Finding typeck patterns..."
grep -r "typeck\|TypeckResults" /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/ --include="*.rs" | head -10

echo -e "\n=== Creating mkshim mapping template ==="
cat > /tmp/mkshim_mappings.txt << 'EOF'
# Older Version -> Newer Version Mappings
# Format: older_pattern => newer_pattern

# TyCtxt patterns
TyCtxt => TyCtxt<'_>
TyCtxt<'tcx> => TyCtxt<'tcx>

# HIR patterns  
tcx.hir().node(hir_id) => tcx.hir_node(hir_id)
tcx.hir_expect_item(def_id) => tcx.hir_expect_item(def_id)

# Typeck patterns
tcx.typeck_tables_of(def_id) => tcx.typeck(def_id)
TypeckTables => TypeckResults

# Build patterns
mkbuild!() => mkbuild!()

# Add more mappings as found...
EOF

echo "Template created at /tmp/mkshim_mappings.txt"
echo "Edit this file to add your mappings, then run the wrapper generator."
