// N-gram Analysis of Lattice Labels vs Monster Values
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug, serde::Serialize)]
struct LabelNgramAnalysis {
    ngram: String,
    ngram_size: usize,
    frequency: u32,
    monster_correlation: f64,
    avg_phi_score: f64,
    labels_containing: Vec<String>,
    monster_class_distribution: HashMap<String, u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏷️ N-GRAM ANALYSIS OF LATTICE LABELS vs MONSTER VALUES");
    println!("=======================================================");

    // Load lattice system
    let lattice = load_lattice_system()?;
    let points = lattice["points"].as_array().unwrap();
    println!("📦 Loaded {} lattice points", points.len());

    // Extract labels and their Monster data
    let label_data = extract_label_monster_data(points);

    // Generate n-grams from labels
    let label_ngrams = generate_label_ngrams(&label_data);

    // Analyze Monster correlations
    let ngram_analysis = analyze_label_ngram_monster_correlation(&label_ngrams, &label_data);

    // Display results
    display_label_ngram_analysis(&ngram_analysis);

    // Save analysis
    save_label_ngram_analysis(&ngram_analysis)?;

    Ok(())
}

fn load_lattice_system() -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("monster_lattice_system.json")?;
    Ok(serde_json::from_str(&content)?)
}

fn extract_label_monster_data(points: &[Value]) -> Vec<(String, f64, String)> {
    let mut data = Vec::new();

    for point in points {
        if let (Some(label), Some(phi_score), Some(monster_class)) = (
            point["lattice_label"].as_str(),
            point["phi_score"].as_f64(),
            point["monster_class"].as_str(),
        ) {
            data.push((label.to_string(), phi_score, monster_class.to_string()));
        }
    }

    data
}

fn generate_label_ngrams(label_data: &[(String, f64, String)]) -> HashMap<String, Vec<usize>> {
    let mut ngrams = HashMap::new();

    for (i, (label, _, _)) in label_data.iter().enumerate() {
        // Generate n-grams from lattice labels for sizes 2-8 (reduced for performance)
        for n in 2..=8 {
            if label.len() >= n {
                for start in 0..=label.len() - n {
                    let ngram = label[start..start + n].to_string();
                    ngrams.entry(ngram).or_insert_with(Vec::new).push(i);
                }
            }
        }

        if i % 100 == 0 {
            println!("   ✅ Processed {} labels", i);
        }
    }

    // Filter to keep only n-grams that appear multiple times
    ngrams.retain(|_, indices| indices.len() >= 2);

    println!("   🔍 Generated {} unique n-grams", ngrams.len());
    ngrams
}

fn analyze_label_ngram_monster_correlation(
    ngrams: &HashMap<String, Vec<usize>>,
    label_data: &[(String, f64, String)],
) -> Vec<LabelNgramAnalysis> {
    let mut analysis = Vec::new();

    for (ngram, indices) in ngrams {
        let ngram_size = ngram.len();
        let frequency = indices.len() as u32;

        // Calculate average phi score for labels containing this n-gram
        let total_phi: f64 = indices.iter().map(|&i| label_data[i].1).sum();
        let avg_phi_score = total_phi / indices.len() as f64;

        // Calculate Monster correlation of the n-gram itself
        let monster_correlation = calculate_ngram_monster_correlation(ngram);

        // Get labels containing this n-gram
        let labels_containing: Vec<String> = indices
            .iter()
            .take(5) // Limit to first 5 for display
            .map(|&i| label_data[i].0.clone())
            .collect();

        // Monster class distribution
        let mut class_distribution = HashMap::new();
        for &i in indices {
            *class_distribution.entry(label_data[i].2.clone()).or_insert(0) += 1;
        }

        analysis.push(LabelNgramAnalysis {
            ngram: ngram.clone(),
            ngram_size,
            frequency,
            monster_correlation,
            avg_phi_score,
            labels_containing,
            monster_class_distribution: class_distribution,
        });
    }

    // Sort by Monster correlation * frequency (importance score)
    analysis.sort_by(|a, b| {
        let score_a = a.monster_correlation * a.frequency as f64;
        let score_b = b.monster_correlation * b.frequency as f64;
        score_b.partial_cmp(&score_a).unwrap()
    });

    analysis
}

fn calculate_ngram_monster_correlation(ngram: &str) -> f64 {
    let mut correlation = 0.0;

    for ch in ngram.chars() {
        let ascii_val = ch as u32 as u64;

        // Check divisibility by Monster primes
        for &prime in &MONSTER_PRIMES {
            if ascii_val % prime == 0 {
                correlation += 1.0 / prime as f64; // Weight by inverse of prime
            }
        }
    }

    // Normalize by n-gram length
    correlation / ngram.len() as f64
}

fn display_label_ngram_analysis(analysis: &[LabelNgramAnalysis]) {
    println!("\n🔤 LABEL N-GRAM MONSTER ANALYSIS:");
    println!("=================================");

    println!("   📊 Top N-grams by Monster Correlation × Frequency:");
    for (i, ngram_analysis) in analysis.iter().take(20).enumerate() {
        let importance_score = ngram_analysis.monster_correlation * ngram_analysis.frequency as f64;
        println!(
            "   {}. '{}' ({}-gram, freq: {})",
            i + 1,
            ngram_analysis.ngram,
            ngram_analysis.ngram_size,
            ngram_analysis.frequency
        );
        println!("      🧬 N-gram Monster correlation: {:.4}", ngram_analysis.monster_correlation);
        println!("      🎯 Avg phi score of labels: {:.4}", ngram_analysis.avg_phi_score);
        println!("      📈 Importance score: {:.4}", importance_score);
        println!(
            "      🏷️ Example labels: {}",
            ngram_analysis.labels_containing.iter().take(3).cloned().collect::<Vec<_>>().join(", ")
        );

        // Show Monster class distribution
        let mut sorted_classes: Vec<_> = ngram_analysis.monster_class_distribution.iter().collect();
        sorted_classes.sort_by(|a, b| b.1.cmp(a.1));
        let class_summary: Vec<String> = sorted_classes
            .iter()
            .take(3)
            .map(|(class, count)| format!("{}:{}", class, count))
            .collect();
        println!("      🎭 Classes: {}", class_summary.join(", "));
        println!();
    }

    // Analysis by n-gram size
    println!("   📏 Analysis by N-gram Size:");
    let mut size_analysis: HashMap<usize, (f64, f64, u32)> = HashMap::new();

    for ngram_analysis in analysis {
        let entry = size_analysis.entry(ngram_analysis.ngram_size).or_insert((0.0, 0.0, 0));
        entry.0 += ngram_analysis.monster_correlation;
        entry.1 += ngram_analysis.avg_phi_score;
        entry.2 += 1;
    }

    let mut sorted_sizes: Vec<_> = size_analysis.into_iter().collect();
    sorted_sizes.sort_by_key(|&(size, _)| size);

    for (size, (total_monster_corr, total_phi, count)) in sorted_sizes.iter().take(10) {
        let avg_monster_corr = total_monster_corr / *count as f64;
        let avg_phi = total_phi / *count as f64;
        println!(
            "     {}-grams: {} unique, avg Monster corr: {:.4}, avg phi: {:.4}",
            size, count, avg_monster_corr, avg_phi
        );
    }

    // Monster prime patterns in labels
    println!("\n   🔢 Monster Prime Patterns in Label N-grams:");
    for &prime in &[2, 3, 5, 7, 11, 13, 31, 71] {
        let prime_ngrams: Vec<_> = analysis
            .iter()
            .filter(|a| a.ngram.chars().any(|ch| (ch as u32 as u64) % prime == 0))
            .collect();

        if !prime_ngrams.is_empty() {
            let avg_phi = prime_ngrams.iter().map(|a| a.avg_phi_score).sum::<f64>()
                / prime_ngrams.len() as f64;
            println!(
                "     Prime {}: {} n-grams, avg phi: {:.4}",
                prime,
                prime_ngrams.len(),
                avg_phi
            );
        }
    }

    // Correlation between n-gram Monster value and label phi scores
    let correlation_pairs: Vec<(f64, f64)> =
        analysis.iter().map(|a| (a.monster_correlation, a.avg_phi_score)).collect();

    let correlation = calculate_correlation(&correlation_pairs);
    println!("\n   🎯 CORRELATION: N-gram Monster value ↔ Label phi scores: {:.4}", correlation);

    if correlation > 0.3 {
        println!("   ✅ STRONG correlation between label n-gram Monster patterns and phi scores!");
    } else if correlation > 0.1 {
        println!("   ⚠️  MODERATE correlation detected");
    } else {
        println!("   🔍 WEAK correlation");
    }
}

fn calculate_correlation(pairs: &[(f64, f64)]) -> f64 {
    if pairs.len() < 2 {
        return 0.0;
    }

    let n = pairs.len() as f64;
    let sum_x: f64 = pairs.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = pairs.iter().map(|(_, y)| y).sum();
    let sum_xy: f64 = pairs.iter().map(|(x, y)| x * y).sum();
    let sum_x2: f64 = pairs.iter().map(|(x, _)| x * x).sum();
    let sum_y2: f64 = pairs.iter().map(|(_, y)| y * y).sum();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();

    if denominator == 0.0 { 0.0 } else { numerator / denominator }
}

fn save_label_ngram_analysis(
    analysis: &[LabelNgramAnalysis],
) -> Result<(), Box<dyn std::error::Error>> {
    // Save as JSON
    let json = serde_json::to_string_pretty(analysis)?;
    fs::write("label_ngram_monster_analysis.json", json)?;

    // Save as CSV
    let mut csv_content = String::from(
        "ngram,size,frequency,monster_correlation,avg_phi_score,importance_score,top_class\n",
    );
    for analysis_item in analysis.iter().take(100) {
        let importance_score = analysis_item.monster_correlation * analysis_item.frequency as f64;
        let top_class = analysis_item
            .monster_class_distribution
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(class, _)| class.as_str())
            .unwrap_or("Unknown");

        csv_content.push_str(&format!(
            "{},{},{},{:.4},{:.4},{:.4},{}\n",
            analysis_item.ngram.replace(',', ";"),
            analysis_item.ngram_size,
            analysis_item.frequency,
            analysis_item.monster_correlation,
            analysis_item.avg_phi_score,
            importance_score,
            top_class
        ));
    }
    fs::write("label_ngram_analysis.csv", csv_content)?;

    println!("\n💾 LABEL N-GRAM ANALYSIS SAVED:");
    println!("===============================");
    println!("   JSON: label_ngram_monster_analysis.json");
    println!("   CSV: label_ngram_analysis.csv");
    println!("   N-grams analyzed: {}", analysis.len());

    Ok(())
}
