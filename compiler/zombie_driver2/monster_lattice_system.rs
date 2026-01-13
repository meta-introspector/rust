// Auto-Labeled Monster Lattice Coordinate System
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug, Serialize, Deserialize)]
struct MonsterLatticePoint {
    coordinates: (f64, f64, f64), // (x, y, z) in Monster space
    symbol_name: String,
    phi_score: f64,
    monster_class: String,
    lattice_label: String,
    prime_vector: Vec<u64>,
    neighbors: Vec<String>,
    cluster_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MonsterLattice {
    points: Vec<MonsterLatticePoint>,
    dimensions: u32,
    lattice_constants: Vec<f64>,
    symmetry_groups: HashMap<u64, Vec<usize>>,
    cluster_map: HashMap<u32, Vec<usize>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔮 AUTO-LABELED MONSTER LATTICE COORDINATE SYSTEM");
    println!("=================================================");

    // Load labeled Monster symbols
    let labeled_symbols = load_labeled_symbols()?;
    println!("📦 Loaded {} labeled Monster symbols", labeled_symbols.len());

    // Generate lattice coordinates
    let lattice = generate_monster_lattice(&labeled_symbols);

    // Auto-label lattice points
    let labeled_lattice = auto_label_lattice(lattice);

    // Analyze lattice structure
    analyze_lattice_structure(&labeled_lattice);

    // Save lattice system
    save_lattice_system(&labeled_lattice)?;

    Ok(())
}

fn load_labeled_symbols() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("monster_labeled_symbols.json")?;
    let symbols: Vec<Value> = serde_json::from_str(&content)?;
    Ok(symbols)
}

fn generate_monster_lattice(symbols: &[Value]) -> MonsterLattice {
    println!("\n🔮 GENERATING MONSTER LATTICE:");
    println!("==============================");

    let mut points = Vec::new();
    let mut symmetry_groups = HashMap::new();

    for (i, symbol) in symbols.iter().enumerate() {
        if let (
            Some(name),
            Some(phi_score),
            Some(monster_class),
            Some(monster_sig),
            Some(dominant_prime),
        ) = (
            symbol["name"].as_str(),
            symbol["phi_score"].as_f64(),
            symbol["monster_class"].as_str(),
            symbol["monster_signature"].as_array(),
            symbol["dominant_prime"].as_u64(),
        ) {
            // Convert Monster signature to prime vector
            let prime_vector: Vec<u64> = monster_sig.iter().filter_map(|v| v.as_u64()).collect();

            // Calculate lattice coordinates from Monster signature
            let coordinates = calculate_lattice_coordinates(&prime_vector, phi_score);

            // Generate lattice label
            let lattice_label = generate_lattice_label(&coordinates, monster_class, dominant_prime);

            let point = MonsterLatticePoint {
                coordinates,
                symbol_name: name.to_string(),
                phi_score,
                monster_class: monster_class.to_string(),
                lattice_label,
                prime_vector,
                neighbors: Vec::new(), // Will be filled later
                cluster_id: 0,         // Will be assigned later
            };

            points.push(point);

            // Group by dominant prime
            symmetry_groups.entry(dominant_prime).or_insert_with(Vec::new).push(i);
        }
    }

    println!("   ✅ Generated {} lattice points", points.len());
    println!("   🧬 Found {} symmetry groups", symmetry_groups.len());

    MonsterLattice {
        points,
        dimensions: 3,                          // 3D lattice
        lattice_constants: vec![1.0, 1.0, 1.0], // Unit lattice
        symmetry_groups,
        cluster_map: HashMap::new(),
    }
}

fn calculate_lattice_coordinates(prime_vector: &[u64], phi_score: f64) -> (f64, f64, f64) {
    // Map Monster signature to 3D coordinates
    let x = prime_vector.iter().take(12).sum::<u64>() as f64 / 1000.0; // First 12 primes
    let y = prime_vector.iter().skip(12).take(12).sum::<u64>() as f64 / 1000.0; // Next 12 primes
    let z = phi_score; // Phi score as height

    // Apply Monster Group scaling
    let scaled_x = x * (prime_vector[0] as f64).sqrt(); // Scale by first prime
    let scaled_y = y * (prime_vector[1] as f64).sqrt(); // Scale by second prime
    let scaled_z = z * 10.0; // Scale phi for visibility

    (scaled_x, scaled_y, scaled_z)
}

fn generate_lattice_label(
    coordinates: &(f64, f64, f64),
    monster_class: &str,
    dominant_prime: u64,
) -> String {
    let (x, y, z) = coordinates;

    // Quantize coordinates to lattice positions
    let lattice_x = (x * 10.0).round() as i32;
    let lattice_y = (y * 10.0).round() as i32;
    let lattice_z = (z * 10.0).round() as i32;

    // Generate hierarchical label
    let class_prefix = match monster_class {
        "UltraMonster" => "UM",
        "HighMonster" => "HM",
        "ModerateMonster" => "MM",
        "LowMonster" => "LM",
        _ => "NM",
    };

    format!("{}-P{}-[{},{},{}]", class_prefix, dominant_prime, lattice_x, lattice_y, lattice_z)
}

fn auto_label_lattice(mut lattice: MonsterLattice) -> MonsterLattice {
    println!("\n🏷️ AUTO-LABELING LATTICE POINTS:");
    println!("=================================");

    // Find neighbors for each point
    for i in 0..lattice.points.len() {
        let neighbors = find_lattice_neighbors(&lattice.points, i);
        lattice.points[i].neighbors = neighbors;
    }

    // Cluster points by proximity and Monster class
    let clusters = cluster_lattice_points(&lattice.points);
    lattice.cluster_map = clusters;

    // Assign cluster IDs
    for (cluster_id, point_indices) in &lattice.cluster_map {
        for &point_idx in point_indices {
            lattice.points[point_idx].cluster_id = *cluster_id;
        }
    }

    println!("   ✅ Found neighbors for {} points", lattice.points.len());
    println!("   🎯 Created {} clusters", lattice.cluster_map.len());

    lattice
}

fn find_lattice_neighbors(points: &[MonsterLatticePoint], point_idx: usize) -> Vec<String> {
    let current = &points[point_idx];
    let mut neighbors = Vec::new();

    for (i, other) in points.iter().enumerate() {
        if i != point_idx {
            let distance = calculate_lattice_distance(&current.coordinates, &other.coordinates);

            // Consider neighbors within distance threshold
            if distance < 5.0 {
                neighbors.push(other.lattice_label.clone());
            }
        }
    }

    neighbors.sort();
    neighbors.truncate(8); // Max 8 neighbors
    neighbors
}

fn calculate_lattice_distance(coord1: &(f64, f64, f64), coord2: &(f64, f64, f64)) -> f64 {
    let dx = coord1.0 - coord2.0;
    let dy = coord1.1 - coord2.1;
    let dz = coord1.2 - coord2.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn cluster_lattice_points(points: &[MonsterLatticePoint]) -> HashMap<u32, Vec<usize>> {
    let mut clusters = HashMap::new();
    let mut cluster_id = 0;

    // Group by Monster class and proximity
    let mut class_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, point) in points.iter().enumerate() {
        class_groups.entry(point.monster_class.clone()).or_insert_with(Vec::new).push(i);
    }

    for (class, indices) in class_groups {
        // Further cluster by spatial proximity within each class
        let mut remaining: Vec<usize> = indices;

        while !remaining.is_empty() {
            let seed = remaining.remove(0);
            let mut cluster = vec![seed];

            // Find nearby points
            let mut i = 0;
            while i < remaining.len() {
                let distance = calculate_lattice_distance(
                    &points[seed].coordinates,
                    &points[remaining[i]].coordinates,
                );

                if distance < 3.0 {
                    // Cluster threshold
                    cluster.push(remaining.remove(i));
                } else {
                    i += 1;
                }
            }

            clusters.insert(cluster_id, cluster);
            cluster_id += 1;
        }
    }

    clusters
}

fn analyze_lattice_structure(lattice: &MonsterLattice) {
    println!("\n🔬 LATTICE STRUCTURE ANALYSIS:");
    println!("==============================");

    // Coordinate statistics
    let x_coords: Vec<f64> = lattice.points.iter().map(|p| p.coordinates.0).collect();
    let y_coords: Vec<f64> = lattice.points.iter().map(|p| p.coordinates.1).collect();
    let z_coords: Vec<f64> = lattice.points.iter().map(|p| p.coordinates.2).collect();

    let x_range = x_coords.iter().fold(0.0f64, |a, &b| a.max(b))
        - x_coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let y_range = y_coords.iter().fold(0.0f64, |a, &b| a.max(b))
        - y_coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let z_range = z_coords.iter().fold(0.0f64, |a, &b| a.max(b))
        - z_coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    println!("   📏 Lattice Dimensions:");
    println!("     X range: {:.3}", x_range);
    println!("     Y range: {:.3}", y_range);
    println!("     Z range: {:.3}", z_range);
    println!("     Volume: {:.3}", x_range * y_range * z_range);

    // Cluster analysis
    println!("\n   🎯 Cluster Analysis:");
    for (cluster_id, point_indices) in &lattice.cluster_map {
        let cluster_points: Vec<&MonsterLatticePoint> =
            point_indices.iter().map(|&i| &lattice.points[i]).collect();

        let avg_phi =
            cluster_points.iter().map(|p| p.phi_score).sum::<f64>() / cluster_points.len() as f64;
        let class = &cluster_points[0].monster_class;

        println!(
            "     Cluster {}: {} points, class={}, avg_φ={:.3}",
            cluster_id,
            cluster_points.len(),
            class,
            avg_phi
        );
    }

    // Symmetry group analysis
    println!("\n   🧬 Symmetry Groups:");
    for (&prime, indices) in &lattice.symmetry_groups {
        let group_points: Vec<&MonsterLatticePoint> =
            indices.iter().map(|&i| &lattice.points[i]).collect();

        let avg_phi =
            group_points.iter().map(|p| p.phi_score).sum::<f64>() / group_points.len() as f64;

        println!("     Prime {}: {} points, avg_φ={:.3}", prime, group_points.len(), avg_phi);
    }

    // Lattice density
    let total_volume = x_range * y_range * z_range;
    let density = lattice.points.len() as f64 / total_volume;
    println!("\n   📊 Lattice Density: {:.6} points/unit³", density);

    // Most connected points
    println!("\n   🕸️ Most Connected Points:");
    let mut connected_points: Vec<_> =
        lattice.points.iter().map(|p| (p.lattice_label.clone(), p.neighbors.len())).collect();
    connected_points.sort_by(|a, b| b.1.cmp(&a.1));

    for (label, neighbor_count) in connected_points.iter().take(5) {
        println!("     {}: {} neighbors", label, neighbor_count);
    }
}

fn save_lattice_system(lattice: &MonsterLattice) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete lattice
    let json = serde_json::to_string_pretty(lattice)?;
    fs::write("monster_lattice_system.json", json)?;

    // Create coordinate CSV for visualization
    let mut csv_content = String::from("x,y,z,phi_score,monster_class,lattice_label,cluster_id\n");
    for point in &lattice.points {
        csv_content.push_str(&format!(
            "{:.6},{:.6},{:.6},{:.4},{},{},{}\n",
            point.coordinates.0,
            point.coordinates.1,
            point.coordinates.2,
            point.phi_score,
            point.monster_class,
            point.lattice_label,
            point.cluster_id
        ));
    }
    fs::write("monster_lattice_coordinates.csv", csv_content)?;

    println!("\n💾 LATTICE SYSTEM SAVED:");
    println!("========================");
    println!("   Complete lattice: monster_lattice_system.json");
    println!("   Coordinates CSV: monster_lattice_coordinates.csv");
    println!("   Points: {}", lattice.points.len());
    println!("   Clusters: {}", lattice.cluster_map.len());
    println!("   Symmetry groups: {}", lattice.symmetry_groups.len());

    Ok(())
}
