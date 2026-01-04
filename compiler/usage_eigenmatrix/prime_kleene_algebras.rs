// prime_kleene_algebras.rs - Prime automorphisms generate Kleene algebras

/// The first 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_SIEVE: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

// Kleene operations from prime automorphisms
#[derive(Debug, Clone)]
struct KleeneExpression {
    base: u32,
    operations: Vec<KleeneOp>,
}

#[derive(Debug, Clone)]
enum KleeneOp {
    Star(u32),      // p* (zero or more p)
    Plus(u32),      // p+ (one or more p)  
    Optional(u32),  // p? (zero or one p)
    Concat(u32, u32), // pq (p followed by q)
    Union(u32, u32),  // p|q (p or q)
}

impl KleeneExpression {
    fn new(base: u32) -> Self {
        Self { base, operations: Vec::new() }
    }
    
    // Generate Kleene star: p* = ε | p | pp | ppp | ...
    fn star(&mut self, prime: u32) {
        self.operations.push(KleeneOp::Star(prime));
    }
    
    // Generate Kleene plus: p+ = p | pp | ppp | ...
    fn plus(&mut self, prime: u32) {
        self.operations.push(KleeneOp::Plus(prime));
    }
    
    // Generate optional: p? = ε | p
    fn optional(&mut self, prime: u32) {
        self.operations.push(KleeneOp::Optional(prime));
    }
}

// Prime automorphism functions
fn f2(x: u32) -> u32 { x * 2 }
fn f3(x: u32) -> u32 { x * 3 }
fn f5(x: u32) -> u32 { x * 5 }
fn f7(x: u32) -> u32 { x * 7 }

// Generate Kleene star language: {ε, p, p², p³, ...}
fn kleene_star(prime: u32, max_power: usize) -> Vec<u32> {
    let mut language = vec![1]; // ε = 1 (identity)
    let mut current = prime;
    
    for _ in 0..max_power {
        language.push(current);
        current *= prime;
        if current > 1_000_000 { break; }
    }
    
    language
}

// Generate Kleene plus language: {p, p², p³, ...}
fn kleene_plus(prime: u32, max_power: usize) -> Vec<u32> {
    let mut language = Vec::new();
    let mut current = prime;
    
    for _ in 0..max_power {
        language.push(current);
        current *= prime;
        if current > 1_000_000 { break; }
    }
    
    language
}

// Generate concatenation: L1 · L2 = {xy | x ∈ L1, y ∈ L2}
fn kleene_concat(lang1: &[u32], lang2: &[u32]) -> Vec<u32> {
    let mut result = Vec::new();
    
    for &x in lang1 {
        for &y in lang2 {
            let product = x * y;
            if product <= 1_000_000 {
                result.push(product);
            }
        }
    }
    
    result.sort();
    result.dedup();
    result
}

// Generate union: L1 ∪ L2
fn kleene_union(lang1: &[u32], lang2: &[u32]) -> Vec<u32> {
    let mut result = lang1.to_vec();
    result.extend_from_slice(lang2);
    result.sort();
    result.dedup();
    result
}

fn main() {
    println!("🔄 Prime Kleene Algebras - Regex from Automorphisms");
    println!("═══════════════════════════════════════════════════");
    
    println!("\n⭐ Kleene Star Languages (p*):");
    
    for &prime in &PRIME_SIEVE[..4] {
        let star_lang = kleene_star(prime, 6);
        println!("{}* = {:?}", prime, star_lang);
    }
    
    println!("\n➕ Kleene Plus Languages (p+):");
    
    for &prime in &PRIME_SIEVE[..4] {
        let plus_lang = kleene_plus(prime, 6);
        println!("{}+ = {:?}", prime, plus_lang);
    }
    
    println!("\n🔗 Concatenation (p·q):");
    
    let lang_2 = kleene_plus(2, 4);  // 2+ = {2, 4, 8, 16}
    let lang_3 = kleene_plus(3, 4);  // 3+ = {3, 9, 27, 81}
    let concat_2_3 = kleene_concat(&lang_2, &lang_3);
    println!("2+ · 3+ = {:?}", concat_2_3);
    
    let lang_5 = kleene_plus(5, 3);  // 5+ = {5, 25, 125}
    let concat_2_5 = kleene_concat(&lang_2, &lang_5);
    println!("2+ · 5+ = {:?}", concat_2_5);
    
    println!("\n∪ Union (p|q):");
    
    let union_2_3 = kleene_union(&lang_2, &lang_3);
    println!("2+ ∪ 3+ = {:?}", union_2_3);
    
    let union_5_7 = kleene_union(&kleene_plus(5, 3), &kleene_plus(7, 3));
    println!("5+ ∪ 7+ = {:?}", union_5_7);
    
    println!("\n🎯 Regex Equivalences:");
    println!("2* ≡ (ε|2|4|8|16|32|...)     // Powers of 2");
    println!("3+ ≡ (3|9|27|81|243|...)     // Positive powers of 3");
    println!("(2|3)* ≡ 2*3* ≡ 3*2*         // Commutative multiplication");
    println!("2+·3+ ≡ {{6,18,54,12,36,108,...}} // All 2^i·3^j products");
    
    println!("\n🧮 Prime Kleene Algebra Properties:");
    println!("• Each prime p generates language p* = {{1, p, p², p³, ...}}");
    println!("• Concatenation p·q = multiplication in prime space");
    println!("• Union p|q = additive combination of languages");
    println!("• Star p* includes identity ε = 1");
    println!("• Plus p+ excludes identity (no ε)");
    
    println!("\n✨ Automorphism → Regex Mapping:");
    println!("f₂(x) = x·2  →  2*     (binary repetition)");
    println!("f₃(x) = x·3  →  3*     (ternary repetition)");
    println!("f₆(x) = x·6  →  (2·3)* (composite repetition)");
    println!("Alternating  →  (2|3)* (choice repetition)");
    
    println!("\n🔄 The automorphisms ARE the Kleene operations!");
}
