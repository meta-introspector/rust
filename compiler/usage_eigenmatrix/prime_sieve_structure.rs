// prime_sieve_structure.rs - Sieve of first 8 primes as inherent data structure

/// The first 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_SIEVE: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

#[derive(Debug, Clone)]
pub struct PrimeSieveStructure {
    pub data: [bool; 8],  // Each position corresponds to a prime
    pub encoding: u8,     // 8-bit encoding of the sieve state
}

impl PrimeSieveStructure {
    pub fn new() -> Self {
        Self {
            data: [false; 8],
            encoding: 0,
        }
    }
    
    pub fn from_encoding(encoding: u8) -> Self {
        let mut data = [false; 8];
        for i in 0..8 {
            data[i] = (encoding & (1 << i)) != 0;
        }
        Self { data, encoding }
    }
    
    pub fn set_prime(&mut self, prime: u32, active: bool) {
        if let Some(index) = PRIME_SIEVE.iter().position(|&p| p == prime) {
            self.data[index] = active;
            self.update_encoding();
        }
    }
    
    pub fn get_prime(&self, prime: u32) -> Option<bool> {
        PRIME_SIEVE.iter().position(|&p| p == prime)
            .map(|index| self.data[index])
    }
    
    fn update_encoding(&mut self) {
        self.encoding = 0;
        for (i, &active) in self.data.iter().enumerate() {
            if active {
                self.encoding |= 1 << i;
            }
        }
    }
    
    pub fn active_primes(&self) -> Vec<u32> {
        self.data.iter().enumerate()
            .filter_map(|(i, &active)| if active { Some(PRIME_SIEVE[i]) } else { None })
            .collect()
    }
    
    pub fn complexity(&self) -> u32 {
        self.active_primes().iter().sum()
    }
}

pub struct PrimeSieveGenerator;

impl PrimeSieveGenerator {
    pub fn generate_all_combinations() -> Vec<PrimeSieveStructure> {
        (0..=255u8).map(PrimeSieveStructure::from_encoding).collect()
    }
    
    pub fn generate_by_complexity(max_complexity: u32) -> Vec<PrimeSieveStructure> {
        Self::generate_all_combinations()
            .into_iter()
            .filter(|s| s.complexity() <= max_complexity)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prime_sieve_basic() {
        let mut sieve = PrimeSieveStructure::new();
        sieve.set_prime(2, true);
        sieve.set_prime(7, true);
        
        assert_eq!(sieve.encoding, 0b00001001); // bits 0 and 3 set
        assert_eq!(sieve.active_primes(), vec![2, 7]);
        assert_eq!(sieve.complexity(), 9);
    }
    
    #[test]
    fn test_encoding_roundtrip() {
        let original = PrimeSieveStructure::from_encoding(0b10101010);
        let reconstructed = PrimeSieveStructure::from_encoding(original.encoding);
        assert_eq!(original.data, reconstructed.data);
    }
}
