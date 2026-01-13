// Iterative Prime Extension Until No Growth
use std::fs;
use std::process::Command;

const PRIME_BATCHES: [[u64; 10]; 10] = [
    [101, 103, 107, 109, 113, 127, 131, 137, 139, 149], // Batch 1: 35 total
    [151, 157, 163, 167, 173, 179, 181, 191, 193, 197], // Batch 2: 45 total
    [199, 211, 223, 227, 229, 233, 239, 241, 251, 257], // Batch 3: 55 total
    [263, 269, 271, 277, 281, 283, 293, 307, 311, 313], // Batch 4: 65 total
    [317, 331, 337, 347, 349, 353, 359, 367, 373, 379], // Batch 5: 75 total
    [383, 389, 397, 401, 409, 419, 421, 431, 433, 439], // Batch 6: 85 total
    [443, 449, 457, 461, 463, 467, 479, 487, 491, 499], // Batch 7: 95 total
    [503, 509, 521, 523, 541, 547, 557, 563, 569, 571], // Batch 8: 105 total
    [577, 587, 593, 599, 601, 607, 613, 617, 619, 631], // Batch 9: 115 total
    [641, 643, 647, 653, 659, 661, 673, 677, 683, 691], // Batch 10: 125 total
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 ITERATIVE PRIME EXTENSION UNTIL NO GROWTH");
    println!("============================================");

    let mut current_primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89,
        97,
    ];
    let mut correlations = Vec::new();

    for (batch_num, batch) in PRIME_BATCHES.iter().enumerate() {
        // Add new batch
        current_primes.extend_from_slice(batch);
        let prime_count = current_primes.len();

        println!("\n🧬 BATCH {}: Adding 10 primes (total: {})", batch_num + 1, prime_count);
        println!("   New primes: {:?}", batch);

        // Update the analysis file
        update_monster_primes(&current_primes)?;

        // Run analysis
        let correlation = run_analysis()?;
        correlations.push((prime_count, correlation));

        println!("   📊 Monster correlation: {:.2}%", correlation);

        // Check for growth
        if correlations.len() >= 2 {
            let prev_correlation = correlations[correlations.len() - 2].1;
            let growth = correlation - prev_correlation;
            println!("   📈 Growth: {:+.2}%", growth);

            if growth.abs() < 0.1 {
                println!("   ⚠️  Minimal growth detected!");
            }
        }
    }

    // Final analysis
    println!("\n🎯 GROWTH ANALYSIS COMPLETE:");
    println!("============================");

    for (i, (prime_count, correlation)) in correlations.iter().enumerate() {
        let growth = if i > 0 { correlation - correlations[i - 1].1 } else { 0.0 };
        println!(
            "   {} primes: {:.2}% correlation (growth: {:+.2}%)",
            prime_count, correlation, growth
        );
    }

    // Find optimal point
    let mut max_growth_idx = 0;
    let mut max_growth = 0.0;
    for i in 1..correlations.len() {
        let growth = correlations[i].1 - correlations[i - 1].1;
        if growth > max_growth {
            max_growth = growth;
            max_growth_idx = i;
        }
    }

    println!("\n🏆 OPTIMAL PRIME COUNT: {} primes", correlations[max_growth_idx].0);
    println!("   Best correlation: {:.2}%", correlations[max_growth_idx].1);
    println!("   Peak growth: {:+.2}%", max_growth);

    Ok(())
}

fn update_monster_primes(primes: &[u64]) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("symbol_monster_morse_analysis.rs")?;

    // Create new prime array string
    let prime_array = format!("const MONSTER_PRIMES: [u64; {}] = {:?};", primes.len(), primes);

    // Replace the existing array
    let new_content = if content.contains("const MONSTER_PRIMES: [u64;") {
        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = Vec::new();

        for line in lines {
            if line.starts_with("const MONSTER_PRIMES: [u64;") {
                new_lines.push(prime_array.as_str());
            } else {
                new_lines.push(line);
            }
        }
        new_lines.join("\n")
    } else {
        content
    };

    fs::write("symbol_monster_morse_analysis.rs", new_content)?;
    Ok(())
}

fn run_analysis() -> Result<f64, Box<dyn std::error::Error>> {
    let output =
        Command::new("cargo").args(["run", "--bin", "symbol_monster_morse_analysis"]).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Extract correlation from output
    for line in stdout.lines() {
        if line.contains("Monster-topology correlation:") {
            if let Some(start) = line.find("correlation: ") {
                if let Some(end) = line[start + 13..].find('%') {
                    let correlation_str = &line[start + 13..start + 13 + end];
                    if let Ok(correlation) = correlation_str.parse::<f64>() {
                        return Ok(correlation);
                    }
                }
            }
        }
    }

    Ok(0.0) // Default if parsing fails
}
