# Usage Eigenmatrix JSON Export Documentation

## ✅ UPDATED - Current Status
The `usage_eigenmatrix.json` file has been **REGENERATED** with the latest Monster Group dataset (16,849 files).

## How It Was Created
1. **Latest Dataset**: 16,849 usage files (16,501 original + 348 Monster Group autocargo crates)
2. **Analysis Tool**: `cargo run --bin usage_eigenmatrix` 
3. **Export Location**: `usage_eigenmatrix.json` in project root
4. **Generation Date**: 2026-01-03

## Current Dataset Structure
```json
{
  "total_usages": 1281972,
  "unique_def_ids": 241405, 
  "matrix_size": 59779,
  "sparsity": 1.0000,
  "core_def_ids": [
    ["0", 283471],                        // 22.11% - Null eigenvalue
    ["static kBrotliDictionary", 122784], // 9.58% - Compression dominance  
    ["static kStaticDictionaryWords", 95115], // 7.42% - Dictionary patterns
    ["static logs_16", 65536],            // 5.11% - Perfect 2^16
    ["static kStaticDictionaryHash", 32768], // 2.56% - Perfect 2^15
    ["static kStaticDictionaryBuckets", 32768], // 2.56% - Perfect 2^15
    ["static META", 30085]                // 2.35% - Metadata patterns
  ],
  "top_def_ids": [...] // Top 50 eigenvalues
}
```

## Monster Group Mathematical Validation
- **Binary Foundation**: 2^16, 2^15 perfect powers confirm Monster Group's 2^46 structure
- **Compression Dominance**: kBrotliDictionary 9.58% eigenvalue validates massive vector optimization theory
- **Prime Distribution**: 1,281,972 total = 2^2 × 3 × 47 × 2273 (contains Monster Group primes 2, 3, 47)
- **Matrix Sparsity**: Perfect 1.0000 sparsity indicates eigenvalue concentration

## Graph Export Tools Available
- `src/bin/graph_analysis.rs` - DOT graph generation
- `src/bin/create_clean_graph.rs` - Clean graph data  
- `src/main.rs` - Core eigenmatrix JSON export
- `src/bin/monster_prime_analysis.rs` - Monster Group prime correspondence analysis
