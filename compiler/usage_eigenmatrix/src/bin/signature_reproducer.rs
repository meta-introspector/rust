use std::fs;

/// Signature Reproduction Engine
/// Generates code that produces the same Monster Group signature

#[derive(Debug)]
struct SignatureReproducer {
    target_signature: u128,
    prime_generators: [u8; 8],
}

impl SignatureReproducer {
    fn new(target_signature: u128) -> Self {
        Self {
            target_signature,
            prime_generators: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }
    
    fn calculate_signature(&self, code: &str) -> u128 {
        let mut signature = 1u128;
        for (i, byte) in code.bytes().enumerate() {
            let prime_idx = i % 8;
            let prime = self.prime_generators[prime_idx] as u128;
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u128);
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
        println!("🎯 Target signature: 0x{:032X}", self.target_signature);
        
        let variants = self.generate_prime_sieve_variants();
        
        for (i, variant) in variants.iter().enumerate() {
            let signature = self.calculate_signature(variant);
            println!("Variant {}: 0x{:032X}", i + 1, signature);
            
            if signature == self.target_signature {
                println!("✅ EXACT MATCH FOUND! Variant {}", i + 1);
                return Some(variant.clone());
            }
        }
        
        // Try mutations if no exact match
        println!("🧬 No exact match, trying mutations...");
        
        for (i, variant) in variants.iter().enumerate() {
            for mutation in 0..100 {
                let mutated = self.mutate_code(variant, mutation);
                let signature = self.calculate_signature(&mutated);
                
                if signature == self.target_signature {
                    println!("✅ MUTATION MATCH FOUND! Variant {} mutation {}", i + 1, mutation);
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
    println!("🔄 Signature Reproduction Engine");
    println!("================================");
    
    // Use the signature from our simple prime sieve (0x1 means no enum-to-string found)
    // Let's try to reproduce a more interesting signature
    let target_signature = 0x0000000000000001u128;
    
    let reproducer = SignatureReproducer::new(target_signature);
    
    match reproducer.find_matching_signature() {
        Some(matching_code) => {
            println!("\n🎉 SUCCESS! Found code that produces target signature:");
            println!("====================================================");
            
            // Save the matching code
            match fs::write("reproduced_prime_sieve.rs", &matching_code) {
                Ok(()) => println!("📁 Saved to: reproduced_prime_sieve.rs"),
                Err(e) => eprintln!("❌ Error saving: {}", e),
            }
            
            // Verify by compiling with Monster compiler
            println!("\n🧬 Verifying with Monster compiler...");
        }
        None => {
            println!("❌ Could not find code that produces target signature");
            println!("💡 Try different mutations or signature targets");
        }
    }
    
    println!("\n🎯 Signature reproduction attempt complete!");
}
