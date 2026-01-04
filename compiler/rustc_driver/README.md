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
  - **NEW**: Enhanced collection of structs, enums, and constants

### 2. Eigenmatrix Analyzer (`usage_eigenmatrix`)
- **Purpose**: Processes usage data to build eigenmatrices
- **Output**: Statistical analysis of usage patterns
- **Key Metrics**:
  - Usage frequency rankings (eigenvalues)
  - Matrix sparsity analysis
  - Core vs peripheral function identification

### 3. Enhanced Analysis Tools
- **Constant Usage Report**: Analyzes core constants, literals, statics, and enums
- **Literals Report**: Categorizes string, numeric, and boolean literals
- **Similarity Finder**: Discovers similar code patterns across programs
- **Usage Line Similarity**: Finds similar usage patterns

## Usage

### Collect Usage Data
```bash
cd compiler/rustc_driver
./build_with_collector.sh
```

### Analyze Eigenmatrix
```bash
cd compiler/usage_eigenmatrix
cargo run --bin usage_eigenmatrix
```

### Run Analysis Reports
```bash
# Comprehensive constant analysis
cargo run --bin constant_usage_report

# Detailed literals breakdown
cargo run --bin literals_report

# Find similar programs
cargo run --bin similarity_finder
```

## Latest Results (Updated Dataset)

The system now reveals comprehensive Rust ecosystem patterns:

### Core Eigenvalues (Top Usage Patterns)
- **Top eigenvalue**: `static META` (30,490 usages - 8.91%)
- **Matrix properties**: 99.99% sparse, 342,106 total relationships
- **Unique DefIds**: 100,186 across the codebase

### Key Insights
- **Core constants**: `false` (8,590), `Option::Some/None` (4,042 each)
- **Most used literals**: "message" (4,806), "0" (65,743 occurrences)
- **Pattern analysis**: Constructor calls (45,516), Iterator traits (14,279)
- **Crate distribution**: core (147,498), rustc_* (66,427), std (6,585)

### Applications
- **Dependency suggestion**: Based on usage similarity patterns
- **Code similarity detection**: Find programs with similar usage patterns
- **Optimization targets**: Identify most frequently used patterns
- **Ecosystem analysis**: Understand fundamental Rust usage patterns

## Files Generated
- `usage_data/*.json` - Raw usage data per module (10,000+ files)
- `usage_eigenmatrix.json` - Processed eigenmatrix analysis
- Enhanced analysis reports with detailed breakdowns

This enables data-driven understanding of Rust program structure and ecosystem dependencies at unprecedented scale and detail.
