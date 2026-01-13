// Complete Bit Pattern & N-gram Analysis with Monster Powers
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug, Serialize, Deserialize)]
struct BitPatternAnalysis {
    offset: u64,
    u8_values: Vec<u8>,
    u16_values: Vec<u16>,
    u32_values: Vec<u32>,
    u64_values: Vec<u64>,
    u8_ngrams: HashMap<String, u32>,
    u16_ngrams: HashMap<String, u32>,
    u32_ngrams: HashMap<String, u32>,
    u64_ngrams: HashMap<String, u32>,
    monster_powers: MonsterPowers,
}

#[derive(Debug, Serialize, Deserialize)]
struct MonsterPowers {
    u8_monster_scores: Vec<f64>,
    u16_monster_scores: Vec<f64>,
    u32_monster_scores: Vec<f64>,
    u64_monster_scores: Vec<f64>,
    ngram_monster_scores: HashMap<String, f64>,
    total_monster_correlation: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 COMPLETE BIT PATTERN & N-GRAM ANALYSIS WITH MONSTER POWERS");
    println!("==============================================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    println!("📦 Loaded binary: {} bytes", binary.len());

    // Analyze bit patterns in chunks
    let chunk_size = 1024; // 1KB chunks
    let mut all_analyses = Vec::new();

    for (i, chunk) in binary.chunks(chunk_size).enumerate().take(1000) {
        // First 1000 chunks = 1MB
        let offset = (i * chunk_size) as u64;
        let analysis = analyze_chunk_bit_patterns(chunk, offset);
        all_analyses.push(analysis);

        if (i + 1) % 100 == 0 {
            println!("   ✅ Analyzed {} chunks ({} KB)", i + 1, (i + 1) * chunk_size / 1024);
        }
    }

    // Aggregate and analyze patterns
    analyze_global_patterns(&all_analyses);

    // Save results
    save_bit_pattern_analysis(&all_analyses)?;

    Ok(())
}

fn analyze_chunk_bit_patterns(chunk: &[u8], offset: u64) -> BitPatternAnalysis {
    // Extract all uint types
    let u8_values: Vec<u8> = chunk.to_vec();
    let u16_values: Vec<u16> =
        chunk.chunks_exact(2).map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]])).collect();
    let u32_values: Vec<u32> = chunk
        .chunks_exact(4)
        .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        .collect();
    let u64_values: Vec<u64> = chunk
        .chunks_exact(8)
        .map(|bytes| {
            u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        })
        .collect();

    // Generate n-grams for each type
    let u8_ngrams = generate_uint_ngrams(&u8_values);
    let u16_ngrams = generate_uint_ngrams(&u16_values);
    let u32_ngrams = generate_uint_ngrams(&u32_values);
    let u64_ngrams = generate_uint_ngrams(&u64_values);

    // Calculate Monster powers
    let monster_powers = calculate_monster_powers(
        &u8_values,
        &u16_values,
        &u32_values,
        &u64_values,
        &u8_ngrams,
        &u16_ngrams,
        &u32_ngrams,
        &u64_ngrams,
    );

    BitPatternAnalysis {
        offset,
        u8_values,
        u16_values,
        u32_values,
        u64_values,
        u8_ngrams,
        u16_ngrams,
        u32_ngrams,
        u64_ngrams,
        monster_powers,
    }
}

fn generate_uint_ngrams<T: std::fmt::Display + Copy>(values: &[T]) -> HashMap<String, u32> {
    let mut ngrams = HashMap::new();

    // Generate 2-grams, 3-grams, 4-grams
    for n in 2..=4 {
        if values.len() >= n {
            for window in values.windows(n) {
                let ngram = window.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(",");
                *ngrams.entry(ngram).or_insert(0) += 1;
            }
        }
    }

    // Keep only frequent n-grams
    ngrams.retain(|_, &mut count| count > 1);
    ngrams
}

fn calculate_monster_powers(
    u8_values: &[u8],
    u16_values: &[u16],
    u32_values: &[u32],
    u64_values: &[u64],
    u8_ngrams: &HashMap<String, u32>,
    u16_ngrams: &HashMap<String, u32>,
    u32_ngrams: &HashMap<String, u32>,
    u64_ngrams: &HashMap<String, u32>,
) -> MonsterPowers {
    // Calculate Monster scores for each uint type
    let u8_monster_scores: Vec<f64> =
        u8_values.iter().map(|&val| calculate_uint_monster_score(val as u64)).collect();

    let u16_monster_scores: Vec<f64> =
        u16_values.iter().map(|&val| calculate_uint_monster_score(val as u64)).collect();

    let u32_monster_scores: Vec<f64> =
        u32_values.iter().map(|&val| calculate_uint_monster_score(val as u64)).collect();

    let u64_monster_scores: Vec<f64> =
        u64_values.iter().map(|&val| calculate_uint_monster_score(val)).collect();

    // Calculate Monster scores for n-grams
    let mut ngram_monster_scores = HashMap::new();

    for (ngram, &count) in u8_ngrams {
        let score = calculate_ngram_monster_score(ngram) * count as f64;
        ngram_monster_scores.insert(format!("u8_{}", ngram), score);
    }

    for (ngram, &count) in u16_ngrams {
        let score = calculate_ngram_monster_score(ngram) * count as f64;
        ngram_monster_scores.insert(format!("u16_{}", ngram), score);
    }

    for (ngram, &count) in u32_ngrams {
        let score = calculate_ngram_monster_score(ngram) * count as f64;
        ngram_monster_scores.insert(format!("u32_{}", ngram), score);
    }

    for (ngram, &count) in u64_ngrams {
        let score = calculate_ngram_monster_score(ngram) * count as f64;
        ngram_monster_scores.insert(format!("u64_{}", ngram), score);
    }

    // Calculate total correlation
    let total_monster_correlation = u8_monster_scores.iter().sum::<f64>()
        + u16_monster_scores.iter().sum::<f64>()
        + u32_monster_scores.iter().sum::<f64>()
        + u64_monster_scores.iter().sum::<f64>()
        + ngram_monster_scores.values().sum::<f64>();

    MonsterPowers {
        u8_monster_scores,
        u16_monster_scores,
        u32_monster_scores,
        u64_monster_scores,
        ngram_monster_scores,
        total_monster_correlation,
    }
}

fn calculate_uint_monster_score(value: u64) -> f64 {
    let mut score = 0.0;

    for &prime in &MONSTER_PRIMES {
        if value % prime == 0 {
            // Score based on how many times the prime divides the value
            let mut temp_val = value;
            let mut power = 0;
            while temp_val % prime == 0 {
                temp_val /= prime;
                power += 1;
            }
            score += power as f64 / prime as f64; // Weight by inverse of prime
        }
    }

    score
}

fn calculate_ngram_monster_score(ngram: &str) -> f64 {
    let mut score = 0.0;

    // Parse n-gram values and calculate combined Monster score
    let values: Vec<u64> = ngram.split(',').filter_map(|s| s.parse().ok()).collect();

    for value in values {
        score += calculate_uint_monster_score(value);
    }

    score / ngram.split(',').count() as f64 // Normalize by n-gram size
}

fn analyze_global_patterns(analyses: &[BitPatternAnalysis]) {
    println!("\n📊 GLOBAL BIT PATTERN ANALYSIS:");
    println!("===============================");

    // Aggregate Monster scores by type
    let total_u8_score: f64 =
        analyses.iter().map(|a| a.monster_powers.u8_monster_scores.iter().sum::<f64>()).sum();
    let total_u16_score: f64 =
        analyses.iter().map(|a| a.monster_powers.u16_monster_scores.iter().sum::<f64>()).sum();
    let total_u32_score: f64 =
        analyses.iter().map(|a| a.monster_powers.u32_monster_scores.iter().sum::<f64>()).sum();
    let total_u64_score: f64 =
        analyses.iter().map(|a| a.monster_powers.u64_monster_scores.iter().sum::<f64>()).sum();

    println!("   🧬 Monster Scores by Type:");
    println!("     u8 total: {:.2}", total_u8_score);
    println!("     u16 total: {:.2}", total_u16_score);
    println!("     u32 total: {:.2}", total_u32_score);
    println!("     u64 total: {:.2}", total_u64_score);

    let grand_total = total_u8_score + total_u16_score + total_u32_score + total_u64_score;
    println!("     Grand total: {:.2}", grand_total);

    // Most frequent n-grams across all chunks
    let mut global_u8_ngrams = HashMap::new();
    let mut global_u16_ngrams = HashMap::new();
    let mut global_u32_ngrams = HashMap::new();
    let mut global_u64_ngrams = HashMap::new();

    for analysis in analyses {
        for (ngram, &count) in &analysis.u8_ngrams {
            *global_u8_ngrams.entry(ngram.clone()).or_insert(0) += count;
        }
        for (ngram, &count) in &analysis.u16_ngrams {
            *global_u16_ngrams.entry(ngram.clone()).or_insert(0) += count;
        }
        for (ngram, &count) in &analysis.u32_ngrams {
            *global_u32_ngrams.entry(ngram.clone()).or_insert(0) += count;
        }
        for (ngram, &count) in &analysis.u64_ngrams {
            *global_u64_ngrams.entry(ngram.clone()).or_insert(0) += count;
        }
    }

    println!("\n   🔢 Most Frequent N-grams:");

    // Top u8 n-grams
    let mut sorted_u8: Vec<_> = global_u8_ngrams.into_iter().collect();
    sorted_u8.sort_by(|a, b| b.1.cmp(&a.1));
    println!("     Top u8 n-grams:");
    for (ngram, count) in sorted_u8.iter().take(5) {
        let monster_score = calculate_ngram_monster_score(ngram);
        println!("       '{}': {} occurrences (Monster: {:.3})", ngram, count, monster_score);
    }

    // Top u16 n-grams
    let mut sorted_u16: Vec<_> = global_u16_ngrams.into_iter().collect();
    sorted_u16.sort_by(|a, b| b.1.cmp(&a.1));
    println!("     Top u16 n-grams:");
    for (ngram, count) in sorted_u16.iter().take(5) {
        let monster_score = calculate_ngram_monster_score(ngram);
        println!("       '{}': {} occurrences (Monster: {:.3})", ngram, count, monster_score);
    }

    // Top u32 n-grams
    let mut sorted_u32: Vec<_> = global_u32_ngrams.into_iter().collect();
    sorted_u32.sort_by(|a, b| b.1.cmp(&a.1));
    println!("     Top u32 n-grams:");
    for (ngram, count) in sorted_u32.iter().take(5) {
        let monster_score = calculate_ngram_monster_score(ngram);
        println!("       '{}': {} occurrences (Monster: {:.3})", ngram, count, monster_score);
    }

    // Monster prime distribution analysis
    println!("\n   🎭 Monster Prime Distribution:");
    for &prime in &[2, 3, 5, 7, 11, 13, 31, 71] {
        let prime_hits =
            analyses.iter().map(|a| count_prime_hits_in_analysis(a, prime)).sum::<u32>();
        println!("     Prime {}: {} hits across all patterns", prime, prime_hits);
    }

    // Highest Monster correlation chunks
    let mut by_correlation: Vec<_> = analyses
        .iter()
        .enumerate()
        .map(|(i, a)| (i, a.monster_powers.total_monster_correlation))
        .collect();
    by_correlation.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n   🏆 Highest Monster Correlation Chunks:");
    for (i, (chunk_idx, correlation)) in by_correlation.iter().take(5).enumerate() {
        let offset = analyses[*chunk_idx].offset;
        println!(
            "     {}: Chunk {} (offset 0x{:x}) - correlation: {:.2}",
            i + 1,
            chunk_idx,
            offset,
            correlation
        );
    }
}

fn count_prime_hits_in_analysis(analysis: &BitPatternAnalysis, prime: u64) -> u32 {
    let mut hits = 0;

    // Count in u8 values
    hits += analysis.u8_values.iter().filter(|&&v| v as u64 % prime == 0).count() as u32;

    // Count in u16 values
    hits += analysis.u16_values.iter().filter(|&&v| v as u64 % prime == 0).count() as u32;

    // Count in u32 values
    hits += analysis.u32_values.iter().filter(|&&v| v as u64 % prime == 0).count() as u32;

    // Count in u64 values
    hits += analysis.u64_values.iter().filter(|&&v| v % prime == 0).count() as u32;

    hits
}

fn save_bit_pattern_analysis(
    analyses: &[BitPatternAnalysis],
) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete analysis (first 100 chunks to avoid huge files)
    let limited_analyses: Vec<_> = analyses.iter().take(100).collect();
    let json = serde_json::to_string_pretty(&limited_analyses)?;
    fs::write("complete_bit_pattern_analysis.json", json)?;

    // Save summary CSV
    let mut csv_content = String::from(
        "offset,u8_count,u16_count,u32_count,u64_count,u8_ngrams,u16_ngrams,u32_ngrams,u64_ngrams,total_monster_correlation\n",
    );
    for analysis in analyses.iter().take(1000) {
        csv_content.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{:.4}\n",
            analysis.offset,
            analysis.u8_values.len(),
            analysis.u16_values.len(),
            analysis.u32_values.len(),
            analysis.u64_values.len(),
            analysis.u8_ngrams.len(),
            analysis.u16_ngrams.len(),
            analysis.u32_ngrams.len(),
            analysis.u64_ngrams.len(),
            analysis.monster_powers.total_monster_correlation
        ));
    }
    fs::write("bit_pattern_summary.csv", csv_content)?;

    println!("\n💾 BIT PATTERN ANALYSIS SAVED:");
    println!("==============================");
    println!("   Complete analysis: complete_bit_pattern_analysis.json");
    println!("   Summary CSV: bit_pattern_summary.csv");
    println!("   Chunks analyzed: {}", analyses.len());

    let total_patterns: usize = analyses
        .iter()
        .map(|a| a.u8_ngrams.len() + a.u16_ngrams.len() + a.u32_ngrams.len() + a.u64_ngrams.len())
        .sum();

    println!("   Total n-gram patterns: {}", total_patterns);

    Ok(())
}
