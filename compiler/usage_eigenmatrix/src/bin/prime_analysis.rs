use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔢 Prime Number Analysis in Rust Constants");
    
    let numbers = extract_numbers()?;
    let primes = find_primes(&numbers);
    let factors = analyze_prime_factors(&numbers, &primes);
    let powers_of_two = find_powers_of_two(&numbers);
    
    println!("\n🎯 Prime Numbers Found ({} total):", primes.len());
    for (prime, count) in primes.iter().take(15) {
        println!("  {} - appears {} times", prime, count);
    }
    
    println!("\n🧮 Most Common Prime Factors:");
    for (prime, factor_count) in factors.iter().take(10) {
        let percentage = (*factor_count as f64 / numbers.len() as f64) * 100.0;
        println!("  {} divides {} numbers ({:.1}%)", prime, factor_count, percentage);
    }
    
    println!("\n⚡ Powers of 2 (Computer Science Favorites):");
    for (power, base, count) in powers_of_two.iter().take(10) {
        println!("  2^{} = {} - appears {} times", power, base, count);
    }
    
    println!("\n📊 Mathematical Insights:");
    analyze_mathematical_patterns(&numbers, &primes);
    
    Ok(())
}

fn extract_numbers() -> Result<Vec<u64>> {
    let mut numbers = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let def_id = usage_obj["used_def_id"].as_str().unwrap_or("");
                    
                    if let Ok(num) = def_id.parse::<u64>() {
                        if num > 1 && num < 1_000_000 {
                            numbers.push(num);
                        }
                    }
                }
            }
        }
    }
    
    numbers.sort();
    numbers.dedup();
    Ok(numbers)
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn find_primes(numbers: &[u64]) -> Vec<(u64, usize)> {
    let mut prime_counts = HashMap::new();
    
    for &num in numbers {
        if is_prime(num) {
            *prime_counts.entry(num).or_insert(0) += 1;
        }
    }
    
    let mut sorted: Vec<_> = prime_counts.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by prime value
    sorted
}

fn analyze_prime_factors(numbers: &[u64], primes: &[(u64, usize)]) -> Vec<(u64, usize)> {
    let mut factor_counts = HashMap::new();
    
    for &(prime, _) in primes {
        let mut count = 0;
        for &num in numbers {
            if num != prime && num % prime == 0 {
                count += 1;
            }
        }
        if count > 0 {
            factor_counts.insert(prime, count);
        }
    }
    
    let mut sorted: Vec<_> = factor_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn find_powers_of_two(numbers: &[u64]) -> Vec<(u32, u64, usize)> {
    let mut powers = Vec::new();
    
    for power in 1..=20 {
        let value = 2u64.pow(power);
        if value > 1_000_000 { break; }
        
        let count = numbers.iter().filter(|&&n| n == value).count();
        if count > 0 {
            powers.push((power, value, count));
        }
    }
    
    powers.sort_by(|a, b| b.2.cmp(&a.2));
    powers
}

fn analyze_mathematical_patterns(numbers: &[u64], primes: &[(u64, usize)]) {
    let total_numbers = numbers.len();
    let prime_count = primes.len();
    let composite_count = total_numbers - prime_count;
    
    println!("  Total unique numbers: {}", total_numbers);
    println!("  Prime numbers: {} ({:.1}%)", prime_count, (prime_count as f64 / total_numbers as f64) * 100.0);
    println!("  Composite numbers: {} ({:.1}%)", composite_count, (composite_count as f64 / total_numbers as f64) * 100.0);
    
    // Check for specific interesting primes
    let interesting_primes = vec![31, 71];
    for &prime in &interesting_primes {
        if numbers.contains(&prime) {
            println!("  ⭐ Found interesting prime: {}", prime);
        }
    }
    
    // Find interesting number patterns
    let fibonacci_in_data: Vec<u64> = vec![2, 3, 5, 8, 13, 21, 31, 34, 55, 71, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765]
        .into_iter()
        .filter(|&fib| numbers.contains(&fib))
        .collect();
    
    if !fibonacci_in_data.is_empty() {
        println!("  Fibonacci-like numbers found: {:?}", fibonacci_in_data);
    }
    
    let perfect_squares: Vec<u64> = (1..=1000)
        .map(|i| i * i)
        .filter(|&sq| numbers.contains(&sq))
        .collect();
    
    if !perfect_squares.is_empty() {
        println!("  Perfect squares found: {} numbers", perfect_squares.len());
    }
    
    // Mersenne primes (2^n - 1)
    let mersenne_candidates = vec![3, 7, 31, 127, 8191];
    let mersenne_found: Vec<u64> = mersenne_candidates
        .into_iter()
        .filter(|&m| numbers.contains(&m))
        .collect();
    
    if !mersenne_found.is_empty() {
        println!("  Mersenne primes found: {:?}", mersenne_found);
    }
}
