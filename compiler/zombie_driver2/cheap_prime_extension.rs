// Cheap Prime Extension Analysis
use std::collections::HashSet;

const CURRENT_MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn main() {
    println!("🔢 CHEAP PRIME EXTENSION ANALYSIS");
    println!("=================================");

    // Generate next primes cheaply
    let extended_primes = generate_cheap_primes(100);
    let new_primes: Vec<u64> = extended_primes
        .iter()
        .filter(|&&p| !CURRENT_MONSTER_PRIMES.contains(&p))
        .cloned()
        .collect();

    println!("📊 CURRENT: {} Monster primes", CURRENT_MONSTER_PRIMES.len());
    println!("➕ CHEAP ADDITIONS: {} new primes", new_primes.len());
    println!("🎯 TOTAL POSSIBLE: {} primes", CURRENT_MONSTER_PRIMES.len() + new_primes.len());

    // Show computational cost
    println!("\n💰 COMPUTATIONAL COST ANALYSIS:");
    println!("   Current Monster primes: {} operations per symbol", CURRENT_MONSTER_PRIMES.len());
    println!(
        "   With cheap extensions: {} operations per symbol",
        CURRENT_MONSTER_PRIMES.len() + new_primes.len()
    );
    println!(
        "   Cost increase: {:.1}x",
        (CURRENT_MONSTER_PRIMES.len() + new_primes.len()) as f64
            / CURRENT_MONSTER_PRIMES.len() as f64
    );

    // Show new primes
    println!("\n🆕 CHEAP PRIME EXTENSIONS:");
    for (i, &prime) in new_primes.iter().take(20).enumerate() {
        println!("   {}: {}", i + 1, prime);
    }

    // Memory cost
    let current_memory = CURRENT_MONSTER_PRIMES.len() * 8; // u64 = 8 bytes
    let extended_memory = (CURRENT_MONSTER_PRIMES.len() + new_primes.len()) * 8;
    println!("\n💾 MEMORY COST:");
    println!("   Current: {} bytes", current_memory);
    println!("   Extended: {} bytes", extended_memory);
    println!("   Additional: {} bytes", extended_memory - current_memory);

    // Optimal extension recommendation
    let optimal_count = recommend_optimal_extension(&new_primes);
    println!("\n🎯 OPTIMAL RECOMMENDATION:");
    println!("   Add {} more primes for best cost/benefit", optimal_count);
    println!("   Total would be: {} primes", CURRENT_MONSTER_PRIMES.len() + optimal_count);

    let optimal_primes: Vec<u64> = new_primes.iter().take(optimal_count).cloned().collect();
    println!("   Recommended additions: {:?}", optimal_primes);
}

fn generate_cheap_primes(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; (limit + 1) as usize];

    // Sieve of Eratosthenes - very cheap
    for i in 2..=limit {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = i * i;
            while j <= limit {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    primes
}

fn recommend_optimal_extension(new_primes: &[u64]) -> usize {
    // Cost/benefit analysis
    // - Each prime adds linear cost to analysis
    // - Diminishing returns after certain point
    // - Sweet spot around 25-50 total primes

    let current_count = CURRENT_MONSTER_PRIMES.len();

    if current_count < 25 {
        // Add up to 25 total
        (25 - current_count).min(new_primes.len())
    } else if current_count < 50 {
        // Add up to 50 total, but slower growth
        ((50 - current_count) / 2).min(new_primes.len())
    } else {
        // Very conservative additions beyond 50
        5.min(new_primes.len())
    }
}
