# Rustc Dependency Scanner

Enhanced dependency scanner for analyzing the Rust compiler source code and building complete symbol dependency graphs.

## Scripts

### `scan_rustc_main.sh`
Runs the enhanced scanner on rustc compilation with parallel processing.

**Usage:**
```bash
./scan_rustc_main.sh
```

**Features:**
- Builds the scanner automatically
- Uses 20 parallel jobs for maximum CPU utilization
- Creates timestamped log files
- Generates scan_results/ directory with JSON output
- Provides monitoring commands

**Output:**
- `rustc_main_scan_YYYYMMDD_HHMMSS.log` - Complete build log
- `scan_results/*.json` - Individual crate symbol data
- `scan_results/*_summary.json` - Symbol type summaries

### `clean_scan.sh`
Cleans all scan data, logs, and build artifacts.

**Usage:**
```bash
./clean_scan.sh
```

**Cleans:**
- All `*.log` files
- `scan_results/` directory
- Cargo build artifacts

## Scanner Features

- **Post-resolution analysis** - Captures symbols after rustc has resolved all imports
- **Stops after analysis** - Skips codegen for 10x faster scanning
- **Parallel processing** - Utilizes all available CPU cores
- **Visibility detection** - Captures pub/private modifiers
- **Import/export mapping** - Tracks use statements and extern crates
- **Structured output** - JSON format with spans and item types

## Monitoring

**Watch progress:**
```bash
tail -f rustc_main_scan_*.log
watch 'find scan_results/ -name "*.json" | wc -l'
htop
```

**Check results:**
```bash
ls scan_results/ | wc -l  # Count captured crates
head scan_results/rustc_middle.json  # Sample data
```
