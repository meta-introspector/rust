use std::collections::{HashMap, HashSet};
use std::fs;

/// Label Signature Extractor
/// Shows Monster signatures for each enum type and its string mappings

#[derive(Debug, Clone)]
struct LabelSignature {
    enum_type: String,
    enum_signature: u128,
    string_values: Vec<String>,
    string_signatures: Vec<u128>,
    monster_cell: u32,
    function_name: String,
}

#[derive(Debug)]
struct LabelSignatureExtractor {
    signatures: Vec<LabelSignature>,
    prime_generators: [u8; 8],
}

impl LabelSignatureExtractor {
    fn new() -> Self {
        Self {
            signatures: Vec::new(),
            prime_generators: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }
    
    fn calculate_monster_signature(&self, data: &str) -> u128 {
        let mut signature = 1u128;
        for (i, byte) in data.bytes().enumerate() {
            let prime_idx = i % 8;
            let prime = self.prime_generators[prime_idx] as u128;
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u128);
        }
        signature
    }
    
    fn hash_to_monster_cell(&self, data: &str) -> u32 {
        let mut hash = 1u64;
        for (i, byte) in data.bytes().enumerate() {
            let prime_idx = i % 8;
            hash = hash.wrapping_mul(self.prime_generators[prime_idx] as u64)
                      .wrapping_add(byte as u64);
        }
        (hash % (1u64 << 24)) as u32
    }
    
    fn extract_from_analysis(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string("enum_string_bridge_analysis.md")?;
        let lines: Vec<&str> = content.lines().collect();
        
        let mut in_table = false;
        
        for line in lines {
            if line.starts_with("| Function | Enum Type |") {
                in_table = true;
                continue;
            }
            
            if in_table && line.starts_with("|") && !line.starts_with("|-------") {
                if let Some(signature) = self.parse_table_row(line) {
                    self.signatures.push(signature);
                }
            }
            
            if in_table && !line.starts_with("|") {
                break;
            }
        }
        
        Ok(())
    }
    
    fn parse_table_row(&self, line: &str) -> Option<LabelSignature> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 6 {
            return None;
        }
        
        let function_name = parts[1].trim().trim_matches('`').to_string();
        let enum_type = parts[2].trim().trim_matches('`').to_string();
        let string_patterns = parts[4].trim().to_string();
        
        // Calculate signatures
        let enum_signature = self.calculate_monster_signature(&enum_type);
        let monster_cell = self.hash_to_monster_cell(&format!("{}::{}", enum_type, function_name));
        
        // Parse string values
        let string_values: Vec<String> = if string_patterns.is_empty() {
            vec!["<empty>".to_string()]
        } else {
            string_patterns.split(", ").map(|s| s.trim().to_string()).collect()
        };
        
        let string_signatures: Vec<u128> = string_values.iter()
            .map(|s| self.calculate_monster_signature(s))
            .collect();
        
        Some(LabelSignature {
            enum_type,
            enum_signature,
            string_values,
            string_signatures,
            monster_cell,
            function_name,
        })
    }
    
    fn generate_signature_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Label Signature Analysis: Enum Types → String Values\n\n");
        report.push_str("## Monster Signatures for Each Domain Type and Range Values\n\n");
        
        // Group by enum type
        let mut type_groups: HashMap<String, Vec<&LabelSignature>> = HashMap::new();
        for sig in &self.signatures {
            type_groups.entry(sig.enum_type.clone()).or_insert_with(Vec::new).push(sig);
        }
        
        report.push_str("### Enum Type Signatures\n");
        report.push_str("| Enum Type | Monster Signature | Functions | String Count |\n");
        report.push_str("|-----------|-------------------|-----------|-------------|\n");
        
        let mut sorted_types: Vec<_> = type_groups.keys().collect();
        sorted_types.sort();
        
        for enum_type in sorted_types {
            let sigs = &type_groups[enum_type];
            let enum_signature = sigs[0].enum_signature;
            let total_strings: usize = sigs.iter().map(|s| s.string_values.len()).sum();
            
            report.push_str(&format!(
                "| `{}` | `0x{:032X}` | {} | {} |\n",
                enum_type, enum_signature, sigs.len(), total_strings
            ));
        }
        
        report.push_str("\n### Detailed Label Mappings\n");
        
        for enum_type in type_groups.keys() {
            let sigs = &type_groups[enum_type];
            
            report.push_str(&format!("\n#### Enum Type: `{}`\n", enum_type));
            report.push_str(&format!("**Monster Signature**: `0x{:032X}`\n\n", sigs[0].enum_signature));
            
            for sig in sigs {
                report.push_str(&format!("**Function**: `{}`\n", sig.function_name));
                report.push_str(&format!("**Monster Cell**: `0x{:06X}`\n\n", sig.monster_cell));
                
                report.push_str("**String Mappings**:\n");
                for (i, (string_val, string_sig)) in sig.string_values.iter().zip(&sig.string_signatures).enumerate() {
                    report.push_str(&format!("{}. `\"{}\"` → `0x{:032X}`\n", 
                        i + 1, string_val, string_sig));
                }
                report.push_str("\n");
            }
        }
        
        // Calculate collective signatures
        report.push_str("### Collective Signatures\n");
        
        let mut all_enum_sigs = 1u128;
        let mut all_string_sigs = 1u128;
        
        for sig in &self.signatures {
            all_enum_sigs = all_enum_sigs.wrapping_mul(sig.enum_signature);
            for &string_sig in &sig.string_signatures {
                all_string_sigs = all_string_sigs.wrapping_mul(string_sig);
            }
        }
        
        report.push_str(&format!("- **All Enum Types**: `0x{:032X}`\n", all_enum_sigs));
        report.push_str(&format!("- **All String Values**: `0x{:032X}`\n", all_string_sigs));
        
        let domain_range_signature = all_enum_sigs.wrapping_mul(all_string_sigs);
        report.push_str(&format!("- **Domain × Range**: `0x{:032X}`\n", domain_range_signature));
        
        report.push_str("\n### Mathematical Interpretation\n");
        report.push_str("Each enum type has a unique Monster signature based on its name.\n");
        report.push_str("Each string value has a unique Monster signature based on its content.\n");
        report.push_str("The mapping F: Enum → String creates a mathematical bridge between:\n");
        report.push_str("- **Domain signatures** (enum type identities)\n");
        report.push_str("- **Range signatures** (string value identities)\n\n");
        report.push_str("This establishes the foundational label signature system for Monster Group theory.\n");
        
        report
    }
}

fn main() {
    println!("🏷️ Label Signature Extractor");
    println!("============================");
    println!("Extracting Monster signatures for enum types and string values...");
    
    let mut extractor = LabelSignatureExtractor::new();
    
    match extractor.extract_from_analysis() {
        Ok(()) => {
            println!("✅ Extracted {} label signatures", extractor.signatures.len());
            
            let report = extractor.generate_signature_report();
            
            match fs::write("label_signature_analysis.md", &report) {
                Ok(()) => println!("📊 Report saved: label_signature_analysis.md"),
                Err(e) => eprintln!("❌ Error saving report: {}", e),
            }
            
            println!("\n🎯 LABEL SIGNATURE ANALYSIS COMPLETE!");
            println!("====================================");
            
            // Show summary
            let unique_types: HashSet<_> = extractor.signatures.iter().map(|s| &s.enum_type).collect();
            let total_strings: usize = extractor.signatures.iter().map(|s| s.string_values.len()).sum();
            
            println!("Unique enum types: {}", unique_types.len());
            println!("Total string mappings: {}", total_strings);
            println!("Label signatures: {}", extractor.signatures.len());
            
            println!("\n🧬 Monster signatures established for all enum → string mappings!");
        }
        Err(e) => {
            eprintln!("❌ Error extracting signatures: {}", e);
        }
    }
}
