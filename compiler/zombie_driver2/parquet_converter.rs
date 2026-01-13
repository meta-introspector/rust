use arrow::array::{Array, Int64Array, StringArray, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PARQUET CONVERTER - Converting Lattice Data to HuggingFace Standard");
    println!("======================================================================");

    // Load CSV data
    let csv_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/sqrt_n_sample.csv";
    let csv_content = fs::read_to_string(csv_path)?;

    let mut samples = Vec::new();
    for (i, line) in csv_content.lines().enumerate() {
        if i == 0 {
            continue;
        } // Skip header

        // Parse CSV line properly
        let parts: Vec<&str> = line.splitn(2, ',').collect();
        if parts.len() >= 2 {
            let index = parts[0].parse::<i64>().unwrap_or(0);

            // Find coordinates in quotes
            if let Some(coords_start) = line.find('"') {
                if let Some(coords_end) = line.rfind('"') {
                    let coords_str = &line[coords_start + 1..coords_end];

                    // Parse remaining fields after coordinates
                    let after_coords = &line[coords_end + 1..];
                    let remaining: Vec<&str> =
                        after_coords.trim_start_matches(',').split(',').collect();

                    if remaining.len() >= 4 {
                        let count = remaining[0].parse::<i64>().unwrap_or(0);
                        let total_size = remaining[1].parse::<u64>().unwrap_or(0);
                        let addresses = remaining[2].trim_matches('"');
                        let coord_sum = remaining[3].parse::<u64>().unwrap_or(0);

                        samples.push(SampleData {
                            index,
                            coordinates: coords_str.to_string(),
                            count,
                            total_size,
                            addresses: addresses.to_string(),
                            coord_sum,
                        });
                    }
                }
            }
        }
    }

    println!("📊 Parsed {} samples from CSV", samples.len());

    // Convert to Arrow format
    let arrow_batch = convert_to_arrow(&samples)?;
    println!("✅ Converted to Arrow RecordBatch with {} rows", arrow_batch.num_rows());

    // Write to Parquet
    let parquet_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rustc_lattice_samples.parquet";
    write_parquet(&arrow_batch, parquet_path)?;
    println!("💾 Written to Parquet: {}", parquet_path);

    // Generate HuggingFace dataset info
    generate_hf_dataset_info(&samples, parquet_path)?;

    // Create sample analysis in Arrow format
    create_sample_analysis_parquet(&samples)?;

    Ok(())
}

fn convert_to_arrow(samples: &[SampleData]) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    // Define schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("index", DataType::Int64, false),
        Field::new("coordinates", DataType::Utf8, false),
        Field::new("count", DataType::Int64, false),
        Field::new("total_size", DataType::UInt64, false),
        Field::new("addresses", DataType::Utf8, false),
        Field::new("coord_sum", DataType::UInt64, false),
    ]));

    // Create arrays
    let index_array = Int64Array::from(samples.iter().map(|s| s.index).collect::<Vec<_>>());
    let coords_array =
        StringArray::from(samples.iter().map(|s| s.coordinates.as_str()).collect::<Vec<_>>());
    let count_array = Int64Array::from(samples.iter().map(|s| s.count).collect::<Vec<_>>());
    let size_array = UInt64Array::from(samples.iter().map(|s| s.total_size).collect::<Vec<_>>());
    let addr_array =
        StringArray::from(samples.iter().map(|s| s.addresses.as_str()).collect::<Vec<_>>());
    let sum_array = UInt64Array::from(samples.iter().map(|s| s.coord_sum).collect::<Vec<_>>());

    // Create record batch
    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(index_array),
            Arc::new(coords_array),
            Arc::new(count_array),
            Arc::new(size_array),
            Arc::new(addr_array),
            Arc::new(sum_array),
        ],
    )?;

    Ok(batch)
}

fn write_parquet(batch: &RecordBatch, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::create(path)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;

    writer.write(batch)?;
    writer.close()?;

    Ok(())
}

fn generate_hf_dataset_info(
    samples: &[SampleData],
    parquet_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let info_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/dataset_card.md";
    let mut content = String::new();

    content.push_str("---\n");
    content.push_str("license: mit\n");
    content.push_str("task_categories:\n");
    content.push_str("- other\n");
    content.push_str("language:\n");
    content.push_str("- en\n");
    content.push_str("tags:\n");
    content.push_str("- compiler-analysis\n");
    content.push_str("- rust\n");
    content.push_str("- mathematical-lattice\n");
    content.push_str("- binary-analysis\n");
    content.push_str("size_categories:\n");
    content.push_str("- n<1K\n");
    content.push_str("---\n\n");

    content.push_str("# Rust Compiler Lattice Analysis Dataset\n\n");
    content.push_str("## Dataset Description\n\n");
    content.push_str("This dataset contains mathematical lattice analysis of Rust compiler (rustc) binary functions.\n");
    content.push_str("Each function is represented as coordinates in a 12-dimensional modular prime lattice space.\n\n");

    content.push_str("## Dataset Structure\n\n");
    content.push_str("- **Format**: Parquet\n");
    content.push_str(&format!("- **Size**: {} samples\n", samples.len()));
    content.push_str("- **Columns**:\n");
    content.push_str("  - `index`: Function index in original dataset\n");
    content.push_str("  - `coordinates`: 12D lattice coordinates [mod 2, mod 3, ..., mod 37]\n");
    content.push_str("  - `count`: Number of duplicate functions with same coordinates\n");
    content.push_str("  - `total_size`: Function size in bytes\n");
    content.push_str("  - `addresses`: Memory addresses (hex)\n");
    content.push_str("  - `coord_sum`: Sum of all lattice coordinates\n\n");

    content.push_str("## Mathematical Framework\n\n");
    content.push_str(
        "Functions are mapped to 12-dimensional lattice using prime modular arithmetic:\n",
    );
    content.push_str("- **Prime basis**: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]\n");
    content
        .push_str("- **Coordinates**: Each dimension represents function properties mod prime\n");
    content.push_str("- **Energy**: Sum of coordinates represents mathematical 'energy'\n\n");

    content.push_str("## Sampling Methodology\n\n");
    content.push_str("Dataset created using sqrt(n)+1 sampling from full rustc binary analysis:\n");
    content.push_str("- **Original dataset**: 4,901 functions\n");
    content.push_str("- **Sample size**: 71 functions (sqrt(4901)+1)\n");
    content.push_str("- **Sampling rate**: 1.45%\n");
    content.push_str("- **Coverage**: Representative distribution across lattice space\n\n");

    content.push_str("## Usage\n\n");
    content.push_str("```python\n");
    content.push_str("import pandas as pd\n");
    content.push_str("import pyarrow.parquet as pq\n\n");
    content.push_str("# Load dataset\n");
    content.push_str("df = pd.read_parquet('rustc_lattice_samples.parquet')\n");
    content.push_str("print(df.head())\n");
    content.push_str("```\n\n");

    content.push_str("## Citation\n\n");
    content.push_str("If you use this dataset, please cite:\n");
    content.push_str("```\n");
    content.push_str("Rust Compiler Mathematical Lattice Analysis Dataset\n");
    content.push_str("12-Dimensional Modular Prime Function Fingerprinting\n");
    content.push_str("Generated: 2026-01-08\n");
    content.push_str("```\n");

    fs::write(info_path, content)?;
    println!("📄 HuggingFace dataset card written to: {}", info_path);

    Ok(())
}

fn create_sample_analysis_parquet(
    samples: &[SampleData],
) -> Result<(), Box<dyn std::error::Error>> {
    // Create analysis data
    let mut analysis_data = Vec::new();

    for (i, sample) in samples.iter().enumerate() {
        // Parse coordinates
        let coords_clean = sample.coordinates.trim_matches(['[', ']']);
        let coordinates: Vec<u64> =
            coords_clean.split(", ").filter_map(|s| s.parse().ok()).collect();

        if coordinates.len() == 12 {
            let max_coord = coordinates.iter().max().unwrap_or(&0);
            let min_coord = coordinates.iter().min().unwrap_or(&0);
            let avg_coord = coordinates.iter().sum::<u64>() as f64 / 12.0;

            // Classify function
            let function_type = if sample.count > 1 {
                "DUPLICATE"
            } else if sample.total_size < 50 {
                "SMALL"
            } else if sample.total_size > 150 {
                "LARGE"
            } else if sample.coord_sum > 100 {
                "HIGH_ENERGY"
            } else {
                "STANDARD"
            };

            analysis_data.push(AnalysisData {
                sample_id: i as i64,
                function_type: function_type.to_string(),
                max_coordinate: *max_coord,
                min_coordinate: *min_coord,
                avg_coordinate: avg_coord,
                energy_density: sample.coord_sum as f64 / sample.total_size as f64,
            });
        }
    }

    // Convert to Arrow
    let schema = Arc::new(Schema::new(vec![
        Field::new("sample_id", DataType::Int64, false),
        Field::new("function_type", DataType::Utf8, false),
        Field::new("max_coordinate", DataType::UInt64, false),
        Field::new("min_coordinate", DataType::UInt64, false),
        Field::new("avg_coordinate", DataType::Float64, false),
        Field::new("energy_density", DataType::Float64, false),
    ]));

    let id_array = Int64Array::from(analysis_data.iter().map(|a| a.sample_id).collect::<Vec<_>>());
    let type_array = StringArray::from(
        analysis_data.iter().map(|a| a.function_type.as_str()).collect::<Vec<_>>(),
    );
    let max_array =
        UInt64Array::from(analysis_data.iter().map(|a| a.max_coordinate).collect::<Vec<_>>());
    let min_array =
        UInt64Array::from(analysis_data.iter().map(|a| a.min_coordinate).collect::<Vec<_>>());
    let avg_array = arrow::array::Float64Array::from(
        analysis_data.iter().map(|a| a.avg_coordinate).collect::<Vec<_>>(),
    );
    let density_array = arrow::array::Float64Array::from(
        analysis_data.iter().map(|a| a.energy_density).collect::<Vec<_>>(),
    );

    let analysis_batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(id_array),
            Arc::new(type_array),
            Arc::new(max_array),
            Arc::new(min_array),
            Arc::new(avg_array),
            Arc::new(density_array),
        ],
    )?;

    let analysis_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rustc_lattice_analysis.parquet";
    write_parquet(&analysis_batch, analysis_path)?;
    println!("📊 Analysis parquet written to: {}", analysis_path);

    Ok(())
}

#[derive(Debug)]
struct SampleData {
    index: i64,
    coordinates: String,
    count: i64,
    total_size: u64,
    addresses: String,
    coord_sum: u64,
}

#[derive(Debug)]
struct AnalysisData {
    sample_id: i64,
    function_type: String,
    max_coordinate: u64,
    min_coordinate: u64,
    avg_coordinate: f64,
    energy_density: f64,
}
