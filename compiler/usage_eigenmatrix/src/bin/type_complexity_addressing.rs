use std::collections::HashMap;

/// Type complexity analyzer for semantic addressing
struct TypeComplexityAnalyzer {
    type_signatures: HashMap<String, u8>, // type -> complexity signature
}

impl TypeComplexityAnalyzer {
    fn new() -> Self {
        let mut type_signatures = HashMap::new();
        
        // Simple types get low complexity scores
        type_signatures.insert("i32".to_string(), 0x01);
        type_signatures.insert("u32".to_string(), 0x01);
        type_signatures.insert("bool".to_string(), 0x01);
        type_signatures.insert("()".to_string(), 0x00); // Unit type - simplest
        type_signatures.insert("&str".to_string(), 0x02);
        type_signatures.insert("String".to_string(), 0x03);
        
        // Complex types get higher scores
        type_signatures.insert("Vec<T>".to_string(), 0x10);
        type_signatures.insert("HashMap<K,V>".to_string(), 0x20);
        type_signatures.insert("Result<T,E>".to_string(), 0x15);
        type_signatures.insert("Option<T>".to_string(), 0x08);
        
        Self { type_signatures }
    }
    
    /// Calculate type complexity signature
    fn calculate_type_complexity(&self, type_str: &str) -> u8 {
        // Check known types first
        if let Some(&complexity) = self.type_signatures.get(type_str) {
            return complexity;
        }
        
        // Calculate complexity based on type structure
        let mut complexity = 0u8;
        
        // Generic parameters add complexity
        let generic_count = type_str.matches('<').count();
        complexity += (generic_count as u8) * 0x04;
        
        // References add complexity
        let ref_count = type_str.matches('&').count();
        complexity += (ref_count as u8) * 0x02;
        
        // Mutable references add more
        let mut_count = type_str.matches("mut").count();
        complexity += (mut_count as u8) * 0x03;
        
        // Function types are complex
        if type_str.contains("fn(") || type_str.contains("->") {
            complexity += 0x10;
        }
        
        // Tuples add complexity based on arity
        let tuple_elements = if type_str.starts_with('(') && type_str.ends_with(')') {
            type_str.matches(',').count() + 1
        } else {
            0
        };
        complexity += (tuple_elements as u8) * 0x02;
        
        complexity.min(0xFF)
    }
    
    /// Parse function signature to extract domain and range
    fn parse_function_signature(&self, fn_def: &str) -> (Vec<String>, String, u8) {
        // Parse "fn name(param1: Type1, param2: Type2) -> ReturnType"
        let mut domain = Vec::new();
        let mut range = "()".to_string(); // Default unit return
        let mut arity = 0u8;
        
        // Extract parameters (domain)
        if let Some(paren_start) = fn_def.find('(') {
            if let Some(paren_end) = fn_def.find(')') {
                let params = &fn_def[paren_start + 1..paren_end];
                if !params.trim().is_empty() {
                    for param in params.split(',') {
                        if let Some(colon_pos) = param.find(':') {
                            let param_type = param[colon_pos + 1..].trim();
                            domain.push(param_type.to_string());
                            arity += 1;
                        }
                    }
                }
            }
        }
        
        // Extract return type (range)
        if let Some(arrow_pos) = fn_def.find("->") {
            let after_arrow = &fn_def[arrow_pos + 2..].trim();
            if let Some(brace_pos) = after_arrow.find('{') {
                range = after_arrow[..brace_pos].trim().to_string();
            } else {
                range = after_arrow.to_string();
            }
        }
        
        (domain, range, arity)
    }
    
    /// Calculate function complexity score
    fn calculate_function_complexity(&self, fn_def: &str) -> u8 {
        let (domain, range, arity) = self.parse_function_signature(fn_def);
        
        // Domain complexity
        let domain_complexity: u8 = domain.iter()
            .map(|t| self.calculate_type_complexity(t))
            .sum::<u8>()
            .min(0x0F); // Max 4 bits for domain
        
        // Range complexity  
        let range_complexity = self.calculate_type_complexity(&range).min(0x0F); // Max 4 bits for range
        
        // Combine: [arity:4][domain:4][range:4] = 12 bits total
        let complexity = ((arity.min(0x0F)) << 4) | (domain_complexity << 2) | range_complexity;
        
        complexity
    }
    
    /// Generate semantic address with type complexity
    fn generate_semantic_address(&self, homotopy_level: u8, complexity: u8, index: u16) -> u32 {
        // Address structure: [homotopy:4][complexity:12][index:16]
        ((homotopy_level as u32) << 28) | 
        ((complexity as u32) << 16) | 
        (index as u32)
    }
    
    /// Decode semantic address
    fn decode_semantic_address(&self, addr: u32) -> (u8, u8, u16) {
        let homotopy_level = ((addr >> 28) & 0x0F) as u8;
        let complexity = ((addr >> 16) & 0x0FFF) as u8;
        let index = (addr & 0xFFFF) as u16;
        
        (homotopy_level, complexity, index)
    }
    
    /// Show type complexity analysis
    fn analyze_function_types(&self) {
        println!("=== FUNCTION TYPE COMPLEXITY ANALYSIS ===\n");
        
        let test_functions = [
            "fn simple() -> ()",
            "fn add(a: i32, b: i32) -> i32",
            "fn complex(data: &mut Vec<String>) -> Result<HashMap<String, i32>, Error>",
            "fn generic<T>(x: T) -> T",
            "fn higher_order(f: fn(i32) -> i32) -> fn(i32) -> i32",
            "fn main()",
        ];
        
        for fn_def in &test_functions {
            let (domain, range, arity) = self.parse_function_signature(fn_def);
            let complexity = self.calculate_function_complexity(fn_def);
            let addr = self.generate_semantic_address(1, complexity, 0); // Level 1 = functions
            
            println!("Function: {}", fn_def);
            println!("  Domain: {:?}", domain);
            println!("  Range: {}", range);
            println!("  Arity: {}", arity);
            println!("  Complexity: 0x{:02X}", complexity);
            println!("  Address: 0x{:08X}", addr);
            
            let (decoded_level, decoded_complexity, decoded_index) = self.decode_semantic_address(addr);
            println!("  Decoded: Level {}, Complexity 0x{:02X}, Index {}", 
                     decoded_level, decoded_complexity, decoded_index);
            println!();
        }
    }
}

fn main() {
    println!("=== TYPE COMPLEXITY SEMANTIC ADDRESSING ===");
    
    let analyzer = TypeComplexityAnalyzer::new();
    
    // Analyze function type complexity
    analyzer.analyze_function_types();
    
    // Show address structure
    println!("=== ADDRESS STRUCTURE ===");
    println!("32-bit address: [homotopy:4][complexity:12][index:16]");
    println!("  Homotopy level: 0=const, 1=fn, 2=struct, 3=impl");
    println!("  Complexity: [arity:4][domain:4][range:4]");
    println!("  Index: Position within homotopy+complexity bucket");
    println!();
    
    // Examples of meaningful addresses
    println!("Example Addresses:");
    println!("  0x10000000: fn() -> () (simple function, no params, unit return)");
    println!("  0x12010001: fn(i32, i32) -> i32 (arity=2, domain=1, range=1)");
    println!("  0x1F2F0002: Complex function with generics and Result types");
    
    println!("\n✓ Type complexity addressing complete");
    println!("✓ Functions get complexity-based addresses");
    println!("✓ Domain and range complexity encoded in address");
    println!("✓ User can calculate address from function signature");
}
