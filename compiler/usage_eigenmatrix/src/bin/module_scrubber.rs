use std::collections::HashMap;
use std::fs;
use serde_json::Value;

/// Scrub module IDs from DefIds to find duplicate code patterns
struct ModuleIdScrubber {
    scrubbed_signatures: HashMap<String, Vec<String>>, // scrubbed_defid -> original_defids
    prime_basis: [u64; 8],
}

impl ModuleIdScrubber {
    fn new() -> Self {
        Self {
            scrubbed_signatures: HashMap::new(),
            prime_basis: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }
    
    /// Remove module/crate IDs from DefId to find structural duplicates
    fn scrub_defid(&self, defid: &str) -> Option<String> {
        // Parse DefId(107:1247 ~ rustc_errors[740a]::diagnostic::emit)
        if let Some(tilde_pos) = defid.find(" ~ ") {
            let after_tilde = &defid[tilde_pos + 3..];
            if let Some(bracket_pos) = after_tilde.find('[') {
                let crate_name = &after_tilde[..bracket_pos];
                if let Some(bracket_end) = after_tilde.find(']') {
                    if bracket_end + 3 < after_tilde.len() {
                        let path = &after_tilde[bracket_end + 3..];
                        let path = if path.ends_with(')') { &path[..path.len()-1] } else { path };
                        // Return scrubbed form: crate::path (no module IDs)
                        return Some(format!("{}::{}", crate_name, path));
                    }
                }
            }
        }
        None
    }
    
    /// Calculate signature for scrubbed DefId
    fn calculate_signature(&self, scrubbed_defid: &str) -> u32 {
        let mut signature = 0u64;
        for (i, &prime) in self.prime_basis.iter().enumerate() {
            let char_sum: u64 = scrubbed_defid.chars()
                .enumerate()
                .map(|(j, c)| (c as u64) * (j as u64 + 1))
                .sum();
            signature += (char_sum % prime) << (i * 3);
        }
        (signature & 0xFFFFFF) as u32
    }
    
    /// Process DefId and track duplicates
    fn process_defid(&mut self, defid: &str) {
        if let Some(scrubbed) = self.scrub_defid(defid) {
            self.scrubbed_signatures
                .entry(scrubbed)
                .or_insert_with(Vec::new)
                .push(defid.to_string());
        }
    }
    
    /// Find duplicate code patterns
    fn find_duplicates(&self) -> Vec<(String, Vec<String>)> {
        self.scrubbed_signatures
            .iter()
            .filter(|(_, defids)| defids.len() > 1)
            .map(|(scrubbed, defids)| (scrubbed.clone(), defids.clone()))
            .collect()
    }
}

fn main() {
    let mut scrubber = ModuleIdScrubber::new();
    
    // Process collision file to find module-scrubbed duplicates
    if let Ok(content) = fs::read_to_string("collisions.txt") {
        for line in content.lines() {
            if let Some((_, defids)) = line.split_once(' ') {
                if let Some((defid1, defid2)) = defids.split_once(" | ") {
                    scrubber.process_defid(defid1);
                    scrubber.process_defid(defid2);
                }
            }
        }
    }
    
    // Also process usage data files for more comprehensive analysis
    let usage_dir = "../../usage_data";
    if let Ok(entries) = fs::read_dir(usage_dir) {
        let mut files_processed = 0;
        for entry in entries.flatten().take(100) { // Sample first 100 files
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(usages) = json.as_array() {
                        for usage in usages {
                            if let Some(used_def_id) = usage["used_def_id"].as_str() {
                                scrubber.process_defid(used_def_id);
                            }
                        }
                    }
                }
            }
            files_processed += 1;
        }
        println!("Processed {} usage files for duplicate detection", files_processed);
    }
    
    // Find and report duplicates
    let duplicates = scrubber.find_duplicates();
    let mut sorted_duplicates = duplicates.clone();
    sorted_duplicates.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("\nTop 10 Duplicate Code Patterns (Module ID Scrubbed):\n");
    
    for (i, (scrubbed_defid, original_defids)) in sorted_duplicates.iter().take(10).enumerate() {
        println!("{}. Pattern: {} ({} instances)", i + 1, scrubbed_defid, original_defids.len());
        
        // Show first 3 instances
        for (j, defid) in original_defids.iter().take(3).enumerate() {
            println!("   Instance {}: {}", j + 1, defid);
        }
        
        // Calculate signature for this pattern
        let signature = scrubber.calculate_signature(scrubbed_defid);
        println!("   Signature: 0x{:06X}", signature);
        println!();
    }
    
    println!("Total duplicate patterns found: {}", duplicates.len());
    let total_duplicates: usize = duplicates.iter().map(|(_, defids)| defids.len()).sum();
    println!("Total duplicate instances: {}", total_duplicates);
}
