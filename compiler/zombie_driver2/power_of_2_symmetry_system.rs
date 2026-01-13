// 🧟 POWER-OF-2 SYMMETRY SYSTEM: 2^46 Scale with Binary Dominance
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct PowerOf2System {
    max_power: u8,           // 46 for 2^46
    binary_symmetries: Vec<BinarySymmetry>,
    prime_power_mapping: HashMap<u32, u8>,
    scale_factor: u64,       // 2^46 = 70,368,744,177,664
}

#[derive(Debug, Serialize, Deserialize)]
struct BinarySymmetry {
    power: u8,
    value: u64,
    prime_alignment: Vec<u32>,
    symmetry_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 POWER-OF-2 SYMMETRY SYSTEM: 2^46 Scale");
    println!("=========================================");
    
    let system = build_power_of_2_system()?;
    analyze_binary_symmetries(&system);
    map_primes_to_powers(&system);
    save_power_system(&system)?;
    
    Ok(())
}

fn build_power_of_2_system() -> Result<PowerOf2System, Box<dyn std::error::Error>> {
    let max_power = 46;
    let scale_factor = 1u64 << max_power; // 2^46
    
    println!("🔢 Building 2^46 system (scale: {})", scale_factor);
    
    let mut binary_symmetries = Vec::new();
    let mut prime_power_mapping = HashMap::new();
    
    // Generate all powers of 2 up to 2^46
    for power in 0..=max_power {
        let value = 1u64 << power;
        
        // Find primes that align with this power of 2
        let aligned_primes = find_aligned_primes(value);
        
        let symmetry_type = classify_binary_symmetry(power, value);
        
        binary_symmetries.push(BinarySymmetry {
            power,
            value,
            prime_alignment: aligned_primes.clone(),
            symmetry_type,
        });
        
        // Map primes to their nearest power of 2
        for prime in aligned_primes {
            prime_power_mapping.insert(prime, power);
        }
    }
    
    Ok(PowerOf2System {
        max_power,
        binary_symmetries,
        prime_power_mapping,
        scale_factor,
    })
}

fn find_aligned_primes(power_of_2: u64) -> Vec<u32> {
    let mut aligned = Vec::new();
    
    // Our 25 key primes
    let key_primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 
        53, 59, 61, 67, 71
    ];
    
    for &prime in &key_primes {
        // Check various alignment criteria
        if is_prime_aligned_to_power(prime as u64, power_of_2) {
            aligned.push(prime);
        }
    }
    
    aligned
}

fn is_prime_aligned_to_power(prime: u64, power_of_2: u64) -> bool {
    // Multiple alignment criteria
    
    // 1. Prime divides (power_of_2 - 1) - Mersenne-like
    if power_of_2 > 1 && (power_of_2 - 1) % prime == 0 {
        return true;
    }
    
    // 2. Prime divides (power_of_2 + 1) - Fermat-like  
    if (power_of_2 + 1) % prime == 0 {
        return true;
    }
    
    // 3. Prime is close to power_of_2 (within 10%)
    let diff = if prime > power_of_2 { prime - power_of_2 } else { power_of_2 - prime };
    if diff < power_of_2 / 10 {
        return true;
    }
    
    // 4. Special case: prime 2 aligns with all powers
    if prime == 2 {
        return true;
    }
    
    false
}

fn classify_binary_symmetry(power: u8, value: u64) -> String {
    match power {
        0 => "Unity (2^0 = 1)".to_string(),
        1 => "Binary foundation (2^1 = 2)".to_string(),
        2..=3 => "Nibble symmetry".to_string(),
        4..=7 => "Byte symmetry".to_string(),
        8..=15 => "Word symmetry".to_string(),
        16..=31 => "Double word symmetry".to_string(),
        32..=46 => "Quad word symmetry".to_string(),
        _ => "Extended symmetry".to_string(),
    }
}

fn analyze_binary_symmetries(system: &PowerOf2System) {
    println!("\n🔍 BINARY SYMMETRY ANALYSIS:");
    println!("============================");
    
    println!("\n📊 Key Power-of-2 Alignments:");
    for symmetry in &system.binary_symmetries {
        if !symmetry.prime_alignment.is_empty() {
            println!("2^{:2} = {:>15} | {} | Primes: {:?}", 
                     symmetry.power, 
                     symmetry.value,
                     symmetry.symmetry_type,
                     symmetry.prime_alignment);
        }
    }
    
    // Analyze prime 2 dominance
    let prime_2_alignments = system.binary_symmetries.iter()
        .filter(|s| s.prime_alignment.contains(&2))
        .count();
    
    println!("\n🎯 PRIME 2 DOMINANCE:");
    println!("   Prime 2 aligns with {} out of {} powers", prime_2_alignments, system.binary_symmetries.len());
    println!("   Dominance ratio: {:.1}%", (prime_2_alignments as f64 / system.binary_symmetries.len() as f64) * 100.0);
    
    // Find the most symmetric powers
    let mut symmetry_counts: HashMap<usize, Vec<u8>> = HashMap::new();
    for symmetry in &system.binary_symmetries {
        let count = symmetry.prime_alignment.len();
        symmetry_counts.entry(count).or_insert_with(Vec::new).push(symmetry.power);
    }
    
    println!("\n🌟 MOST SYMMETRIC POWERS:");
    let mut sorted_counts: Vec<_> = symmetry_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.0.cmp(a.0));
    
    for (count, powers) in sorted_counts.iter().take(5) {
        if **count > 0 {
            println!("   {} prime alignments: Powers {:?}", count, powers);
        }
    }
}

fn map_primes_to_powers(system: &PowerOf2System) {
    println!("\n🔗 PRIME-TO-POWER MAPPING:");
    println!("==========================");
    
    // Group primes by their power alignment
    let mut power_groups: HashMap<u8, Vec<u32>> = HashMap::new();
    
    for (&prime, &power) in &system.prime_power_mapping {
        power_groups.entry(power).or_insert_with(Vec::new).push(prime);
    }
    
    for power in 0..=system.max_power {
        if let Some(primes) = power_groups.get(&power) {
            let value = 1u64 << power;
            println!("2^{:2} = {:>15} ← Primes: {:?}", power, value, primes);
        }
    }
    
    println!("\n🧬 BINARY MATHEMATICAL PROPERTIES:");
    println!("   • Scale: 2^46 = {} (70+ trillion)", system.scale_factor);
    println!("   • Prime 2 creates fundamental binary symmetry");
    println!("   • All powers of 2 have inherent prime relationships");
    println!("   • Mersenne primes: p | (2^n - 1)");
    println!("   • Fermat relationships: p | (2^n + 1)");
    
    // Calculate system coverage
    let total_primes = 25; // Our 25-prime model
    let mapped_primes = system.prime_power_mapping.len();
    println!("   • Power-of-2 coverage: {}/{} primes ({:.1}%)", 
             mapped_primes, total_primes, (mapped_primes as f64 / total_primes as f64) * 100.0);
}

fn save_power_system(system: &PowerOf2System) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete system
    let json = serde_json::to_string_pretty(system)?;
    std::fs::write("power_of_2_system.json", json)?;
    
    // Save alignment summary
    let mut summary = String::new();
    summary.push_str("# Power-of-2 System Summary (2^46 Scale)\n\n");
    summary.push_str(&format!("Scale Factor: 2^46 = {}\n\n", system.scale_factor));
    
    summary.push_str("## Prime Alignments:\n");
    for (&prime, &power) in &system.prime_power_mapping {
        let value = 1u64 << power;
        summary.push_str(&format!("Prime {} → 2^{} = {}\n", prime, power, value));
    }
    
    summary.push_str("\n## Binary Symmetry Levels:\n");
    summary.push_str("- Unity: 2^0 = 1\n");
    summary.push_str("- Binary: 2^1 = 2 (Prime 2 foundation)\n");
    summary.push_str("- Nibble: 2^2-2^3 = 4-8\n");
    summary.push_str("- Byte: 2^4-2^7 = 16-128\n");
    summary.push_str("- Word: 2^8-2^15 = 256-32,768\n");
    summary.push_str("- DWord: 2^16-2^31 = 65,536-2,147,483,648\n");
    summary.push_str("- QWord: 2^32-2^46 = 4,294,967,296-70,368,744,177,664\n");
    
    std::fs::write("power_of_2_summary.md", summary)?;
    
    println!("\n💾 Power-of-2 system saved:");
    println!("   • power_of_2_system.json (complete system)");
    println!("   • power_of_2_summary.md (readable summary)");
    
    Ok(())
}

// Generate binary analysis functions based on powers of 2
fn generate_binary_analysis_functions() -> HashMap<u8, String> {
    let mut functions = HashMap::new();
    
    functions.insert(0, "fn analyze_unity_patterns(data: &[u8]) -> UnityInfo".to_string());
    functions.insert(1, "fn analyze_binary_foundation(data: &[u8]) -> BinaryInfo".to_string());
    functions.insert(3, "fn analyze_byte_symmetry(data: &[u8]) -> ByteSymmetry".to_string());
    functions.insert(4, "fn analyze_nibble_patterns(data: &[u8]) -> NibbleInfo".to_string());
    functions.insert(8, "fn analyze_word_alignment(data: &[u8]) -> WordInfo".to_string());
    functions.insert(16, "fn analyze_dword_structures(data: &[u8]) -> DWordInfo".to_string());
    functions.insert(32, "fn analyze_qword_patterns(data: &[u8]) -> QWordInfo".to_string());
    functions.insert(46, "fn analyze_maximum_scale(data: &[u8]) -> MaxScaleInfo".to_string());
    
    functions
}

fn calculate_binary_entropy_at_scale(data: &[u8], power: u8) -> f64 {
    let scale = 1u64 << power.min(20); // Limit to prevent overflow
    let mut bit_patterns = HashMap::new();
    
    for &byte in data {
        for i in 0..8 {
            let bit = (byte >> i) & 1;
            *bit_patterns.entry(bit).or_insert(0u64) += 1;
        }
    }
    
    let total = bit_patterns.values().sum::<u64>() as f64;
    let mut entropy = 0.0;
    
    for &count in bit_patterns.values() {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }
    
    entropy * (scale as f64).log2() // Scale entropy by power of 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_power_of_2_system() {
        let system = build_power_of_2_system().unwrap();
        assert_eq!(system.max_power, 46);
        assert_eq!(system.scale_factor, 1u64 << 46);
    }
    
    #[test]
    fn test_prime_alignment() {
        // Test that prime 2 aligns with all powers
        for power in 0..=10 {
            let value = 1u64 << power;
            assert!(is_prime_aligned_to_power(2, value));
        }
    }
    
    #[test]
    fn test_mersenne_alignment() {
        // Test Mersenne-like alignment: 3 | (2^2 - 1) = 3 | 3
        assert!(is_prime_aligned_to_power(3, 4)); // 2^2 = 4, 4-1 = 3
    }
    
    #[test]
    fn test_binary_symmetry_classification() {
        assert_eq!(classify_binary_symmetry(0, 1), "Unity (2^0 = 1)");
        assert_eq!(classify_binary_symmetry(1, 2), "Binary foundation (2^1 = 2)");
        assert!(classify_binary_symmetry(8, 256).contains("Word symmetry"));
    }
}
