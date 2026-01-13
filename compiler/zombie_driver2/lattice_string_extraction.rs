// Extract Strings and Related Objects from Lattice Coordinates
use goblin::elf::Elf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct LatticeLocationData {
    coordinates: (f64, f64, f64),
    lattice_label: String,
    extracted_strings: Vec<String>,
    related_objects: Vec<RelatedObject>,
    code_bytes: Vec<u8>,
    string_patterns: HashMap<String, u32>,
    monster_correlation: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelatedObject {
    name: String,
    object_type: String,
    distance: f64,
    correlation_strength: f64,
    shared_strings: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 LATTICE COORDINATE STRING & OBJECT EXTRACTION");
    println!("================================================");

    // Load lattice system
    let lattice = load_lattice_system()?;
    println!("📦 Loaded lattice with {} points", lattice["points"].as_array().unwrap().len());

    // Load original binary for string extraction
    let binary = load_rustc_binary()?;
    let elf = Elf::parse(&binary)?;
    println!("🔍 Loaded binary: {} bytes, {} symbols", binary.len(), elf.syms.len());

    // Extract strings and objects for each coordinate
    let location_data = extract_location_data(&lattice, &binary, &elf)?;

    // Analyze string patterns
    analyze_string_patterns(&location_data);

    // Save extracted data
    save_location_data(&location_data)?;

    Ok(())
}

fn load_lattice_system() -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("monster_lattice_system.json")?;
    let lattice: Value = serde_json::from_str(&content)?;
    Ok(lattice)
}

fn load_rustc_binary() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    Ok(fs::read(path)?)
}

fn extract_location_data(
    lattice: &Value,
    binary: &[u8],
    elf: &Elf,
) -> Result<Vec<LatticeLocationData>, Box<dyn std::error::Error>> {
    println!("\n🔤 EXTRACTING STRINGS & OBJECTS FROM COORDINATES:");
    println!("=================================================");

    let points = lattice["points"].as_array().unwrap();
    let mut location_data = Vec::new();

    for (i, point) in points.iter().enumerate().take(100) {
        // Process first 100 points
        if let (Some(coords), Some(label), Some(symbol_name)) = (
            point["coordinates"].as_array(),
            point["lattice_label"].as_str(),
            point["symbol_name"].as_str(),
        ) {
            let coordinates = (
                coords[0].as_f64().unwrap(),
                coords[1].as_f64().unwrap(),
                coords[2].as_f64().unwrap(),
            );

            // Find symbol in ELF
            if let Some(symbol) = find_symbol_by_name(elf, symbol_name) {
                // Extract code bytes
                let code_bytes = extract_symbol_bytes(binary, &symbol);

                // Extract strings from code
                let extracted_strings = extract_strings_from_bytes(&code_bytes);

                // Find related objects
                let related_objects = find_related_objects(points, i, &coordinates);

                // Analyze string patterns
                let string_patterns = analyze_string_patterns_in_bytes(&code_bytes);

                // Calculate Monster correlation
                let monster_correlation =
                    calculate_location_monster_correlation(&extracted_strings, &code_bytes);

                location_data.push(LatticeLocationData {
                    coordinates,
                    lattice_label: label.to_string(),
                    extracted_strings,
                    related_objects,
                    code_bytes,
                    string_patterns,
                    monster_correlation,
                });
            }
        }

        if (i + 1) % 20 == 0 {
            println!("   ✅ Processed {} coordinates", i + 1);
        }
    }

    println!("   🎯 Extracted data from {} locations", location_data.len());
    Ok(location_data)
}

fn find_symbol_by_name<'a>(elf: &'a Elf, name: &str) -> Option<goblin::elf::Sym> {
    for sym in &elf.syms {
        if let Some(sym_name) = elf.strtab.get_at(sym.st_name) {
            if sym_name == name {
                return Some(sym.clone());
            }
        }
    }
    None
}

fn extract_symbol_bytes(binary: &[u8], symbol: &goblin::elf::Sym) -> Vec<u8> {
    let start = symbol.st_value as usize;
    let size = symbol.st_size as usize;

    if start + size <= binary.len() && size > 0 {
        binary[start..start + size].to_vec()
    } else {
        Vec::new()
    }
}

fn extract_strings_from_bytes(bytes: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current_string = Vec::new();

    for &byte in bytes {
        if byte.is_ascii_graphic() || byte == b' ' {
            current_string.push(byte);
        } else {
            if current_string.len() >= 4 {
                // Minimum string length
                if let Ok(s) = String::from_utf8(current_string.clone()) {
                    strings.push(s);
                }
            }
            current_string.clear();
        }
    }

    // Final string
    if current_string.len() >= 4 {
        if let Ok(s) = String::from_utf8(current_string) {
            strings.push(s);
        }
    }

    strings.truncate(20); // Limit to 20 strings per location
    strings
}

fn find_related_objects(
    points: &[Value],
    current_idx: usize,
    coordinates: &(f64, f64, f64),
) -> Vec<RelatedObject> {
    let mut related = Vec::new();

    for (i, point) in points.iter().enumerate() {
        if i != current_idx {
            if let (Some(other_coords), Some(other_name), Some(_other_label)) = (
                point["coordinates"].as_array(),
                point["symbol_name"].as_str(),
                point["lattice_label"].as_str(),
            ) {
                let other_coordinates = (
                    other_coords[0].as_f64().unwrap(),
                    other_coords[1].as_f64().unwrap(),
                    other_coords[2].as_f64().unwrap(),
                );

                let distance = calculate_distance(coordinates, &other_coordinates);

                if distance < 10.0 {
                    // Within proximity threshold
                    let correlation_strength =
                        calculate_object_correlation(coordinates, &other_coordinates);
                    let shared_strings = find_shared_strings(other_name);

                    let object_type = classify_object_type(other_name);

                    related.push(RelatedObject {
                        name: other_name.to_string(),
                        object_type,
                        distance,
                        correlation_strength,
                        shared_strings,
                    });
                }
            }
        }
    }

    // Sort by correlation strength and keep top 10
    related.sort_by(|a, b| b.correlation_strength.partial_cmp(&a.correlation_strength).unwrap());
    related.truncate(10);

    related
}

fn calculate_distance(coord1: &(f64, f64, f64), coord2: &(f64, f64, f64)) -> f64 {
    let dx = coord1.0 - coord2.0;
    let dy = coord1.1 - coord2.1;
    let dz = coord1.2 - coord2.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn calculate_object_correlation(coord1: &(f64, f64, f64), coord2: &(f64, f64, f64)) -> f64 {
    let distance = calculate_distance(coord1, coord2);
    let max_distance = 10.0;

    // Inverse distance correlation
    (max_distance - distance) / max_distance
}

fn find_shared_strings(symbol_name: &str) -> Vec<String> {
    let mut shared = Vec::new();

    // Extract common patterns from symbol names
    if symbol_name.contains("rustc") {
        shared.push("rustc".to_string());
    }
    if symbol_name.contains("driver") {
        shared.push("driver".to_string());
    }
    if symbol_name.contains("jiff") {
        shared.push("jiff".to_string());
    }
    if symbol_name.contains("alloc") {
        shared.push("alloc".to_string());
    }
    if symbol_name.contains("core") {
        shared.push("core".to_string());
    }
    if symbol_name.contains("std") {
        shared.push("std".to_string());
    }

    shared
}

fn classify_object_type(symbol_name: &str) -> String {
    if symbol_name.contains("rustc_driver") {
        "Compiler Core".to_string()
    } else if symbol_name.contains("jiff") {
        "Time Library".to_string()
    } else if symbol_name.contains("alloc") {
        "Memory Management".to_string()
    } else if symbol_name.contains("tracing") {
        "Logging System".to_string()
    } else if symbol_name.contains("core") {
        "Core Library".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn analyze_string_patterns_in_bytes(bytes: &[u8]) -> HashMap<String, u32> {
    let mut patterns = HashMap::new();

    // Generate n-grams up to reasonable size (limited for performance)
    for n in 2..=20 {
        let mut ngram_counts = HashMap::new();

        // Extract n-grams from bytes
        for window in bytes.windows(n) {
            if let Ok(s) = String::from_utf8(window.to_vec()) {
                if s.chars().all(|c| c.is_ascii_graphic() || c == ' ') && s.len() >= 2 {
                    *ngram_counts.entry(s).or_insert(0) += 1;
                }
            }
        }

        // Keep top 2 n-grams for this size
        let mut sorted: Vec<_> = ngram_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        for (i, (ngram, count)) in sorted.iter().take(2).enumerate() {
            if *count > 1 {
                // Only keep n-grams that appear multiple times
                patterns.insert(format!("{}-gram-{}: {}", n, i + 1, ngram), *count);
            }
        }
    }

    // Also look for common programming patterns
    let common_patterns = [
        ("error", "error".as_bytes()),
        ("warning", "warning".as_bytes()),
        ("debug", "debug".as_bytes()),
        ("rustc", "rustc".as_bytes()),
        ("impl", "impl".as_bytes()),
        ("fn", "fn".as_bytes()),
        ("struct", "struct".as_bytes()),
    ];

    for (name, pattern) in &common_patterns {
        let count = count_pattern_occurrences(bytes, pattern);
        if count > 0 {
            patterns.insert(format!("keyword-{}", name), count);
        }
    }

    patterns
}

fn count_pattern_occurrences(bytes: &[u8], pattern: &[u8]) -> u32 {
    let mut count = 0;
    for window in bytes.windows(pattern.len()) {
        if window == pattern {
            count += 1;
        }
    }
    count
}

fn calculate_location_monster_correlation(strings: &[String], bytes: &[u8]) -> f64 {
    let mut correlation = 0.0;

    // String-based correlation
    for string in strings {
        for ch in string.chars() {
            let ascii_val = ch as u32 as u64;
            // Check divisibility by key Monster primes
            for &prime in &[2, 3, 5, 7, 11, 31, 71] {
                if ascii_val % prime == 0 {
                    correlation += 1.0 / prime as f64;
                }
            }
        }
    }

    // Byte-based correlation
    for &byte in bytes.iter().take(100) {
        // Sample first 100 bytes
        for &prime in &[2, 3, 5, 7, 11, 31, 71] {
            if byte as u64 % prime == 0 {
                correlation += 0.1 / prime as f64;
            }
        }
    }

    correlation / (strings.len() + 1) as f64 // Normalize
}

fn analyze_string_patterns(location_data: &[LatticeLocationData]) {
    println!("\n📊 STRING PATTERN ANALYSIS:");
    println!("===========================");

    let mut all_strings = HashMap::new();
    let mut all_patterns = HashMap::new();

    for location in location_data {
        // Collect all strings
        for string in &location.extracted_strings {
            *all_strings.entry(string.clone()).or_insert(0) += 1;
        }

        // Collect all patterns
        for (pattern, count) in &location.string_patterns {
            *all_patterns.entry(pattern.clone()).or_insert(0) += count;
        }
    }

    println!("   🔤 Most Common Extracted Strings:");
    let mut sorted_strings: Vec<_> = all_strings.into_iter().collect();
    sorted_strings.sort_by(|a, b| b.1.cmp(&a.1));

    for (string, count) in sorted_strings.iter().take(10) {
        println!("     '{}': {} occurrences", string, count);
    }

    println!("\n   🔍 Most Common Byte Patterns:");
    let mut sorted_patterns: Vec<_> = all_patterns.into_iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(&a.1));

    for (pattern, count) in sorted_patterns.iter().take(8) {
        println!("     '{}': {} occurrences", pattern, count);
    }

    // Monster correlation statistics
    let avg_correlation = location_data.iter().map(|l| l.monster_correlation).sum::<f64>()
        / location_data.len() as f64;
    let max_correlation =
        location_data.iter().map(|l| l.monster_correlation).fold(0.0f64, |a, b| a.max(b));

    println!("\n   🧬 Monster Correlation in Strings:");
    println!("     Average correlation: {:.4}", avg_correlation);
    println!("     Maximum correlation: {:.4}", max_correlation);

    // Most Monster-correlated locations
    let mut sorted_locations: Vec<_> = location_data.iter().collect();
    sorted_locations
        .sort_by(|a, b| b.monster_correlation.partial_cmp(&a.monster_correlation).unwrap());

    println!("\n   🎯 Most Monster-Correlated Locations:");
    for (i, location) in sorted_locations.iter().take(5).enumerate() {
        println!(
            "     {}: {} (correlation: {:.4})",
            i + 1,
            location.lattice_label,
            location.monster_correlation
        );
        if !location.extracted_strings.is_empty() {
            println!(
                "       Strings: {}",
                location.extracted_strings.iter().take(3).cloned().collect::<Vec<_>>().join(", ")
            );
        }
    }
}

fn save_location_data(
    location_data: &[LatticeLocationData],
) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete location data
    let json = serde_json::to_string_pretty(location_data)?;
    fs::write("lattice_location_strings.json", json)?;

    // Create summary CSV
    let mut csv_content = String::from(
        "lattice_label,x,y,z,string_count,pattern_count,monster_correlation,top_strings\n",
    );
    for location in location_data {
        let top_strings =
            location.extracted_strings.iter().take(3).cloned().collect::<Vec<_>>().join(";");
        csv_content.push_str(&format!(
            "{},{:.3},{:.3},{:.3},{},{},{:.4},\"{}\"\n",
            location.lattice_label,
            location.coordinates.0,
            location.coordinates.1,
            location.coordinates.2,
            location.extracted_strings.len(),
            location.string_patterns.len(),
            location.monster_correlation,
            top_strings
        ));
    }
    fs::write("lattice_strings_summary.csv", csv_content)?;

    println!("\n💾 LOCATION STRING DATA SAVED:");
    println!("==============================");
    println!("   Complete data: lattice_location_strings.json");
    println!("   Summary CSV: lattice_strings_summary.csv");
    println!("   Locations processed: {}", location_data.len());

    let total_strings: usize = location_data.iter().map(|l| l.extracted_strings.len()).sum();
    let total_patterns: usize = location_data.iter().map(|l| l.string_patterns.len()).sum();

    println!("   Total strings extracted: {}", total_strings);
    println!("   Total patterns found: {}", total_patterns);

    Ok(())
}
