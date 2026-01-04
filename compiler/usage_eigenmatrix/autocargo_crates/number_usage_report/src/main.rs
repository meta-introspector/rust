// 🔢 LITERAL CONSTANTS AND NUMBERS USAGE REPORT
// Analyze usage of all numbers up to 71 in our codebase

use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 LITERAL CONSTANTS AND NUMBERS USAGE REPORT");
    println!("═══════════════════════════════════════════════");
    println!("Analyzing usage of numbers 0-71 in codebase...\n");
    
    let mut number_usage = HashMap::new();
    
    // Initialize counters for numbers 0-71
    for i in 0..=71 {
        number_usage.insert(i, Vec::new());
    }
    
    // Scan all Rust source files
    scan_directory("src", &mut number_usage)?;
    scan_directory(".", &mut number_usage)?; // Root level files
    
    println!("📊 NUMBER USAGE STATISTICS");
    println!("─────────────────────────────");
    
    // Sort by usage frequency
    let mut usage_stats: Vec<(i32, Vec<String>)> = number_usage.into_iter().collect();
    usage_stats.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (number, contexts) in &usage_stats {
        if !contexts.is_empty() {
            println!("\n🔢 Number {}: {} occurrences", number, contexts.len());
            for (i, context) in contexts.iter().enumerate() {
                if i < 5 { // Show first 5 contexts
                    println!("   📍 {}", context);
                } else if i == 5 {
                    println!("   ... and {} more occurrences", contexts.len() - 5);
                    break;
                }
            }
        }
    }
    
    println!("\n🎯 MOST FREQUENTLY USED NUMBERS");
    println!("─────────────────────────────────");
    
    let top_numbers: Vec<_> = usage_stats.iter()
        .filter(|(_, contexts)| !contexts.is_empty())
        .take(10)
        .collect();
    
    for (i, (number, contexts)) in top_numbers.iter().enumerate() {
        println!("{}. Number {}: {} uses", i + 1, number, contexts.len());
    }
    
    println!("\n🔍 MATHEMATICAL SIGNIFICANCE ANALYSIS");
    println!("────────────────────────────────────────");
    
    // Analyze mathematical significance
    for (number, contexts) in &usage_stats {
        if !contexts.is_empty() {
            let significance = analyze_mathematical_significance(*number, contexts.len());
            if !significance.is_empty() {
                println!("🧮 Number {}: {}", number, significance);
            }
        }
    }
    
    println!("\n📈 USAGE DISTRIBUTION");
    println!("────────────────────────");
    
    let total_used_numbers = usage_stats.iter().filter(|(_, contexts)| !contexts.is_empty()).count();
    let total_occurrences: usize = usage_stats.iter().map(|(_, contexts)| contexts.len()).sum();
    
    println!("📊 {} unique numbers used (out of 0-71)", total_used_numbers);
    println!("📊 {} total number occurrences", total_occurrences);
    println!("📊 Average {:.2} uses per number", total_occurrences as f64 / total_used_numbers as f64);
    
    Ok(())
}

fn scan_directory(dir_path: &str, number_usage: &mut HashMap<i32, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir_path);
    if !path.exists() {
        return Ok(());
    }
    
    if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
        scan_file(path, number_usage)?;
        return Ok(());
    }
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            
            if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "rs") {
                scan_file(&entry_path, number_usage)?;
            } else if entry_path.is_dir() {
                let dir_name = entry_path.file_name().unwrap().to_string_lossy();
                if !dir_name.starts_with('.') && dir_name != "target" {
                    scan_directory(&entry_path.to_string_lossy(), number_usage)?;
                }
            }
        }
    }
    
    Ok(())
}

fn scan_file(file_path: &Path, number_usage: &mut HashMap<i32, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let file_name = file_path.to_string_lossy();
    
    for (line_num, line) in content.lines().enumerate() {
        // Look for numbers 0-71 in various contexts
        for number in 0..=71 {
            let number_str = number.to_string();
            
            // Find all occurrences of this number
            let mut start = 0;
            while let Some(pos) = line[start..].find(&number_str) {
                let actual_pos = start + pos;
                
                // Check if it's a standalone number (not part of a larger number)
                let is_standalone = {
                    let before_ok = actual_pos == 0 || 
                        !line.chars().nth(actual_pos - 1).unwrap_or(' ').is_ascii_digit();
                    let after_ok = actual_pos + number_str.len() >= line.len() ||
                        !line.chars().nth(actual_pos + number_str.len()).unwrap_or(' ').is_ascii_digit();
                    before_ok && after_ok
                };
                
                if is_standalone {
                    let context = format!("{}:{} - {}", 
                        file_name, 
                        line_num + 1, 
                        line.trim()
                    );
                    
                    if let Some(contexts) = number_usage.get_mut(&number) {
                        contexts.push(context);
                    }
                }
                
                start = actual_pos + 1;
            }
        }
    }
    
    Ok(())
}

fn analyze_mathematical_significance(number: i32, usage_count: usize) -> String {
    let mut significance = Vec::new();
    
    // Mathematical properties
    match number {
        0 => significance.push("Zero - additive identity, null element"),
        1 => significance.push("One - multiplicative identity, unit element"),
        2 => significance.push("Two - first prime, base of binary"),
        3 => significance.push("Three - first odd prime, triangle base"),
        4 => significance.push("Four - first composite, 2²"),
        5 => significance.push("Five - Fibonacci number, pentagon sides"),
        6 => significance.push("Six - first perfect number (1+2+3=6)"),
        7 => significance.push("Seven - Mersenne prime, mystical number"),
        8 => significance.push("Eight - first cube (2³), octal base"),
        9 => significance.push("Nine - 3², digital root properties"),
        10 => significance.push("Ten - decimal base, tetractys"),
        12 => significance.push("Twelve - highly composite, dozen"),
        16 => significance.push("Sixteen - 2⁴, hexadecimal base"),
        17 => significance.push("Seventeen - Fermat prime"),
        24 => significance.push("Twenty-four - highly composite, factorial"),
        25 => significance.push("Twenty-five - 5², perfect square"),
        32 => significance.push("Thirty-two - 2⁵, computer word size"),
        36 => significance.push("Thirty-six - 6², highly composite"),
        42 => significance.push("Forty-two - Answer to Ultimate Question"),
        49 => significance.push("Forty-nine - 7², perfect square"),
        64 => significance.push("Sixty-four - 2⁶, 8², chess squares"),
        71 => significance.push("Seventy-one - prime, palindromic in binary"),
        _ => {}
    }
    
    // Usage frequency significance
    if usage_count > 20 {
        significance.push("HEAVILY USED - core system constant");
    } else if usage_count > 10 {
        significance.push("FREQUENTLY USED - important parameter");
    } else if usage_count > 5 {
        significance.push("MODERATELY USED - recurring value");
    }
    
    significance.join(", ")
}
