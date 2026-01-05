use std::collections::HashMap;
use std::fs;

/// System G: Complete compilation unit collector
struct SystemG {
    constants: HashMap<u32, String>,        // signature -> constant definition
    nodes: HashMap<u32, NodeData>,         // signature -> node data
    arrows: HashMap<u32, Vec<Arrow>>,      // from_signature -> arrows
    regions: HashMap<u16, Vec<u32>>,       // region_id -> signatures
    compilation_unit: String,              // source file being processed
}

#[derive(Clone)]
struct NodeData {
    node_type: String,    // "const", "fn", "struct", "impl", etc.
    name: String,         // identifier name
    content: String,      // source content
    region: u16,          // region allocation
}

#[derive(Clone)]
struct Arrow {
    to_signature: u32,
    arrow_type: String,   // "calls", "uses", "defines", "implements"
    is_cross_region: bool,
}

impl SystemG {
    fn new() -> Self {
        Self {
            constants: HashMap::new(),
            nodes: HashMap::new(),
            arrows: HashMap::new(),
            regions: HashMap::new(),
            compilation_unit: String::new(),
        }
    }
    
    fn calculate_signature(&self, source: &str) -> u32 {
        let prime_basis = [2, 3, 5, 7, 11, 13, 17, 19];
        let mut signature = 0u64;
        for (i, &prime) in prime_basis.iter().enumerate() {
            let char_sum: u64 = source.chars()
                .enumerate()
                .map(|(j, c)| (c as u64) * (j as u64 + 1))
                .sum();
            signature += (char_sum % prime) << (i * 3);
        }
        (signature & 0xFFFFFF) as u32
    }
    
    fn signature_to_region(&self, signature: u32) -> u16 {
        (signature / 4096) as u16
    }
    
    /// Collect entire compilation unit
    fn collect_compilation_unit(&mut self, source_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let source = fs::read_to_string(source_file)?;
        self.compilation_unit = source_file.to_string();
        
        println!("=== COLLECTING COMPILATION UNIT: {} ===\n", source_file);
        
        // Parse and collect all elements
        self.collect_constants(&source);
        self.collect_nodes(&source);
        self.collect_arrows(&source);
        self.allocate_regions();
        
        Ok(())
    }
    
    /// Collect all constants
    fn collect_constants(&mut self, source: &str) {
        println!("Collecting constants:");
        
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("const ") && trimmed.contains(" = ") {
                let signature = self.calculate_signature(trimmed);
                self.constants.insert(signature, trimmed.to_string());
                
                // Extract constant name
                let name = if let Some(space_pos) = trimmed.find(' ') {
                    if let Some(colon_pos) = trimmed.find(':') {
                        trimmed[space_pos + 1..colon_pos].trim().to_string()
                    } else if let Some(eq_pos) = trimmed.find(" = ") {
                        trimmed[space_pos + 1..eq_pos].trim().to_string()
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    "unknown".to_string()
                };
                
                let region = self.signature_to_region(signature);
                let node = NodeData {
                    node_type: "const".to_string(),
                    name,
                    content: trimmed.to_string(),
                    region,
                };
                
                self.nodes.insert(signature, node);
                println!("  0x{:06X} -> {}", signature, trimmed);
            }
        }
    }
    
    /// Collect all nodes (functions, structs, etc.)
    fn collect_nodes(&mut self, source: &str) {
        println!("\nCollecting nodes:");
        
        for line in source.lines() {
            let trimmed = line.trim();
            
            // Functions
            if trimmed.starts_with("fn ") {
                let signature = self.calculate_signature(trimmed);
                let name = self.extract_function_name(trimmed);
                let region = self.signature_to_region(signature);
                
                let node = NodeData {
                    node_type: "fn".to_string(),
                    name,
                    content: trimmed.to_string(),
                    region,
                };
                
                self.nodes.insert(signature, node);
                println!("  0x{:06X} -> {}", signature, trimmed);
            }
            
            // Structs
            if trimmed.starts_with("struct ") {
                let signature = self.calculate_signature(trimmed);
                let name = self.extract_struct_name(trimmed);
                let region = self.signature_to_region(signature);
                
                let node = NodeData {
                    node_type: "struct".to_string(),
                    name,
                    content: trimmed.to_string(),
                    region,
                };
                
                self.nodes.insert(signature, node);
                println!("  0x{:06X} -> {}", signature, trimmed);
            }
        }
    }
    
    /// Collect all arrows (relationships)
    fn collect_arrows(&mut self, source: &str) {
        println!("\nCollecting arrows:");
        
        // Find function calls, uses, etc.
        for line in source.lines() {
            let trimmed = line.trim();
            let line_sig = self.calculate_signature(trimmed);
            
            // Function calls
            if trimmed.contains("(") && trimmed.contains(")") {
                // Look for function calls to our collected nodes
                for (target_sig, node) in &self.nodes {
                    if node.node_type == "fn" && trimmed.contains(&node.name) {
                        let from_region = self.signature_to_region(line_sig);
                        let to_region = self.signature_to_region(*target_sig);
                        
                        let arrow = Arrow {
                            to_signature: *target_sig,
                            arrow_type: "calls".to_string(),
                            is_cross_region: from_region != to_region,
                        };
                        
                        self.arrows.entry(line_sig).or_insert_with(Vec::new).push(arrow);
                        println!("  0x{:06X} --calls--> 0x{:06X} ({})", 
                                line_sig, target_sig, node.name);
                    }
                }
            }
            
            // Constant usage
            for (const_sig, node) in &self.nodes {
                if node.node_type == "const" && trimmed.contains(&node.name) && line_sig != *const_sig {
                    let from_region = self.signature_to_region(line_sig);
                    let to_region = self.signature_to_region(*const_sig);
                    
                    let arrow = Arrow {
                        to_signature: *const_sig,
                        arrow_type: "uses".to_string(),
                        is_cross_region: from_region != to_region,
                    };
                    
                    self.arrows.entry(line_sig).or_insert_with(Vec::new).push(arrow);
                    println!("  0x{:06X} --uses--> 0x{:06X} ({})", 
                            line_sig, const_sig, node.name);
                }
            }
        }
    }
    
    /// Allocate all signatures to regions
    fn allocate_regions(&mut self) {
        println!("\nAllocating regions:");
        
        for &signature in self.nodes.keys() {
            let region = self.signature_to_region(signature);
            self.regions.entry(region).or_insert_with(Vec::new).push(signature);
        }
        
        for region_id in self.regions.keys() {
            let sigs = &self.regions[region_id];
            println!("  Region {}: {} signatures", region_id, sigs.len());
        }
    }
    
    /// Generate System G summary
    fn generate_system_g(&self) {
        println!("\n=== SYSTEM G COMPLETE ===\n");
        
        println!("Compilation Unit: {}", self.compilation_unit);
        println!("Constants: {}", self.constants.len());
        println!("Nodes: {}", self.nodes.len());
        println!("Arrows: {}", self.arrows.values().map(|v| v.len()).sum::<usize>());
        println!("Regions: {}", self.regions.len());
        
        // Cross-region analysis
        let cross_region_arrows = self.arrows.values()
            .flatten()
            .filter(|arrow| arrow.is_cross_region)
            .count();
        
        println!("Cross-region arrows: {}", cross_region_arrows);
        
        // System G representation
        println!("\nSystem G = (Constants, Nodes, Arrows, Regions)");
        println!("  |Constants| = {}", self.constants.len());
        println!("  |Nodes| = {}", self.nodes.len());
        println!("  |Arrows| = {}", self.arrows.values().map(|v| v.len()).sum::<usize>());
        println!("  |Regions| = {}", self.regions.len());
        
        let total_elements = self.constants.len() + self.nodes.len() + 
                           self.arrows.values().map(|v| v.len()).sum::<usize>();
        println!("  Total elements: {}", total_elements);
    }
    
    // Helper functions
    fn extract_function_name(&self, line: &str) -> String {
        if let Some(fn_pos) = line.find("fn ") {
            let after_fn = &line[fn_pos + 3..];
            if let Some(paren_pos) = after_fn.find('(') {
                after_fn[..paren_pos].trim().to_string()
            } else {
                "unknown".to_string()
            }
        } else {
            "unknown".to_string()
        }
    }
    
    fn extract_struct_name(&self, line: &str) -> String {
        if let Some(struct_pos) = line.find("struct ") {
            let after_struct = &line[struct_pos + 7..];
            if let Some(space_pos) = after_struct.find(' ') {
                after_struct[..space_pos].trim().to_string()
            } else if let Some(brace_pos) = after_struct.find('{') {
                after_struct[..brace_pos].trim().to_string()
            } else {
                after_struct.trim().to_string()
            }
        } else {
            "unknown".to_string()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system_g = SystemG::new();
    
    // Collect our prime constants program as System G
    system_g.collect_compilation_unit("prime_constants_program.rs")?;
    system_g.generate_system_g();
    
    println!("\n✓ Complete compilation unit collected as System G");
    println!("✓ Constants + Nodes + Arrows + Regions = Complete system");
    
    Ok(())
}
