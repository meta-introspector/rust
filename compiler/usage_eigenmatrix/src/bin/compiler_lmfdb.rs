use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug)]
struct LMFDBEntry {
    name: String,
    conductor: u64,
    weight: u32,
    level: u64,
    score: f64,
    coords: (u64, u32, u64),
}

fn main() {
    println!("🧮 COMPILER LMFDB: Assigning weight, level, score to each eigenmatrix entry");
    
    let eigenmatrix = fs::read_to_string("usage_eigenmatrix.json")
        .expect("Failed to read eigenmatrix");
    
    let data: Value = serde_json::from_str(&eigenmatrix)
        .expect("Failed to parse eigenmatrix JSON");
    
    let mut lmfdb_entries = Vec::new();
    
    // Process each entry in the eigenmatrix
    if let Some(entries) = data.as_object() {
        for (key, value) in entries {
            let entry = create_lmfdb_entry(key, value);
            lmfdb_entries.push(entry);
        }
    }
    
    // Sort by score (highest first)
    lmfdb_entries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    
    // Output results
    println!("📊 LMFDB Classification Results:");
    println!("Total entries: {}", lmfdb_entries.len());
    
    for (i, entry) in lmfdb_entries.iter().take(20).enumerate() {
        println!("{:2}: {} → conductor={}, weight={}, level={}, score={:.2}", 
                 i+1, entry.name, entry.conductor, entry.weight, entry.level, entry.score);
    }
    
    // Generate LMFDB database file
    generate_lmfdb_database(&lmfdb_entries);
    
    println!("✅ Compiler LMFDB database generated!");
}

fn create_lmfdb_entry(name: &str, value: &Value) -> LMFDBEntry {
    let conductor = calculate_conductor(name);
    let weight = calculate_weight(name, value);
    let level = calculate_level(name);
    let score = calculate_score(conductor, weight, level, value);
    
    LMFDBEntry {
        name: name.to_string(),
        conductor,
        weight,
        level,
        score,
        coords: (conductor, weight, level),
    }
}

fn calculate_conductor(name: &str) -> u64 {
    // Hash name to get conductor (prime factorization-like)
    let hash = name.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    
    // Map to Monster Group primes
    let monster_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
    let prime_index = (hash % 15) as usize;
    monster_primes[prime_index]
}

fn calculate_weight(name: &str, value: &Value) -> u32 {
    let mut weight = 2; // Start with weight 2 (simplest)
    
    // Increase weight based on complexity indicators
    if name.contains("::") { weight += 2; }        // Namespace depth
    if name.contains("<") { weight += 2; }         // Generics
    if name.contains("impl") { weight += 4; }      // Implementation
    if name.contains("macro") { weight += 6; }     // Macros
    if name.contains("unsafe") { weight += 8; }    // Unsafe code
    
    // Factor in usage count if available
    if let Some(usage) = value.as_u64() {
        if usage > 1000 { weight += 2; }
        if usage > 10000 { weight += 4; }
    }
    
    // Ensure even weight (modular forms requirement)
    if weight % 2 == 1 { weight += 1; }
    
    std::cmp::min(weight, 24) // Cap at weight 24
}

fn calculate_level(name: &str) -> u64 {
    // Map to Monster Group exponents
    let hash = name.bytes().fold(0u64, |acc, b| acc.wrapping_mul(17).wrapping_add(b as u64));
    
    match hash % 6 {
        0 => 1,      // Level 1 (no torsion)
        1 => 46,     // Level 2^46
        2 => 20,     // Level 3^20  
        3 => 9,      // Level 5^9
        4 => 6,      // Level 7^6
        5 => 2,      // Level 11^2
        _ => 1,
    }
}

fn calculate_score(conductor: u64, weight: u32, level: u64, value: &Value) -> f64 {
    let base_score = (conductor as f64).log2() + (weight as f64) + (level as f64).log2();
    
    // Factor in usage frequency
    let usage_multiplier = if let Some(usage) = value.as_u64() {
        (usage as f64 + 1.0).log10()
    } else {
        1.0
    };
    
    base_score * usage_multiplier
}

fn generate_lmfdb_database(entries: &[LMFDBEntry]) {
    let mut database = serde_json::json!({
        "title": "Compiler L-functions and Modular Forms Database",
        "description": "LMFDB-style classification of rustc compiler functions",
        "total_entries": entries.len(),
        "weight_distribution": calculate_weight_distribution(entries),
        "level_distribution": calculate_level_distribution(entries),
        "conductor_distribution": calculate_conductor_distribution(entries),
        "entries": entries.iter().map(|e| serde_json::json!({
            "name": e.name,
            "conductor": e.conductor,
            "weight": e.weight,
            "level": e.level,
            "score": e.score,
            "coordinates": format!("({}, {}, {})", e.conductor, e.weight, e.level),
            "l_function": format!("L(s, f_{}_{})", e.conductor, e.weight),
            "modular_form": format!("f_{}(τ)", e.weight)
        })).collect::<Vec<_>>()
    });
    
    fs::write("compiler_lmfdb.json", serde_json::to_string_pretty(&database).unwrap())
        .expect("Failed to write LMFDB database");
}

fn calculate_weight_distribution(entries: &[LMFDBEntry]) -> HashMap<u32, usize> {
    let mut dist = HashMap::new();
    for entry in entries {
        *dist.entry(entry.weight).or_insert(0) += 1;
    }
    dist
}

fn calculate_level_distribution(entries: &[LMFDBEntry]) -> HashMap<u64, usize> {
    let mut dist = HashMap::new();
    for entry in entries {
        *dist.entry(entry.level).or_insert(0) += 1;
    }
    dist
}

fn calculate_conductor_distribution(entries: &[LMFDBEntry]) -> HashMap<u64, usize> {
    let mut dist = HashMap::new();
    for entry in entries {
        *dist.entry(entry.conductor).or_insert(0) += 1;
    }
    dist
}
