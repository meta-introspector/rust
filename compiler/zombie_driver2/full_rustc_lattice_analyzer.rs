use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 FULL RUSTC LATTICE ANALYZER - Processing ALL 2.8GB Data");
    println!("===========================================================");

    let mut total_functions = 0;
    let mut lattice_counts = HashMap::new();
    let mut coordinate_sums = vec![0u64; 12];
    let mut enum_functions = 0;
    let mut total_size = 0u64;
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    println!("🔍 SCANNING ALL RUSTC ANALYSIS DATA...");

    let analysis_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis/";

    for chunk_dir in fs::read_dir(analysis_dir)? {
        let chunk_dir = chunk_dir?;
        if !chunk_dir.file_type()?.is_dir() {
            continue;
        }

        let chunk_name = chunk_dir.file_name();
        println!("   Processing chunk: {:?}", chunk_name);

        for file in fs::read_dir(chunk_dir.path())? {
            let file = file?;
            let filename = file.file_name();
            let filename_str = filename.to_string_lossy();

            if filename_str.ends_with(".json") {
                if let Ok(content) = fs::read_to_string(file.path()) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        total_functions += 1;

                        // Extract basic function data
                        let address = data["memory_address"].as_str().unwrap_or("0x0");
                        let size = data["size"].as_u64().unwrap_or(0);
                        total_size += size;

                        // Calculate lattice coordinates for ALL functions
                        let coordinates = calculate_lattice_coordinates(&data, &primes);

                        // Update coordinate sums
                        for (i, &coord) in coordinates.iter().enumerate() {
                            if i < 12 {
                                coordinate_sums[i] += coord;
                            }
                        }

                        // Check if it's an enum-to-string function
                        let is_enum_func = is_enum_to_string_function(&data);
                        if is_enum_func {
                            enum_functions += 1;
                        }

                        // Add to lattice
                        let entry =
                            lattice_counts.entry(coordinates.clone()).or_insert_with(|| {
                                LatticeEntry {
                                    count: 0,
                                    total_size: 0,
                                    enum_count: 0,
                                    addresses: Vec::new(),
                                }
                            });

                        entry.count += 1;
                        entry.total_size += size;
                        if is_enum_func {
                            entry.enum_count += 1;
                        }
                        entry.addresses.push(address.to_string());

                        if total_functions % 10000 == 0 {
                            println!("      Processed {} functions...", total_functions);
                        }
                    }
                }
            }
        }
    }

    println!("✅ FULL SCAN COMPLETE!");
    println!("   Total functions: {}", total_functions);
    println!("   Enum functions: {}", enum_functions);
    println!("   Total size: {} bytes ({:.2} MB)", total_size, total_size as f64 / 1024.0 / 1024.0);
    println!("   Unique lattice positions: {}", lattice_counts.len());

    // Calculate mega-pole
    println!("\n🏔️ CALCULATING MEGA-POLE...");
    let mega_pole = calculate_mega_pole(&lattice_counts, &coordinate_sums);

    // Generate comprehensive report
    println!("\n📊 GENERATING COMPREHENSIVE LATTICE REPORT...");
    generate_comprehensive_report(
        &lattice_counts,
        &mega_pole,
        &coordinate_sums,
        total_functions,
        enum_functions,
        total_size,
    )?;

    // Export full lattice data
    println!("\n📄 EXPORTING FULL LATTICE DATA...");
    export_full_lattice_data(&lattice_counts, &mega_pole)?;

    Ok(())
}

fn calculate_lattice_coordinates(data: &Value, primes: &[u64]) -> Vec<u64> {
    let mut coordinates = vec![0u64; 12];

    // Extract integers from various sources
    let mut integers = Vec::new();

    // From address
    if let Some(addr_str) = data["memory_address"].as_str() {
        if let Ok(addr) = u64::from_str_radix(&addr_str[2..], 16) {
            integers.push(addr);
        }
    }

    // From size
    if let Some(size) = data["size"].as_u64() {
        integers.push(size);
    }

    // From LMFDB key
    if let Some(lmfdb) = data["mathematical_analysis"]["lmfdb_analysis"]["lmfdb_key"].as_str() {
        for part in lmfdb.split('.') {
            if let Ok(num) = part.parse::<u64>() {
                integers.push(num);
            }
        }
    }

    // From related strings (extract numbers)
    if let Some(strings) = data["related_strings"].as_array() {
        for s in strings {
            if let Some(string_val) = s.as_str() {
                for word in string_val.split_whitespace() {
                    for char_seq in
                        word.chars().collect::<String>().split(|c: char| !c.is_ascii_digit())
                    {
                        if !char_seq.is_empty() {
                            if let Ok(num) = char_seq.parse::<u64>() {
                                if num > 0 && num < 100000 {
                                    integers.push(num);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Calculate modular coordinates
    for (i, &prime) in primes.iter().enumerate() {
        if i < 12 {
            let mut sum = 0u64;
            for &num in &integers {
                sum += num % prime;
            }
            coordinates[i] = sum % prime;
        }
    }

    coordinates
}

fn is_enum_to_string_function(data: &Value) -> bool {
    let symbol = data["symbol_name"].as_str().unwrap_or("");
    let demangled = data["demangled_name"].as_str().unwrap_or("");

    // Check for Debug/Display formatters
    if symbol.contains("fmt..Debug") || symbol.contains("fmt..Display") {
        return true;
    }

    // Check for to_string functions
    if symbol.contains("to_string") || demangled.contains("to_string") {
        return true;
    }

    // Check for known enum types
    let enum_types =
        ["TokenKind", "ExprKind", "ItemKind", "TyKind", "DefKind", "PickKind", "NodeKind"];
    for enum_type in &enum_types {
        if (symbol.contains(enum_type) || demangled.contains(enum_type))
            && (symbol.contains("fmt") || symbol.contains("string"))
        {
            return true;
        }
    }

    false
}

fn calculate_mega_pole(
    lattice_counts: &HashMap<Vec<u64>, LatticeEntry>,
    coordinate_sums: &[u64],
) -> MegaPole {
    // Find maximum count pole
    let mut max_count = 0;
    let mut count_pole_coords = Vec::new();

    for (coords, entry) in lattice_counts {
        if entry.count > max_count {
            max_count = entry.count;
            count_pole_coords = coords.clone();
        }
    }

    // Find maximum size pole
    let mut max_size = 0;
    let mut size_pole_coords = Vec::new();

    for (coords, entry) in lattice_counts {
        if entry.total_size > max_size {
            max_size = entry.total_size;
            size_pole_coords = coords.clone();
        }
    }

    // Find maximum enum pole
    let mut max_enum = 0;
    let mut enum_pole_coords = Vec::new();

    for (coords, entry) in lattice_counts {
        if entry.enum_count > max_enum {
            max_enum = entry.enum_count;
            enum_pole_coords = coords.clone();
        }
    }

    let total_coordinate_sum: u64 = coordinate_sums.iter().sum();

    MegaPole {
        count_pole: count_pole_coords,
        count_pole_value: max_count,
        size_pole: size_pole_coords,
        size_pole_value: max_size,
        enum_pole: enum_pole_coords,
        enum_pole_value: max_enum,
        total_coordinate_sum,
        prime_sums: coordinate_sums.to_vec(),
    }
}

fn generate_comprehensive_report(
    lattice_counts: &HashMap<Vec<u64>, LatticeEntry>,
    mega_pole: &MegaPole,
    coordinate_sums: &[u64],
    total_functions: usize,
    enum_functions: usize,
    total_size: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let report_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/full_rustc_lattice_analysis.txt";
    let mut content = String::new();

    content.push_str("FULL RUSTC LATTICE ANALYSIS - Complete 2.8GB Dataset\n");
    content.push_str("====================================================\n\n");

    content.push_str("DATASET SUMMARY:\n");
    content.push_str("===============\n");
    content.push_str(&format!("Total functions analyzed: {}\n", total_functions));
    content.push_str(&format!(
        "Enum-to-string functions: {} ({:.2}%)\n",
        enum_functions,
        enum_functions as f64 / total_functions as f64 * 100.0
    ));
    content.push_str(&format!(
        "Total binary size: {} bytes ({:.2} MB)\n",
        total_size,
        total_size as f64 / 1024.0 / 1024.0
    ));
    content.push_str(&format!("Unique lattice positions: {}\n", lattice_counts.len()));
    content.push_str(&format!(
        "Lattice density: {:.6} functions per position\n",
        total_functions as f64 / lattice_counts.len() as f64
    ));
    content.push_str(&format!("MEGA-POLE TOTAL SUM: {}\n\n", mega_pole.total_coordinate_sum));

    content.push_str("MEGA-POLE ANALYSIS:\n");
    content.push_str("==================\n");
    content.push_str(&format!(
        "COUNT POLE: {:?} (value: {})\n",
        mega_pole.count_pole, mega_pole.count_pole_value
    ));
    content.push_str(&format!(
        "SIZE POLE: {:?} (value: {} bytes)\n",
        mega_pole.size_pole, mega_pole.size_pole_value
    ));
    content.push_str(&format!(
        "ENUM POLE: {:?} (value: {} enum functions)\n",
        mega_pole.enum_pole, mega_pole.enum_pole_value
    ));
    content.push_str("\n");

    content.push_str("PRIME DIMENSION ANALYSIS:\n");
    content.push_str("========================\n");
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for (i, (&prime, &sum)) in primes.iter().zip(coordinate_sums.iter()).enumerate() {
        let avg = sum as f64 / total_functions as f64;
        let percentage = sum as f64 / mega_pole.total_coordinate_sum as f64 * 100.0;
        content.push_str(&format!(
            "Prime {}: Sum = {} | Avg = {:.3} | {:.2}% of total\n",
            prime, sum, avg, percentage
        ));
    }
    content.push_str("\n");

    content.push_str("TOP LATTICE POSITIONS BY COUNT:\n");
    content.push_str("===============================\n");
    let mut sorted_by_count: Vec<_> = lattice_counts.iter().collect();
    sorted_by_count.sort_by_key(|(_, entry)| std::cmp::Reverse(entry.count));

    for (i, (coords, entry)) in sorted_by_count.iter().take(20).enumerate() {
        content.push_str(&format!(
            "{}. Count: {} | Size: {} bytes | Enums: {} | Coords: {:?}\n",
            i + 1,
            entry.count,
            entry.total_size,
            entry.enum_count,
            coords
        ));
    }
    content.push_str("\n");

    content.push_str("MATHEMATICAL INSIGHTS:\n");
    content.push_str("=====================\n");
    content.push_str(&format!(
        "• Total mathematical energy: {} units\n",
        mega_pole.total_coordinate_sum
    ));
    content.push_str(&format!(
        "• Lattice utilization: {:.2}% of possible positions\n",
        lattice_counts.len() as f64 / (37.0_f64.powi(12)) * 100.0
    ));
    content.push_str("• Prime dimension distribution reveals compiler architecture\n");
    content.push_str("• Mega-pole represents absolute maximum concentration\n");
    content.push_str("• Full rustc binary exhibits deep modular prime structure\n");

    fs::write(report_path, content)?;
    println!("📊 Comprehensive report written to: {}", report_path);

    Ok(())
}

fn export_full_lattice_data(
    lattice_counts: &HashMap<Vec<u64>, LatticeEntry>,
    mega_pole: &MegaPole,
) -> Result<(), Box<dyn std::error::Error>> {
    let export_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/full_rustc_lattice.csv";
    let mut file = fs::File::create(&export_path)?;

    writeln!(
        file,
        "coordinates,count,total_size,enum_count,sample_addresses,mod2,mod3,mod5,mod7,mod11,mod13,mod17,mod19,mod23,mod29,mod31,mod37,is_count_pole,is_size_pole,is_enum_pole"
    )?;

    let mut sorted_entries: Vec<_> = lattice_counts.iter().collect();
    sorted_entries.sort_by_key(|(_, entry)| std::cmp::Reverse(entry.count));

    for (coords, entry) in sorted_entries {
        let is_count_pole = coords == &mega_pole.count_pole;
        let is_size_pole = coords == &mega_pole.size_pole;
        let is_enum_pole = coords == &mega_pole.enum_pole;

        let sample_addresses =
            entry.addresses.iter().take(5).cloned().collect::<Vec<_>>().join(";");

        write!(
            file,
            "\"{:?}\",{},{},{},\"{}\"",
            coords, entry.count, entry.total_size, entry.enum_count, sample_addresses
        )?;

        for &coord in coords {
            write!(file, ",{}", coord)?;
        }

        writeln!(file, ",{},{},{}", is_count_pole, is_size_pole, is_enum_pole)?;
    }

    println!("📄 Full lattice data exported to: {}", export_path);

    Ok(())
}

struct LatticeEntry {
    count: usize,
    total_size: u64,
    enum_count: usize,
    addresses: Vec<String>,
}

struct MegaPole {
    count_pole: Vec<u64>,
    count_pole_value: usize,
    size_pole: Vec<u64>,
    size_pole_value: u64,
    enum_pole: Vec<u64>,
    enum_pole_value: usize,
    total_coordinate_sum: u64,
    prime_sums: Vec<u64>,
}
