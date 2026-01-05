use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("collisions.txt").expect("Failed to open collisions.txt");
    let reader = BufReader::new(file);
    
    let mut signature_counts: HashMap<String, Vec<String>> = HashMap::new();
    
    // Parse collision file (format: signature defid1 | defid2)
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some((sig, defids)) = line.split_once(' ') {
                if let Some((defid1, defid2)) = defids.split_once(" | ") {
                    // Only process DefId entries
                    if defid1.starts_with("DefId(") || defid2.starts_with("DefId(") {
                        let entry = signature_counts.entry(sig.to_string()).or_insert_with(Vec::new);
                        if defid1.starts_with("DefId(") { entry.push(defid1.to_string()); }
                        if defid2.starts_with("DefId(") { entry.push(defid2.to_string()); }
                    }
                }
            }
        }
    }
    
    // Sort by collision count
    let mut sorted: Vec<_> = signature_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("Top 10 Most Collided 24-bit Signatures:\n");
    
    for (i, (signature, defids)) in sorted.iter().take(10).enumerate() {
        println!("{}. Signature 0x{} - {} collisions", i + 1, signature, defids.len());
        
        // Show 3 different samples
        let unique_samples: std::collections::HashSet<_> = defids.iter().collect();
        let samples: Vec<_> = unique_samples.into_iter().take(3).collect();
        
        for (j, defid) in samples.iter().enumerate() {
            println!("   Sample {}: {}", j + 1, defid);
        }
        
        // Analyze pattern
        analyze_pattern(defids);
        println!();
    }
}

fn analyze_pattern(defids: &[String]) {
    let mut crates = HashMap::new();
    let mut modules = HashMap::new();
    let mut items = HashMap::new();
    
    for defid in defids {
        if let Some(parts) = extract_parts(defid) {
            *crates.entry(parts.0).or_insert(0) += 1;
            *modules.entry(parts.1).or_insert(0) += 1;
            *items.entry(parts.2).or_insert(0) += 1;
        }
    }
    
    let top_crate = crates.iter().max_by_key(|(_, &count)| count);
    let top_module = modules.iter().max_by_key(|(_, &count)| count);
    let top_item = items.iter().max_by_key(|(_, &count)| count);
    
    if let Some((crate_name, count)) = top_crate {
        println!("   Primary crate: {} ({} occurrences)", crate_name, count);
    }
    if let Some((module, count)) = top_module {
        println!("   Primary module: {} ({} occurrences)", module, count);
    }
    if let Some((item, count)) = top_item {
        println!("   Primary item: {} ({} occurrences)", item, count);
    }
}

fn extract_parts(defid: &str) -> Option<(String, String, String)> {
    // Parse DefId(2:3260 ~ core[4720]::cmp::PartialOrd::le)
    if let Some(tilde_pos) = defid.find(" ~ ") {
        let after_tilde = &defid[tilde_pos + 3..];
        if let Some(bracket_pos) = after_tilde.find('[') {
            let crate_name = after_tilde[..bracket_pos].to_string();
            if let Some(bracket_end) = after_tilde.find(']') {
                if bracket_end + 3 < after_tilde.len() {
                    let path = &after_tilde[bracket_end + 3..];
                    let path = if path.ends_with(')') { &path[..path.len()-1] } else { path };
                    let parts: Vec<&str> = path.split("::").collect();
                    let module = parts.get(0).unwrap_or(&"").to_string();
                    let item = parts.last().unwrap_or(&"").to_string();
                    return Some((crate_name, module, item));
                }
            }
        }
    }
    None
}
