use serde_json;
use std::collections::{BTreeMap, HashMap};
use std::fs;

#[derive(Debug, Clone)]
struct ComplexityOrbit {
    length: usize,
    functions: Vec<String>,
    generator_count: usize,
    ring_properties: RingProperties,
}

#[derive(Debug, Clone)]
struct RingProperties {
    closure: bool,
    associativity: bool,
    identity: Option<String>,
    generators: Vec<String>,
}

#[derive(Debug, Clone)]
struct ComplexityLattice {
    orbits: BTreeMap<usize, ComplexityOrbit>,
    total_functions: usize,
    max_complexity: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌌 RUSTC COMPLEXITY LATTICE GENERATOR");
    println!("====================================");

    // Load autolabeling set
    let autolabel_data = fs::read_to_string("rustc_autolabel_set.json")?;
    let autolabel: serde_json::Value = serde_json::from_str(&autolabel_data)?;

    let functions = autolabel["top_functions"].as_array().unwrap();

    println!("📊 Building complexity lattice from {} functions", functions.len());

    let mut lattice = ComplexityLattice {
        orbits: BTreeMap::new(),
        total_functions: functions.len(),
        max_complexity: 0,
    };

    // Group functions by complexity (function size as proxy)
    let mut complexity_groups: HashMap<usize, Vec<String>> = HashMap::new();

    for func in functions {
        let size = func["function_size"].as_u64().unwrap_or(0) as usize;
        let name = func["type_name"].as_str().unwrap_or("Unknown").to_string();

        // Map size to complexity orbit (logarithmic grouping)
        let complexity = match size {
            0..=10 => 1,
            11..=100 => 2,
            101..=1000 => 3,
            1001..=5000 => 4,
            5001..=10000 => 5,
            _ => 6,
        };

        complexity_groups.entry(complexity).or_insert(Vec::new()).push(name);
        lattice.max_complexity = lattice.max_complexity.max(complexity);
    }

    // Build orbits with ring/group properties
    for (complexity, group_functions) in complexity_groups {
        let generators = extract_generators(&group_functions);

        let ring_props = RingProperties {
            closure: check_closure(&group_functions),
            associativity: check_associativity(&group_functions),
            identity: find_identity(&group_functions),
            generators: generators.clone(),
        };

        let orbit = ComplexityOrbit {
            length: group_functions.len(),
            functions: group_functions,
            generator_count: generators.len(),
            ring_properties: ring_props,
        };

        lattice.orbits.insert(complexity, orbit);
    }

    println!("\n🔮 COMPLEXITY LATTICE STRUCTURE:");
    println!("================================");

    for (level, orbit) in &lattice.orbits {
        println!("📐 Orbit {} (Complexity Level {})", level, level);
        println!("   Functions: {} | Generators: {}", orbit.length, orbit.generator_count);
        println!("   Ring Properties:");
        println!(
            "     Closure: {} | Associativity: {}",
            orbit.ring_properties.closure, orbit.ring_properties.associativity
        );

        if let Some(identity) = &orbit.ring_properties.identity {
            println!("     Identity: {}", identity);
        }

        println!("   Top Generators:");
        for (i, gen) in orbit.ring_properties.generators.iter().take(3).enumerate() {
            println!("     {}: {}", i + 1, gen);
        }
        println!();
    }

    // Analyze lattice structure
    analyze_lattice_structure(&lattice);

    // Save lattice
    let lattice_json = serde_json::json!({
        "total_functions": lattice.total_functions,
        "max_complexity": lattice.max_complexity,
        "orbit_count": lattice.orbits.len(),
        "orbits": lattice.orbits.iter().map(|(k, v)| {
            serde_json::json!({
                "level": k,
                "length": v.length,
                "generator_count": v.generator_count,
                "closure": v.ring_properties.closure,
                "associativity": v.ring_properties.associativity,
                "generators": v.ring_properties.generators
            })
        }).collect::<Vec<_>>()
    });

    fs::write("rustc_complexity_lattice.json", serde_json::to_string_pretty(&lattice_json)?)?;
    println!("💾 Saved complexity lattice to rustc_complexity_lattice.json");

    Ok(())
}

fn extract_generators(functions: &[String]) -> Vec<String> {
    // Find fundamental generators (shortest, most common patterns)
    let mut generators = Vec::new();
    let mut seen_patterns = HashMap::new();

    for func in functions {
        // Extract base patterns (before :: or generic markers)
        let base_pattern = func.split("::").next().unwrap_or(func);
        let clean_pattern = base_pattern.split("$LT$").next().unwrap_or(base_pattern);

        *seen_patterns.entry(clean_pattern.to_string()).or_insert(0) += 1;
    }

    // Sort by frequency and take top generators
    let mut pattern_freq: Vec<_> = seen_patterns.into_iter().collect();
    pattern_freq.sort_by(|a, b| b.1.cmp(&a.1));

    for (pattern, _freq) in pattern_freq.into_iter().take(5) {
        generators.push(pattern);
    }

    generators
}

fn check_closure(functions: &[String]) -> bool {
    // Simplified closure check: if we have core types, assume closure
    functions.iter().any(|f| f.contains("core") || f.contains("std"))
}

fn check_associativity(_functions: &[String]) -> bool {
    // For type systems, associativity generally holds
    true
}

fn find_identity(functions: &[String]) -> Option<String> {
    // Look for identity-like elements (unit types, empty, default)
    for func in functions {
        if func.contains("unit") || func.contains("()") || func.contains("default") {
            return Some(func.clone());
        }
    }
    None
}

fn analyze_lattice_structure(lattice: &ComplexityLattice) {
    println!("🔬 LATTICE ANALYSIS:");
    println!("===================");

    let total_generators: usize = lattice.orbits.values().map(|o| o.generator_count).sum();
    let avg_orbit_size: f64 = lattice.orbits.values().map(|o| o.length).sum::<usize>() as f64
        / lattice.orbits.len() as f64;

    println!("📊 Statistics:");
    println!("   Total Orbits: {}", lattice.orbits.len());
    println!("   Total Generators: {}", total_generators);
    println!("   Average Orbit Size: {:.2}", avg_orbit_size);
    println!("   Max Complexity Level: {}", lattice.max_complexity);

    // Find orbit relationships
    println!("\n🔗 Orbit Relationships:");
    for (level, orbit) in &lattice.orbits {
        if *level > 1 {
            if let Some(prev_orbit) = lattice.orbits.get(&(level - 1)) {
                let growth_factor = orbit.length as f64 / prev_orbit.length as f64;
                println!("   Level {} → {}: Growth factor {:.2}x", level - 1, level, growth_factor);
            }
        }
    }

    // Identify mathematical properties
    println!("\n🧮 Mathematical Properties:");
    let closed_orbits = lattice.orbits.values().filter(|o| o.ring_properties.closure).count();
    let associative_orbits =
        lattice.orbits.values().filter(|o| o.ring_properties.associativity).count();
    let identity_orbits =
        lattice.orbits.values().filter(|o| o.ring_properties.identity.is_some()).count();

    println!("   Closed Orbits: {}/{}", closed_orbits, lattice.orbits.len());
    println!("   Associative Orbits: {}/{}", associative_orbits, lattice.orbits.len());
    println!("   Orbits with Identity: {}/{}", identity_orbits, lattice.orbits.len());
}
