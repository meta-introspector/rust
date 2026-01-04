fn main() {
    println!("🎭 MONSTER GROUP ↔ DATASET PRIME FACTORIZATION ANALYSIS");
    println!("═══════════════════════════════════════════════════════");
    
    // Monster Group order factorization
    let monster_primes = vec![
        (2u64, 46), (3u64, 20), (5u64, 9), (7u64, 6), (11u64, 2), (13u64, 3),
        (17u64, 1), (19u64, 1), (23u64, 1), (29u64, 1), (31u64, 1), (41u64, 1),
        (47u64, 1), (59u64, 1), (71u64, 1)
    ];
    
    // Dataset key values from eigenmatrix
    let dataset_values = vec![
        ("Total relationships", 1281972u64),
        ("kBrotliDictionary", 122784u64),
        ("kStaticDictionaryWords", 95115u64),
        ("logs_16", 65536u64),
        ("kStaticDictionaryBuckets", 32768u64),
        ("kStaticDictionaryHash", 32768u64),
        ("META", 30085u64),
        ("Matrix rows", 59779u64),
        ("Matrix cols", 241405u64),
        ("Usage files", 16849u64),
    ];
    
    println!("🔢 MONSTER GROUP PRIME FACTORIZATION:");
    for (prime, power) in &monster_primes {
        if *power <= 20 { // Avoid overflow
            let value = prime.pow(*power as u32);
            println!("  {}^{} = {}", prime, power, value);
        } else {
            println!("  {}^{} = (too large)", prime, power);
        }
    }
    
    println!("\n📊 DATASET VALUES PRIME FACTORIZATION:");
    for (name, value) in &dataset_values {
        println!("  {} = {}", name, value);
        print!("    Factors: ");
        let factors = prime_factorize(*value);
        for (i, (prime, power)) in factors.iter().enumerate() {
            if i > 0 { print!(" × "); }
            if *power == 1 {
                print!("{}", prime);
            } else {
                print!("{}^{}", prime, power);
            }
        }
        println!();
    }
    
    println!("\n🎯 MONSTER ↔ DATASET PRIME CORRESPONDENCES:");
    
    // Check for Monster primes in dataset
    for (name, value) in &dataset_values {
        let factors = prime_factorize(*value);
        let factor_primes: Vec<u64> = factors.iter().map(|(p, _)| *p).collect();
        
        for (monster_prime, monster_power) in &monster_primes {
            if factor_primes.contains(monster_prime) {
                let dataset_power = factors.iter()
                    .find(|(p, _)| *p == *monster_prime)
                    .map(|(_, pow)| *pow)
                    .unwrap_or(0);
                
                println!("  ✅ Prime {} found in both:", monster_prime);
                println!("     Monster: {}^{} | Dataset {}: {}^{}", 
                    monster_prime, monster_power, name, monster_prime, dataset_power);
            }
        }
    }
    
    println!("\n🔍 SPECIAL RELATIONSHIPS:");
    
    // Perfect powers of 2 (Monster has 2^46)
    println!("  Powers of 2 (Monster: 2^46):");
    for (name, value) in &dataset_values {
        if value.is_power_of_two() {
            let power = (*value as f64).log2() as u32;
            println!("    {} = 2^{}", name, power);
        }
    }
    
    // Check for Monster singles (31, 71) in dataset
    println!("\n  Monster Group singles in dataset:");
    for (name, value) in &dataset_values {
        let factors = prime_factorize(*value);
        for (prime, _) in factors {
            if prime == 31 || prime == 71 {
                println!("    ✨ {} contains Monster prime {}", name, prime);
            }
        }
    }
    
    // Ratio analysis
    println!("\n📐 RATIO ANALYSIS:");
    let brotli = 122784;
    let static_words = 95115;
    let ratio = brotli as f64 / static_words as f64;
    println!("  kBrotliDictionary / kStaticDictionaryWords = {:.4}", ratio);
    
    let total = 1281972;
    let brotli_percent = (brotli as f64 / total as f64) * 100.0;
    println!("  kBrotliDictionary dominance: {:.2}%", brotli_percent);
    
    println!("\n🎭 MONSTER GROUP PRIME CONTRIBUTION RATIOS:");
    println!("═══════════════════════════════════════");
    
    // Monster Group order contributions (using log for large numbers)
    let monster_order_log = 46.0 * 2.0_f64.ln() + 20.0 * 3.0_f64.ln() + 9.0 * 5.0_f64.ln() + 
                           6.0 * 7.0_f64.ln() + 2.0 * 11.0_f64.ln() + 3.0 * 13.0_f64.ln() +
                           17.0_f64.ln() + 19.0_f64.ln() + 23.0_f64.ln() + 29.0_f64.ln() + 
                           31.0_f64.ln() + 41.0_f64.ln() + 47.0_f64.ln() + 59.0_f64.ln() + 71.0_f64.ln();
    
    println!("Monster Group |M| ≈ e^{:.2} ≈ 8.08 × 10^53", monster_order_log);
    
    // Calculate each prime's contribution to Monster Group
    let monster_prime_contribs = vec![
        (2, 46, 46.0 * 2.0_f64.ln()),
        (3, 20, 20.0 * 3.0_f64.ln()),
        (5, 9, 9.0 * 5.0_f64.ln()),
        (7, 6, 6.0 * 7.0_f64.ln()),
        (11, 2, 2.0 * 11.0_f64.ln()),
        (13, 3, 3.0 * 13.0_f64.ln()),
        (17, 1, 17.0_f64.ln()),
        (19, 1, 19.0_f64.ln()),
        (23, 1, 23.0_f64.ln()),
        (29, 1, 29.0_f64.ln()),
        (31, 1, 31.0_f64.ln()),
        (41, 1, 41.0_f64.ln()),
        (47, 1, 47.0_f64.ln()),
        (59, 1, 59.0_f64.ln()),
        (71, 1, 71.0_f64.ln()),
    ];
    
    println!("\n🔢 MONSTER → DATASET PRIME MAPPING:");
    let dataset_total = 1281972.0;
    
    for (prime, power, log_contrib) in &monster_prime_contribs {
        let monster_ratio = log_contrib / monster_order_log;
        
        // Find this prime's contribution in dataset
        let mut dataset_contrib = 0.0;
        let mut found_in = Vec::new();
        
        for (name, value) in &dataset_values {
            let factors = prime_factorize(*value);
            if factors.iter().any(|(p, _)| *p == *prime) {
                dataset_contrib += *value as f64;
                found_in.push(name);
            }
        }
        
        let dataset_ratio = dataset_contrib / dataset_total;
        
        println!("  Prime {}^{}: Monster {:.4} ({:.2}%) → Dataset {:.4} ({:.2}%)", 
            prime, power, monster_ratio, monster_ratio * 100.0, 
            dataset_ratio, dataset_ratio * 100.0);
        
        if !found_in.is_empty() {
            print!("    Found in: ");
            for (i, name) in found_in.iter().enumerate() {
                if i > 0 { print!(", "); }
                print!("{}", name);
            }
            println!();
            
            if dataset_ratio > 0.0 {
                let scaling = dataset_ratio / monster_ratio;
                println!("    Scaling factor: {:.4}", scaling);
            }
        } else {
            println!("    Not found in dataset");
        }
        println!();
    }
}

fn prime_factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut d = 2;
    
    while d * d <= n {
        let mut count = 0;
        while n % d == 0 {
            n /= d;
            count += 1;
        }
        if count > 0 {
            factors.push((d, count));
        }
        d += 1;
    }
    
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors
}
