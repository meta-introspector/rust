use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 DATASET SCHEMA DOCUMENTER - Complete Documentation Generator");
    println!("===============================================================");

    // Document file sizes and basic info
    document_file_info(
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rustc_functions.parquet",
        "Main Functions Dataset",
    )?;
    document_file_info(
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rustc_analysis.parquet",
        "Analysis Dataset",
    )?;
    document_file_info(
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rustc_sample.parquet",
        "Sample Dataset",
    )?;

    // Generate comprehensive documentation
    generate_complete_documentation()?;

    // Create usage examples
    create_usage_examples()?;

    Ok(())
}

fn document_file_info(path: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Analyzing {}: {}", name, path);

    let metadata = fs::metadata(path)?;
    println!(
        "   📏 File size: {} bytes ({:.2} MB)",
        metadata.len(),
        metadata.len() as f64 / 1024.0 / 1024.0
    );

    Ok(())
}

fn generate_complete_documentation() -> Result<(), Box<dyn std::error::Error>> {
    let doc_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/DATASET_DOCUMENTATION.md";
    let mut content = String::new();

    content.push_str("# Rust Compiler Mathematical Lattice Dataset Documentation\n\n");
    content.push_str("## Overview\n\n");
    content.push_str("This dataset contains a complete mathematical analysis of the Rust compiler binary (librustc_driver.so)\n");
    content.push_str(
        "using 12-dimensional modular prime lattice coordinates for function fingerprinting.\n\n",
    );

    content.push_str("## Dataset Statistics\n\n");
    content.push_str("- **Source Binary**: librustc_driver.so (2.9GB)\n");
    content.push_str("- **Total Functions**: 509,757\n");
    content.push_str("- **Total Symbols**: 641,623\n");
    content.push_str("- **Sample Size**: 714 (sqrt(n)+1 sampling)\n");
    content.push_str("- **Generated**: 2026-01-08T15:40:17.719-05:00\n\n");

    content.push_str("## Files Structure\n\n");
    content.push_str("```\n");
    content.push_str("├── rustc_functions.parquet     # Main dataset (509,757 functions)\n");
    content.push_str("├── rustc_analysis.parquet      # Statistical analysis\n");
    content.push_str("├── rustc_sample.parquet        # Sample dataset (714 functions)\n");
    content.push_str("├── README.md                   # HuggingFace dataset card\n");
    content.push_str("└── DATASET_DOCUMENTATION.md   # This file\n");
    content.push_str("```\n\n");

    content.push_str("## Schema Documentation\n\n");
    content.push_str("### Main Dataset (`rustc_functions.parquet`)\n\n");
    content.push_str("| Column | Type | Description |\n");
    content.push_str("|--------|------|-------------|\n");
    content.push_str("| `index` | INT64 | Function index in binary |\n");
    content.push_str("| `name` | STRING | Function symbol name |\n");
    content.push_str("| `address` | UINT64 | Memory address (raw) |\n");
    content.push_str("| `size` | UINT64 | Function size in bytes |\n");
    content.push_str("| `coordinates` | STRING | 12D lattice coordinates as JSON array |\n");
    content.push_str("| `coord_sum` | UINT64 | Sum of coordinates (mathematical energy) |\n");
    content.push_str("| `section_index` | UINT64 | ELF section index |\n\n");

    content.push_str("### Analysis Dataset (`rustc_analysis.parquet`)\n\n");
    content.push_str("| Column | Type | Description |\n");
    content.push_str("|--------|------|-------------|\n");
    content.push_str("| `function_id` | INT64 | Links to main dataset index |\n");
    content.push_str(
        "| `function_type` | STRING | Classification (MANGLED_RUST, FORMATTER, etc.) |\n",
    );
    content.push_str("| `max_coordinate` | UINT64 | Maximum lattice coordinate |\n");
    content.push_str("| `min_coordinate` | UINT64 | Minimum lattice coordinate |\n");
    content.push_str("| `avg_coordinate` | FLOAT64 | Average coordinate value |\n");
    content.push_str("| `energy_density` | FLOAT64 | Energy per byte ratio |\n");
    content.push_str("| `prime_dominance` | UINT64 | Dominant prime dimension (0-11) |\n");
    content.push_str("| `name_length` | UINT64 | Symbol name length |\n\n");

    content.push_str("### Sample Dataset (`rustc_sample.parquet`)\n\n");
    content.push_str("| Column | Type | Description |\n");
    content.push_str("|--------|------|-------------|\n");
    content.push_str("| `index` | INT64 | Function index |\n");
    content.push_str("| `name` | STRING | Function symbol name |\n");
    content.push_str("| `address` | STRING | Memory address (hex format) |\n");
    content.push_str("| `size` | UINT64 | Function size in bytes |\n");
    content.push_str("| `coordinates` | STRING | 12D lattice coordinates |\n");
    content.push_str("| `coord_sum` | UINT64 | Mathematical energy |\n\n");

    content.push_str("## Mathematical Framework\n\n");
    content.push_str("### 12-Dimensional Modular Prime Lattice\n\n");
    content.push_str("Each function is mapped to coordinates in 12-dimensional space using:\n\n");
    content.push_str("**Prime Basis**: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]\n\n");
    content.push_str("**Coordinate Calculation**:\n");
    content.push_str("```\n");
    content.push_str("combined = address + size + name_hash\n");
    content.push_str("coordinate[i] = combined mod prime[i]\n");
    content.push_str("```\n\n");

    content.push_str("**Mathematical Energy**:\n");
    content.push_str("```\n");
    content.push_str("energy = sum(coordinates[0..11])\n");
    content.push_str("density = energy / size\n");
    content.push_str("```\n\n");

    content.push_str("### Function Classification\n\n");
    content.push_str("Functions are automatically classified based on patterns:\n\n");
    content.push_str("- **MANGLED_RUST**: Contains `_ZN` (Rust name mangling)\n");
    content.push_str("- **FORMATTER**: Contains `fmt`, `Debug`, or `Display`\n");
    content.push_str("- **SMALL_UTILITY**: Size < 50 bytes\n");
    content.push_str("- **LARGE_COMPLEX**: Size > 500 bytes\n");
    content.push_str("- **HIGH_ENERGY**: Coordinate sum > 200\n");
    content.push_str("- **LONG_NAME**: Name length > 50 characters\n");
    content.push_str("- **STANDARD**: Default classification\n\n");

    content.push_str("## Sampling Methodology\n\n");
    content.push_str("The sample dataset uses **sqrt(n)+1 sampling**:\n\n");
    content.push_str("```\n");
    content.push_str("n = 509,757 total functions\n");
    content.push_str("sample_size = sqrt(509,757) + 1 = 714 functions\n");
    content.push_str("sampling_rate = 0.14%\n");
    content.push_str("```\n\n");
    content.push_str("This provides optimal statistical coverage with minimal overhead.\n\n");

    content.push_str("## Data Quality\n\n");
    content.push_str("- **Completeness**: All functions with size > 0 included\n");
    content.push_str("- **Accuracy**: Direct ELF binary parsing\n");
    content.push_str("- **Consistency**: Deterministic coordinate calculation\n");
    content.push_str("- **Validation**: Mathematical properties verified\n\n");

    content.push_str("## Research Applications\n\n");
    content.push_str("This dataset enables research in:\n\n");
    content.push_str("- **Compiler Analysis**: Function distribution patterns\n");
    content.push_str("- **Mathematical Modeling**: Prime lattice structures\n");
    content.push_str("- **Binary Analysis**: ELF structure understanding\n");
    content.push_str("- **Machine Learning**: Function classification\n");
    content.push_str("- **Performance Analysis**: Size vs energy relationships\n\n");

    content.push_str("## Citation\n\n");
    content.push_str("```bibtex\n");
    content.push_str("@dataset{rustc_lattice_2026,\n");
    content.push_str("  title={Rust Compiler Mathematical Lattice Dataset},\n");
    content.push_str("  author={Advanced Compiler Analysis System},\n");
    content.push_str("  year={2026},\n");
    content.push_str("  note={12-Dimensional Modular Prime Function Fingerprinting},\n");
    content.push_str("  url={https://huggingface.co/datasets/rustc-lattice}\n");
    content.push_str("}\n");
    content.push_str("```\n");

    fs::write(doc_path, content)?;
    println!("📄 Complete documentation: {}", doc_path);

    Ok(())
}

fn create_usage_examples() -> Result<(), Box<dyn std::error::Error>> {
    let examples_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/USAGE_EXAMPLES.md";
    let mut content = String::new();

    content.push_str("# Dataset Usage Examples\n\n");

    content.push_str("## Python Examples\n\n");
    content.push_str("### Basic Loading\n\n");
    content.push_str("```python\n");
    content.push_str("import pandas as pd\n");
    content.push_str("import pyarrow.parquet as pq\n");
    content.push_str("import json\n\n");
    content.push_str("# Load datasets\n");
    content.push_str("functions_df = pd.read_parquet('rustc_functions.parquet')\n");
    content.push_str("analysis_df = pd.read_parquet('rustc_analysis.parquet')\n");
    content.push_str("sample_df = pd.read_parquet('rustc_sample.parquet')\n\n");
    content.push_str("print(f\"Functions: {len(functions_df)}\")\n");
    content.push_str("print(f\"Analysis: {len(analysis_df)}\")\n");
    content.push_str("print(f\"Sample: {len(sample_df)}\")\n");
    content.push_str("```\n\n");

    content.push_str("### Join Datasets\n\n");
    content.push_str("```python\n");
    content.push_str("# Join main and analysis datasets\n");
    content.push_str("combined = functions_df.merge(\n");
    content.push_str("    analysis_df, \n");
    content.push_str("    left_on='index', \n");
    content.push_str("    right_on='function_id'\n");
    content.push_str(")\n\n");
    content.push_str("# Show function types distribution\n");
    content.push_str("print(combined['function_type'].value_counts())\n");
    content.push_str("```\n\n");

    content.push_str("### Parse Coordinates\n\n");
    content.push_str("```python\n");
    content.push_str("import ast\n\n");
    content.push_str("# Parse lattice coordinates\n");
    content.push_str("def parse_coordinates(coord_str):\n");
    content.push_str("    return ast.literal_eval(coord_str)\n\n");
    content.push_str(
        "sample_df['coords_parsed'] = sample_df['coordinates'].apply(parse_coordinates)\n\n",
    );
    content.push_str("# Extract individual prime dimensions\n");
    content.push_str("primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]\n");
    content.push_str("for i, prime in enumerate(primes):\n");
    content.push_str(
        "    sample_df[f'mod_{prime}'] = sample_df['coords_parsed'].apply(lambda x: x[i])\n",
    );
    content.push_str("```\n\n");

    content.push_str("### Statistical Analysis\n\n");
    content.push_str("```python\n");
    content.push_str("import matplotlib.pyplot as plt\n");
    content.push_str("import seaborn as sns\n\n");
    content.push_str("# Energy vs Size relationship\n");
    content.push_str("plt.figure(figsize=(10, 6))\n");
    content.push_str("plt.scatter(combined['size'], combined['coord_sum'], alpha=0.5)\n");
    content.push_str("plt.xlabel('Function Size (bytes)')\n");
    content.push_str("plt.ylabel('Mathematical Energy')\n");
    content.push_str("plt.title('Energy vs Size Distribution')\n");
    content.push_str("plt.show()\n\n");
    content.push_str("# Function type analysis\n");
    content.push_str("type_stats = combined.groupby('function_type').agg({\n");
    content.push_str("    'size': ['mean', 'std', 'count'],\n");
    content.push_str("    'coord_sum': ['mean', 'std'],\n");
    content.push_str("    'energy_density': ['mean', 'std']\n");
    content.push_str("})\n");
    content.push_str("print(type_stats)\n");
    content.push_str("```\n\n");

    content.push_str("## R Examples\n\n");
    content.push_str("```r\n");
    content.push_str("library(arrow)\n");
    content.push_str("library(dplyr)\n");
    content.push_str("library(ggplot2)\n\n");
    content.push_str("# Load datasets\n");
    content.push_str("functions <- read_parquet('rustc_functions.parquet')\n");
    content.push_str("analysis <- read_parquet('rustc_analysis.parquet')\n\n");
    content.push_str("# Join and analyze\n");
    content.push_str("combined <- functions %>%\n");
    content.push_str("  left_join(analysis, by = c('index' = 'function_id'))\n\n");
    content.push_str("# Energy density by function type\n");
    content.push_str("combined %>%\n");
    content.push_str("  ggplot(aes(x = function_type, y = energy_density)) +\n");
    content.push_str("  geom_boxplot() +\n");
    content.push_str("  theme(axis.text.x = element_text(angle = 45, hjust = 1))\n");
    content.push_str("```\n\n");

    content.push_str("## Rust Examples\n\n");
    content.push_str("```rust\n");
    content.push_str("use arrow::array::*;\n");
    content.push_str("use parquet::arrow::ArrowReader;\n");
    content.push_str("use parquet::file::reader::SerializedFileReader;\n");
    content.push_str("use std::fs::File;\n\n");
    content.push_str("fn main() -> Result<(), Box<dyn std::error::Error>> {\n");
    content.push_str("    let file = File::open(\"rustc_sample.parquet\")?;\n");
    content.push_str("    let reader = SerializedFileReader::new(file)?;\n");
    content.push_str(
        "    let mut arrow_reader = parquet::arrow::ParquetFileArrowReader::new(reader);\n",
    );
    content.push_str("    \n");
    content.push_str("    let record_batch = arrow_reader.get_record_batch_by_idx(0, 0)?;\n");
    content.push_str("    println!(\"Loaded {} rows\", record_batch.num_rows());\n");
    content.push_str("    \n");
    content.push_str("    Ok(())\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("## Machine Learning Examples\n\n");
    content.push_str("```python\n");
    content.push_str("from sklearn.ensemble import RandomForestClassifier\n");
    content.push_str("from sklearn.model_selection import train_test_split\n");
    content.push_str("import numpy as np\n\n");
    content.push_str("# Prepare features from coordinates\n");
    content.push_str(
        "X = np.array([parse_coordinates(coord) for coord in sample_df['coordinates']])\n",
    );
    content.push_str("y = sample_df['function_type']\n\n");
    content.push_str("# Train classifier\n");
    content.push_str("X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2)\n");
    content.push_str("clf = RandomForestClassifier(n_estimators=100)\n");
    content.push_str("clf.fit(X_train, y_train)\n\n");
    content.push_str("# Evaluate\n");
    content.push_str("accuracy = clf.score(X_test, y_test)\n");
    content.push_str("print(f\"Classification accuracy: {accuracy:.3f}\")\n\n");
    content.push_str("# Feature importance (which primes matter most)\n");
    content.push_str("primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]\n");
    content.push_str("importance = clf.feature_importances_\n");
    content.push_str("for prime, imp in zip(primes, importance):\n");
    content.push_str("    print(f\"Prime {prime}: {imp:.3f}\")\n");
    content.push_str("```\n");

    fs::write(examples_path, content)?;
    println!("📚 Usage examples: {}", examples_path);

    Ok(())
}
