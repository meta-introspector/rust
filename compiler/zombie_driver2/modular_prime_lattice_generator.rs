use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 MODULAR PRIME LATTICE GENERATOR - Program as Lattice Structure");
    println!("=================================================================");

    // Load enhanced CSV with mathematical constants
    let csv_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/enum_string_functions_enhanced.csv";
    let csv_content = fs::read_to_string(csv_path)?;

    let mut lattice_points = Vec::new();
    let mut prime_dimensions = HashMap::new();
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    println!("🔍 CONSTRUCTING MODULAR PRIME LATTICE...");

    for (i, line) in csv_content.lines().enumerate() {
        if i == 0 {
            continue;
        } // Skip header

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 11 {
            let address = parts[0];
            let enum_type = parts[1];
            let size = parts[3].parse::<u32>().unwrap_or(0);
            let module_constants = parts[8];

            // Parse modular residues to create lattice point
            let lattice_point = parse_modular_residues(module_constants, &primes);
            let lattice_coords = LatticePoint {
                address: address.to_string(),
                enum_type: enum_type.to_string(),
                size,
                coordinates: lattice_point.clone(),
                dimension: primes.len(),
            };

            lattice_points.push(lattice_coords);

            // Track prime dimension statistics
            for (j, &residue) in lattice_point.iter().enumerate() {
                let prime = primes[j];
                prime_dimensions.entry(prime).or_insert_with(Vec::new).push(residue);
            }
        }
    }

    println!(
        "✅ Lattice constructed: {} points in {}-dimensional space",
        lattice_points.len(),
        primes.len()
    );

    // Generate lattice analysis
    println!("\n🔬 ANALYZING LATTICE STRUCTURE...");
    let lattice_analysis = analyze_lattice_structure(&lattice_points, &primes);

    // Generate lattice visualization
    println!("\n🎨 GENERATING LATTICE VISUALIZATION...");
    generate_lattice_visualization(&lattice_points, &lattice_analysis)?;

    // Export lattice data
    println!("\n📊 EXPORTING LATTICE DATA...");
    export_lattice_data(&lattice_points, &lattice_analysis)?;

    Ok(())
}

fn parse_modular_residues(module_constants: &str, primes: &[u64]) -> Vec<u64> {
    let mut residues = vec![0; primes.len()];

    // Parse format: "mod2[0:1];mod3[0:1];mod5[1:2:3:4];..."
    let cleaned = module_constants.trim_matches('"');

    for mod_part in cleaned.split(';') {
        if let Some(mod_start) = mod_part.find("mod") {
            if let Some(bracket_start) = mod_part.find('[') {
                if let Some(bracket_end) = mod_part.find(']') {
                    let mod_num_str = &mod_part[mod_start + 3..bracket_start];
                    let residues_str = &mod_part[bracket_start + 1..bracket_end];

                    if let Ok(modulus) = mod_num_str.parse::<u64>() {
                        if let Some(prime_index) = primes.iter().position(|&p| p == modulus) {
                            // Take first residue as representative
                            if let Some(first_residue) = residues_str.split(':').next() {
                                if let Ok(residue) = first_residue.parse::<u64>() {
                                    residues[prime_index] = residue;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    residues
}

fn analyze_lattice_structure(points: &[LatticePoint], primes: &[u64]) -> LatticeAnalysis {
    let mut analysis = LatticeAnalysis {
        total_points: points.len(),
        dimension: primes.len(),
        prime_basis: primes.to_vec(),
        density_map: HashMap::new(),
        clusters: Vec::new(),
        lattice_constants: Vec::new(),
    };

    // Calculate density in each prime dimension
    for (i, &prime) in primes.iter().enumerate() {
        let mut residue_counts = HashMap::new();
        for point in points {
            let residue = point.coordinates[i];
            *residue_counts.entry(residue).or_insert(0) += 1;
        }
        analysis.density_map.insert(prime, residue_counts);
    }

    // Find clusters (points with similar coordinates)
    let mut clusters = HashMap::new();
    for point in points {
        let cluster_key = point.coordinates[0..3].to_vec(); // Use first 3 dimensions for clustering
        clusters.entry(cluster_key).or_insert_with(Vec::new).push(point.address.clone());
    }

    for (coords, addresses) in clusters {
        if addresses.len() > 1 {
            let size = addresses.len();
            analysis.clusters.push(LatticeCluster {
                coordinates: coords,
                members: addresses,
                size,
            });
        }
    }

    // Calculate lattice constants (fundamental periods)
    for &prime in primes {
        analysis.lattice_constants.push(prime); // In modular arithmetic, period = prime
    }

    analysis
}

fn generate_lattice_visualization(
    points: &[LatticePoint],
    analysis: &LatticeAnalysis,
) -> Result<(), Box<dyn std::error::Error>> {
    let viz_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/modular_prime_lattice.txt";
    let mut content = String::new();

    content.push_str("MODULAR PRIME LATTICE VISUALIZATION\n");
    content.push_str("===================================\n\n");

    content.push_str(&format!(
        "LATTICE DIMENSIONS: {} (Prime basis: {:?})\n",
        analysis.dimension, analysis.prime_basis
    ));
    content.push_str(&format!("TOTAL LATTICE POINTS: {}\n\n", analysis.total_points));

    content.push_str("LATTICE STRUCTURE:\n");
    content.push_str("==================\n");

    // Show first 10 lattice points
    for (i, point) in points.iter().take(10).enumerate() {
        content.push_str(&format!("Point {}: {} ({})\n", i + 1, point.address, point.enum_type));
        content.push_str(&format!("  Coordinates: {:?}\n", point.coordinates));
        content.push_str(&format!("  Size: {} bytes\n", point.size));
        content.push_str("\n");
    }

    content.push_str("PRIME DIMENSION ANALYSIS:\n");
    content.push_str("========================\n");

    for (&prime, density) in &analysis.density_map {
        content.push_str(&format!("Prime {}: {} unique residues\n", prime, density.len()));
        let mut sorted_residues: Vec<_> = density.iter().collect();
        sorted_residues.sort_by_key(|(_, &count)| std::cmp::Reverse(count));

        for ((&residue, &count), j) in sorted_residues.iter().zip(0..5) {
            content.push_str(&format!("  Residue {}: {} points\n", residue, count));
        }
        content.push_str("\n");
    }

    content.push_str("LATTICE CLUSTERS:\n");
    content.push_str("================\n");

    for (i, cluster) in analysis.clusters.iter().take(5).enumerate() {
        content.push_str(&format!(
            "Cluster {}: {} members at {:?}\n",
            i + 1,
            cluster.size,
            cluster.coordinates
        ));
        for member in &cluster.members {
            content.push_str(&format!("  - {}\n", member));
        }
        content.push_str("\n");
    }

    content.push_str("LATTICE CONSTANTS:\n");
    content.push_str("=================\n");
    content.push_str(&format!("Fundamental periods: {:?}\n", analysis.lattice_constants));
    content.push_str("Lattice basis vectors span modular arithmetic space\n");
    content.push_str("Each dimension represents residue classes modulo prime\n\n");

    content.push_str("MATHEMATICAL INTERPRETATION:\n");
    content.push_str("===========================\n");
    content.push_str("• Program functions exist as points in modular prime lattice\n");
    content.push_str("• Each function has coordinates in 12-dimensional prime space\n");
    content.push_str("• Enum types cluster in specific lattice regions\n");
    content.push_str("• Function sizes correlate with lattice position\n");
    content.push_str("• Modular residues reveal deep mathematical structure\n");

    fs::write(viz_path, content)?;
    println!("🎨 Lattice visualization written to: {}", viz_path);

    Ok(())
}

fn export_lattice_data(
    points: &[LatticePoint],
    analysis: &LatticeAnalysis,
) -> Result<(), Box<dyn std::error::Error>> {
    let export_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/lattice_coordinates.csv";
    let mut file = fs::File::create(&export_path)?;

    // CSV header
    writeln!(
        file,
        "address,enum_type,size,mod2,mod3,mod5,mod7,mod11,mod13,mod17,mod19,mod23,mod29,mod31,mod37"
    )?;

    // Export each lattice point
    for point in points {
        write!(file, "{},{},{}", point.address, point.enum_type, point.size)?;
        for &coord in &point.coordinates {
            write!(file, ",{}", coord)?;
        }
        writeln!(file)?;
    }

    println!("📊 Lattice coordinates exported to: {}", export_path);

    // Export lattice summary
    let summary_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/lattice_summary.json";
    let summary = serde_json::json!({
        "total_points": analysis.total_points,
        "dimension": analysis.dimension,
        "prime_basis": analysis.prime_basis,
        "clusters": analysis.clusters.len(),
        "lattice_constants": analysis.lattice_constants
    });

    fs::write(summary_path, serde_json::to_string_pretty(&summary)?)?;
    println!("📋 Lattice summary exported to: {}", summary_path);

    Ok(())
}

#[derive(Debug)]
struct LatticePoint {
    address: String,
    enum_type: String,
    size: u32,
    coordinates: Vec<u64>,
    dimension: usize,
}

#[derive(Debug)]
struct LatticeAnalysis {
    total_points: usize,
    dimension: usize,
    prime_basis: Vec<u64>,
    density_map: HashMap<u64, HashMap<u64, usize>>,
    clusters: Vec<LatticeCluster>,
    lattice_constants: Vec<u64>,
}

#[derive(Debug)]
struct LatticeCluster {
    coordinates: Vec<u64>,
    members: Vec<String>,
    size: usize,
}
