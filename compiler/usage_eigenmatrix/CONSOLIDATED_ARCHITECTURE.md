# Consolidated Compiler Architecture

## Final Binary Structure (6 Core Tools)

### **Production Tools (4)**

#### 1. `defid_24bit_mapper`
- **Purpose**: Core 24-bit DefId mapping system with paged memory
- **Usage**: `cargo run --bin defid_24bit_mapper`
- **Features**: 16MB matrix, sparse allocation, prime basis signatures

#### 2. `usage_data_matrix` 
- **Purpose**: Process complete rustc usage dataset (1.4M DefIds)
- **Usage**: `cargo run --bin usage_data_matrix`
- **Features**: File-based collision tracking, 100% coverage analysis

#### 3. `collision_analyzer`
- **Purpose**: Analyze collision patterns and find top collided signatures
- **Usage**: `cargo run --bin collision_analyzer`
- **Features**: Semantic analysis, crate/function breakdown

#### 4. `module_scrubber`
- **Purpose**: Duplicate code detection via module ID scrubbing
- **Usage**: `cargo run --bin module_scrubber`
- **Features**: Cross-context duplicate detection, structural pattern matching

### **Unified Tools (2)**

#### 5. `unified_compiler_driver`
- **Purpose**: All driver functionality in one tool
- **Usage**: `cargo run --bin unified_compiler_driver <COMMAND>`
- **Commands**:
  - `prime-constants` - Generate/test prime constant signatures
  - `monster-analysis` - Monster Group mathematical analysis
  - `monster-compiler` - Alternative compiler demonstration

#### 6. `collision_accountant`
- **Purpose**: Comprehensive collision accounting (100% coverage)
- **Usage**: `cargo run --bin collision_accountant`
- **Features**: Complete collision type breakdown, 250K+ collision analysis

## Archived Components

### **Experimental Mappers** (`archive/experimental/`)
- `ast_homotopy_mapper.rs` - AST homotopy mapping research
- `feature_lattice_mapper.rs` - Feature lattice experiments
- `kleene_lattice_mapper.rs` - Kleene algebra research
- `syn_hir_defid_mapper.rs` - Syn to HIR mapping
- `topological_hole_mapper.rs` - Topological analysis
- `source_24bit_mapper.rs` - Source code mapping
- `defid_24bit_mapper.rs` - Original mapper (superseded)

### **Legacy Drivers** (`archive/experimental/`)
- `meta_bootstrap_driver.rs` - Original prime constant driver
- `monster_rustc_driver.rs` - Original monster analysis
- `monster_compiler_driver.rs` - Alternative implementation

## System Capabilities

### **Mathematical Mapping**
- 24-bit signature space (16.7M possible locations)
- Prime basis [2,3,5,7,11,13,17,19] for stable signatures
- Paged memory allocation (4096 × 4KB pages)
- 82.88% unique mapping rate across rustc ecosystem

### **Collision Analysis**
- 100% collision coverage (250,974 collisions analyzed)
- Type breakdown: 56.39% numeric, 34.50% mixed, 8.76% string
- Cross-crate collision detection
- Semantic relationship analysis

### **Duplicate Detection**
- Module ID scrubbing for structural similarity
- 62 duplicate patterns found across rustc
- Cross-compilation context duplicate detection
- Mathematical signature-based matching

## Usage Examples

```bash
# Core 24-bit mapping
cargo run --bin defid_24bit_mapper

# Process complete dataset
cargo run --bin usage_data_matrix

# Analyze collisions
cargo run --bin collision_analyzer

# Find duplicates
cargo run --bin module_scrubber

# Generate prime constants
cargo run --bin unified_compiler_driver prime-constants

# Complete collision accounting
cargo run --bin collision_accountant
```

## Architecture Benefits

1. **Reduced Complexity**: 18 binaries → 6 focused tools
2. **Clear Separation**: Production vs experimental vs unified tools
3. **Maintained Functionality**: All capabilities preserved
4. **Better Documentation**: Clear purpose and usage for each tool
5. **Easier Maintenance**: Consolidated related functionality

This architecture provides a clean, maintainable foundation for 24-bit mathematical mapping of Rust compilation while preserving all research and experimental work in organized archives.
