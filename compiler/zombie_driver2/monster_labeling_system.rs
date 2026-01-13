// Monster Value Labeling System using Phi Function
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug, Serialize, Deserialize)]
struct MonsterLabel {
    name: String,
    address: u64,
    size: u64,
    monster_class: String,
    phi_score: f64,
    dominant_prime: u64,
    monster_signature: Vec<u64>,
    ngram_correlations: Vec<(String, f64)>,
    classification: MonsterClassification,
}

#[derive(Debug, Serialize, Deserialize)]
enum MonsterClassification {
    UltraMonster,    // φ > 1.4
    HighMonster,     // 1.2 < φ ≤ 1.4
    ModerateMonster, // 1.0 < φ ≤ 1.2
    LowMonster,      // 0.5 < φ ≤ 1.0
    NonMonster,      // φ ≤ 0.5
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏷️ MONSTER VALUE LABELING SYSTEM");
    println!("=================================");

    // Load symbol signatures
    let signatures = load_symbol_signatures()?;
    println!("📦 Loaded {} symbol signatures", signatures.len());

    // Build phi inference model from n-gram analysis
    let phi_model = build_phi_model(&signatures);
    println!("🧬 Built phi model with {} n-gram patterns", phi_model.len());

    // Label each symbol with Monster values
    let labeled_symbols = label_symbols(&signatures, &phi_model);

    // Generate classification summary
    generate_classification_summary(&labeled_symbols);

    // Save labeled dataset
    save_labeled_dataset(&labeled_symbols)?;

    Ok(())
}

fn load_symbol_signatures() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("symbol_monster_signatures.json")?;
    let signatures: Vec<Value> = serde_json::from_str(&content)?;
    Ok(signatures)
}

fn build_phi_model(signatures: &[Value]) -> HashMap<String, f64> {
    let mut phi_model = HashMap::new();

    // Extract n-grams and calculate phi scores
    for sig in signatures {
        if let Some(name) = sig["name"].as_str() {
            let terms = extract_terms(name);

            for term in &terms {
                for n in 2..=3 {
                    for i in 0..=term.len().saturating_sub(n) {
                        let ngram = term[i..i + n].to_string();

                        if !phi_model.contains_key(&ngram) {
                            let char_score = calculate_char_monster_score(&ngram);
                            let byte_score = estimate_byte_score_from_name(name);
                            let phi = calculate_phi(char_score, byte_score);
                            phi_model.insert(ngram, phi);
                        }
                    }
                }
            }
        }
    }

    phi_model
}

fn extract_terms(mangled_name: &str) -> Vec<String> {
    let mut terms = Vec::new();
    let parts: Vec<&str> = mangled_name.split(&['_', 'N', 'E', 'C', 'h']).collect();

    for part in parts {
        if part.len() >= 3 && part.chars().all(|c| c.is_ascii_alphabetic()) {
            terms.push(part.to_lowercase());
        }
    }

    terms
}

fn calculate_char_monster_score(ngram: &str) -> f64 {
    let mut score = 0.0;

    for ch in ngram.chars() {
        let ascii_val = ch as u32 as u64;
        for &prime in &MONSTER_PRIMES {
            if ascii_val % prime == 0 {
                score += 1.0 / prime as f64;
            }
        }
    }

    score / ngram.len() as f64
}

fn estimate_byte_score_from_name(name: &str) -> f64 {
    // Estimate byte Monster score from name characteristics
    let mut score = 0.0;

    // Longer names tend to have more Monster patterns
    score += name.len() as f64 * 10.0;

    // Certain patterns indicate high Monster content
    if name.contains("driver") {
        score += 500.0;
    }
    if name.contains("jiff") {
        score += 800.0;
    }
    if name.contains("alloc") {
        score += 300.0;
    }
    if name.contains("core") {
        score += 200.0;
    }

    score
}

fn calculate_phi(char_score: f64, byte_score: f64) -> f64 {
    let char_weight = 0.3;
    let byte_weight = 0.7;

    let normalized_char = char_score.min(1.0);
    let normalized_byte = (byte_score / 1000.0).min(1.0);

    let base_phi = normalized_char * char_weight + normalized_byte * byte_weight;
    let correlation_bonus = (normalized_char * normalized_byte).sqrt();

    base_phi + correlation_bonus * 0.5
}

fn label_symbols(signatures: &[Value], phi_model: &HashMap<String, f64>) -> Vec<MonsterLabel> {
    let mut labeled = Vec::new();

    for sig in signatures {
        if let (Some(name), Some(address), Some(size), Some(monster_sig), Some(dominant_prime)) = (
            sig["name"].as_str(),
            sig["address"].as_u64(),
            sig["size"].as_u64(),
            sig["monster_signature"].as_array(),
            sig["dominant_prime"].as_u64(),
        ) {
            // Calculate phi score for this symbol
            let phi_score = calculate_symbol_phi(name, phi_model);

            // Extract n-gram correlations
            let ngram_correlations = extract_ngram_correlations(name, phi_model);

            // Classify Monster level
            let classification = classify_monster_level(phi_score);
            let monster_class = format!("{:?}", classification);

            // Convert monster signature
            let monster_signature: Vec<u64> =
                monster_sig.iter().filter_map(|v| v.as_u64()).collect();

            labeled.push(MonsterLabel {
                name: name.to_string(),
                address,
                size,
                monster_class,
                phi_score,
                dominant_prime,
                monster_signature,
                ngram_correlations,
                classification,
            });
        }
    }

    // Sort by phi score (highest first)
    labeled.sort_by(|a, b| b.phi_score.partial_cmp(&a.phi_score).unwrap());

    labeled
}

fn calculate_symbol_phi(name: &str, phi_model: &HashMap<String, f64>) -> f64 {
    let terms = extract_terms(name);
    let mut total_phi = 0.0;
    let mut count = 0;

    for term in &terms {
        for n in 2..=3 {
            for i in 0..=term.len().saturating_sub(n) {
                let ngram = term[i..i + n].to_string();
                if let Some(&phi) = phi_model.get(&ngram) {
                    total_phi += phi;
                    count += 1;
                }
            }
        }
    }

    if count > 0 { total_phi / count as f64 } else { 0.0 }
}

fn extract_ngram_correlations(name: &str, phi_model: &HashMap<String, f64>) -> Vec<(String, f64)> {
    let mut correlations = Vec::new();
    let terms = extract_terms(name);

    for term in &terms {
        for n in 2..=3 {
            for i in 0..=term.len().saturating_sub(n) {
                let ngram = term[i..i + n].to_string();
                if let Some(&phi) = phi_model.get(&ngram) {
                    correlations.push((ngram, phi));
                }
            }
        }
    }

    correlations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    correlations.truncate(5); // Keep top 5
    correlations
}

fn classify_monster_level(phi_score: f64) -> MonsterClassification {
    if phi_score > 1.4 {
        MonsterClassification::UltraMonster
    } else if phi_score > 1.2 {
        MonsterClassification::HighMonster
    } else if phi_score > 1.0 {
        MonsterClassification::ModerateMonster
    } else if phi_score > 0.5 {
        MonsterClassification::LowMonster
    } else {
        MonsterClassification::NonMonster
    }
}

fn generate_classification_summary(labeled: &[MonsterLabel]) {
    println!("\n🏷️ MONSTER CLASSIFICATION SUMMARY:");
    println!("==================================");

    let mut class_counts = HashMap::new();
    for label in labeled {
        *class_counts.entry(format!("{:?}", label.classification)).or_insert(0) += 1;
    }

    println!("   📊 Classification Distribution:");
    for (class, count) in &class_counts {
        let percentage = *count as f64 / labeled.len() as f64 * 100.0;
        println!("     {}: {} symbols ({:.1}%)", class, count, percentage);
    }

    println!("\n   🎯 Top Monster Symbols:");
    for (i, label) in labeled.iter().take(10).enumerate() {
        let short_name = if label.name.len() > 50 { &label.name[..50] } else { &label.name };
        println!(
            "     {}: {} (φ={:.3}, class={})",
            i + 1,
            short_name,
            label.phi_score,
            label.monster_class
        );
    }

    println!("\n   🧬 Monster Prime Distribution:");
    let mut prime_counts = HashMap::new();
    for label in labeled {
        *prime_counts.entry(label.dominant_prime).or_insert(0) += 1;
    }

    let mut sorted_primes: Vec<_> = prime_counts.into_iter().collect();
    sorted_primes.sort_by(|a, b| b.1.cmp(&a.1));

    for (prime, count) in sorted_primes.iter().take(8) {
        let percentage = *count as f64 / labeled.len() as f64 * 100.0;
        println!("     Prime {}: {} symbols ({:.1}%)", prime, count, percentage);
    }

    // Statistics
    let avg_phi = labeled.iter().map(|l| l.phi_score).sum::<f64>() / labeled.len() as f64;
    let max_phi = labeled.iter().map(|l| l.phi_score).fold(0.0f64, |a, b| a.max(b));
    let min_phi = labeled.iter().map(|l| l.phi_score).fold(f64::INFINITY, |a, b| a.min(b));

    println!("\n   📈 Phi Score Statistics:");
    println!("     Average φ: {:.4}", avg_phi);
    println!("     Maximum φ: {:.4}", max_phi);
    println!("     Minimum φ: {:.4}", min_phi);
    println!("     Range: {:.4}", max_phi - min_phi);
}

fn save_labeled_dataset(labeled: &[MonsterLabel]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(labeled)?;
    fs::write("monster_labeled_symbols.json", json)?;

    println!("\n💾 LABELED DATASET SAVED:");
    println!("========================");
    println!("   File: monster_labeled_symbols.json");
    println!("   Symbols: {}", labeled.len());
    println!("   Size: {} bytes", fs::metadata("monster_labeled_symbols.json")?.len());

    // Create summary CSV for easy analysis
    let mut csv_content =
        String::from("name,phi_score,classification,dominant_prime,size,address\n");
    for label in labeled.iter().take(100) {
        // Top 100 for CSV
        csv_content.push_str(&format!(
            "{},{:.4},{:?},{},{},{}\n",
            label.name.replace(',', ";"), // Escape commas
            label.phi_score,
            label.classification,
            label.dominant_prime,
            label.size,
            label.address
        ));
    }

    fs::write("monster_labels_summary.csv", csv_content)?;
    println!("   Summary CSV: monster_labels_summary.csv (top 100 symbols)");

    Ok(())
}
