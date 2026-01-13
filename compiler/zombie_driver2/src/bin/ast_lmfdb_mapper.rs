use serde_json;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct LMFDBEntry {
    label: String,
    degree: usize,
    discriminant: i64,
    conductor: usize,
    rank: usize,
    torsion_order: usize,
    j_invariant: String,
    cremona_label: String,
    ast_path: String,
    enum_symbol: String,
    mathematical_properties: MathProperties,
}

#[derive(Debug, Clone)]
struct MathProperties {
    is_elliptic: bool,
    genus: usize,
    euler_characteristic: i64,
    betti_numbers: Vec<usize>,
    fundamental_group: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️ RUSTC AST → LMFDB MAPPING GENERATOR");
    println!("======================================");

    // Load our mathematical structures
    let autolabel_data = fs::read_to_string("rustc_autolabel_set.json")?;
    let lattice_data = fs::read_to_string("rustc_enum_string_lattice.json")?;
    let periodic_data = fs::read_to_string("rustc_enum_periodic_table.json")?;

    let autolabel: serde_json::Value = serde_json::from_str(&autolabel_data)?;
    let lattice: serde_json::Value = serde_json::from_str(&lattice_data)?;
    let periodic: serde_json::Value = serde_json::from_str(&periodic_data)?;

    println!("📊 Loaded mathematical structures:");
    println!("   Autolabel functions: {}", autolabel["total_functions"]);
    println!("   Lattice orbits: {}", lattice["total_orbits"]);
    println!("   Periodic elements: {}", periodic["total_elements"]);

    let mut lmfdb_entries = Vec::new();
    let functions = autolabel["top_functions"].as_array().unwrap();
    let orbits = lattice["orbits"].as_array().unwrap();
    let elements = periodic["elements"].as_array().unwrap();

    // Create LMFDB entries for each AST type
    for (i, func) in functions.iter().enumerate() {
        let type_name = func["type_name"].as_str().unwrap_or("Unknown");
        let function_size = func["function_size"].as_u64().unwrap_or(0) as usize;
        let references = func["references"].as_u64().unwrap_or(0) as usize;

        // Find corresponding orbit and element
        let orbit_idx = find_orbit_for_size(function_size, orbits);
        let element_idx = find_element_for_orbit(orbit_idx, elements);

        // Generate LMFDB-style mathematical properties
        let degree = calculate_degree(type_name, function_size);
        let discriminant = calculate_discriminant(type_name, references);
        let conductor = calculate_conductor(function_size, references);
        let rank = calculate_rank(type_name);
        let torsion_order = calculate_torsion_order(references);

        let math_props = MathProperties {
            is_elliptic: is_elliptic_curve_type(type_name),
            genus: calculate_genus(function_size),
            euler_characteristic: calculate_euler_char(degree, genus_from_size(function_size)),
            betti_numbers: calculate_betti_numbers(degree),
            fundamental_group: determine_fundamental_group(type_name),
        };

        let entry = LMFDBEntry {
            label: generate_lmfdb_label(degree, conductor, i),
            degree,
            discriminant,
            conductor,
            rank,
            torsion_order,
            j_invariant: calculate_j_invariant(discriminant),
            cremona_label: generate_cremona_label(conductor, rank, i),
            ast_path: extract_ast_path(type_name),
            enum_symbol: get_element_symbol(element_idx, elements),
            mathematical_properties: math_props,
        };

        lmfdb_entries.push(entry);
    }

    println!("\n🔢 GENERATED LMFDB ENTRIES:");
    println!("===========================");

    // Display sample entries
    for (i, entry) in lmfdb_entries.iter().take(15).enumerate() {
        println!(
            "{:2}: {} | deg:{} disc:{} cond:{} rank:{} tors:{}",
            i + 1,
            entry.label,
            entry.degree,
            entry.discriminant,
            entry.conductor,
            entry.rank,
            entry.torsion_order
        );
        println!(
            "    AST: {} | Symbol: {} | Genus: {} | χ: {}",
            entry.ast_path,
            entry.enum_symbol,
            entry.mathematical_properties.genus,
            entry.mathematical_properties.euler_characteristic
        );

        if entry.mathematical_properties.is_elliptic {
            println!("    🔮 Elliptic curve: j = {}", entry.j_invariant);
        }
        println!();
    }

    // Analyze mathematical distribution
    analyze_lmfdb_distribution(&lmfdb_entries);

    // Save LMFDB mapping
    let lmfdb_json = serde_json::json!({
        "total_entries": lmfdb_entries.len(),
        "database_version": "rustc-ast-1.0",
        "entries": lmfdb_entries.iter().take(100).map(|e| {
            serde_json::json!({
                "label": e.label,
                "degree": e.degree,
                "discriminant": e.discriminant,
                "conductor": e.conductor,
                "rank": e.rank,
                "torsion_order": e.torsion_order,
                "j_invariant": e.j_invariant,
                "cremona_label": e.cremona_label,
                "ast_path": e.ast_path,
                "enum_symbol": e.enum_symbol,
                "properties": {
                    "is_elliptic": e.mathematical_properties.is_elliptic,
                    "genus": e.mathematical_properties.genus,
                    "euler_characteristic": e.mathematical_properties.euler_characteristic,
                    "betti_numbers": e.mathematical_properties.betti_numbers,
                    "fundamental_group": e.mathematical_properties.fundamental_group
                }
            })
        }).collect::<Vec<_>>()
    });

    fs::write("rustc_ast_lmfdb_mapping.json", serde_json::to_string_pretty(&lmfdb_json)?)?;
    println!("💾 Saved LMFDB mapping to rustc_ast_lmfdb_mapping.json");

    Ok(())
}

fn find_orbit_for_size(size: usize, orbits: &[serde_json::Value]) -> usize {
    let length = match size {
        0..=10 => 2,
        11..=100 => 3,
        101..=1000 => 4,
        1001..=5000 => 5,
        _ => 6,
    };

    orbits.iter().position(|o| o["length"].as_u64().unwrap() as usize == length).unwrap_or(0)
}

fn find_element_for_orbit(orbit_idx: usize, elements: &[serde_json::Value]) -> usize {
    orbit_idx.min(elements.len() - 1)
}

fn calculate_degree(type_name: &str, size: usize) -> usize {
    if type_name.contains("rustc") {
        if type_name.contains("ty") {
            2
        } else if type_name.contains("mir") {
            3
        } else if type_name.contains("hir") {
            4
        } else {
            5
        }
    } else {
        (size / 1000).max(1).min(6)
    }
}

fn calculate_discriminant(type_name: &str, references: usize) -> i64 {
    let base = if type_name.contains("Error") { -1 } else { 1 };
    base * (references as i64 * 17 + 23)
}

fn calculate_conductor(size: usize, references: usize) -> usize {
    (size + references * 10).max(1)
}

fn calculate_rank(type_name: &str) -> usize {
    if type_name.contains("rustc_middle") {
        2
    } else if type_name.contains("rustc_hir") {
        1
    } else if type_name.contains("rustc_ast") {
        0
    } else {
        (type_name.len() % 3)
    }
}

fn calculate_torsion_order(references: usize) -> usize {
    match references {
        0 => 1,
        1 => 2,
        2..=3 => 4,
        4..=7 => 8,
        _ => 16,
    }
}

fn is_elliptic_curve_type(type_name: &str) -> bool {
    type_name.contains("Curve") || type_name.contains("Float") || type_name.contains("Complex")
}

fn calculate_genus(size: usize) -> usize {
    match size {
        0..=100 => 0,
        101..=1000 => 1,
        1001..=5000 => 2,
        _ => 3,
    }
}

fn genus_from_size(size: usize) -> usize {
    calculate_genus(size)
}

fn calculate_euler_char(degree: usize, genus: usize) -> i64 {
    2 - 2 * genus as i64 - degree as i64
}

fn calculate_betti_numbers(degree: usize) -> Vec<usize> {
    match degree {
        1 => vec![1, 0],
        2 => vec![1, 2, 1],
        3 => vec![1, 3, 3, 1],
        4 => vec![1, 4, 6, 4, 1],
        _ => vec![1, degree, degree * (degree - 1) / 2, degree, 1],
    }
}

fn determine_fundamental_group(type_name: &str) -> String {
    if type_name.contains("rustc_middle") {
        "π₁ = Z".to_string()
    } else if type_name.contains("rustc_hir") {
        "π₁ = Z/2Z".to_string()
    } else if type_name.contains("rustc_ast") {
        "π₁ = {1}".to_string()
    } else if type_name.contains("Error") {
        "π₁ = F₂".to_string()
    } else {
        "π₁ = Z * Z".to_string()
    }
}

fn calculate_j_invariant(discriminant: i64) -> String {
    if discriminant == 0 {
        "∞".to_string()
    } else {
        format!("{}", 1728 * discriminant / (discriminant + 1))
    }
}

fn generate_lmfdb_label(degree: usize, conductor: usize, index: usize) -> String {
    format!("{}.{}.{}.1", degree, conductor, index + 1)
}

fn generate_cremona_label(conductor: usize, rank: usize, index: usize) -> String {
    let letter = (b'a' + (index % 26) as u8) as char;
    format!("{}{}{}", conductor, letter, rank + 1)
}

fn extract_ast_path(type_name: &str) -> String {
    if type_name.contains("::") {
        type_name.split("::").take(3).collect::<Vec<_>>().join("::")
    } else {
        type_name.to_string()
    }
}

fn get_element_symbol(element_idx: usize, elements: &[serde_json::Value]) -> String {
    elements.get(element_idx).and_then(|e| e["symbol"].as_str()).unwrap_or("Ux").to_string()
}

fn analyze_lmfdb_distribution(entries: &[LMFDBEntry]) {
    println!("🔬 LMFDB DISTRIBUTION ANALYSIS:");
    println!("===============================");

    let elliptic_count = entries.iter().filter(|e| e.mathematical_properties.is_elliptic).count();
    let avg_degree: f64 =
        entries.iter().map(|e| e.degree).sum::<usize>() as f64 / entries.len() as f64;
    let avg_genus: f64 = entries.iter().map(|e| e.mathematical_properties.genus).sum::<usize>()
        as f64
        / entries.len() as f64;

    println!("📊 Statistics:");
    println!("   Total entries: {}", entries.len());
    println!(
        "   Elliptic curves: {} ({:.1}%)",
        elliptic_count,
        100.0 * elliptic_count as f64 / entries.len() as f64
    );
    println!("   Average degree: {:.2}", avg_degree);
    println!("   Average genus: {:.2}", avg_genus);

    // Rank distribution
    let mut rank_dist = HashMap::new();
    for entry in entries {
        *rank_dist.entry(entry.rank).or_insert(0) += 1;
    }

    println!("\n📈 Rank Distribution:");
    for (rank, count) in rank_dist {
        println!("   Rank {}: {} entries", rank, count);
    }
}
