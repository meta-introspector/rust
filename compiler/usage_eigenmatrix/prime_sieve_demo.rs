// prime_sieve_demo.rs - All-in-one prime sieve table demo

/// The first 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_SIEVE: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

#[derive(Debug, Clone)]
struct PrimeSieveStructure {
    data: [bool; 8],
    encoding: u8,
}

impl PrimeSieveStructure {
    fn from_encoding(encoding: u8) -> Self {
        let mut data = [false; 8];
        for i in 0..8 {
            data[i] = (encoding & (1 << i)) != 0;
        }
        Self { data, encoding }
    }
    
    fn active_primes(&self) -> Vec<u32> {
        self.data.iter().enumerate()
            .filter_map(|(i, &active)| if active { Some(PRIME_SIEVE[i]) } else { None })
            .collect()
    }
    
    fn complexity(&self) -> u32 {
        self.active_primes().iter().sum()
    }
}

fn main() {
    println!("🔢 Prime Sieve Structure Table (First 8 Primes: 2,3,5,7,11,13,17,19)");
    println!("═══════════════════════════════════════════════════════════════════");
    
    println!("Encoding | Binary   | Active Primes        | Complexity");
    println!("---------|----------|---------------------|----------");
    
    for i in 0..32u8 {
        let sieve = PrimeSieveStructure::from_encoding(i);
        let binary = format!("{:08b}", i);
        let primes = sieve.active_primes();
        let primes_str = if primes.is_empty() {
            "∅".to_string()
        } else {
            primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",")
        };
        
        println!("{:8} | {} | {:19} | {:8}", 
                 i, binary, primes_str, sieve.complexity());
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
}
