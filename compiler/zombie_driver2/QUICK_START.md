# Quick Start Guide

## Running the Analysis

### 1. Basic Pattern Analysis
```bash
cd compiler/zombie_driver2
cargo run --bin text_bit_analysis
cargo run --bin merged_bit_constant_analysis
```

### 2. Decoder Discovery
```bash
cargo run --bin function_opcode_matrix
cargo run --bin custom_demangler
cargo run --bin symbol_ngram_clustering
```

### 3. Proof System
```bash
cargo run --bin instruction_decoding_proof
cargo run --bin macro_pattern_proof
cargo run --bin auto_suggest_bad_data
```

## Key Files

- `SELF_REFERENTIAL_DECODER_DOCUMENTATION.md` - Complete analysis documentation
- `text_bit_analysis.rs` - Basic instruction pattern extraction
- `function_opcode_matrix.rs` - Self-referential pattern detection
- `custom_demangler.rs` - Rust symbol demangling with pattern search
- `instruction_decoding_proof.rs` - Step-by-step decoder proof system
- `auto_suggest_bad_data.rs` - Intelligent instruction classification

## Expected Output

The analysis will reveal:
- Self-referential decoder functions in `compiler_builtins` and `rustc_driver_impl`
- Instruction patterns that exist both as code and data
- Auto-suggested classifications for unknown opcodes
- Mathematical correlations with Monster Group theory

## Binary Requirements

Analysis targets: `/path/to/rustc_driver.so` (debug build recommended for maximum symbol information)
