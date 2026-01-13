use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏔️ LATTICE POLE CALCULATOR - Count per Entry & Sum at Pole");
    println!("==========================================================");

    // Load lattice coordinates
    let csv_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/lattice_coordinates.csv";
    let csv_content = fs::read_to_string(csv_path)?;

    let mut lattice_counts = HashMap::new();
    let mut coordinate_sums = vec![0u64; 12]; // Sum for each prime dimension
    let mut total_functions = 0;
    let mut pole_analysis = PoleAnalysis::new();

    println!("🔍 CALCULATING LATTICE ENTRY COUNTS...");

    for (i, line) in csv_content.lines().enumerate() {
        if i == 0 {
            continue;
        } // Skip header

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 15 {
            let address = parts[0];
            let enum_type = parts[1];
            let size = parts[2].parse::<u32>().unwrap_or(0);

            // Parse coordinates
            let mut coordinates = Vec::new();
            for j in 3..15 {
                if let Ok(coord) = parts[j].parse::<u64>() {
                    coordinates.push(coord);
                    coordinate_sums[j - 3] += coord;
                }
            }

            // Count occurrences of each coordinate vector
            let coord_key = coordinates.clone();
            let entry = lattice_counts.entry(coord_key).or_insert_with(|| LatticeEntry {
                coordinates: coordinates.clone(),
                count: 0,
                functions: Vec::new(),
                total_size: 0,
                enum_types: HashMap::new(),
            });

            entry.count += 1;
            entry.functions.push(address.to_string());
            entry.total_size += size;
            *entry.enum_types.entry(enum_type.to_string()).or_insert(0) += 1;

            total_functions += 1;

            // Update pole analysis
            pole_analysis.add_point(&coordinates, size, enum_type);
        }
    }

    println!(
        "✅ Analysis complete: {} unique lattice positions, {} total functions",
        lattice_counts.len(),
        total_functions
    );

    // Calculate pole (highest concentration point)
    println!("\n🏔️ CALCULATING LATTICE POLE...");
    let pole_position = calculate_pole(&lattice_counts, &coordinate_sums);

    // Generate pole analysis report
    println!("\n📊 GENERATING POLE ANALYSIS REPORT...");
    generate_pole_report(&lattice_counts, &pole_position, &coordinate_sums, total_functions)?;

    // Export enhanced lattice data with counts
    println!("\n📄 EXPORTING ENHANCED LATTICE DATA...");
    export_enhanced_lattice(&lattice_counts, &pole_position)?;

    Ok(())
}

fn calculate_pole(
    lattice_counts: &HashMap<Vec<u64>, LatticeEntry>,
    coordinate_sums: &[u64],
) -> PolePosition {
    // Find the lattice point with highest count (most functions)
    let mut max_count = 0;
    let mut pole_coords = Vec::new();
    let mut pole_functions = Vec::new();

    for (coords, entry) in lattice_counts {
        if entry.count > max_count {
            max_count = entry.count;
            pole_coords = coords.clone();
            pole_functions = entry.functions.clone();
        }
    }

    // Calculate total sum at pole (sum of all coordinate sums)
    let total_sum: u64 = coordinate_sums.iter().sum();

    // Calculate weighted pole (considering function sizes)
    let mut max_weighted_score = 0u64;
    let mut weighted_pole_coords = Vec::new();

    for (coords, entry) in lattice_counts {
        let weighted_score = entry.count as u64 * entry.total_size as u64;
        if weighted_score > max_weighted_score {
            max_weighted_score = weighted_score;
            weighted_pole_coords = coords.clone();
        }
    }

    PolePosition {
        count_pole: pole_coords,
        count_pole_value: max_count,
        count_pole_functions: pole_functions,
        weighted_pole: weighted_pole_coords,
        weighted_pole_value: max_weighted_score,
        total_coordinate_sum: total_sum,
        prime_dimension_sums: coordinate_sums.to_vec(),
    }
}

fn generate_pole_report(
    lattice_counts: &HashMap<Vec<u64>, LatticeEntry>,
    pole: &PolePosition,
    coordinate_sums: &[u64],
    total_functions: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let report_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/lattice_pole_analysis.txt";
    let mut content = String::new();

    content.push_str("LATTICE POLE ANALYSIS - Count per Entry & Sum at Pole\n");
    content.push_str("=====================================================\n\n");

    content.push_str("POLE ANALYSIS SUMMARY:\n");
    content.push_str("=====================\n");
    content.push_str(&format!("Total functions in lattice: {}\n", total_functions));
    content.push_str(&format!("Unique lattice positions: {}\n", lattice_counts.len()));
    content.push_str(&format!("Total coordinate sum at pole: {}\n", pole.total_coordinate_sum));
    content.push_str("\n");

    content.push_str("COUNT POLE (Highest Function Density):\n");
    content.push_str("======================================\n");
    content.push_str(&format!("Coordinates: {:?}\n", pole.count_pole));
    content.push_str(&format!("Function count: {}\n", pole.count_pole_value));
    content.push_str("Functions at pole:\n");
    for func in &pole.count_pole_functions {
        content.push_str(&format!("  - {}\n", func));
    }
    content.push_str("\n");

    content.push_str("WEIGHTED POLE (Highest Size*Count Score):\n");
    content.push_str("=========================================\n");
    content.push_str(&format!("Coordinates: {:?}\n", pole.weighted_pole));
    content.push_str(&format!("Weighted score: {}\n", pole.weighted_pole_value));
    content.push_str("\n");

    content.push_str("PRIME DIMENSION SUMS:\n");
    content.push_str("====================\n");
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for (i, (&prime, &sum)) in primes.iter().zip(coordinate_sums.iter()).enumerate() {
        content.push_str(&format!(
            "Prime {}: Sum = {} (avg = {:.2})\n",
            prime,
            sum,
            sum as f64 / total_functions as f64
        ));
    }
    content.push_str("\n");

    content.push_str("TOP LATTICE POSITIONS BY COUNT:\n");
    content.push_str("===============================\n");
    let mut sorted_entries: Vec<_> = lattice_counts.iter().collect();
    sorted_entries.sort_by_key(|(_, entry)| std::cmp::Reverse(entry.count));

    for (i, (coords, entry)) in sorted_entries.iter().take(10).enumerate() {
        content.push_str(&format!("{}. Count: {} | Coords: {:?}\n", i + 1, entry.count, coords));
        content.push_str(&format!(
            "   Total size: {} bytes | Enum types: {:?}\n",
            entry.total_size, entry.enum_types
        ));
        content.push_str("\n");
    }

    content.push_str("MATHEMATICAL SIGNIFICANCE:\n");
    content.push_str("=========================\n");
    content.push_str("• Pole represents highest concentration of rustc functionality\n");
    content.push_str("• Coordinate sums reveal total 'energy' in each prime dimension\n");
    content.push_str("• High pole values indicate core compiler operations\n");
    content.push_str("• Prime dimension distribution shows mathematical structure\n");
    content.push_str("• Weighted pole considers both frequency and complexity\n");

    fs::write(report_path, content)?;
    println!("📊 Pole analysis report written to: {}", report_path);

    Ok(())
}

fn export_enhanced_lattice(
    lattice_counts: &HashMap<Vec<u64>, LatticeEntry>,
    pole: &PolePosition,
) -> Result<(), Box<dyn std::error::Error>> {
    let export_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/lattice_with_counts.csv";
    let mut file = fs::File::create(&export_path)?;

    // CSV header
    writeln!(
        file,
        "coordinates,count,total_size,enum_types,functions,mod2,mod3,mod5,mod7,mod11,mod13,mod17,mod19,mod23,mod29,mod31,mod37,is_pole"
    )?;

    // Sort by count (descending)
    let mut sorted_entries: Vec<_> = lattice_counts.iter().collect();
    sorted_entries.sort_by_key(|(_, entry)| std::cmp::Reverse(entry.count));

    for (coords, entry) in sorted_entries {
        let is_pole = coords == &pole.count_pole;
        let enum_types_str = entry
            .enum_types
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join(";");
        let functions_str = entry.functions.join(";");

        write!(
            file,
            "\"{:?}\",{},{},\"{}\",\"{}\"",
            coords, entry.count, entry.total_size, enum_types_str, functions_str
        )?;

        for &coord in coords {
            write!(file, ",{}", coord)?;
        }

        writeln!(file, ",{}", is_pole)?;
    }

    println!("📄 Enhanced lattice data exported to: {}", export_path);

    Ok(())
}

struct LatticeEntry {
    coordinates: Vec<u64>,
    count: usize,
    functions: Vec<String>,
    total_size: u32,
    enum_types: HashMap<String, usize>,
}

struct PolePosition {
    count_pole: Vec<u64>,
    count_pole_value: usize,
    count_pole_functions: Vec<String>,
    weighted_pole: Vec<u64>,
    weighted_pole_value: u64,
    total_coordinate_sum: u64,
    prime_dimension_sums: Vec<u64>,
}

struct PoleAnalysis {
    total_points: usize,
    max_coordinate: Vec<u64>,
    min_coordinate: Vec<u64>,
}

impl PoleAnalysis {
    fn new() -> Self {
        PoleAnalysis {
            total_points: 0,
            max_coordinate: vec![0; 12],
            min_coordinate: vec![u64::MAX; 12],
        }
    }

    fn add_point(&mut self, coordinates: &[u64], _size: u32, _enum_type: &str) {
        self.total_points += 1;

        for (i, &coord) in coordinates.iter().enumerate() {
            if i < 12 {
                self.max_coordinate[i] = self.max_coordinate[i].max(coord);
                self.min_coordinate[i] = self.min_coordinate[i].min(coord);
            }
        }
    }
}
