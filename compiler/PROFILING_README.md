# Rust Compiler Profiling and Usage Analysis

This directory contains enhanced build scripts that combine usage data collection with comprehensive profiling of the Rust compiler build process.

## Scripts Overview

### Enhanced Test Environment Scripts

- **`introspector-collector/test_env_debug_with_profiling.sh`** - Multi-phase build with both usage collection and profiling
- **`rustc_driver/build_with_comprehensive_profiling.sh`** - Comprehensive rustc_driver build with detailed profiling

### Original Scripts

- **`introspector-collector/test_env_debug.sh`** - Original mycelial network build script
- **`rustc_driver/build_with_profiling.sh`** - Original rustc_driver profiling script

## Features

### Usage Data Collection
- Collects detailed usage statistics during compilation
- Generates JSON files with function call patterns
- Tracks compiler component interactions

### Self-Profiling
- Uses Rust's `-Z self-profile` flag for detailed timing data
- Generates multiple profile datasets for different build phases
- Creates Chrome DevTools compatible profiler data

### Multi-Phase Analysis
1. **Self-Analysis**: Collector compiling itself
2. **Dependency Building**: Core compiler components
3. **Tool Building**: Analysis and diagnostic tools

## Prerequisites

Install measureme tools for Chrome profiler data generation:
```bash
cargo install measureme
```

## Usage

### Run Enhanced Test Environment
```bash
cd introspector-collector
./test_env_debug_with_profiling.sh
```

### Run Comprehensive rustc_driver Build
```bash
cd rustc_driver  
./build_with_comprehensive_profiling.sh
```

## Output Structure

```
profiling_data/          # Test environment profiling data
├── *_profile/          # Raw self-profile data directories
└── *.json             # Chrome DevTools format

profiling_output/        # rustc_driver profiling data
├── *_profile/          # Raw self-profile data directories  
└── *.json             # Chrome DevTools format

test_usage_data/         # Usage statistics (test environment)
└── *.json             # Function usage patterns

usage_data/              # Usage statistics (rustc_driver)
└── *.json             # Function usage patterns
```

## Analysis

### Chrome DevTools
1. Open Chrome DevTools (F12)
2. Go to Performance tab
3. Click "Load profile" 
4. Select generated `.json` files

### Usage Data
- JSON files contain detailed function call statistics
- Can be analyzed with custom tools or scripts
- Provides insights into compiler component usage patterns

## Build Logs

All scripts generate comprehensive build logs:
- `build_with_profiling.log` (test environment)
- `build_profiling.log` (rustc_driver)

These logs contain timing information and build details for analysis.
