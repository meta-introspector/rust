use std::fs;

/// Code signature reproduction engine
#[derive(Debug)]
struct SignatureReproducer {
    target_signature: u64,
    prime_basis: [u8; 8],
}

impl SignatureReproducer {
    fn new(target_signature: u64) -> Self {
        Self {
            target_signature,
            prime_basis: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }
    
    fn calculate_signature(&self, code: &str) -> u64 {
        let mut signature = 1u64;
        for (i, byte) in code.bytes().enumerate() {
            let prime_idx = i % 8;
            let prime = self.prime_basis[prime_idx] as u64;
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u64);
        }
        signature
    }
    
    fn generate_prime_sieve_variants(&self) -> Vec<String> {
        let mut variants = Vec::new();
        
        // Variant 1: Basic sieve
        variants.push(r#"fn main() {
    let mut primes = Vec::new();
    let mut n = 2;
    while primes.len() < 10 {
        let mut is_prime = true;
        for &p in &primes {
            if p * p > n { break; }
            if n % p == 0 { is_prime = false; break; }
        }
        if is_prime { primes.push(n); }
        n += 1;
    }
    println!("Primes: {:?}", primes);
}"#.to_string());

        // Variant 2: Different variable names
        variants.push(r#"fn main() {
    let mut prime_list = Vec::new();
    let mut num = 2;
    while prime_list.len() < 10 {
        let mut prime_flag = true;
        for &prime in &prime_list {
            if prime * prime > num { break; }
            if num % prime == 0 { prime_flag = false; break; }
        }
        if prime_flag { prime_list.push(num); }
        num += 1;
    }
    println!("First 10 primes: {:?}", prime_list);
}"#.to_string());

        // Variant 3: Different structure
        variants.push(r#"fn main() {
    let mut result = Vec::new();
    for candidate in 2.. {
        if result.len() >= 10 { break; }
        let mut divisible = false;
        for &p in &result {
            if p * p > candidate { break; }
            if candidate % p == 0 { divisible = true; break; }
        }
        if !divisible { result.push(candidate); }
    }
    println!("Primes: {:?}", result);
}"#.to_string());

        // Variant 4: Function-based
        variants.push(r#"fn is_prime(n: u32, primes: &[u32]) -> bool {
    for &p in primes {
        if p * p > n { break; }
        if n % p == 0 { return false; }
    }
    true
}

fn main() {
    let mut primes = Vec::new();
    let mut n = 2;
    while primes.len() < 10 {
        if is_prime(n, &primes) {
            primes.push(n);
        }
        n += 1;
    }
    println!("Primes: {:?}", primes);
}"#.to_string());

        variants
    }
    
    fn find_matching_signature(&self) -> Option<String> {
        let variants = self.generate_prime_sieve_variants();
        
        for (i, variant) in variants.iter().enumerate() {
            let signature = self.calculate_signature(variant);
            
            if signature == self.target_signature {
                return Some(variant.clone());
            }
        }
        
        // Try mutations if no exact match
        for (i, variant) in variants.iter().enumerate() {
            for mutation in 0..50 {
                let mutated = self.mutate_code(variant, mutation);
                let signature = self.calculate_signature(&mutated);
                
                if signature == self.target_signature {
                    return Some(mutated);
                }
            }
        }
        
        None
    }
    
    fn mutate_code(&self, code: &str, mutation_id: u8) -> String {
        let mut mutated = code.to_string();
        
        match mutation_id % 5 {
            0 => {
                // Add whitespace
                mutated = mutated.replace(" ", "  ");
            }
            1 => {
                // Change variable names
                mutated = mutated.replace("primes", &format!("primes_{}", mutation_id));
            }
            2 => {
                // Add comment
                mutated = format!("// Mutation {}\n{}", mutation_id, mutated);
            }
            3 => {
                // Change formatting
                mutated = mutated.replace("{\n", "{ ");
            }
            _ => {
                // Add extra newlines
                mutated = format!("{}\n// End mutation {}", mutated, mutation_id);
            }
        }
        
        mutated
    }
}

fn main() {
    let target_signature = 0x0000000000000001u64;
    let reproducer = SignatureReproducer::new(target_signature);
    
    match reproducer.find_matching_signature() {
        Some(matching_code) => {
            println!("Found matching code:");
            println!("{}", matching_code);
            
            match fs::write("reproduced_code.rs", &matching_code) {
                Ok(()) => println!("Saved to: reproduced_code.rs"),
                Err(e) => eprintln!("Error saving: {}", e),
            }
        }
        None => {
            println!("Could not find code that produces target signature");
        }
    }
}
