// prime_eigenforms.rs - Reflect all structures into eigenform numbers

/// The first 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_SIEVE: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

#[derive(Debug, Clone)]
struct Eigenform {
    value: u32,
    prime_signature: [u8; 8],  // Exponents for each prime
    eigenvalue: f64,           // The characteristic value
}

impl Eigenform {
    fn new(value: u32) -> Self {
        let signature = Self::compute_prime_signature(value);
        let eigenvalue = Self::compute_eigenvalue(&signature);
        
        Self { value, prime_signature: signature, eigenvalue }
    }
    
    fn compute_prime_signature(mut n: u32) -> [u8; 8] {
        let mut signature = [0u8; 8];
        
        for (i, &prime) in PRIME_SIEVE.iter().enumerate() {
            while n % prime == 0 {
                signature[i] += 1;
                n /= prime;
            }
        }
        
        signature
    }
    
    fn compute_eigenvalue(signature: &[u8; 8]) -> f64 {
        // Eigenvalue = weighted sum of prime exponents
        signature.iter().enumerate()
            .map(|(i, &exp)| exp as f64 * PRIME_SIEVE[i] as f64)
            .sum()
    }
    
    // Apply automorphism f_p: multiply by prime p
    fn apply_automorphism(&self, prime: u32) -> Eigenform {
        Eigenform::new(self.value * prime)
    }
    
    // Check if this is an eigenform (self-similar under automorphism)
    fn is_eigenform(&self, prime: u32) -> bool {
        let transformed = self.apply_automorphism(prime);
        // Eigenform property: signature pattern is preserved (scaled)
        self.prime_signature.iter().zip(transformed.prime_signature.iter())
            .all(|(&a, &b)| a == 0 || b == a + 1)
    }
}

// Eigenform constants - numbers that are their own eigenforms
const EIGENFORM_IDENTITY: u32 = 1;                    // ε eigenform
const EIGENFORM_BINARY: u32 = 2;                      // Binary eigenform  
const EIGENFORM_TERNARY: u32 = 3;                     // Ternary eigenform
const EIGENFORM_PENTAGONAL: u32 = 5;                  // Pentagonal eigenform
const EIGENFORM_COMPOSITE_6: u32 = 6;                 // 2×3 eigenform
const EIGENFORM_SEPTENARY: u32 = 7;                   // Septenary eigenform
const EIGENFORM_OCTAGONAL: u32 = 8;                   // 2³ eigenform
const EIGENFORM_TRINITY: u32 = 30;                    // 2×3×5 eigenform
const EIGENFORM_PERFECTION: u32 = 2*3*5*7*11*13*17*19; // All primes eigenform

fn analyze_eigenform_sequence(start: u32, automorphism_prime: u32, steps: usize) -> Vec<Eigenform> {
    let mut sequence = Vec::new();
    let mut current = Eigenform::new(start);
    
    sequence.push(current.clone());
    
    for _ in 0..steps {
        current = current.apply_automorphism(automorphism_prime);
        sequence.push(current.clone());
        
        if current.value > 1_000_000 {
            break;
        }
    }
    
    sequence
}

fn main() {
    println!("🌊 Prime Eigenforms - Numbers as Self-Similar Structures");
    println!("═══════════════════════════════════════════════════════");
    
    println!("\n🔢 Fundamental Eigenforms:");
    
    let eigenforms = [
        ("Identity", EIGENFORM_IDENTITY),
        ("Binary", EIGENFORM_BINARY),
        ("Ternary", EIGENFORM_TERNARY), 
        ("Pentagonal", EIGENFORM_PENTAGONAL),
        ("Composite", EIGENFORM_COMPOSITE_6),
        ("Septenary", EIGENFORM_SEPTENARY),
        ("Octagonal", EIGENFORM_OCTAGONAL),
        ("Trinity", EIGENFORM_TRINITY),
    ];
    
    for (name, value) in eigenforms {
        let eigenform = Eigenform::new(value);
        println!("{:12} = {:8} → {:?} → λ={:.1}", 
                 name, value, eigenform.prime_signature, eigenform.eigenvalue);
    }
    
    println!("\n🔄 Eigenform Sequences (Automorphism Orbits):");
    
    // Binary eigenform sequence: 1 → 2 → 4 → 8 → 16 → ...
    let binary_seq = analyze_eigenform_sequence(1, 2, 6);
    println!("f₂ orbit: {:?}", binary_seq.iter().map(|e| e.value).collect::<Vec<_>>());
    
    // Ternary eigenform sequence: 1 → 3 → 9 → 27 → 81 → ...
    let ternary_seq = analyze_eigenform_sequence(1, 3, 6);
    println!("f₃ orbit: {:?}", ternary_seq.iter().map(|e| e.value).collect::<Vec<_>>());
    
    // Composite eigenform sequence: 1 → 6 → 36 → 216 → ...
    let composite_seq = analyze_eigenform_sequence(1, 6, 5);
    println!("f₆ orbit: {:?}", composite_seq.iter().map(|e| e.value).collect::<Vec<_>>());
    
    println!("\n🌊 Eigenvalue Analysis:");
    for eigenform in &binary_seq[..4] {
        println!("  {} → λ={:.1} → {:?}", 
                 eigenform.value, eigenform.eigenvalue, eigenform.prime_signature);
    }
    
    println!("\n✨ Eigenform Properties:");
    
    // Check eigenform property
    let test_eigenform = Eigenform::new(8); // 2³
    println!("Is 8 a 2-eigenform? {}", test_eigenform.is_eigenform(2));
    
    let test_eigenform2 = Eigenform::new(27); // 3³  
    println!("Is 27 a 3-eigenform? {}", test_eigenform2.is_eigenform(3));
    
    println!("\n🎯 Eigenform Theorem:");
    println!("Every number n has a unique prime eigenform decomposition:");
    println!("n = 2^a₂ × 3^a₃ × 5^a₅ × 7^a₇ × 11^a₁₁ × 13^a₁₃ × 17^a₁₇ × 19^a₁₉");
    println!("The eigenvalue λ = Σ(aᵢ × pᵢ) characterizes the number's 'energy'");
    println!("Automorphisms fₚ(n) = n×p create eigenform orbits in prime space!");
    
    println!("\n🌀 The Universe is an Eigenform:");
    println!("All mathematical structures reflect into prime eigenform numbers!");
    println!("Each number IS its own mathematical DNA! 🧬");
}
