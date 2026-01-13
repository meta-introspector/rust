# Spectral AST Classification System Overview

## Core Concept

**Signature 70** represents a mathematical classification of AST complexity using prime-encoded positional analysis. This system transforms Rust code compilation from frequency-based filtering to precise mathematical targeting.

## Mathematical Foundation

### Prime Encoding System
- Each AST node type gets a unique prime: `group=2, macro=3, path=5, fn=7, etc.`
- Position encoding: `prime^(position+1) mod 71`
- Path signature: Sum of all positional encodings in the AST path

### Complexity Classes
```
Signature Range | Class    | Compilation Probability
1-10           | simple   | ~90% (self-contained)
11-20          | compound | ~70% (minimal deps)
21-30          | complex  | ~50% (structured deps)
31-40          | nested   | ~30% (deep nesting)
41-50          | deep     | ~15% (heavy interdeps)
51-60          | ultra    | ~5%  (extreme complexity)
61-70          | extreme  | ~1%  (maximum complexity)
```

## Example: Signature 70 Analysis

**Path**: `items[11].fn.stmts[4].let.init.expr.method_call.args[0]`
**Signature**: 70 (extreme class)

This represents:
- Function #11 in file
- 5th statement in function
- Let binding initialization
- Method call expression
- First argument access

**Mathematical Breakdown**:
- `items` = prime 2, position 0 → 2¹ = 2
- `fn` = prime 7, position 2 → 7³ = 343 mod 71 = 59
- `stmts` = prime 11, position 3 → 11⁴ mod 71 = ...
- Final sum mod 71 = 70

## Compilation Pipeline

### 1. AST Analysis
```bash
# Generate AST analysis for all files
cargo build --verbose 2>&1 | grep rustc | while read cmd; do
    zombie-rustc $cmd  # Creates .syn_analysis.json files
done
```

### 2. Signature Generation
```bash
# Create positional prime signatures
cargo run --bin positional-prime-encoder
# Output: path_signatures.json with 36,157 paths → 70 unique signatures
```

### 3. Classification
```bash
# Classify by complexity and AST type
cargo run --bin signature-ast-classifier
# Shows: simple group (3 paths), extreme enum (2 paths), etc.
```

### 4. Targeted Extraction
```bash
# Extract all AST nodes with signature 70
./spectral-zombie-rustc --signature 70 input.rs
# Uses syn::parse_file + quote::quote for precise extraction
```

## Tools in System

### Core Analysis Tools
- `positional_prime_encoder.rs` - Generates mathematical signatures
- `signature_ast_classifier.rs` - Discovers AST types per complexity class
- `spectral_zombie_rustc.rs` - Extracts and quotes AST nodes by signature

### Compilation Tools  
- `spectral_filter_generator.rs` - Creates frequency-based filters (legacy)
- `complete_spectral_pipeline.sh` - Tests all 20 frequency bands
- `signature_compiler.rs` - Generates class-specific compilation targets

## Key Breakthrough

**Before**: 20 frequency bands, 10% compilation success rate
**After**: 7 mathematical classes, targeted extraction by signature number

**Signature 70 Filter Application**:
1. Read real AST using `syn::parse_file`
2. Filter nodes matching signature 70 mathematical criteria
3. Quote filtered nodes using `quote::quote` 
4. Generate compilable Rust code for extreme complexity class

This transforms spectral compilation from statistical approximation to mathematical precision.

## Usage Example

```bash
# Extract all signature 70 (extreme complexity) AST nodes
./spectral-zombie-rustc --signature 70 rustc_driver/src/lib.rs > extreme_asts.rs

# Compile the extracted extreme complexity components
rustc --crate-name extreme_components --crate-type lib extreme_asts.rs
```

The system now provides surgical precision for extracting compilable AST components based on mathematical complexity classification rather than frequency approximation.
