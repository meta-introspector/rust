use arrow::array::{Float64Array, Int64Array, StringArray, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use goblin::elf::Elf;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 SO->PARQUET CONVERTER - Direct Binary to HuggingFace Format");
    println!("==============================================================");

    // Load rustc binary
    let rustc_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let buffer = std::fs::read(rustc_path)?;
    let elf = Elf::parse(&buffer)?;

    println!("📊 Analyzing rustc binary: {} bytes", buffer.len());
    println!("   Sections: {}", elf.section_headers.len());
    println!("   Symbols: {}", elf.syms.len());

    // Extract function data
    let mut functions = Vec::new();

    for (i, sym) in elf.syms.iter().enumerate() {
        if sym.st_type() == 2 {
            // STT_FUNC
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if !name.is_empty() && sym.st_size > 0 {
                    let coordinates =
                        calculate_lattice_coordinates(sym.st_value, sym.st_size, name);
                    let coord_sum = coordinates.iter().sum::<u64>();

                    functions.push(FunctionData {
                        index: i as i64,
                        name: name.to_string(),
                        address: sym.st_value,
                        size: sym.st_size,
                        coordinates,
                        coord_sum,
                        section_index: sym.st_shndx as u64,
                    });
                }
            }
        }
    }

    println!("✅ Extracted {} functions from binary", functions.len());

    // Convert to Arrow RecordBatch
    let batch = convert_to_arrow_batch(&functions)?;
    println!(
        "🏹 Created Arrow batch with {} rows, {} columns",
        batch.num_rows(),
        batch.num_columns()
    );

    // Split main dataset into chunks under 10MB
    let chunk_size = 50000; // Functions per chunk
    let base_path =
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch";

    for (chunk_idx, chunk) in functions.chunks(chunk_size).enumerate() {
        let chunk_batch = convert_to_arrow_batch(chunk)?;
        let filename = format!("{}/rustc_functions_part_{:03}.parquet", base_path, chunk_idx + 1);
        write_parquet(&chunk_batch, &filename)?;

        let file_size = std::fs::metadata(&filename)?.len();
        println!(
            "💾 Part {}: {} functions ({:.2} MB)",
            chunk_idx + 1,
            chunk.len(),
            file_size as f64 / 1024.0 / 1024.0
        );
    }

    // Split analysis dataset
    let analysis_batch = create_analysis_batch(&functions)?;
    let analysis_filename = format!("{}/rustc_analysis.parquet", base_path);
    write_parquet(&analysis_batch, &analysis_filename)?;

    let analysis_size = std::fs::metadata(&analysis_filename)?.len();
    if analysis_size > 10 * 1024 * 1024 {
        // Split analysis if over 10MB
        std::fs::remove_file(&analysis_filename)?;
        for (chunk_idx, chunk) in functions.chunks(chunk_size).enumerate() {
            let chunk_analysis = create_analysis_batch(chunk)?;
            let filename =
                format!("{}/rustc_analysis_part_{:03}.parquet", base_path, chunk_idx + 1);
            write_parquet(&chunk_analysis, &filename)?;
            let file_size = std::fs::metadata(&filename)?.len();
            println!(
                "📊 Analysis Part {}: {} functions ({:.2} MB)",
                chunk_idx + 1,
                chunk.len(),
                file_size as f64 / 1024.0 / 1024.0
            );
        }
    } else {
        println!("📊 Analysis dataset: {:.2} MB", analysis_size as f64 / 1024.0 / 1024.0);
    }

    // Generate HuggingFace dataset card
    generate_dataset_card(&functions)?;

    // Create sample for detailed review
    create_sample_dataset(&functions)?;

    Ok(())
}

fn calculate_lattice_coordinates(address: u64, size: u64, name: &str) -> Vec<u64> {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let mut coordinates = Vec::new();

    // Combine address, size, and name hash for coordinate calculation
    let name_hash = name.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    let combined = address.wrapping_add(size).wrapping_add(name_hash);

    for &prime in &primes {
        coordinates.push(combined % prime as u64);
    }

    coordinates
}

fn convert_to_arrow_batch(
    functions: &[FunctionData],
) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("index", DataType::Int64, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("address", DataType::UInt64, false),
        Field::new("size", DataType::UInt64, false),
        Field::new("coordinates", DataType::Utf8, false), // JSON string of coordinates
        Field::new("coord_sum", DataType::UInt64, false),
        Field::new("section_index", DataType::UInt64, false),
    ]));

    let index_array = Int64Array::from(functions.iter().map(|f| f.index).collect::<Vec<_>>());
    let name_array =
        StringArray::from(functions.iter().map(|f| f.name.as_str()).collect::<Vec<_>>());
    let addr_array = UInt64Array::from(functions.iter().map(|f| f.address).collect::<Vec<_>>());
    let size_array = UInt64Array::from(functions.iter().map(|f| f.size).collect::<Vec<_>>());
    let coords_array = StringArray::from(
        functions.iter().map(|f| format!("{:?}", f.coordinates)).collect::<Vec<_>>(),
    );
    let sum_array = UInt64Array::from(functions.iter().map(|f| f.coord_sum).collect::<Vec<_>>());
    let section_array =
        UInt64Array::from(functions.iter().map(|f| f.section_index).collect::<Vec<_>>());

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(index_array),
            Arc::new(name_array),
            Arc::new(addr_array),
            Arc::new(size_array),
            Arc::new(coords_array),
            Arc::new(sum_array),
            Arc::new(section_array),
        ],
    )?;

    Ok(batch)
}

fn create_analysis_batch(
    functions: &[FunctionData],
) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("function_id", DataType::Int64, false),
        Field::new("function_type", DataType::Utf8, false),
        Field::new("max_coordinate", DataType::UInt64, false),
        Field::new("min_coordinate", DataType::UInt64, false),
        Field::new("avg_coordinate", DataType::Float64, false),
        Field::new("energy_density", DataType::Float64, false),
        Field::new("prime_dominance", DataType::UInt64, false), // Which prime index dominates
        Field::new("name_length", DataType::UInt64, false),
    ]));

    let mut analysis_data = Vec::new();

    for func in functions {
        let max_coord = *func.coordinates.iter().max().unwrap_or(&0);
        let min_coord = *func.coordinates.iter().min().unwrap_or(&0);
        let avg_coord = func.coord_sum as f64 / 12.0;
        let energy_density =
            if func.size > 0 { func.coord_sum as f64 / func.size as f64 } else { 0.0 };
        let prime_dominance =
            func.coordinates.iter().position(|&x| x == max_coord).unwrap_or(0) as u64;

        let function_type = classify_function(&func.name, func.size, func.coord_sum);

        analysis_data.push(AnalysisData {
            function_id: func.index,
            function_type,
            max_coordinate: max_coord,
            min_coordinate: min_coord,
            avg_coordinate: avg_coord,
            energy_density,
            prime_dominance,
            name_length: func.name.len() as u64,
        });
    }

    let id_array =
        Int64Array::from(analysis_data.iter().map(|a| a.function_id).collect::<Vec<_>>());
    let type_array = StringArray::from(
        analysis_data.iter().map(|a| a.function_type.as_str()).collect::<Vec<_>>(),
    );
    let max_array =
        UInt64Array::from(analysis_data.iter().map(|a| a.max_coordinate).collect::<Vec<_>>());
    let min_array =
        UInt64Array::from(analysis_data.iter().map(|a| a.min_coordinate).collect::<Vec<_>>());
    let avg_array =
        Float64Array::from(analysis_data.iter().map(|a| a.avg_coordinate).collect::<Vec<_>>());
    let density_array =
        Float64Array::from(analysis_data.iter().map(|a| a.energy_density).collect::<Vec<_>>());
    let dominance_array =
        UInt64Array::from(analysis_data.iter().map(|a| a.prime_dominance).collect::<Vec<_>>());
    let length_array =
        UInt64Array::from(analysis_data.iter().map(|a| a.name_length).collect::<Vec<_>>());

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(id_array),
            Arc::new(type_array),
            Arc::new(max_array),
            Arc::new(min_array),
            Arc::new(avg_array),
            Arc::new(density_array),
            Arc::new(dominance_array),
            Arc::new(length_array),
        ],
    )?;

    Ok(batch)
}

fn classify_function(name: &str, size: u64, coord_sum: u64) -> String {
    if name.contains("_ZN") {
        "MANGLED_RUST"
    } else if name.contains("fmt") || name.contains("Debug") || name.contains("Display") {
        "FORMATTER"
    } else if size < 50 {
        "SMALL_UTILITY"
    } else if size > 500 {
        "LARGE_COMPLEX"
    } else if coord_sum > 200 {
        "HIGH_ENERGY"
    } else if name.len() > 50 {
        "LONG_NAME"
    } else {
        "STANDARD"
    }
    .to_string()
}

fn write_parquet(batch: &RecordBatch, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let props =
        WriterProperties::builder().set_compression(parquet::basic::Compression::SNAPPY).build();
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;

    writer.write(batch)?;
    writer.close()?;

    Ok(())
}

fn generate_dataset_card(functions: &[FunctionData]) -> Result<(), Box<dyn std::error::Error>> {
    let card_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/README.md";
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
    content.push_str("- binary-analysis\n");
    content.push_str("- mathematical-lattice\n");
    content.push_str("- elf-analysis\n");
    content.push_str("size_categories:\n");
    content.push_str(&format!(
        "- {}K<n<{}K\n",
        functions.len() / 1000,
        (functions.len() / 1000) + 1
    ));
    content.push_str("---\n\n");

    content.push_str("# Rust Compiler Binary Function Analysis Dataset\n\n");
    content.push_str("## Overview\n\n");
    content.push_str("Direct analysis of rustc binary functions using 12-dimensional modular prime lattice coordinates.\n");
    content
        .push_str("Each function extracted from ELF binary with mathematical fingerprinting.\n\n");

    content.push_str("## Dataset Files\n\n");
    content.push_str("- `rustc_functions.parquet`: Main function dataset\n");
    content.push_str("- `rustc_analysis.parquet`: Statistical analysis of functions\n\n");

    content.push_str("## Schema\n\n");
    content.push_str("### Main Dataset (`rustc_functions.parquet`)\n");
    content.push_str("- `index`: Function index\n");
    content.push_str("- `name`: Function symbol name\n");
    content.push_str("- `address`: Memory address (hex)\n");
    content.push_str("- `size`: Function size in bytes\n");
    content.push_str("- `coordinates`: 12D lattice coordinates [mod 2, mod 3, ..., mod 37]\n");
    content.push_str("- `coord_sum`: Sum of lattice coordinates (mathematical energy)\n");
    content.push_str("- `section_index`: ELF section index\n\n");

    content.push_str("### Analysis Dataset (`rustc_analysis.parquet`)\n");
    content.push_str("- `function_id`: Links to main dataset\n");
    content.push_str("- `function_type`: Classification (MANGLED_RUST, FORMATTER, etc.)\n");
    content.push_str("- `max_coordinate`: Maximum lattice coordinate\n");
    content.push_str("- `min_coordinate`: Minimum lattice coordinate\n");
    content.push_str("- `avg_coordinate`: Average coordinate value\n");
    content.push_str("- `energy_density`: Energy per byte ratio\n");
    content.push_str("- `prime_dominance`: Which prime dimension dominates (0-11)\n");
    content.push_str("- `name_length`: Symbol name length\n\n");

    content.push_str("## Statistics\n\n");
    content.push_str(&format!("- **Total Functions**: {}\n", functions.len()));

    let total_size: u64 = functions.iter().map(|f| f.size).sum();
    let total_energy: u64 = functions.iter().map(|f| f.coord_sum).sum();
    content.push_str(&format!(
        "- **Total Size**: {} bytes ({:.2} MB)\n",
        total_size,
        total_size as f64 / 1024.0 / 1024.0
    ));
    content.push_str(&format!("- **Total Energy**: {} units\n", total_energy));
    content.push_str(&format!(
        "- **Average Function Size**: {:.1} bytes\n",
        total_size as f64 / functions.len() as f64
    ));
    content.push_str(&format!(
        "- **Average Energy**: {:.1} units\n",
        total_energy as f64 / functions.len() as f64
    ));

    content.push_str("\n## Usage\n\n");
    content.push_str("```python\n");
    content.push_str("import pandas as pd\n");
    content.push_str("import pyarrow.parquet as pq\n\n");
    content.push_str("# Load main dataset\n");
    content.push_str("functions_df = pd.read_parquet('rustc_functions.parquet')\n");
    content.push_str("analysis_df = pd.read_parquet('rustc_analysis.parquet')\n\n");
    content.push_str("# Join datasets\n");
    content.push_str(
        "combined = functions_df.merge(analysis_df, left_on='index', right_on='function_id')\n",
    );
    content.push_str("print(combined.head())\n");
    content.push_str("```\n\n");

    content.push_str("## Mathematical Framework\n\n");
    content.push_str("Functions mapped to 12-dimensional lattice using modular arithmetic:\n");
    content.push_str("- **Prime Basis**: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]\n");
    content.push_str("- **Coordinate Calculation**: (address + size + name_hash) mod prime\n");
    content.push_str("- **Energy**: Sum of all coordinates\n");
    content.push_str("- **Density**: Energy per byte ratio\n\n");

    content.push_str("Generated: 2026-01-08 via direct ELF binary analysis\n");

    std::fs::write(card_path, content)?;
    println!("📄 Dataset card: {}", card_path);

    Ok(())
}

fn create_sample_dataset(functions: &[FunctionData]) -> Result<(), Box<dyn std::error::Error>> {
    // Take sqrt(n)+1 sample
    let n = functions.len();
    let sample_size = ((n as f64).sqrt() + 1.0) as usize;
    let step = n / sample_size;

    let mut sample_functions = Vec::new();
    for i in (0..n).step_by(step).take(sample_size) {
        sample_functions.push(&functions[i]);
    }

    println!("📊 Creating sample dataset: {} functions", sample_functions.len());

    // Convert sample to Arrow
    let sample_batch = convert_sample_to_arrow(&sample_functions)?;
    let sample_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rustc_sample.parquet";
    write_parquet(&sample_batch, sample_path)?;
    println!("🎯 Sample dataset: {}", sample_path);

    Ok(())
}

fn convert_sample_to_arrow(
    functions: &[&FunctionData],
) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("index", DataType::Int64, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("address", DataType::Utf8, false), // Hex string
        Field::new("size", DataType::UInt64, false),
        Field::new("coordinates", DataType::Utf8, false),
        Field::new("coord_sum", DataType::UInt64, false),
    ]));

    let index_array = Int64Array::from(functions.iter().map(|f| f.index).collect::<Vec<_>>());
    let name_array =
        StringArray::from(functions.iter().map(|f| f.name.as_str()).collect::<Vec<_>>());
    let addr_array = StringArray::from(
        functions.iter().map(|f| format!("0x{:x}", f.address)).collect::<Vec<_>>(),
    );
    let size_array = UInt64Array::from(functions.iter().map(|f| f.size).collect::<Vec<_>>());
    let coords_array = StringArray::from(
        functions.iter().map(|f| format!("{:?}", f.coordinates)).collect::<Vec<_>>(),
    );
    let sum_array = UInt64Array::from(functions.iter().map(|f| f.coord_sum).collect::<Vec<_>>());

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(index_array),
            Arc::new(name_array),
            Arc::new(addr_array),
            Arc::new(size_array),
            Arc::new(coords_array),
            Arc::new(sum_array),
        ],
    )?;

    Ok(batch)
}

#[derive(Debug)]
struct FunctionData {
    index: i64,
    name: String,
    address: u64,
    size: u64,
    coordinates: Vec<u64>,
    coord_sum: u64,
    section_index: u64,
}

#[derive(Debug)]
struct AnalysisData {
    function_id: i64,
    function_type: String,
    max_coordinate: u64,
    min_coordinate: u64,
    avg_coordinate: f64,
    energy_density: f64,
    prime_dominance: u64,
    name_length: u64,
}
