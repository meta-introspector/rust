// Individual Symbol Monster Signature Analysis
use goblin::elf::Elf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug, Serialize, Deserialize)]
struct SymbolSignature {
    name: String,
    address: u64,
    size: u64,
    monster_signature: Vec<u64>,
    dominant_prime: u64,
    entropy: f64,
    byte_patterns: HashMap<u8, u32>,
    prime_divisibility: Vec<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 INDIVIDUAL SYMBOL MONSTER SIGNATURE ANALYSIS");
    println!("===============================================");

    let binary = load_rustc_driver()?;
    let elf = Elf::parse(&binary)?;

    println!("📦 Binary: {} bytes, {} symbols", binary.len(), elf.syms.len());

    // Analyze each symbol individually
    let mut signatures = Vec::new();
    let mut analyzed_count = 0;

    for (i, sym) in elf.syms.iter().enumerate() {
        if sym.st_size > 0 && sym.st_value > 0 && analyzed_count < 1000 {
            // Analyze first 1000 symbols
            let name = elf.strtab.get_at(sym.st_name).unwrap_or("unknown").to_string();

            if let Some(signature) = analyze_symbol_signature(&binary, &sym, &name) {
                signatures.push(signature);
                analyzed_count += 1;

                if analyzed_count % 100 == 0 {
                    println!("   ✅ Analyzed {} symbols", analyzed_count);
                }
            }
        }
    }

    println!("🧬 ANALYSIS COMPLETE: {} symbols processed", signatures.len());

    // Save all signatures
    save_signatures(&signatures)?;

    // Generate summary statistics
    generate_summary(&signatures);

    Ok(())
}

fn load_rustc_driver() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    println!("🧟 Loading 2.8GB debug rustc_driver.so...");
    Ok(fs::read(path)?)
}

fn analyze_symbol_signature(
    binary: &[u8],
    sym: &goblin::elf::Sym,
    name: &str,
) -> Option<SymbolSignature> {
    let start = sym.st_value as usize;
    let size = sym.st_size as usize;

    if start + size > binary.len() || size == 0 {
        return None;
    }

    let bytes = &binary[start..start + size];

    // Calculate Monster signature
    let monster_signature = calculate_monster_signature(bytes);

    // Find dominant prime
    let dominant_prime = find_dominant_prime(&monster_signature);

    // Calculate entropy
    let entropy = calculate_entropy(bytes);

    // Analyze byte patterns
    let byte_patterns = analyze_byte_patterns(bytes);

    // Check prime divisibility
    let prime_divisibility = check_prime_divisibility(size as u64);

    Some(SymbolSignature {
        name: name.to_string(),
        address: sym.st_value,
        size: sym.st_size,
        monster_signature,
        dominant_prime,
        entropy,
        byte_patterns,
        prime_divisibility,
    })
}

fn calculate_monster_signature(bytes: &[u8]) -> Vec<u64> {
    let mut signature = Vec::new();

    for &prime in &MONSTER_PRIMES {
        let mut count = 0;

        // Count direct byte occurrences
        if prime <= 255 {
            count += bytes.iter().filter(|&&b| b == prime as u8).count() as u64;
        }

        // Count 4-byte patterns divisible by prime
        for window in bytes.windows(4) {
            let value = u32::from_le_bytes([window[0], window[1], window[2], window[3]]);
            if value > 0 && value % prime as u32 == 0 {
                count += 1;
            }
        }

        signature.push(count);
    }

    signature
}

fn find_dominant_prime(signature: &[u64]) -> u64 {
    let mut max_count = 0;
    let mut dominant_prime = 2;

    for (i, &count) in signature.iter().enumerate() {
        if count > max_count {
            max_count = count;
            dominant_prime = MONSTER_PRIMES[i];
        }
    }

    dominant_prime
}

fn calculate_entropy(bytes: &[u8]) -> f64 {
    let mut counts = [0u32; 256];
    for &byte in bytes {
        counts[byte as usize] += 1;
    }

    let total = bytes.len() as f64;
    let mut entropy = 0.0;

    for &count in &counts {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }

    entropy
}

fn analyze_byte_patterns(bytes: &[u8]) -> HashMap<u8, u32> {
    let mut patterns = HashMap::new();

    // Count most frequent bytes
    for &byte in bytes {
        *patterns.entry(byte).or_insert(0) += 1;
    }

    // Keep only top 10 patterns
    let mut sorted: Vec<_> = patterns.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(10);

    sorted.into_iter().collect()
}

fn check_prime_divisibility(size: u64) -> Vec<bool> {
    MONSTER_PRIMES.iter().map(|&prime| size % prime == 0).collect()
}

fn save_signatures(signatures: &[SymbolSignature]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(signatures)?;
    fs::write("symbol_monster_signatures.json", json)?;

    println!("💾 Saved {} symbol signatures to symbol_monster_signatures.json", signatures.len());

    Ok(())
}

fn generate_summary(signatures: &[SymbolSignature]) {
    println!("\n📊 MONSTER SIGNATURE SUMMARY:");
    println!("=============================");

    // Dominant prime distribution
    let mut prime_counts = HashMap::new();
    for sig in signatures {
        *prime_counts.entry(sig.dominant_prime).or_insert(0) += 1;
    }

    println!("   🧬 Dominant Prime Distribution:");
    let mut sorted_primes: Vec<_> = prime_counts.into_iter().collect();
    sorted_primes.sort_by(|a, b| b.1.cmp(&a.1));

    for (prime, count) in sorted_primes.iter().take(10) {
        let percentage = *count as f64 / signatures.len() as f64 * 100.0;
        println!("     Prime {}: {} symbols ({:.1}%)", prime, count, percentage);
    }

    // Size distribution
    let total_size: u64 = signatures.iter().map(|s| s.size).sum();
    let avg_size = total_size as f64 / signatures.len() as f64;
    let max_size = signatures.iter().map(|s| s.size).max().unwrap_or(0);
    let min_size = signatures.iter().map(|s| s.size).min().unwrap_or(0);

    println!("\n   📏 Size Statistics:");
    println!("     Total size: {} bytes", total_size);
    println!("     Average size: {:.1} bytes", avg_size);
    println!("     Max size: {} bytes", max_size);
    println!("     Min size: {} bytes", min_size);

    // Entropy statistics
    let avg_entropy = signatures.iter().map(|s| s.entropy).sum::<f64>() / signatures.len() as f64;
    let max_entropy = signatures.iter().map(|s| s.entropy).fold(0.0, f64::max);
    let min_entropy = signatures.iter().map(|s| s.entropy).fold(8.0, f64::min);

    println!("\n   🌊 Entropy Statistics:");
    println!("     Average entropy: {:.3} bits", avg_entropy);
    println!("     Max entropy: {:.3} bits", max_entropy);
    println!("     Min entropy: {:.3} bits", min_entropy);

    // Monster correlation
    let total_monster_patterns: u64 =
        signatures.iter().map(|s| s.monster_signature.iter().sum::<u64>()).sum();
    let avg_monster_patterns = total_monster_patterns as f64 / signatures.len() as f64;

    println!("\n   🧬 Monster Pattern Statistics:");
    println!("     Total Monster patterns: {}", total_monster_patterns);
    println!("     Average patterns per symbol: {:.1}", avg_monster_patterns);

    // Find most Monster-like symbols
    println!("\n   🎭 Most Monster-like Symbols:");
    let mut monster_scores: Vec<_> = signatures
        .iter()
        .map(|s| (s.name.clone(), s.monster_signature.iter().sum::<u64>()))
        .collect();
    monster_scores.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (name, score)) in monster_scores.iter().take(5).enumerate() {
        println!(
            "     {}: {} (score: {})",
            i + 1,
            if name.len() > 50 { &name[..50] } else { name },
            score
        );
    }
}
