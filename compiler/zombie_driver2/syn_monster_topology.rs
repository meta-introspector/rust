// Monster Group Analysis of SYN AST Topology
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

// Monster Group primes for topological analysis
const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

#[derive(Debug)]
struct SynTopology {
    node_counts: HashMap<String, u64>,
    depth_distribution: Vec<u64>,
    symmetry_groups: HashMap<u64, Vec<String>>,
    monster_correlations: HashMap<u64, f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 MONSTER GROUP ANALYSIS OF SYN AST TOPOLOGY");
    println!("==============================================");

    // Load flattened SYN data
    let syn_data = load_syn_data()?;
    println!("📦 Loaded SYN data: {} entries", syn_data.len());

    // Analyze topology with Monster Group lens
    let topology = analyze_syn_topology(&syn_data);

    // Apply Monster Group symmetry analysis
    apply_monster_symmetries(&topology);

    Ok(())
}

fn load_syn_data() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let paths =
        ["flattened_syn_data.json", "syn_serde_output.json", "rustc_ast_lmfdb_mapping.json"];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            println!("✅ Found SYN data at: {}", path);
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                return Ok(flatten_json_to_vec(json));
            }
        }
    }

    Err("❌ No SYN data found".into())
}

fn flatten_json_to_vec(value: Value) -> Vec<Value> {
    let mut result = Vec::new();
    flatten_recursive(value, &mut result);
    result
}

fn flatten_recursive(value: Value, result: &mut Vec<Value>) {
    match value {
        Value::Array(arr) => {
            for item in arr {
                flatten_recursive(item, result);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                flatten_recursive(v, result);
            }
        }
        _ => result.push(value),
    }
}

fn analyze_syn_topology(data: &[Value]) -> SynTopology {
    let mut topology = SynTopology {
        node_counts: HashMap::new(),
        depth_distribution: vec![0; 20],
        symmetry_groups: HashMap::new(),
        monster_correlations: HashMap::new(),
    };

    println!("\n🔍 TOPOLOGICAL ANALYSIS:");
    println!("------------------------");

    // Count node types
    for value in data {
        let node_type = classify_node(value);
        *topology.node_counts.entry(node_type).or_insert(0) += 1;
    }

    // Group by Monster primes
    for &prime in &MONSTER_PRIMES {
        let mut group = Vec::new();
        for (node_type, &count) in &topology.node_counts {
            if count % prime == 0 {
                group.push(node_type.clone());
            }
        }
        if !group.is_empty() {
            topology.symmetry_groups.insert(prime, group);
        }
    }

    // Display results
    println!("   Node type distribution:");
    for (node_type, count) in topology.node_counts.iter().take(10) {
        println!("     {}: {}", node_type, count);
    }

    topology
}

fn classify_node(value: &Value) -> String {
    match value {
        Value::String(s) => {
            if s.contains("Ident") {
                "Identifier".to_string()
            } else if s.contains("Type") {
                "Type".to_string()
            } else if s.contains("Expr") {
                "Expression".to_string()
            } else if s.contains("Stmt") {
                "Statement".to_string()
            } else {
                "String".to_string()
            }
        }
        Value::Number(_) => "Number".to_string(),
        Value::Bool(_) => "Boolean".to_string(),
        Value::Array(_) => "Array".to_string(),
        Value::Object(_) => "Object".to_string(),
        Value::Null => "Null".to_string(),
    }
}

fn apply_monster_symmetries(topology: &SynTopology) {
    println!("\n🎭 MONSTER GROUP SYMMETRY ANALYSIS:");
    println!("===================================");

    for &prime in &MONSTER_PRIMES {
        if let Some(group) = topology.symmetry_groups.get(&prime) {
            let symmetry_strength = group.len() as f64 / topology.node_counts.len() as f64;
            println!(
                "   Prime {}: {} symmetric nodes ({:.2}% coverage)",
                prime,
                group.len(),
                symmetry_strength * 100.0
            );

            if symmetry_strength > 0.1 {
                println!(
                    "     Strong symmetry detected! Nodes: {:?}",
                    group.iter().take(3).collect::<Vec<_>>()
                );
            }
        }
    }

    // Calculate Monster correlation
    let total_symmetries: usize = topology.symmetry_groups.values().map(|g| g.len()).sum();
    let monster_correlation =
        total_symmetries as f64 / (topology.node_counts.len() * MONSTER_PRIMES.len()) as f64;

    println!("\n🧬 MONSTER TOPOLOGY CORRELATION: {:.2}%", monster_correlation * 100.0);

    if monster_correlation > 0.3 {
        println!("✅ STRONG MONSTER GROUP TOPOLOGY DETECTED!");
    } else if monster_correlation > 0.1 {
        println!("⚠️  MODERATE MONSTER GROUP PATTERNS FOUND");
    } else {
        println!("❌ WEAK MONSTER GROUP CORRELATION");
    }

    // Identify most Monster-like structures
    println!("\n🎯 MOST MONSTER-LIKE STRUCTURES:");
    for &prime in &[31, 71, 47] {
        // Key primes from our analysis
        if let Some(group) = topology.symmetry_groups.get(&prime) {
            println!("   Prime {} group: {:?}", prime, group.iter().take(5).collect::<Vec<_>>());
        }
    }
}
