// prime_sieve_table.rs - Print the prime sieve table

#[derive(Clone)]
struct PrimeSieveStructure {
    encoding: u8,
}

impl PrimeSieveStructure {
    fn from_encoding(encoding: u8) -> Self {
        Self { encoding }
    }
    
    fn active_primes(&self) -> Vec<u32> {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19];
        primes.iter()
            .enumerate()
            .filter(|(i, _)| (self.encoding >> i) & 1 == 1)
            .map(|(_, &prime)| prime)
            .collect()
    }
    
    fn complexity(&self) -> u32 {
        self.encoding.count_ones()
    }
}

struct PrimeSieveGenerator;

impl PrimeSieveGenerator {
    fn generate_all_combinations() -> Vec<PrimeSieveStructure> {
        (0..=255).map(PrimeSieveStructure::from_encoding).collect()
    }
}

fn main() {
    println!("🔢 Prime Sieve Structure Table (First 8 Primes: 2,3,5,7,11,13,17,19)");
    println!("═══════════════════════════════════════════════════════════════════");
    
    let all_sieves = PrimeSieveGenerator::generate_all_combinations();
    
    println!("Encoding | Binary   | Active Primes        | Complexity");
    println!("---------|----------|---------------------|----------");
    
    for (i, sieve) in all_sieves.iter().enumerate().take(32) {
        let binary = format!("{:08b}", sieve.encoding);
        let primes = sieve.active_primes();
        let primes_str = if primes.is_empty() {
            "∅".to_string()
        } else {
            primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",")
        };
        
        println!("{:8} | {} | {:19} | {:8}", 
                 sieve.encoding, binary, primes_str, sieve.complexity());
    }
    
    println!("\n... (showing first 32 of 256 total combinations)");
    
    // Show some interesting patterns
    println!("\n🎯 Interesting Patterns:");
    
    // All primes active
    let all_active = PrimeSieveStructure::from_encoding(255);
    println!("All active (255): {} → complexity {}", 
             all_active.active_primes().iter().map(|p| p.to_string()).collect::<Vec<_>>().join("+"), 
             all_active.complexity());
    
    // Powers of 2
    println!("\nPowers of 2 encodings:");
    for i in 0..8 {
        let sieve = PrimeSieveStructure::from_encoding(1 << i);
        println!("  2^{} = {}: prime {} → complexity {}", 
                 i, 1 << i, sieve.active_primes()[0], sieve.complexity());
    }
    
    // Low complexity structures
    println!("\n🔍 Lowest complexity structures:");
    let mut by_complexity = all_sieves.clone();
    by_complexity.sort_by_key(|s| s.complexity());
    
    for sieve in by_complexity.iter().take(10) {
        let primes_str = if sieve.active_primes().is_empty() {
            "∅".to_string()
        } else {
            sieve.active_primes().iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",")
        };
        println!("  {} → {} (complexity {})", 
                 sieve.encoding, primes_str, sieve.complexity());
    }
}
