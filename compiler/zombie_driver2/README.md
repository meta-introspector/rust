# Zombie Driver2 - Rust Compiler Analysis Suite

## Overview
Advanced Rust compiler analysis and instrumentation system with 200+ tools for deep code analysis, AST processing, and compiler introspection.

## Quick Start - Parser Function Tracing

### 🔬 Trace Parser Calls
```bash
# Run comprehensive parser tracing
./trace_rustc_parser_calls.sh [optional_rust_file]

# This will create timestamped output directory with:
# - rustc self-profile data
# - perf function call traces  
# - strace system call logs
# - zombie rustc instrumentation
# - call graph analysis from 2.8GB rustc_driver.so
```

### 🎯 Goal: Insert Function Trampolines
The tracing identifies call paths from `main()` to parser functions so we can:
1. **Wrap parser functions** with our own code
2. **Capture parsing data** in real-time  
3. **Extract AST structures** during compilation
4. **Build next-char/word predictors** from live parsing

## Key Components

### Core Analysis Tools
- **`analyze_syn_ast.sh`** - Comprehensive syn AST analysis pipeline
- **`frequency_analysis.rs`** - Character/word frequency analysis for next-char prediction
- **`syn_monster_topology.rs`** - Mathematical topology analysis of syn AST structures
- **`call_graph_exporter.rs`** - Export function call graphs from compiled code

### Compiler Integration
- **`live_rustc_caller.rs`** - Direct integration with rustc compiler functions
- **`compiler_simulator.rs`** - Simulate compiler behavior for analysis
- **`rustc_wrapper.sh`** - Wrapper for rustc integration and instrumentation
- **`cargo_hijack_system.rs`** - Hook into cargo build system for analysis

### Data Processing & Export
- **`huggingface_dataset_exporter.rs`** - Export analysis results to HuggingFace datasets
- **`parquet_converter.rs`** - Convert analysis data to Parquet format
- **`string_codec_extractor.rs`** - Extract string encoding patterns from code
- **`rdf_linked_data_generator.rs`** - Generate RDF linked data from analysis

## Dependencies

### Required Files
- **rustc_driver.so** (2.8GB): `/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so`
- **Rust source files**: 954,505 files indexed in `~/nix/vendor/rust/cargo2nix/files.txt`

### Build Requirements
- Rust toolchain with syn, quote, proc-macro2
- Access to rustc compiler internals
- Large memory capacity (2.8GB+ for rustc_driver.so)

## Usage

### 1. Build Syn Library
```bash
./build_syn_so.sh
```
This creates `syn.so` by building the `syn_moonshine` library with compiler instrumentation.

### 2. Run AST Analysis
```bash
./analyze_syn_ast.sh ~/nix/vendor/rust/cargo2nix/files.txt
```
Processes all 954K Rust files for AST analysis and mathematical topology.

### 3. Extract Functions
```bash
cargo run --bin call_graph_exporter
cargo run --bin live_rustc_caller
```

### 4. Frequency Analysis
```bash
cargo run --bin frequency_analysis
cargo run --bin ngram_monster_phi
```

### 5. Export Results
```bash
cargo run --bin huggingface_dataset_exporter
cargo run --bin parquet_converter
```

## Architecture

### Zombie Compilation System
The system uses "zombie compilation" - instrumenting the actual Rust compiler (rustc_driver.so) to capture:
- Function call patterns during compilation
- AST transformation sequences
- Memory access patterns
- Compilation decision trees

### Mathematical Analysis
- **Monster Group Theory**: Applied to AST structures for mathematical classification
- **Lattice Analysis**: Topological analysis of code structures
- **Eigenmatrix Computation**: Mathematical signatures for code patterns
- **Moonshine Fusion**: Unification of syn AST with rustc AST

### Data Pipeline
```
Rust Files → syn AST → Mathematical Analysis → Function Extraction → Export
    ↓           ↓            ↓                    ↓              ↓
  954K files  Topology   Monster Groups    Call Graphs    HuggingFace
```

## Key Features

### 1. Complete Coverage
- Processes all Rust code in the ecosystem (954K files)
- Integrates with actual compiler for real compilation data
- Mathematical analysis of code structures

### 2. Advanced Analysis
- Next-character/word prediction from code corpus
- Function usage pattern extraction
- Compiler decision tree analysis
- AST mathematical topology

### 3. Export Capabilities
- HuggingFace dataset format
- Parquet files for big data analysis
- RDF linked data for semantic web
- JSON/CSV for general use

## Files Overview

### Analysis Scripts (20+)
- `analyze_*.sh` - Various analysis pipelines
- `build_*.sh` - Build and compilation scripts
- `test_*.sh` - Testing and validation scripts

### Core Analyzers (50+)
- `*_analyzer.rs` - Specialized analysis tools
- `*_exporter.rs` - Data export utilities
- `*_generator.rs` - Code/data generation tools

### Mathematical Tools (30+)
- `monster_*.rs` - Monster group theory applications
- `lattice_*.rs` - Lattice structure analysis
- `eigenmatrix_*.rs` - Mathematical signature computation

### Integration Tools (40+)
- `rustc_*.rs` - Rust compiler integration
- `cargo_*.rs` - Cargo build system hooks
- `p2p_*.rs` - Peer-to-peer compilation cluster

## Performance Notes

- **Memory**: Requires 4GB+ RAM for full analysis
- **Storage**: 2.8GB for rustc_driver.so + analysis results
- **Processing**: Can handle 954K files with parallel processing
- **Time**: Full analysis takes several hours on modern hardware

## Integration with Meta-Introspector

This directory provides the core analysis engine for the meta-introspector ecosystem:
- Processes all forked repositories for code analysis
- Generates training data for AI models
- Provides mathematical foundations for code understanding
- Exports results for universal data formats

The zombie_driver2 system is the analytical heart of the meta-introspector project, providing deep insights into Rust code structure and compilation processes.
