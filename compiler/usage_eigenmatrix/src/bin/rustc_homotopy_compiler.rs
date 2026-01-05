use std::fs;
use std::collections::HashMap;
use serde_json::Value;

/// Homotopy compiler for actual rustc objects
struct RustcHomotopyCompiler {
    homotopy_bases: HashMap<u8, u32>,
    objects: HashMap<u32, (String, u8)>, // address -> (defid, homotopy_level)
    level_counters: HashMap<u8, u32>,
}

impl RustcHomotopyCompiler {
    fn new() -> Self {
        let mut homotopy_bases = HashMap::new();
        homotopy_bases.insert(0, 0x000000); // Level 0: Constants, primitives
        homotopy_bases.insert(1, 0x100000); // Level 1: Functions
        homotopy_bases.insert(2, 0x200000); // Level 2: Structures, enums
        homotopy_bases.insert(3, 0x300000); // Level 3: Implementations, traits
        
        Self {
            homotopy_bases,
            objects: HashMap::new(),
            level_counters: HashMap::new(),
        }
    }
    
    /// Calculate homotopy level from rustc DefId
    fn calculate_homotopy_level(&self, defid: &str) -> u8 {
        // Parse DefId content to determine topological complexity
        if defid.contains("const") || defid.contains("PRIME_") {
            0 // Constants - trivial homotopy
        } else if defid.contains("::fn") || defid.contains("main") || defid.contains("emit_") {
            1 // Functions - 1-dimensional
        } else if defid.contains("struct") || defid.contains("enum") || defid.contains("Diagnostic") {
            2 // Data structures - 2-dimensional  
        } else if defid.contains("impl") || defid.contains("trait") || defid.contains("{impl#") {
            3 // Implementations - 3-dimensional
        } else {
            0 // Default to constants
        }
    }
    
    /// Allocate homotopy address for DefId
    fn allocate_homotopy_address(&mut self, defid: String) -> u32 {
        let level = self.calculate_homotopy_level(&defid);
        let counter = self.level_counters.entry(level).or_insert(0);
        let address = self.homotopy_bases[&level] + *counter;
        *counter += 1;
        
        self.objects.insert(address, (defid, level));
        address
    }
    
    /// Process usage data files with homotopy addressing
    fn process_usage_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== PROCESSING RUSTC USAGE DATA WITH HOMOTOPY ADDRESSING ===\n");
        
        let usage_dir = "../../usage_data";
        if !std::path::Path::new(usage_dir).exists() {
            println!("Usage data directory not found, using sample data");
            return self.process_sample_data();
        }
        
        let mut files_processed = 0;
        let entries = fs::read_dir(usage_dir)?;
        
        for entry in entries.flatten().take(50) { // Process first 50 files
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(usages) = json.as_array() {
                        for usage in usages {
                            if let Some(used_def_id) = usage["used_def_id"].as_str() {
                                self.allocate_homotopy_address(used_def_id.to_string());
                            }
                            if let Some(user_def_id) = usage["user_def_id"].as_str() {
                                self.allocate_homotopy_address(user_def_id.to_string());
                            }
                        }
                    }
                }
            }
            files_processed += 1;
        }
        
        println!("Processed {} usage data files", files_processed);
        Ok(())
    }
    
    /// Process sample data if usage_data not available
    fn process_sample_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Use actual DefIds from our collision analysis
        let rustc_defids = [
            "DefId(2:3260 ~ core[4720]::cmp::PartialOrd::le)",
            "DefId(73:41 ~ tracing_core[5cb0]::callsite::Callsite::metadata)", 
            "DefId(2:11870 ~ core[4720]::fmt::rt::{impl#0}::new_display)",
            "DefId(2:11899 ~ core[4720]::fmt::rt::{impl#1}::new_v1)",
            "DefId(2:9429 ~ core[4720]::iter::traits::iterator::Iterator::next)",
            "DefId(131:1247 ~ rustc_errors[740a]::{impl#9}::emit_err)",
            "DefId(107:1239 ~ rustc_errors[740a]::{impl#9}::struct_span_err)",
            "DefId(0:923 ~ regex_syntax[d4f9]::hir::{impl#18}::new)",
            "DefId(131:2072 ~ rustc_errors[740a]::diagnostic::{impl#18}::span_label)",
            "DefId(107:431 ~ rustc_errors[740a]::diagnostic::{impl#18}::emit)",
            "DefId(202:3117 ~ rustc_middle[f25a]::mir::{impl#14}::with_source_info)",
            "DefId(131:2116 ~ rustc_errors[740a]::diagnostic::{impl#18}::span_suggestion_verbose)",
            "DefId(107:2080 ~ rustc_errors[740a]::diagnostic::{impl#18}::note)",
            "DefId(131:1245 ~ rustc_errors[740a]::{impl#9}::create_err)",
            // Add constants from our prime analysis
            "const PRIME_2: u32 = 2",
            "const PRIME_3: u32 = 3", 
            "const PRIME_5: u32 = 5",
            "const PRIME_7: u32 = 7",
            "const PRIME_11: u32 = 11",
            // Add structures
            "struct DiagnosticBuilder",
            "struct DefId",
            "enum TyKind",
            "struct Span",
        ];
        
        for defid in &rustc_defids {
            self.allocate_homotopy_address(defid.to_string());
        }
        
        println!("Processed {} rustc DefIds from compiler analysis", rustc_defids.len());
        Ok(())
    }
    
    /// Show homotopy analysis of rustc objects
    fn show_homotopy_analysis(&self) {
        println!("\n=== RUSTC HOMOTOPY ANALYSIS ===\n");
        
        for level in 0..4 {
            let base = self.homotopy_bases[&level];
            let objects_at_level: Vec<_> = self.objects.iter()
                .filter(|(_, (_, l))| *l == level)
                .collect();
            
            let description = match level {
                0 => "Constants/Primitives (trivial homotopy)",
                1 => "Functions (1-dimensional)",
                2 => "Structures/Enums (2-dimensional)",
                3 => "Implementations/Traits (3-dimensional)",
                _ => "Unknown",
            };
            
            println!("Level {} (0x{:06X}): {} - {} objects", 
                     level, base, description, objects_at_level.len());
            
            // Show samples
            for (addr, (defid, _)) in objects_at_level.iter().take(5) {
                let display = if defid.len() > 60 { 
                    format!("{}...", &defid[..57]) 
                } else { 
                    defid.clone() 
                };
                println!("  0x{:06X}: {}", addr, display);
            }
            
            if objects_at_level.len() > 5 {
                println!("  ... ({} more)", objects_at_level.len() - 5);
            }
            println!();
        }
        
        // Show distribution
        println!("Homotopy Distribution:");
        let total = self.objects.len();
        for level in 0..4 {
            let count = self.objects.values().filter(|(_, l)| *l == level).count();
            let percentage = (count as f64 / total as f64) * 100.0;
            println!("  Level {}: {} objects ({:.1}%)", level, count, percentage);
        }
    }
    
    /// Save homotopy mapping
    fn save_homotopy_mapping(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut output = String::new();
        output.push_str("# Rustc Homotopy Address Mapping\n\n");
        
        for level in 0..4 {
            output.push_str(&format!("## Level {} Objects\n", level));
            
            let mut level_objects: Vec<_> = self.objects.iter()
                .filter(|(_, (_, l))| *l == level)
                .collect();
            level_objects.sort_by_key(|(addr, _)| *addr);
            
            for (addr, (defid, _)) in level_objects {
                output.push_str(&format!("0x{:06X}: {}\n", addr, defid));
            }
            output.push('\n');
        }
        
        fs::write(filename, output)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = RustcHomotopyCompiler::new();
    
    // Process actual rustc usage data
    compiler.process_usage_data()?;
    
    // Show homotopy analysis
    compiler.show_homotopy_analysis();
    
    // Save mapping
    compiler.save_homotopy_mapping("rustc_homotopy_mapping.txt")?;
    
    println!("✓ Rustc homotopy addressing complete");
    println!("✓ Mapping saved to rustc_homotopy_mapping.txt");
    println!("✓ Real rustc objects classified by topological complexity");
    
    Ok(())
}
