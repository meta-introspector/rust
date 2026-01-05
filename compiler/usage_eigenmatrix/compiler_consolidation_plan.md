# Compiler Driver Consolidation Plan

## Current State Analysis

We have **18 compiler-related binaries** that need consolidation:

### **Drivers (3)**
- `meta_bootstrap_driver.rs` - Prime constant signature generation with roundtrip testing
- `monster_rustc_driver.rs` - Monster Group mathematical analysis driver  
- `monster_compiler_driver.rs` - Alternative monster compiler implementation

### **Plugins (4 + 2 fixed)**
- `rustc_24bit_plugin.rs` - Core 24-bit DefId mapping plugin for rustc
- `rustc_matrix_plugin.rs` - Matrix-based rustc plugin
- `dwim_compiler_plugin.rs` - DWIM (Do What I Mean) compiler plugin
- `monster_24bit_plugin.rs` - Monster Group 24-bit plugin
- `*.rs.fixed` - Auto-fixed versions

### **Collectors (2)**
- `working_matrix_collector.rs` - Working matrix data collector (copied from introspector-collector)
- `matrix_24bit_collector.rs` - 24-bit matrix collector

### **Mappers (7)**
- `defid_24bit_mapper.rs` - **CORE** 24-bit DefId mapping system
- `source_24bit_mapper.rs` - Source code to 24-bit mapping
- `syn_hir_defid_mapper.rs` - Syn AST to HIR DefId mapping
- `ast_homotopy_mapper.rs` - AST homotopy mapping
- `feature_lattice_mapper.rs` - Feature lattice mapping
- `kleene_lattice_mapper.rs` - Kleene algebra lattice mapping
- `topological_hole_mapper.rs` - Topological hole mapping

## Consolidation Strategy

### **Keep Core Working Systems (4)**
1. **`defid_24bit_mapper.rs`** - Main 24-bit mapping system ✅
2. **`usage_data_matrix.rs`** - Complete usage data processing ✅
3. **`collision_analyzer.rs`** - Collision analysis ✅
4. **`module_scrubber.rs`** - Duplicate code detection ✅

### **Consolidate into Unified Driver (1)**
Create **`unified_compiler_driver.rs`** combining:
- `meta_bootstrap_driver.rs` (prime constants)
- `monster_rustc_driver.rs` (monster analysis)
- `monster_compiler_driver.rs` (alternative implementation)

### **Consolidate into Unified Plugin (1)**
Create **`unified_rustc_plugin.rs`** combining:
- `rustc_24bit_plugin.rs` (core functionality)
- `rustc_matrix_plugin.rs` (matrix operations)
- `monster_24bit_plugin.rs` (monster group features)

### **Archive Experimental Mappers (7)**
Move to `archive/experimental/`:
- All 7 mapper files (keep for research reference)

### **Remove Duplicates/Fixed Files (3)**
- `working_matrix_collector.rs` (duplicate of introspector-collector)
- `matrix_24bit_collector.rs` (superseded by usage_data_matrix)
- `*.rs.fixed` files (auto-generated)

## Final Structure (6 binaries total)

### **Core Production (4)**
- `defid_24bit_mapper` - 24-bit DefId mapping
- `usage_data_matrix` - Usage data processing  
- `collision_analyzer` - Collision analysis
- `module_scrubber` - Duplicate detection

### **Unified Tools (2)**
- `unified_compiler_driver` - All driver functionality
- `unified_rustc_plugin` - All plugin functionality

## Implementation Plan

1. Create unified driver and plugin
2. Test functionality preservation
3. Archive experimental mappers
4. Remove duplicates
5. Update Cargo.toml
6. Document final architecture

This reduces from **18 compiler binaries** to **6 focused tools** while preserving all functionality.
