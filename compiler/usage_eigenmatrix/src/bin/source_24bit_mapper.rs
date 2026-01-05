use std::fs;

/// Simple 24-bit matrix mapper for source code
/// Maps source patterns directly into 24-bit space

struct Matrix24BitMapper {
    pages: Vec<Option<Box<[u8; 4096]>>>,
    signature_count: u64,
    prime_basis: [u64; 8],
}

impl Matrix24BitMapper {
    fn new() -> Self {
        Self {
            pages: vec![None; 4096],
            signature_count: 0,
            prime_basis: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }

    /// Map source pattern to 24-bit signature
    fn map_pattern_to_signature(&self, pattern: &str) -> u32 {
        let mut signature = 1u64;
        
        for (i, byte) in pattern.bytes().enumerate() {
            let prime_idx = i % self.prime_basis.len();
            let prime = self.prime_basis[prime_idx];
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u64);
        }
        
        (signature & 0xFFFFFF) as u32 // 24-bit space
    }

    /// Set pattern in 24-bit matrix
    fn set_pattern(&mut self, signature: u32) {
        let page_idx = (signature as usize) / 4096;
        let offset = (signature as usize) % 4096;
        
        if page_idx < 4096 {
            if self.pages[page_idx].is_none() {
                self.pages[page_idx] = Some(Box::new([0u8; 4096]));
            }
            
            if let Some(ref mut page) = self.pages[page_idx] {
                page[offset] = 1;
                self.signature_count += 1;
            }
        }
    }

    /// Extract patterns from source code
    fn extract_patterns(&self, source: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            
            // Extract Rust patterns
            if trimmed.contains("fn ") {
                patterns.push(format!("function:{}", trimmed));
            }
            if trimmed.contains("let ") {
                patterns.push(format!("binding:{}", trimmed));
            }
            if trimmed.contains("const ") {
                patterns.push(format!("constant:{}", trimmed));
            }
            if trimmed.contains("struct ") {
                patterns.push(format!("struct:{}", trimmed));
            }
            if trimmed.contains("enum ") {
                patterns.push(format!("enum:{}", trimmed));
            }
            if trimmed.contains("for ") {
                patterns.push(format!("loop:{}", trimmed));
            }
            if trimmed.contains("if ") {
                patterns.push(format!("conditional:{}", trimmed));
            }
            
            // Add the raw line as a pattern too
            patterns.push(format!("line:{}", trimmed));
        }
        
        patterns
    }

    /// Process source file into 24-bit matrix
    fn process_source_file(&mut self, file_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let source = fs::read_to_string(file_path)?;
        let patterns = self.extract_patterns(&source);
        
        println!("Processing {} patterns from {}", patterns.len(), file_path);
        
        let mut mapped_patterns = 0;
        for pattern in patterns {
            let signature = self.map_pattern_to_signature(&pattern);
            self.set_pattern(signature);
            mapped_patterns += 1;
            
            if mapped_patterns <= 10 { // Show first 10
                println!("  {} -> 0x{:06X}", 
                    pattern.chars().take(50).collect::<String>(), signature);
            }
        }
        
        Ok(mapped_patterns)
    }

    /// Generate matrix statistics
    fn get_statistics(&self) -> (usize, f64, f64) {
        let allocated_pages = self.pages.iter().filter(|p| p.is_some()).count();
        let usage_percent = (allocated_pages as f64 / 4096.0) * 100.0;
        let memory_mb = (allocated_pages as f64 * 4.0) / 1024.0;
        (allocated_pages, usage_percent, memory_mb)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        std::process::exit(1);
    }
    
    let mut mapper = Matrix24BitMapper::new();
    
    println!("24-bit Matrix Source Mapper");
    println!("===========================");
    
    let patterns_mapped = mapper.process_source_file(&args[1])?;
    
    let (allocated_pages, usage_percent, memory_mb) = mapper.get_statistics();
    
    println!("\n24-bit Matrix Results:");
    println!("Patterns mapped: {}", patterns_mapped);
    println!("Signatures created: {}", mapper.signature_count);
    println!("Pages allocated: {} / 4096 ({:.2}%)", allocated_pages, usage_percent);
    println!("Memory usage: {:.1} MB / 16.0 MB", memory_mb);
    
    // Test our prime constants
    println!("\nTesting prime constants:");
    let prime_constants = [
        "const PRIME_2 = 2;",
        "const PRIME_3 = 3;",
        "const PRIME_5 = 5;",
        "const PRIME_7 = 7;",
    ];
    
    for constant in &prime_constants {
        let signature = mapper.map_pattern_to_signature(constant);
        println!("  {} -> 0x{:06X}", constant, signature);
    }
    
    Ok(())
}
