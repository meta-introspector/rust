use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct MergedAnalysis {
    total_files: usize,
    total_chars: u64,
    combined_char_frequencies: HashMap<char, u64>,
    combined_essential_arrows: Vec<EssentialArrow>,
    file_analyses: HashMap<String, FileAnalysis>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileAnalysis {
    char_count: u64,
    unique_chars: usize,
    essential_arrows: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EssentialArrow {
    pair: (char, char),
    strength: f64,
    must_preserve: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = std::env::args().nth(1).unwrap_or_else(|| "target/debug".to_string());

    println!("🔗 Merging Zombie Rustc Analysis Files");
    println!("=====================================");
    println!("📁 Scanning directory: {}", target_dir);

    let mut merged = MergedAnalysis {
        total_files: 0,
        total_chars: 0,
        combined_char_frequencies: HashMap::new(),
        combined_essential_arrows: Vec::new(),
        file_analyses: HashMap::new(),
    };

    // Find all .zombie_analysis.json files
    let analysis_files = find_analysis_files(&target_dir)?;
    println!("📊 Found {} analysis files", analysis_files.len());

    for file_path in analysis_files {
        println!("🔍 Processing: {}", file_path);
        merge_analysis_file(&file_path, &mut merged)?;
    }

    // Sort arrows by strength
    merged.combined_essential_arrows.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());

    // Save merged analysis
    let output_path = format!("{}/merged_rustc_analysis.json", target_dir);
    let json = serde_json::to_string_pretty(&merged)?;
    fs::write(&output_path, json)?;

    println!("\n🎯 Merge Complete!");
    println!("   Total files: {}", merged.total_files);
    println!("   Total characters: {}", merged.total_chars);
    println!("   Unique character types: {}", merged.combined_char_frequencies.len());
    println!("   Total essential arrows: {}", merged.combined_essential_arrows.len());
    println!("   Output: {}", output_path);

    // Show top character frequencies
    let mut sorted_chars: Vec<_> = merged.combined_char_frequencies.iter().collect();
    sorted_chars.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n🔝 Top 10 Characters in Rustc:");
    for (i, (&ch, &freq)) in sorted_chars.iter().take(10).enumerate() {
        let display = match ch {
            ' ' => "SPC".to_string(),
            '\n' => "\\n".to_string(),
            '\t' => "\\t".to_string(),
            c => c.to_string(),
        };
        println!("   {}. '{}' → {} occurrences", i + 1, display, freq);
    }

    // Show top essential arrows
    println!("\n🎯 Top 10 Essential Arrows in Rustc:");
    for (i, arrow) in merged.combined_essential_arrows.iter().take(10).enumerate() {
        let from = match arrow.pair.0 {
            ' ' => "SPC".to_string(),
            '\n' => "\\n".to_string(),
            c => c.to_string(),
        };
        let to = match arrow.pair.1 {
            ' ' => "SPC".to_string(),
            '\n' => "\\n".to_string(),
            c => c.to_string(),
        };
        println!("   {}. {} → {} (strength: {:.3})", i + 1, from, to, arrow.strength);
    }

    Ok(())
}

fn find_analysis_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    find_analysis_files_recursive(dir, &mut files)?;
    Ok(files)
}

fn find_analysis_files_recursive(
    dir: &str,
    files: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively search subdirectories
            if let Some(dir_str) = path.to_str() {
                find_analysis_files_recursive(dir_str, files)?;
            }
        } else if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.ends_with(".zombie_analysis.json") {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}

fn merge_analysis_file(
    file_path: &str,
    merged: &mut MergedAnalysis,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let analysis: serde_json::Value = serde_json::from_str(&content)?;

    // Extract file name for tracking
    let file_name = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .replace(".zombie_analysis", "");

    // Merge character frequencies
    if let Some(char_freqs) = analysis["char_frequencies"].as_object() {
        for (char_str, freq) in char_freqs {
            if let Some(ch) = char_str.chars().next() {
                if let Some(freq_val) = freq.as_u64() {
                    *merged.combined_char_frequencies.entry(ch).or_insert(0) += freq_val;
                    merged.total_chars += freq_val;
                }
            }
        }
    }

    // Merge essential arrows
    if let Some(arrows) = analysis["essential_arrows"].as_array() {
        for arrow in arrows {
            if let (Some(pair), Some(strength), Some(must_preserve)) = (
                arrow["pair"].as_array(),
                arrow["strength"].as_f64(),
                arrow["must_preserve"].as_bool(),
            ) {
                if pair.len() == 2 {
                    if let (Some(from_str), Some(to_str)) = (pair[0].as_str(), pair[1].as_str()) {
                        if let (Some(from_ch), Some(to_ch)) =
                            (from_str.chars().next(), to_str.chars().next())
                        {
                            merged.combined_essential_arrows.push(EssentialArrow {
                                pair: (from_ch, to_ch),
                                strength,
                                must_preserve,
                            });
                        }
                    }
                }
            }
        }
    }

    // Track file analysis
    let total_chars = analysis["total_chars"].as_u64().unwrap_or(0);
    let unique_chars = analysis["char_frequencies"].as_object().map(|o| o.len()).unwrap_or(0);
    let essential_arrows = analysis["essential_arrows"].as_array().map(|a| a.len()).unwrap_or(0);

    merged.file_analyses.insert(
        file_name,
        FileAnalysis { char_count: total_chars, unique_chars, essential_arrows },
    );

    merged.total_files += 1;

    Ok(())
}
