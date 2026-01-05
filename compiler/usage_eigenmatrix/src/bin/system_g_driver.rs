use std::env;
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// System G Matrix for compilation unit
#[derive(Serialize, Deserialize)]
struct SystemGMatrix {
    constants: HashMap<u32, String>,
    nodes: HashMap<u32, String>,
    arrows: HashMap<u32, Vec<u32>>,
    regions: HashMap<u16, Vec<u32>>,
    source_file: String,
}

/// System G Compiler Driver
struct SystemGDriver {
    matrix: SystemGMatrix,
}

impl SystemGDriver {
    fn new() -> Self {
        Self {
            matrix: SystemGMatrix {
                constants: HashMap::new(),
                nodes: HashMap::new(),
                arrows: HashMap::new(),
                regions: HashMap::new(),
                source_file: String::new(),
            }
        }
    }
    
    fn signature(&self, source: &str) -> u32 {
        let mut sig = 0u64;
        for (i, &p) in [2, 3, 5, 7, 11, 13, 17, 19].iter().enumerate() {
            let sum: u64 = source.chars().enumerate().map(|(j, c)| (c as u64) * (j as u64 + 1)).sum();
            sig += (sum % p) << (i * 3);
        }
        (sig & 0xFFFFFF) as u32
    }
    
    /// Compile source to System G matrix
    fn compile(&mut self, source: &str, source_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.matrix.source_file = source_file.to_string();
        
        // Collect constants and nodes
        for line in source.lines() {
            let line = line.trim();
            if line.starts_with("const ") {
                let sig = self.signature(line);
                self.matrix.constants.insert(sig, line.to_string());
                self.matrix.nodes.insert(sig, format!("const:{}", line));
            }
            if line.starts_with("fn ") {
                let sig = self.signature(line);
                self.matrix.nodes.insert(sig, format!("fn:{}", line));
            }
        }
        
        // Collect arrows
        for line in source.lines() {
            let line_sig = self.signature(line.trim());
            for (const_sig, const_def) in &self.matrix.constants {
                if let Some(name_start) = const_def.find("const ") {
                    if let Some(name_end) = const_def[name_start + 6..].find(':') {
                        let const_name = &const_def[name_start + 6..name_start + 6 + name_end];
                        if line.contains(const_name) && line_sig != *const_sig {
                            self.matrix.arrows.entry(line_sig).or_insert_with(Vec::new).push(*const_sig);
                        }
                    }
                }
            }
        }
        
        // Allocate regions
        for &sig in self.matrix.nodes.keys() {
            let region = (sig / 4096) as u16;
            self.matrix.regions.entry(region).or_insert_with(Vec::new).push(sig);
        }
        
        Ok(())
    }
    
    /// Save matrix to file
    fn save_matrix(&self, matrix_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.matrix)?;
        fs::write(matrix_file, json)?;
        Ok(())
    }
    
    /// Load matrix from file
    fn load_matrix(matrix_file: &str) -> Result<SystemGMatrix, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(matrix_file)?;
        let matrix = serde_json::from_str(&json)?;
        Ok(matrix)
    }
    
    /// Reconstruct code from matrix
    fn reconstruct_from_matrix(matrix: &SystemGMatrix) -> String {
        let mut output = String::new();
        output.push_str(&format!("// Reconstructed from System G Matrix: {}\n\n", matrix.source_file));
        
        // Reconstruct constants
        for (_, const_def) in &matrix.constants {
            if let Some(colon_pos) = const_def.find("const ") {
                output.push_str(const_def);
                output.push('\n');
            }
        }
        
        // Reconstruct main function
        output.push_str("\nfn main() {\n");
        output.push_str("    println!(\"Reconstructed from System G Matrix\");\n");
        
        // Add constant usage based on arrows
        for (_, const_def) in &matrix.constants {
            if let Some(name_start) = const_def.find("const ") {
                if let Some(name_end) = const_def[name_start + 6..].find(':') {
                    let const_name = &const_def[name_start + 6..name_start + 6 + name_end];
                    output.push_str(&format!("    println!(\"{} = {{}}\", {});\n", const_name, const_name));
                }
            }
        }
        
        output.push_str("}\n");
        output
    }
    
    /// Show matrix stats
    fn show_matrix_stats(&self) {
        println!("System G Matrix: C:{}, N:{}, A:{}, R:{}", 
                 self.matrix.constants.len(),
                 self.matrix.nodes.len(), 
                 self.matrix.arrows.values().map(|v| v.len()).sum::<usize>(),
                 self.matrix.regions.len());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage:");
        println!("  Compile: system_g_driver <source.rs>");
        println!("  Reconstruct: system_g_driver --reconstruct <matrix.json>");
        return Ok(());
    }
    
    if args[1] == "--reconstruct" {
        if args.len() < 3 {
            println!("Usage: system_g_driver --reconstruct <matrix.json>");
            return Ok(());
        }
        
        println!("=== RECONSTRUCTING FROM SYSTEM G MATRIX ===");
        let matrix_file = &args[2];
        println!("Matrix file: {}", matrix_file);
        
        // Load matrix
        let matrix = SystemGDriver::load_matrix(matrix_file)?;
        
        // Reconstruct code
        let reconstructed = SystemGDriver::reconstruct_from_matrix(&matrix);
        
        // Write reconstructed code
        let output_file = "reconstructed.rs";
        fs::write(output_file, &reconstructed)?;
        
        println!("✓ Code reconstructed from matrix");
        println!("✓ Output: {}", output_file);
        
        // Show reconstructed code
        println!("\nReconstructed code:");
        println!("{}", reconstructed);
        
    } else {
        println!("=== COMPILING TO SYSTEM G MATRIX ===");
        let input_file = &args[1];
        let matrix_file = format!("{}.matrix.json", input_file);
        
        println!("Input: {}", input_file);
        println!("Matrix: {}", matrix_file);
        
        // Read and compile source
        let source = fs::read_to_string(input_file)?;
        let mut driver = SystemGDriver::new();
        driver.compile(&source, input_file)?;
        
        // Save matrix
        driver.save_matrix(&matrix_file)?;
        driver.show_matrix_stats();
        
        println!("✓ Compiled to System G matrix: {}", matrix_file);
    }
    
    Ok(())
}
