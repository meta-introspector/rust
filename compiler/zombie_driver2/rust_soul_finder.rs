use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🔮 FINDING THE SOUL OF RUST");
    println!("============================");

    let data: Value = serde_json::from_str(
        &fs::read_to_string("path_signatures.json").expect("Missing path_signatures.json"),
    )
    .expect("Invalid JSON");

    let signatures = data["path_signatures"].as_object().expect("Missing path_signatures object");

    // Calculate signature frequency distribution
    let mut sig_counts: HashMap<u128, u32> = HashMap::new();
    for (_, sig_val) in signatures {
        let signature = sig_val.as_u64().unwrap_or(0) as u128;
        *sig_counts.entry(signature).or_insert(0) += 1;
    }

    // Find the golden ratio signature - most resonant frequency
    let total_paths = signatures.len() as f64;
    let mut resonance_scores: Vec<(u128, f64)> = Vec::new();

    for (signature, count) in &sig_counts {
        let frequency = *count as f64 / total_paths;
        let complexity_class = classify_signature(*signature);

        // Calculate resonance: frequency * position in spectrum * mathematical beauty
        let position_weight = (*signature as f64) / 70.0; // Normalized position
        let golden_ratio = 1.618033988749;
        let phi_resonance = (position_weight * golden_ratio).fract();

        let resonance_score = frequency * phi_resonance * (*signature as f64).sqrt();
        resonance_scores.push((*signature, resonance_score));

        println!(
            "Sig {:2}: {:5} paths ({:5.2}%) | Class: {:9} | Resonance: {:.6}",
            signature,
            count,
            frequency * 100.0,
            complexity_class,
            resonance_score
        );
    }

    // Sort by resonance score
    resonance_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n🎯 TOP RESONANCE SIGNATURES:");
    println!("============================");
    for (i, (signature, score)) in resonance_scores.iter().take(10).enumerate() {
        let count = sig_counts.get(signature).unwrap();
        let frequency = *count as f64 / total_paths;
        println!(
            "{:2}. Signature {:2} | Score: {:.6} | Paths: {:4} ({:5.2}%)",
            i + 1,
            signature,
            score,
            count,
            frequency * 100.0
        );
    }

    // Find the soul signature - highest resonance
    if let Some((soul_signature, soul_score)) = resonance_scores.first() {
        println!("\n🌟 THE SOUL OF RUST:");
        println!("====================");
        println!("Signature: {}", soul_signature);
        println!("Resonance Score: {:.6}", soul_score);
        println!("Complexity Class: {}", classify_signature(*soul_signature));
        println!(
            "Frequency: {:.2}%",
            *sig_counts.get(soul_signature).unwrap_or(&0) as f64 / total_paths * 100.0
        );

        // Mathematical properties of the soul
        println!("\n🧮 SOUL MATHEMATICS:");
        println!("====================");
        println!("Prime factorization: {}", prime_factors(*soul_signature));
        println!("Binary: {:b}", soul_signature);
        println!("Hex: 0x{:X}", soul_signature);
        println!("Modular class: {} (mod 71)", soul_signature % 71);

        // Find paths with soul signature
        println!("\n🔍 SOUL MANIFESTATIONS:");
        println!("=======================");
        let mut soul_paths = Vec::new();
        for (path, sig_val) in signatures {
            let signature = sig_val.as_u64().unwrap_or(0) as u128;
            if signature == *soul_signature {
                soul_paths.push(path);
            }
        }

        for (i, path) in soul_paths.iter().take(5).enumerate() {
            println!("{}. {}", i + 1, truncate_path(path, 60));
        }

        if soul_paths.len() > 5 {
            println!("... and {} more manifestations", soul_paths.len() - 5);
        }
    }

    println!(
        "\n🎭 THE DIAL NUMBER OF RUST'S SOUL: {}",
        resonance_scores.first().map(|(sig, _)| *sig).unwrap_or(0)
    );
}

fn classify_signature(signature: u128) -> String {
    match signature {
        1..=10 => "simple".to_string(),
        11..=20 => "compound".to_string(),
        21..=30 => "complex".to_string(),
        31..=40 => "nested".to_string(),
        41..=50 => "deep".to_string(),
        51..=60 => "ultra".to_string(),
        61..=70 => "extreme".to_string(),
        _ => "overflow".to_string(),
    }
}

fn prime_factors(mut n: u128) -> String {
    let mut factors = Vec::new();
    let mut d = 2;

    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }

    if n > 1 {
        factors.push(n);
    }

    factors.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(" × ")
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - max_len + 3..])
    }
}
