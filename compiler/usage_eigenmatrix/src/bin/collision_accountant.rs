use std::collections::HashMap;
use std::fs;

/// Comprehensive collision accounting system
struct CollisionAccountant {
    collision_types: HashMap<String, u32>,
    crate_collisions: HashMap<String, u32>,
    function_collisions: HashMap<String, u32>,
    cross_crate_collisions: Vec<(String, String, String)>, // sig, crate1, crate2
    same_function_different_context: u32,
    different_functions_same_sig: u32,
    total_collisions: u32,
}

impl CollisionAccountant {
    fn new() -> Self {
        Self {
            collision_types: HashMap::new(),
            crate_collisions: HashMap::new(),
            function_collisions: HashMap::new(),
            cross_crate_collisions: Vec::new(),
            same_function_different_context: 0,
            different_functions_same_sig: 0,
            total_collisions: 0,
        }
    }
    
    fn extract_crate_and_function(&self, defid: &str) -> Option<(String, String)> {
        if let Some(tilde_pos) = defid.find(" ~ ") {
            let after_tilde = &defid[tilde_pos + 3..];
            if let Some(bracket_pos) = after_tilde.find('[') {
                let crate_name = after_tilde[..bracket_pos].to_string();
                if let Some(bracket_end) = after_tilde.find(']') {
                    if bracket_end + 3 < after_tilde.len() {
                        let path = &after_tilde[bracket_end + 3..];
                        let path = if path.ends_with(')') { &path[..path.len()-1] } else { path };
                        return Some((crate_name, path.to_string()));
                    }
                }
            }
        }
        None
    }
    
    fn analyze_collision(&mut self, signature: &str, defid1: &str, defid2: &str) {
        self.total_collisions += 1;
        
        // Categorize collision types
        let is_defid1 = defid1.starts_with("DefId(");
        let is_defid2 = defid2.starts_with("DefId(");
        let is_numeric1 = defid1.chars().all(|c| c.is_ascii_digit());
        let is_numeric2 = defid2.chars().all(|c| c.is_ascii_digit());
        let is_string1 = defid1.starts_with('"');
        let is_string2 = defid2.starts_with('"');
        
        match (is_defid1, is_defid2, is_numeric1, is_numeric2, is_string1, is_string2) {
            (true, true, _, _, _, _) => {
                // Both are full DefIds - analyze semantically
                if let (Some((crate1, func1)), Some((crate2, func2))) = 
                    (self.extract_crate_and_function(defid1), self.extract_crate_and_function(defid2)) {
                    
                    *self.crate_collisions.entry(crate1.clone()).or_insert(0) += 1;
                    *self.crate_collisions.entry(crate2.clone()).or_insert(0) += 1;
                    
                    if func1 == func2 {
                        self.same_function_different_context += 1;
                        *self.collision_types.entry("full_defid_same_function".to_string()).or_insert(0) += 1;
                        *self.function_collisions.entry(func1.clone()).or_insert(0) += 1;
                        
                        if crate1 != crate2 {
                            self.cross_crate_collisions.push((signature.to_string(), crate1, crate2));
                        }
                    } else {
                        self.different_functions_same_sig += 1;
                        *self.collision_types.entry("full_defid_different_functions".to_string()).or_insert(0) += 1;
                        *self.function_collisions.entry(func1.clone()).or_insert(0) += 1;
                        *self.function_collisions.entry(func2.clone()).or_insert(0) += 1;
                        
                        self.cross_crate_collisions.push((signature.to_string(), format!("{}::{}", crate1, func1), format!("{}::{}", crate2, func2)));
                    }
                } else {
                    *self.collision_types.entry("full_defid_unparseable".to_string()).or_insert(0) += 1;
                }
            },
            (true, false, _, _, _, _) | (false, true, _, _, _, _) => {
                *self.collision_types.entry("mixed_defid_other".to_string()).or_insert(0) += 1;
            },
            (false, false, true, true, false, false) => {
                *self.collision_types.entry("numeric_numeric".to_string()).or_insert(0) += 1;
            },
            (false, false, _, _, true, true) => {
                *self.collision_types.entry("string_string".to_string()).or_insert(0) += 1;
            },
            (false, false, true, false, false, true) | (false, false, false, true, true, false) => {
                *self.collision_types.entry("numeric_string".to_string()).or_insert(0) += 1;
            },
            _ => {
                *self.collision_types.entry("other_collision_type".to_string()).or_insert(0) += 1;
            }
        }
    }
    
    fn print_comprehensive_report(&self) {
        println!("=== COMPREHENSIVE COLLISION ACCOUNTING ===\n");
        
        println!("TOTAL COLLISIONS ANALYZED: {}\n", self.total_collisions);
        
        println!("COLLISION TYPE BREAKDOWN:");
        for (collision_type, count) in &self.collision_types {
            println!("- {}: {} ({:.2}%)", 
                     collision_type,
                     count,
                     (*count as f64 / self.total_collisions as f64) * 100.0);
        }
        println!();
        
        println!("TOP 10 CRATES BY COLLISION INVOLVEMENT:");
        let mut crate_sorted: Vec<_> = self.crate_collisions.iter().collect();
        crate_sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (i, (crate_name, count)) in crate_sorted.iter().take(10).enumerate() {
            println!("{}. {} - {} collisions", i + 1, crate_name, count);
        }
        println!();
        
        println!("TOP 10 FUNCTIONS BY COLLISION FREQUENCY:");
        let mut func_sorted: Vec<_> = self.function_collisions.iter().collect();
        func_sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (i, (func_name, count)) in func_sorted.iter().take(10).enumerate() {
            println!("{}. {} - {} collisions", i + 1, func_name, count);
        }
        println!();
        
        println!("CROSS-CRATE COLLISION SAMPLES (First 10):");
        for (i, (sig, item1, item2)) in self.cross_crate_collisions.iter().take(10).enumerate() {
            println!("{}. Signature 0x{}: {} <-> {}", i + 1, sig, item1, item2);
        }
        println!();
        
        println!("COLLISION DISTRIBUTION ANALYSIS:");
        println!("- Cross-crate collisions: {}", self.cross_crate_collisions.len());
        println!("- Same-crate collisions: {}", self.total_collisions - self.cross_crate_collisions.len() as u32);
        println!("- Mathematical collision rate: {:.4}%", 
                 (self.different_functions_same_sig as f64 / self.total_collisions as f64) * 100.0);
        println!("- Structural duplication rate: {:.4}%", 
                 (self.same_function_different_context as f64 / self.total_collisions as f64) * 100.0);
    }
}

fn main() {
    let mut accountant = CollisionAccountant::new();
    
    println!("Processing collision file for comprehensive analysis...");
    
    if let Ok(content) = fs::read_to_string("collisions.txt") {
        for line in content.lines() {
            if let Some((signature, defids)) = line.split_once(' ') {
                if let Some((defid1, defid2)) = defids.split_once(" | ") {
                    accountant.analyze_collision(signature, defid1, defid2);
                }
            }
        }
    }
    
    accountant.print_comprehensive_report();
    
    // Calculate coverage
    println!("COLLISION COVERAGE VERIFICATION:");
    println!("Expected collisions from matrix: 250,984");
    println!("Analyzed collisions: {}", accountant.total_collisions);
    println!("Coverage: {:.2}%", 
             (accountant.total_collisions as f64 / 250984.0) * 100.0);
}
