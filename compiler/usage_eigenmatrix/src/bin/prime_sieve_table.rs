#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_sieve_structure() {
        let sieve = PrimeSieveStructure::from_encoding(0b00000011); // 2 and 3
        let primes = sieve.active_primes();
        assert_eq!(primes, vec![2, 3]);
        assert_eq!(sieve.complexity(), 2);
    }

    #[test]
    fn test_empty_sieve() {
        let sieve = PrimeSieveStructure::from_encoding(0);
        assert!(sieve.active_primes().is_empty());
        assert_eq!(sieve.complexity(), 0);
    }

    #[test]
    fn test_full_sieve() {
        let sieve = PrimeSieveStructure::from_encoding(255); // All bits set
        let primes = sieve.active_primes();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(sieve.complexity(), 8);
    }
}

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
    
    for (_i, sieve) in all_sieves.iter().enumerate().take(32) {
        let binary = format!("{:08b}", sieve.encoding);
        let primes = sieve.active_primes();
        let primes_str = if primes.is_empty() {
            "∅".to_string()
        } else {
            primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",")
        };
        
        println!("{:8} | {} | {:19} | {:10}", 
                 sieve.encoding, binary, primes_str, sieve.complexity());
    }
    
    println!("\n... (showing first 32 of 256 total combinations)");
    println!("\n🎯 Prime Sieve Analysis:");
    println!("• Total combinations: 256 (2^8)");
    println!("• Empty set (∅): 1 combination");
    println!("• Single primes: 8 combinations");
    println!("• All primes: 1 combination");
    
    let complexity_distribution: std::collections::HashMap<u32, usize> = 
        all_sieves.iter()
            .map(|s| s.complexity())
            .fold(std::collections::HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });
    
    println!("\n📊 Complexity Distribution:");
    for complexity in 0..=8 {
        let count = complexity_distribution.get(&complexity).unwrap_or(&0);
        println!("  Complexity {}: {} combinations", complexity, count);
    }
}
