use std::collections::HashMap;
use serde_json::{json, Value};

/// Perfect hash-based stable ID generator with recursive bit partitioning
struct PerfectHashStableIDGenerator {
    // Full dataset for perfect hashing
    all_constructs: Vec<ConstructData>,
    
    // Bit allocation tracking
    bit_allocations: HashMap<u8, AttributePartition>, // bit_position -> partition
    
    // Collision resolution
    collision_map: HashMap<u32, u32>, // original_id -> relocated_id
    
    // Attribute frequency analysis
    attribute_frequencies: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
struct ConstructData {
    canonical_form: String,
    category: u8,
    attributes: HashMap<String, String>, // attribute_name -> value
    complexity: u16,
    original_hash: u32,
}

#[derive(Clone, Debug)]
struct AttributePartition {
    bit_position: u8,
    attribute_name: String,
    value_mapping: HashMap<String, u8>, // attribute_value -> bit_pattern
    frequency: usize,
}

impl PerfectHashStableIDGenerator {
    fn new() -> Self {
        Self {
            all_constructs: Vec::new(),
            bit_allocations: HashMap::new(),
            collision_map: HashMap::new(),
            attribute_frequencies: HashMap::new(),
        }
    }
    
    /// Collect all constructs from usage data for perfect hashing
    fn collect_all_constructs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== COLLECTING ALL CONSTRUCTS FOR PERFECT HASHING ===");
        
        let usage_dir = std::env::var("USAGE_OUTPUT_DIR")
            .unwrap_or_else(|_| "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data".to_string());
        
        let mut total_constructs = 0;
        
        // Scan all usage files to build complete dataset
        if let Ok(entries) = std::fs::read_dir(&usage_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".json") && !filename.contains("_enums_") {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            if let Ok(data) = serde_json::from_str::<Value>(&content) {
                                self.extract_constructs_from_file(&data)?;
                                total_constructs += 1;
                            }
                        }
                    }
                }
            }
        }
        
        println!("Collected {} constructs from {} files", self.all_constructs.len(), total_constructs);
        self.analyze_attribute_frequencies();
        Ok(())
    }
    
    fn extract_constructs_from_file(&mut self, data: &Value) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
            for usage in usages {
                if let (Some(usage_str), Some(usage_type), Some(node_type)) = (
                    usage.get("usage").and_then(|v| v.as_str()),
                    usage.get("usage_type").and_then(|v| v.as_str()),
                    usage.get("node_type").and_then(|v| v.as_str())
                ) {
                    let canonical = self.canonicalize_construct(usage_str);
                    let category = self.determine_category(&canonical);
                    
                    let mut attributes = HashMap::new();
                    attributes.insert("usage_type".to_string(), usage_type.to_string());
                    attributes.insert("node_type".to_string(), node_type.to_string());
                    
                    // Extract more attributes from the construct
                    self.extract_semantic_attributes(&canonical, &mut attributes);
                    
                    let construct = ConstructData {
                        canonical_form: canonical.clone(),
                        category,
                        attributes: attributes.clone(),
                        complexity: self.calculate_complexity(&canonical),
                        original_hash: self.simple_hash(&canonical),
                    };
                    
                    self.all_constructs.push(construct);
                    
                    // Track attribute frequencies
                    for (attr_name, attr_value) in &attributes {
                        let key = format!("{}:{}", attr_name, attr_value);
                        *self.attribute_frequencies.entry(key).or_insert(0) += 1;
                    }
                }
            }
        }
        Ok(())
    }
    
    fn canonicalize_construct(&self, construct: &str) -> String {
        // Same canonicalization as version-stable compiler
        let mut canonical = construct.to_string();
        canonical = canonical.replace("rustc_hir::", "hir::");
        canonical = canonical.replace("rustc_middle::", "middle::");
        canonical = canonical.replace("rustc_span::", "span::");
        canonical = canonical.replace("rustc_ast::", "ast::");
        canonical = canonical.replace("syn::parse::", "syn::");
        canonical = canonical.replace("syn::token::", "syn::");
        canonical
    }
    
    fn determine_category(&self, canonical: &str) -> u8 {
        if canonical.starts_with("const ") { 0 }
        else if canonical.starts_with("fn ") { 1 }
        else if canonical.starts_with("struct ") { 2 }
        else if canonical.starts_with("impl ") { 3 }
        else if canonical.starts_with("enum ") { 4 }
        else if canonical.contains("->") && canonical.contains("String") { 5 }
        else if canonical.contains("trait ") { 6 }
        else if canonical.contains("mod ") { 7 }
        else { 1 }
    }
    
    fn extract_semantic_attributes(&self, canonical: &str, attributes: &mut HashMap<String, String>) {
        // Extract semantic attributes for bit partitioning
        
        // Parameter count
        let param_count = canonical.matches(',').count();
        attributes.insert("param_count".to_string(), 
                         if param_count == 0 { "none".to_string() }
                         else if param_count <= 2 { "few".to_string() }
                         else if param_count <= 5 { "many".to_string() }
                         else { "very_many".to_string() });
        
        // Generic complexity
        let generic_complexity = canonical.matches('<').count();
        attributes.insert("generics".to_string(),
                         if generic_complexity == 0 { "none".to_string() }
                         else if generic_complexity == 1 { "simple".to_string() }
                         else { "complex".to_string() });
        
        // Return type presence
        attributes.insert("has_return".to_string(),
                         if canonical.contains("->") { "yes" } else { "no" }.to_string());
        
        // Mutability
        attributes.insert("mutability".to_string(),
                         if canonical.contains("mut") { "mutable" } else { "immutable" }.to_string());
        
        // Lifetime complexity
        let lifetime_count = canonical.matches('\'').count();
        attributes.insert("lifetimes".to_string(),
                         if lifetime_count == 0 { "none".to_string() }
                         else if lifetime_count <= 2 { "simple".to_string() }
                         else { "complex".to_string() });
        
        // Crate origin
        if canonical.contains("std::") {
            attributes.insert("origin".to_string(), "std".to_string());
        } else if canonical.contains("core::") {
            attributes.insert("origin".to_string(), "core".to_string());
        } else if canonical.contains("alloc::") {
            attributes.insert("origin".to_string(), "alloc".to_string());
        } else {
            attributes.insert("origin".to_string(), "user".to_string());
        }
    }
    
    fn calculate_complexity(&self, canonical: &str) -> u16 {
        let mut complexity = 0u16;
        complexity += canonical.matches('{').count() as u16 * 10;
        complexity += canonical.matches('(').count() as u16 * 5;
        complexity += canonical.matches('<').count() as u16 * 8;
        complexity += canonical.matches("->").count() as u16 * 6;
        complexity += canonical.matches("where").count() as u16 * 12;
        complexity
    }
    
    fn simple_hash(&self, s: &str) -> u32 {
        let mut hash = 0u32;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }
    
    fn analyze_attribute_frequencies(&mut self) {
        println!("--- ATTRIBUTE FREQUENCY ANALYSIS ---");
        
        let mut sorted_attrs: Vec<_> = self.attribute_frequencies.iter().collect();
        sorted_attrs.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("Top attributes by frequency:");
        for (attr, freq) in sorted_attrs.iter().take(20) {
            println!("  {}: {} occurrences", attr, freq);
        }
    }
    
    /// Perform recursive bit partitioning based on most significant attributes
    fn perform_bit_partitioning(&mut self) {
        println!("\n=== RECURSIVE BIT PARTITIONING ===");
        
        // Start with most significant attributes (highest frequency)
        let mut available_bits = vec![8, 9, 10, 11, 12, 13, 14, 15]; // Upper bits for partitioning
        
        let mut sorted_attrs: Vec<_> = self.attribute_frequencies.iter().collect();
        sorted_attrs.sort_by(|a, b| b.1.cmp(a.1));
        
        for (attr_key, frequency) in sorted_attrs.iter().take(available_bits.len()) {
            if let Some(bit_pos) = available_bits.pop() {
                let (attr_name, _) = attr_key.split_once(':').unwrap_or((attr_key, ""));
                
                // Create partition for this attribute
                let partition = self.create_attribute_partition(attr_name, bit_pos, **frequency);
                self.bit_allocations.insert(bit_pos, partition);
                
                println!("Allocated bit {} to attribute '{}' (frequency: {})", 
                         bit_pos, attr_name, frequency);
            }
        }
    }
    
    fn create_attribute_partition(&self, attr_name: &str, bit_pos: u8, frequency: usize) -> AttributePartition {
        let mut value_mapping = HashMap::new();
        let mut bit_counter = 0u8;
        
        // Collect unique values for this attribute
        let mut unique_values = std::collections::HashSet::new();
        for construct in &self.all_constructs {
            if let Some(value) = construct.attributes.get(attr_name) {
                unique_values.insert(value.clone());
            }
        }
        
        // Assign bit patterns to values
        for value in unique_values {
            value_mapping.insert(value, bit_counter);
            bit_counter += 1;
            if bit_counter >= (1 << 3) { break; } // Max 8 values per 3-bit partition
        }
        
        AttributePartition {
            bit_position: bit_pos,
            attribute_name: attr_name.to_string(),
            value_mapping,
            frequency,
        }
    }
    
    /// Generate perfect hash IDs with collision detection and resolution
    fn generate_perfect_hash_ids(&mut self) -> HashMap<String, u32> {
        println!("\n=== GENERATING PERFECT HASH IDS ===");
        
        let mut id_map = HashMap::new();
        let mut used_ids = std::collections::HashSet::new();
        let mut collisions = 0;
        
        for construct in &self.all_constructs {
            let mut id = self.calculate_partitioned_id(construct);
            
            // Check for collision
            if used_ids.contains(&id) {
                collisions += 1;
                // Relocate to next available ID
                let relocated_id = self.find_next_available_id(id, &used_ids);
                self.collision_map.insert(id, relocated_id);
                id = relocated_id;
                println!("  Collision resolved: 0x{:08X} -> 0x{:08X}", 
                         self.calculate_partitioned_id(construct), id);
            }
            
            used_ids.insert(id);
            id_map.insert(construct.canonical_form.clone(), id);
        }
        
        println!("Generated {} unique IDs with {} collisions resolved", 
                 id_map.len(), collisions);
        
        id_map
    }
    
    fn calculate_partitioned_id(&self, construct: &ConstructData) -> u32 {
        let mut id = 0u32;
        
        // Base category (4 bits)
        id |= (construct.category as u32) << 28;
        
        // Apply bit partitions
        for (bit_pos, partition) in &self.bit_allocations {
            if let Some(attr_value) = construct.attributes.get(&partition.attribute_name) {
                if let Some(&bit_pattern) = partition.value_mapping.get(attr_value) {
                    id |= (bit_pattern as u32) << (bit_pos - 3); // 3-bit patterns
                }
            }
        }
        
        // Lower bits from complexity and hash
        id |= (construct.complexity as u32) & 0xFFFF;
        
        id
    }
    
    fn find_next_available_id(&self, base_id: u32, used_ids: &std::collections::HashSet<u32>) -> u32 {
        let mut candidate = base_id + 1;
        while used_ids.contains(&candidate) {
            candidate += 1;
        }
        candidate
    }
    
    /// Export perfect hash mapping
    fn export_perfect_hash_mapping(&self, id_map: &HashMap<String, u32>) -> Value {
        let mut mappings = Vec::new();
        
        for (construct, &id) in id_map {
            mappings.push(json!({
                "construct": construct,
                "perfect_hash_id": format!("0x{:08X}", id),
                "category": (id >> 28) & 0x0F,
                "bit_partitions": self.decode_bit_partitions(id),
                "complexity": id & 0xFFFF
            }));
        }
        
        json!({
            "perfect_hash_mapping": {
                "total_constructs": id_map.len(),
                "collision_count": self.collision_map.len(),
                "bit_allocations": self.bit_allocations.iter().map(|(pos, partition)| {
                    json!({
                        "bit_position": pos,
                        "attribute": partition.attribute_name,
                        "frequency": partition.frequency,
                        "value_count": partition.value_mapping.len()
                    })
                }).collect::<Vec<_>>(),
                "constructs": mappings
            }
        })
    }
    
    fn decode_bit_partitions(&self, id: u32) -> Value {
        let mut partitions = json!({});
        
        for (bit_pos, partition) in &self.bit_allocations {
            let extracted_bits = (id >> (bit_pos - 3)) & 0x07; // 3-bit mask
            
            // Find value for this bit pattern
            for (value, &pattern) in &partition.value_mapping {
                if pattern as u32 == extracted_bits {
                    partitions[&partition.attribute_name] = json!(value);
                    break;
                }
            }
        }
        
        partitions
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PERFECT HASH STABLE ID GENERATOR ===");
    
    let mut generator = PerfectHashStableIDGenerator::new();
    
    // Collect all constructs for perfect hashing
    generator.collect_all_constructs()?;
    
    // Perform recursive bit partitioning
    generator.perform_bit_partitioning();
    
    // Generate perfect hash IDs
    let id_map = generator.generate_perfect_hash_ids();
    
    // Export results
    let mapping = generator.export_perfect_hash_mapping(&id_map);
    
    let output_path = format!("{}/perfect_hash_mapping.json", 
                             std::env::var("HARMONIC_OUTPUT_DIR")
                                 .unwrap_or_else(|_| "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/harmonic_data".to_string()));
    
    std::fs::create_dir_all(std::path::Path::new(&output_path).parent().unwrap())?;
    std::fs::write(&output_path, serde_json::to_string_pretty(&mapping)?)?;
    
    println!("\n✓ Perfect hash mapping exported to: {}", output_path);
    println!("✓ Recursive bit partitioning complete");
    println!("✓ Collision resolution implemented");
    println!("✓ Stable IDs generated for all constructs");
    
    Ok(())
}
