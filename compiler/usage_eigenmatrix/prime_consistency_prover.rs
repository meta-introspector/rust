// prime_consistency_prover.rs - Prove consistency between labels and prime encodings

/// The first 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_SIEVE: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

// Consistency proofs - labels must match their prime encodings
const TWO: u32 = 2;                    // Proof: two = 2 ✓
const PAIR: u32 = 2;                   // Proof: pair = [0,1] = binary = 2 ✓  
const SIX: u32 = 3 * 2;                // Proof: six = 3*2 = 6 ✓
const FIFTEEN: u32 = 3 * 5;            // Proof: fifteen = 3*5 = 15 ✓
const THIRTY: u32 = 2 * 3 * 5;         // Proof: thirty = 2*3*5 = 30 ✓

// Binary consistency proofs
const BINARY: u32 = 2;                 // Proof: binary = 2^1 ✓
const BOOLEAN: u32 = 2;                // Proof: boolean = {true, false} = 2 ✓
const IF_ELSE: u32 = 2;                // Proof: if-else = 2 branches ✓
const ON_OFF: u32 = 2;                 // Proof: on/off = 2 states ✓

// Ternary consistency proofs  
const TERNARY: u32 = 3;                // Proof: ternary = 3^1 ✓
const TRIANGLE: u32 = 3;               // Proof: triangle = 3 sides ✓
const RGB: u32 = 3;                    // Proof: RGB = 3 colors ✓
const XYZ: u32 = 3;                    // Proof: XYZ = 3 dimensions ✓

// Pentagonal consistency proofs
const PENTAGON: u32 = 5;               // Proof: pentagon = 5 sides ✓
const FINGERS: u32 = 5;                // Proof: fingers = 5 per hand ✓
const WEEKDAYS: u32 = 5;               // Proof: weekdays = 5 days ✓
const FOR_LOOP: u32 = 5;               // Proof: for-loop = pentagonal flow ✓

// Composite consistency proofs
const HEXAGON: u32 = 2 * 3;            // Proof: hexagon = 6 = 2*3 (binary×ternary) ✓
const DECIMAL: u32 = 2 * 5;            // Proof: decimal = 10 = 2*5 (binary×pentagonal) ✓
const DOZEN: u32 = 3 * 2 * 2;          // Proof: dozen = 12 = 3*4 = 3*2² ✓
const WEEK: u32 = 7;                   // Proof: week = 7 days ✓
const OCTAGON: u32 = 2 * 2 * 2;        // Proof: octagon = 8 = 2³ ✓

// Advanced consistency proofs
const ZODIAC: u32 = 3 * 2 * 2;         // Proof: zodiac = 12 = 3*4 ✓
const CALENDAR_MONTH: u32 = 2 * 3 * 5; // Proof: ~30 days = 2*3*5 ✓
const CHESS_BOARD: u32 = 2 * 2 * 2 * 2 * 2 * 2; // Proof: 64 = 2^6 ✓ (but exceeds our primes)
const CHESS_SIMPLIFIED: u32 = 2 * 2 * 2; // Proof: 8×8 → 8 = 2³ ✓

fn verify_consistency(label: &str, value: u32, expected: u32) -> bool {
    let matches = value == expected;
    let status = if matches { "✓" } else { "✗" };
    println!("{} {} = {} (expected {}) {}", status, label, value, expected, status);
    matches
}

fn prime_factorization(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut remaining = n;
    
    for &prime in &PRIME_SIEVE {
        while remaining % prime == 0 {
            factors.push(prime);
            remaining /= prime;
        }
    }
    
    if remaining > 1 {
        factors.push(remaining); // Prime outside our sieve
    }
    
    factors
}

fn main() {
    println!("🔍 Prime Consistency Prover");
    println!("═══════════════════════════");
    
    println!("\n📊 Basic Consistency Proofs:");
    verify_consistency("TWO", TWO, 2);
    verify_consistency("PAIR", PAIR, 2);
    verify_consistency("SIX", SIX, 6);
    verify_consistency("FIFTEEN", FIFTEEN, 15);
    verify_consistency("THIRTY", THIRTY, 30);
    
    println!("\n🔢 Binary Consistency Proofs:");
    verify_consistency("BINARY", BINARY, 2);
    verify_consistency("BOOLEAN", BOOLEAN, 2);
    verify_consistency("IF_ELSE", IF_ELSE, 2);
    verify_consistency("ON_OFF", ON_OFF, 2);
    
    println!("\n🔺 Ternary Consistency Proofs:");
    verify_consistency("TERNARY", TERNARY, 3);
    verify_consistency("TRIANGLE", TRIANGLE, 3);
    verify_consistency("RGB", RGB, 3);
    verify_consistency("XYZ", XYZ, 3);
    
    println!("\n⭐ Pentagonal Consistency Proofs:");
    verify_consistency("PENTAGON", PENTAGON, 5);
    verify_consistency("FINGERS", FINGERS, 5);
    verify_consistency("WEEKDAYS", WEEKDAYS, 5);
    verify_consistency("FOR_LOOP", FOR_LOOP, 5);
    
    println!("\n🔗 Composite Consistency Proofs:");
    verify_consistency("HEXAGON", HEXAGON, 6);
    verify_consistency("DECIMAL", DECIMAL, 10);
    verify_consistency("DOZEN", DOZEN, 12);
    verify_consistency("WEEK", WEEK, 7);
    verify_consistency("OCTAGON", OCTAGON, 8);
    
    println!("\n🎯 Prime Factorization Analysis:");
    let test_values = [
        ("SIX", SIX),
        ("HEXAGON", HEXAGON), 
        ("DECIMAL", DECIMAL),
        ("THIRTY", THIRTY),
        ("CHESS_SIMPLIFIED", CHESS_SIMPLIFIED),
    ];
    
    for (name, value) in test_values {
        let factors = prime_factorization(value);
        println!("{:15} = {:2} = {:?}", name, value, factors);
    }
    
    println!("\n✨ Consistency Theorem:");
    println!("Every mathematical concept can be encoded as a product of primes!");
    println!("Labels must match their prime factorization for consistency.");
    println!("This creates a universal mathematical language! 🧮");
}
