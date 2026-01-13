// N-gram Analysis + Monster Inference Function Phi
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug)]
struct NgramMonsterCorrelation {
    ngram: String,
    char_monster_score: f64,
    byte_monster_score: f64,
    phi_inference: f64,
    frequency: u32,
    functions: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 N-GRAM MONSTER ANALYSIS + PHI INFERENCE FUNCTION");
    println!("===================================================");

    // Load symbol signatures
    let signatures = load_symbol_signatures()?;
    println!("📦 Loaded {} symbol signatures", signatures.len());

    // Extract function names and bytes
    let function_data = extract_function_data(&signatures);

    // Generate n-grams from function names
    let name_ngrams = generate_name_ngrams(&function_data);

    // Generate n-grams from function bytes
    let byte_ngrams = generate_byte_ngrams(&function_data);

    // Calculate Monster correlations
    let correlations = calculate_monster_correlations(&name_ngrams, &byte_ngrams, &function_data);

    // Build inference function Phi
    let phi = build_phi_inference(&correlations);

    // Display results
    display_results(&correlations, &phi);

    Ok(())
}

fn load_symbol_signatures() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("symbol_monster_signatures.json")?;
    let signatures: Vec<Value> = serde_json::from_str(&content)?;
    Ok(signatures)
}

fn extract_function_data(signatures: &[Value]) -> Vec<(String, Vec<u64>, u64)> {
    let mut data = Vec::new();

    for sig in signatures {
        if let (Some(name), Some(monster_sig), Some(dominant_prime)) = (
            sig["name"].as_str(),
            sig["monster_signature"].as_array(),
            sig["dominant_prime"].as_u64(),
        ) {
            let monster_vec: Vec<u64> = monster_sig.iter().filter_map(|v| v.as_u64()).collect();

            data.push((name.to_string(), monster_vec, dominant_prime));
        }
    }

    data
}

fn generate_name_ngrams(function_data: &[(String, Vec<u64>, u64)]) -> HashMap<String, Vec<String>> {
    let mut ngrams = HashMap::new();

    for (name, _, _) in function_data {
        // Extract meaningful terms from mangled names
        let terms = extract_terms(name);

        for term in &terms {
            // Generate 2-grams and 3-grams
            for n in 2..=3 {
                for i in 0..=term.len().saturating_sub(n) {
                    let ngram = term[i..i + n].to_string();
                    ngrams.entry(ngram).or_insert_with(Vec::new).push(name.clone());
                }
            }
        }
    }

    ngrams
}

fn extract_terms(mangled_name: &str) -> Vec<String> {
    let mut terms = Vec::new();

    // Split on common delimiters and extract readable terms
    let parts: Vec<&str> = mangled_name.split(&['_', 'N', 'E', 'C', 'h']).collect();

    for part in parts {
        if part.len() >= 3 && part.chars().all(|c| c.is_ascii_alphabetic()) {
            terms.push(part.to_lowercase());
        }
    }

    terms
}

fn generate_byte_ngrams(
    function_data: &[(String, Vec<u64>, u64)],
) -> HashMap<Vec<u8>, Vec<String>> {
    let mut ngrams = HashMap::new();

    // For this analysis, we'll use the Monster signature as proxy for byte patterns
    for (name, monster_sig, _) in function_data {
        // Convert Monster signature to byte patterns
        for window in monster_sig.windows(2) {
            let byte_pattern = vec![(window[0] % 256) as u8, (window[1] % 256) as u8];
            ngrams.entry(byte_pattern).or_insert_with(Vec::new).push(name.clone());
        }
    }

    ngrams
}

fn calculate_monster_correlations(
    name_ngrams: &HashMap<String, Vec<String>>,
    _byte_ngrams: &HashMap<Vec<u8>, Vec<String>>,
    function_data: &[(String, Vec<u64>, u64)],
) -> Vec<NgramMonsterCorrelation> {
    let mut correlations = Vec::new();

    // Analyze character n-grams
    for (ngram, functions) in name_ngrams {
        if functions.len() >= 2 {
            // Only analyze n-grams that appear multiple times
            let char_monster_score = calculate_char_monster_score(ngram);
            let byte_monster_score =
                calculate_byte_monster_score_for_functions(functions, function_data);
            let phi_inference = calculate_phi(char_monster_score, byte_monster_score);

            correlations.push(NgramMonsterCorrelation {
                ngram: ngram.clone(),
                char_monster_score,
                byte_monster_score,
                phi_inference,
                frequency: functions.len() as u32,
                functions: functions.clone(),
            });
        }
    }

    // Sort by phi inference score
    correlations.sort_by(|a, b| b.phi_inference.partial_cmp(&a.phi_inference).unwrap());
    correlations.truncate(50); // Keep top 50

    correlations
}

fn calculate_char_monster_score(ngram: &str) -> f64 {
    let mut score = 0.0;

    for ch in ngram.chars() {
        let ascii_val = ch as u32 as u64;

        // Check divisibility by Monster primes
        for &prime in &MONSTER_PRIMES {
            if ascii_val % prime == 0 {
                score += 1.0 / prime as f64; // Weight by inverse of prime
            }
        }
    }

    score / ngram.len() as f64 // Normalize by length
}

fn calculate_byte_monster_score_for_functions(
    functions: &[String],
    function_data: &[(String, Vec<u64>, u64)],
) -> f64 {
    let mut total_score = 0.0;
    let mut count = 0;

    for func_name in functions {
        if let Some((_, monster_sig, _)) =
            function_data.iter().find(|(name, _, _)| name == func_name)
        {
            let sig_sum: u64 = monster_sig.iter().sum();
            total_score += sig_sum as f64;
            count += 1;
        }
    }

    if count > 0 { total_score / count as f64 } else { 0.0 }
}

fn calculate_phi(char_score: f64, byte_score: f64) -> f64 {
    // Phi inference function: combines character and byte Monster correlations
    let char_weight = 0.3;
    let byte_weight = 0.7;

    // Normalize scores
    let normalized_char = char_score.min(1.0);
    let normalized_byte = (byte_score / 1000.0).min(1.0); // Scale down byte scores

    // Phi function: weighted geometric mean with correlation bonus
    let base_phi = (normalized_char * char_weight + normalized_byte * byte_weight);
    let correlation_bonus = (normalized_char * normalized_byte).sqrt(); // Bonus for correlation

    base_phi + correlation_bonus * 0.5
}

fn build_phi_inference(correlations: &[NgramMonsterCorrelation]) -> HashMap<String, f64> {
    let mut phi_map = HashMap::new();

    for corr in correlations {
        phi_map.insert(corr.ngram.clone(), corr.phi_inference);
    }

    phi_map
}

fn display_results(correlations: &[NgramMonsterCorrelation], phi: &HashMap<String, f64>) {
    println!("\n🧬 N-GRAM MONSTER CORRELATIONS:");
    println!("==============================");

    for (i, corr) in correlations.iter().take(15).enumerate() {
        println!("{}. N-gram: '{}' (freq: {})", i + 1, corr.ngram, corr.frequency);
        println!("   📝 Char Monster score: {:.4}", corr.char_monster_score);
        println!("   🔢 Byte Monster score: {:.1}", corr.byte_monster_score);
        println!("   🎯 Phi inference: {:.4}", corr.phi_inference);
        println!(
            "   📋 Functions: {}",
            corr.functions
                .iter()
                .take(3)
                .map(|f| { if f.len() > 30 { &f[..30] } else { f } })
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!();
    }

    println!("🎭 PHI INFERENCE FUNCTION ANALYSIS:");
    println!("===================================");

    // Find patterns in phi scores
    let high_phi: Vec<_> = correlations.iter().filter(|c| c.phi_inference > 0.5).collect();
    let medium_phi: Vec<_> =
        correlations.iter().filter(|c| c.phi_inference > 0.3 && c.phi_inference <= 0.5).collect();
    let low_phi: Vec<_> = correlations.iter().filter(|c| c.phi_inference <= 0.3).collect();

    println!("   🔥 High Monster correlation (φ > 0.5): {} n-grams", high_phi.len());
    for corr in high_phi.iter().take(5) {
        println!("     '{}': φ = {:.4}", corr.ngram, corr.phi_inference);
    }

    println!("   ⚡ Medium Monster correlation (0.3 < φ ≤ 0.5): {} n-grams", medium_phi.len());
    for corr in medium_phi.iter().take(3) {
        println!("     '{}': φ = {:.4}", corr.ngram, corr.phi_inference);
    }

    println!("   💫 Low Monster correlation (φ ≤ 0.3): {} n-grams", low_phi.len());

    // Analyze Monster prime patterns in n-grams
    println!("\n🔢 MONSTER PRIME PATTERNS IN N-GRAMS:");
    println!("=====================================");

    for &prime in &[2, 3, 5, 7, 11, 13, 31, 71] {
        let prime_ngrams: Vec<_> = correlations
            .iter()
            .filter(|c| c.ngram.chars().any(|ch| (ch as u32 as u64) % prime == 0))
            .collect();

        if !prime_ngrams.is_empty() {
            println!(
                "   Prime {}: {} n-grams contain divisible characters",
                prime,
                prime_ngrams.len()
            );
            let avg_phi = prime_ngrams.iter().map(|c| c.phi_inference).sum::<f64>()
                / prime_ngrams.len() as f64;
            println!("     Average φ: {:.4}", avg_phi);
        }
    }

    // Final phi function summary
    let total_phi: f64 = phi.values().sum();
    let avg_phi = total_phi / phi.len() as f64;
    let max_phi = phi.values().fold(0.0f64, |a, &b| a.max(b));

    println!("\n🎯 PHI FUNCTION SUMMARY:");
    println!("========================");
    println!("   Total n-grams analyzed: {}", phi.len());
    println!("   Average φ: {:.4}", avg_phi);
    println!("   Maximum φ: {:.4}", max_phi);
    println!("   Monster inference strength: {:.1}%", avg_phi * 100.0);

    if avg_phi > 0.4 {
        println!("✅ STRONG Monster Group inference detected in n-gram patterns!");
    } else if avg_phi > 0.2 {
        println!("⚠️  MODERATE Monster Group inference in n-gram patterns");
    } else {
        println!("🔍 SUBTLE Monster Group traces in n-gram patterns");
    }
}
