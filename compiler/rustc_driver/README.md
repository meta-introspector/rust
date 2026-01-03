# Rust Usage Eigenmatrix System

A comprehensive system for analyzing Rust code usage patterns and building eigenmatrices to understand program structure and dependencies.

## Components

### 1. Usage Collector (`introspector-collector`)
- **Purpose**: Intercepts Rust compilation to collect function/method usage data
- **Output**: JSON files with structured usage information
- **Key Features**:
  - Replaces `rustc` during compilation
  - Tracks DefId relationships (who uses what)
  - Generates clean JSON with proper serialization

### 2. Eigenmatrix Analyzer (`usage_eigenmatrix`)
- **Purpose**: Processes usage data to build eigenmatrices
- **Output**: Statistical analysis of usage patterns
- **Key Metrics**:
  - Usage frequency rankings (eigenvalues)
  - Matrix sparsity analysis
  - Core vs peripheral function identification

## Usage

### Collect Usage Data
```bash
cd compiler/rustc_driver
./build_with_collector.sh
```

### Analyze Eigenmatrix
```bash
cd compiler/usage_eigenmatrix
cargo run
```

## Results

The system reveals fundamental Rust ecosystem patterns:
- **Top eigenvalue**: `rustc_proc_macro::quote::quote` (36% of all usages)
- **Matrix properties**: 99.1% sparse, 644 unique DefIds
- **Applications**: Dependency suggestion, similarity detection, optimization

## Files Generated
- `usage_data/*.json` - Raw usage data per module
- `usage_eigenmatrix.json` - Processed eigenmatrix analysis

This enables data-driven understanding of Rust program structure and ecosystem dependencies.
