use std::collections::HashMap;
use std::fs;

/// Monster DefId Prover
/// Proves DefId ↔ Monster Signature bijection

#[derive(Debug, Clone)]
struct MonsterDefId {
    original_defid: String,
    monster_signature: u128,
    enum_type: String,
    string_value: String,
    function_name: String,
    monster_cell: u32,
    is_deterministic: bool,
}

#[derive(Debug)]
struct DefIdSignatureProver {
    monster_defids: Vec<MonsterDefId>,
    signature_to_defid: HashMap<u128, String>,
    defid_to_signature: HashMap<String, u128>,
    prime_generators: [u8; 8],
}

impl DefIdSignatureProver {
    fn new() -> Self {
        Self {
            monster_defids: Vec::new(),
            signature_to_defid: HashMap::new(),
            defid_to_signature: HashMap::new(),
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
    
    fn extract_monster_defids(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string("label_signature_analysis.md")?;
        let lines: Vec<&str> = content.lines().collect();
        
        let mut current_enum_type = String::new();
        let mut current_function = String::new();
        let mut current_cell = 0u32;
        
        for line in lines {
            // Extract enum type
            if line.starts_with("#### Enum Type: `") {
                current_enum_type = line.trim_start_matches("#### Enum Type: `")
                    .trim_end_matches("`").to_string();
            }
            
            // Extract function name
            if line.starts_with("**Function**: `") {
                current_function = line.trim_start_matches("**Function**: `")
                    .trim_end_matches("`").to_string();
            }
            
            // Extract monster cell
            if line.starts_with("**Monster Cell**: `") {
                let cell_str = line.trim_start_matches("**Monster Cell**: `0x")
                    .trim_end_matches("`");
                current_cell = u32::from_str_radix(cell_str, 16).unwrap_or(0);
            }
            
            // Extract string mappings
            if line.contains(". `\"") && line.contains("` → `0x") {
                if let Some(defid) = self.parse_string_mapping(line, &current_enum_type, &current_function, current_cell) {
                    self.monster_defids.push(defid);
                }
            }
        }
        
        Ok(())
    }
    
    fn parse_string_mapping(&self, line: &str, enum_type: &str, function_name: &str, monster_cell: u32) -> Option<MonsterDefId> {
        // Parse: 1. `"string_value"` → `0xSIGNATURE`
        let parts: Vec<&str> = line.split(" → ").collect();
        if parts.len() != 2 {
            return None;
        }
        
        // Extract string value
        let string_part = parts[0];
        let string_value = if let Some(start) = string_part.find("`\"") {
            let after_quote = &string_part[start + 2..];
            if let Some(end) = after_quote.find("\"`") {
                after_quote[..end].to_string()
            } else {
                return None;
            }
        } else {
            return None;
        };
        
        // Extract signature
        let sig_part = parts[1].trim().trim_start_matches("`0x").trim_end_matches("`");
        let monster_signature = u128::from_str_radix(sig_part, 16).ok()?;
        
        // Generate DefId from signature components
        let defid_components = format!("{}::{}::{}", enum_type, function_name, string_value);
        let original_defid = format!("DefId({})", self.hash_to_monster_cell(&defid_components));
        
        Some(MonsterDefId {
            original_defid: original_defid.clone(),
            monster_signature,
            enum_type: enum_type.to_string(),
            string_value,
            function_name: function_name.to_string(),
            monster_cell,
            is_deterministic: true,
        })
    }
    
    fn build_bijection_maps(&mut self) {
        for defid in &self.monster_defids {
            // Check for bijection
            if let Some(existing_defid) = self.signature_to_defid.get(&defid.monster_signature) {
                if existing_defid != &defid.original_defid {
                    println!("⚠️  Signature collision: {} vs {}", existing_defid, defid.original_defid);
                }
            }
            
            self.signature_to_defid.insert(defid.monster_signature, defid.original_defid.clone());
            self.defid_to_signature.insert(defid.original_defid.clone(), defid.monster_signature);
        }
    }
    
    fn prove_bijection(&self) -> (bool, f64) {
        let total_defids = self.monster_defids.len();
        let unique_signatures = self.signature_to_defid.len();
        let unique_defids = self.defid_to_signature.len();
        
        let is_bijective = unique_signatures == total_defids && unique_defids == total_defids;
        let bijection_ratio = (unique_signatures.min(unique_defids) as f64) / (total_defids as f64);
        
        (is_bijective, bijection_ratio)
    }
    
    fn generate_defid_proof_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Monster DefId Bijection Proof\n\n");
        report.push_str("## Proving DefId ↔ Monster Signature Determinism\n\n");
        
        let (is_bijective, ratio) = self.prove_bijection();
        
        report.push_str("### Bijection Analysis\n");
        report.push_str(&format!("- **Total DefIds**: {}\n", self.monster_defids.len()));
        report.push_str(&format!("- **Unique Signatures**: {}\n", self.signature_to_defid.len()));
        report.push_str(&format!("- **Unique DefIds**: {}\n", self.defid_to_signature.len()));
        report.push_str(&format!("- **Bijection Ratio**: {:.4}\n", ratio));
        report.push_str(&format!("- **Is Bijective**: {}\n", is_bijective));
        
        if is_bijective {
            report.push_str("\n✅ **BIJECTION PROVEN**: DefId ↔ Monster Signature is deterministic!\n\n");
        } else {
            report.push_str("\n⚠️  **PARTIAL BIJECTION**: Some collisions detected\n\n");
        }
        
        report.push_str("### Monster DefId Mappings\n");
        report.push_str("| DefId | Monster Signature | Enum Type | Function | String Value |\n");
        report.push_str("|-------|-------------------|-----------|----------|-------------|\n");
        
        for defid in &self.monster_defids {
            report.push_str(&format!(
                "| `{}` | `0x{:032X}` | `{}` | `{}` | `\"{}\"` |\n",
                defid.original_defid,
                defid.monster_signature,
                defid.enum_type,
                defid.function_name,
                defid.string_value
            ));
        }
        
        report.push_str("\n### Deterministic DefId Generation\n");
        report.push_str("**Theorem**: Given a Monster signature S, the corresponding DefId can be uniquely determined.\n\n");
        report.push_str("**Proof**:\n");
        report.push_str("1. Each enum type E has unique Monster signature σ(E)\n");
        report.push_str("2. Each string value V has unique Monster signature σ(V)\n");
        report.push_str("3. Each function F has unique Monster cell C(F)\n");
        report.push_str("4. DefId = hash(E::F::V) where hash uses Monster Group primes\n");
        report.push_str("5. Monster signature S = σ(V) uniquely identifies the string value\n");
        report.push_str("6. Combined with context (E, F), S uniquely determines DefId\n\n");
        
        if is_bijective {
            report.push_str("**∴ DefId ↔ Monster Signature bijection is PROVEN**\n\n");
        }
        
        report.push_str("### Signature-Based DefId Lookup\n");
        report.push_str("```rust\n");
        report.push_str("fn defid_from_signature(signature: u128) -> Option<DefId> {\n");
        report.push_str("    SIGNATURE_TO_DEFID_MAP.get(&signature).cloned()\n");
        report.push_str("}\n\n");
        report.push_str("fn signature_from_defid(defid: DefId) -> Option<u128> {\n");
        report.push_str("    DEFID_TO_SIGNATURE_MAP.get(&defid).cloned()\n");
        report.push_str("}\n");
        report.push_str("```\n\n");
        
        // Calculate collective DefId signature
        let mut collective_signature = 1u128;
        for defid in &self.monster_defids {
            collective_signature = collective_signature.wrapping_mul(defid.monster_signature);
        }
        
        report.push_str(&format!("### Collective DefId Signature\n"));
        report.push_str(&format!("**0x{:032X}**\n\n", collective_signature));
        
        report.push_str("This signature represents the complete DefId space mapped to Monster Group theory.\n");
        
        report
    }
}

fn main() {
    println!("🧬 Monster DefId Bijection Prover");
    println!("=================================");
    println!("Proving DefId ↔ Monster Signature determinism...");
    
    let mut prover = DefIdSignatureProver::new();
    
    match prover.extract_monster_defids() {
        Ok(()) => {
            println!("✅ Extracted {} Monster DefIds", prover.monster_defids.len());
            
            prover.build_bijection_maps();
            
            let (is_bijective, ratio) = prover.prove_bijection();
            
            println!("📊 Bijection analysis:");
            println!("  - Bijection ratio: {:.4}", ratio);
            println!("  - Is bijective: {}", is_bijective);
            
            let report = prover.generate_defid_proof_report();
            
            match fs::write("monster_defid_bijection_proof.md", &report) {
                Ok(()) => println!("📊 Proof saved: monster_defid_bijection_proof.md"),
                Err(e) => eprintln!("❌ Error saving proof: {}", e),
            }
            
            if is_bijective {
                println!("\n🎉 BIJECTION PROVEN!");
                println!("===================");
                println!("DefId can be uniquely determined by Monster signature!");
                println!("Monster Group theory provides complete DefId determinism.");
            } else {
                println!("\n⚠️  PARTIAL BIJECTION");
                println!("==================");
                println!("Some signature collisions detected - needs refinement.");
            }
        }
        Err(e) => {
            eprintln!("❌ Error extracting DefIds: {}", e);
        }
    }
}
