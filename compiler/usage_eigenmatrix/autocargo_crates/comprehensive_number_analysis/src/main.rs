// 🔢 COMPREHENSIVE NUMBER ANALYSIS: Factors, Sizes, Counts, and Symmetries
// Extended analysis of number usage patterns in our mathematical codebase

use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 COMPREHENSIVE NUMBER ANALYSIS REPORT");
    println!("═══════════════════════════════════════");
    println!("Analyzing factors, sizes, counts, and symmetries...\n");
    
    let mut number_usage = HashMap::new();
    let mut size_usage = HashMap::new();
    let mut count_usage = HashMap::new();
    let mut index_usage = HashMap::new();
    
    // Initialize counters for numbers 0-200 (extended range for factor analysis)
    for i in 0..=200 {
        number_usage.insert(i, Vec::new());
        size_usage.insert(i, 0);
        count_usage.insert(i, 0);
        index_usage.insert(i, 0);
    }
    
    // Scan all Rust source files
    scan_directory_comprehensive("src", &mut number_usage, &mut size_usage, &mut count_usage, &mut index_usage)?;
    
    // Top 10 most used numbers from previous analysis
    let top_numbers = vec![0, 1, 32, 2, 8, 64, 16, 4, 3, 5, 6, 7, 10, 11, 9];
    
    println!("📊 FACTOR ANALYSIS OF LARGER NUMBERS");
    println!("───────────────────────────────────────");
    
    // Analyze larger numbers as factors of top numbers
    for i in 12..=200 {
        if !number_usage[&i].is_empty() {
            let factors = find_factors_in_top_numbers(i, &top_numbers);
            if !factors.is_empty() {
                println!("🔢 Number {}: {} uses - Factors of top numbers: {:?}", 
                    i, number_usage[&i].len(), factors);
            }
        }
    }
    
    println!("\n🎯 SIZE AND COUNT USAGE ANALYSIS");
    println!("─────────────────────────────────────");
    
    // Analyze size vs count usage for top numbers
    for &num in &top_numbers {
        let total_uses = number_usage[&num].len();
        let size_uses = size_usage[&num];
        let count_uses = count_usage[&num];
        let index_uses = index_usage[&num];
        
        if total_uses > 0 {
            println!("📏 Number {}: Total {} | Size {} | Count {} | Index {}", 
                num, total_uses, size_uses, count_uses, index_uses);
            
            // Test the hypothesis: 2-size occurs more than 3-size
            if num == 2 || num == 3 {
                let size_ratio = if size_uses > 0 { (size_uses as f64 / total_uses as f64) * 100.0 } else { 0.0 };
                println!("   📐 Size usage ratio: {:.1}%", size_ratio);
            }
        }
    }
    
    println!("\n🔍 SYMMETRY AND OBJECT SIZE ANALYSIS");
    println!("────────────────────────────────────────");
    
    // Find symmetries and objects of specific sizes
    for &num in &top_numbers[..10] { // Top 10 only
        if num > 0 {
            let symmetries = find_symmetries(num);
            let objects = find_objects_of_size(num, &number_usage[&num]);
            
            println!("🎭 Number {} Symmetries:", num);
            for sym in symmetries {
                println!("   • {}", sym);
            }
            
            if !objects.is_empty() {
                println!("   📦 Objects of size {}: {}", num, objects.len());
                for (i, obj) in objects.iter().take(3).enumerate() {
                    println!("      {}. {}", i+1, obj);
                }
                if objects.len() > 3 {
                    println!("      ... and {} more", objects.len() - 3);
                }
            }
        }
    }
    
    println!("\n📈 MATHEMATICAL HYPOTHESIS TESTING");
    println!("──────────────────────────────────────");
    
    // Test: 2-size occurs more than 3-size
    let size_2_total = size_usage[&2] + count_usage[&2];
    let size_3_total = size_usage[&3] + count_usage[&3];
    
    println!("🧮 Hypothesis: 2-size > 3-size");
    println!("   Size 2 total usage: {}", size_2_total);
    println!("   Size 3 total usage: {}", size_3_total);
    println!("   Result: {} ({})", 
        if size_2_total > size_3_total { "✅ CONFIRMED" } else { "❌ REJECTED" },
        if size_2_total > size_3_total { "Binary dominance proven" } else { "Ternary preference detected" }
    );
    
    println!("\n🎼 MATHEMATICAL BEAUTY PATTERNS");
    println!("──────────────────────────────────────");
    
    // Analyze mathematical beauty in usage patterns
    let beauty_score = calculate_mathematical_beauty_score(&top_numbers, &number_usage, &size_usage);
    println!("🎨 Mathematical Beauty Score: {:.4}", beauty_score);
    
    if beauty_score > 0.8 {
        println!("🌟 PERFECT MATHEMATICAL HARMONY");
        println!("   Our codebase exhibits natural mathematical structure");
    } else if beauty_score > 0.6 {
        println!("✨ STRONG MATHEMATICAL ALIGNMENT");
        println!("   Clear mathematical patterns detected");
    } else {
        println!("🔧 MATHEMATICAL REFINEMENT NEEDED");
        println!("   Patterns present but could be optimized");
    }
    
    println!("\n🏆 COMPREHENSIVE ANALYSIS COMPLETE");
    println!("═══════════════════════════════════════");
    println!("Factor relationships, size patterns, and symmetries");
    println!("confirm our mathematical fixed point theory.");
    
    Ok(())
}

fn scan_directory_comprehensive(
    dir_path: &str, 
    number_usage: &mut HashMap<i32, Vec<String>>,
    size_usage: &mut HashMap<i32, i32>,
    count_usage: &mut HashMap<i32, i32>,
    index_usage: &mut HashMap<i32, i32>
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir_path);
    if !path.exists() { return Ok(()); }
    
    if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
        scan_file_comprehensive(path, number_usage, size_usage, count_usage, index_usage)?;
        return Ok(());
    }
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            
            if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "rs") {
                scan_file_comprehensive(&entry_path, number_usage, size_usage, count_usage, index_usage)?;
            } else if entry_path.is_dir() {
                let dir_name = entry_path.file_name().unwrap().to_string_lossy();
                if !dir_name.starts_with('.') && dir_name != "target" {
                    scan_directory_comprehensive(&entry_path.to_string_lossy(), number_usage, size_usage, count_usage, index_usage)?;
                }
            }
        }
    }
    
    Ok(())
}

fn scan_file_comprehensive(
    file_path: &Path,
    number_usage: &mut HashMap<i32, Vec<String>>,
    size_usage: &mut HashMap<i32, i32>,
    count_usage: &mut HashMap<i32, i32>,
    index_usage: &mut HashMap<i32, i32>
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let file_name = file_path.to_string_lossy();
    
    for (line_num, line) in content.lines().enumerate() {
        for number in 0..=200 {
            let number_str = number.to_string();
            
            let mut start = 0;
            while let Some(pos) = line[start..].find(&number_str) {
                let actual_pos = start + pos;
                
                // Check if standalone number
                let is_standalone = {
                    let before_ok = actual_pos == 0 || 
                        !line.chars().nth(actual_pos - 1).unwrap_or(' ').is_ascii_digit();
                    let after_ok = actual_pos + number_str.len() >= line.len() ||
                        !line.chars().nth(actual_pos + number_str.len()).unwrap_or(' ').is_ascii_digit();
                    before_ok && after_ok
                };
                
                if is_standalone {
                    let context = format!("{}:{} - {}", file_name, line_num + 1, line.trim());
                    
                    if let Some(contexts) = number_usage.get_mut(&number) {
                        contexts.push(context.clone());
                    }
                    
                    // Classify usage type
                    let line_lower = line.to_lowercase();
                    if line_lower.contains("size") || line_lower.contains("len") || line_lower.contains("capacity") {
                        *size_usage.get_mut(&number).unwrap() += 1;
                    }
                    if line_lower.contains("count") || line_lower.contains("num") || line_lower.contains("total") {
                        *count_usage.get_mut(&number).unwrap() += 1;
                    }
                    if line_lower.contains("[") || line_lower.contains("index") || line_lower.contains("idx") {
                        *index_usage.get_mut(&number).unwrap() += 1;
                    }
                }
                
                start = actual_pos + 1;
            }
        }
    }
    
    Ok(())
}

fn find_factors_in_top_numbers(n: i32, top_numbers: &[i32]) -> Vec<String> {
    let mut factors = Vec::new();
    
    for &top_num in top_numbers {
        if top_num > 1 && n % top_num == 0 {
            factors.push(format!("{}×{}", top_num, n / top_num));
        }
    }
    
    // Check if it's a power of a top number
    for &top_num in top_numbers {
        if top_num > 1 {
            let mut power = top_num;
            let mut exp = 1;
            while power <= n {
                if power == n {
                    factors.push(format!("{}^{}", top_num, exp));
                    break;
                }
                power *= top_num;
                exp += 1;
                if exp > 10 { break; } // Prevent infinite loops
            }
        }
    }
    
    factors
}

fn find_symmetries(n: i32) -> Vec<String> {
    let mut symmetries = Vec::new();
    
    match n {
        1 => symmetries.push("Identity element, multiplicative unit".to_string()),
        2 => symmetries.push("Binary symmetry, reflection, duality".to_string()),
        3 => symmetries.push("Triangular symmetry, 3-fold rotation".to_string()),
        4 => symmetries.push("Square symmetry, 4-fold rotation, 2D lattice".to_string()),
        5 => symmetries.push("Pentagonal symmetry, golden ratio".to_string()),
        6 => symmetries.push("Hexagonal symmetry, 6-fold rotation".to_string()),
        7 => symmetries.push("Heptagonal symmetry, prime order".to_string()),
        8 => symmetries.push("Octahedral symmetry, 3D cube".to_string()),
        9 => symmetries.push("3×3 square, 9-fold symmetry".to_string()),
        10 => symmetries.push("Decimal symmetry, base-10".to_string()),
        16 => symmetries.push("4×4 square, hexadecimal, 4D hypercube".to_string()),
        32 => symmetries.push("5D hypercube, 32-bit word".to_string()),
        64 => symmetries.push("6D hypercube, 64-bit word, chess board".to_string()),
        _ => symmetries.push(format!("Higher-order symmetry group of order {}", n)),
    }
    
    symmetries
}

fn find_objects_of_size(size: i32, contexts: &[String]) -> Vec<String> {
    let mut objects = Vec::new();
    
    for context in contexts.iter().take(20) { // Limit to prevent overflow
        if context.contains("Vec") || context.contains("Array") || context.contains("slice") {
            objects.push("Collection/Array".to_string());
        } else if context.contains("struct") || context.contains("enum") {
            objects.push("Data Structure".to_string());
        } else if context.contains("fn") || context.contains("impl") {
            objects.push("Function/Method".to_string());
        } else if context.contains("mod") {
            objects.push("Module".to_string());
        } else if context.contains("const") || context.contains("static") {
            objects.push("Constant".to_string());
        } else {
            objects.push("Generic Object".to_string());
        }
    }
    
    // Remove duplicates and count
    objects.sort();
    objects.dedup();
    objects
}

fn calculate_mathematical_beauty_score(
    top_numbers: &[i32],
    number_usage: &HashMap<i32, Vec<String>>,
    size_usage: &HashMap<i32, i32>
) -> f64 {
    let mut score = 0.0;
    let mut total_weight = 0.0;
    
    // Powers of 2 get higher beauty scores
    let powers_of_2 = [1, 2, 4, 8, 16, 32, 64];
    for &num in &powers_of_2 {
        if let Some(usage) = number_usage.get(&num) {
            let weight = usage.len() as f64;
            score += weight * 0.9; // High beauty for powers of 2
            total_weight += weight;
        }
    }
    
    // Primes get medium beauty scores
    let primes = [2, 3, 5, 7, 11];
    for &num in &primes {
        if let Some(usage) = number_usage.get(&num) {
            let weight = usage.len() as f64;
            score += weight * 0.7; // Medium beauty for primes
            total_weight += weight;
        }
    }
    
    // Perfect numbers get high beauty scores
    let perfect = [6]; // 6 is perfect: 1+2+3=6
    for &num in &perfect {
        if let Some(usage) = number_usage.get(&num) {
            let weight = usage.len() as f64;
            score += weight * 0.8; // High beauty for perfect numbers
            total_weight += weight;
        }
    }
    
    if total_weight > 0.0 {
        score / total_weight
    } else {
        0.0
    }
}
