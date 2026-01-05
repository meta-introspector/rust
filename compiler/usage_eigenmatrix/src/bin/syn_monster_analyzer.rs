use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Syn codebase structural analyzer
#[derive(Debug, Clone)]
struct SynSignature {
    file_path: String,
    signature: u64,
    syn_patterns: Vec<String>,
    hir_mappings: Vec<String>,
    defid_count: u32,
}

#[derive(Debug)]
struct SynAnalyzer {
    signatures: Vec<SynSignature>,
    prime_basis: [u8; 8],
    total_files_analyzed: u32,
}

impl SynAnalyzer {
    fn new() -> Self {
        Self {
            signatures: Vec::new(),
            prime_basis: [2, 3, 5, 7, 11, 13, 17, 19],
            total_files_analyzed: 0,
        }
    }
    
    fn calculate_signature(&self, data: &str) -> u64 {
        let mut signature = 1u64;
        for (i, byte) in data.bytes().enumerate() {
            let prime_idx = i % 8;
            let prime = self.prime_basis[prime_idx] as u64;
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u64);
        }
        signature
    }
    
    fn analyze_syn_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let signature = self.calculate_signature(&content);
        
        let syn_patterns = self.extract_syn_patterns(&content);
        let hir_mappings = self.extract_hir_mappings(&content);
        let defid_count = self.count_defids(&content);
        
        let syn_sig = SynSignature {
            file_path: file_path.to_string_lossy().to_string(),
            signature,
            syn_patterns,
            hir_mappings,
            defid_count,
        };
        
        self.signatures.push(syn_sig);
        self.total_files_analyzed += 1;
        
        Ok(())
    }
    
    fn extract_syn_patterns(&self, content: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.contains("syn::") {
                patterns.push(trimmed.to_string());
            }
            
            if trimmed.contains("Token") && (trimmed.contains("struct") || trimmed.contains("enum")) {
                patterns.push(trimmed.to_string());
            }
            
            if trimmed.contains("parse") || trimmed.contains("Parse") {
                patterns.push(trimmed.to_string());
            }
            
            if trimmed.contains("Expr") || trimmed.contains("Item") || trimmed.contains("Type") {
                patterns.push(trimmed.to_string());
            }
        }
        
        patterns
    }
    
    fn extract_hir_mappings(&self, content: &str) -> Vec<String> {
        let mut mappings = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.contains("hir::") || trimmed.contains("HIR") {
                mappings.push(trimmed.to_string());
            }
            
            if trimmed.contains("DefId") {
                mappings.push(trimmed.to_string());
            }
            
            if trimmed.contains("rustc_") {
                mappings.push(trimmed.to_string());
            }
        }
        
        mappings
    }
    
    fn count_defids(&self, content: &str) -> u32 {
        content.matches("DefId").count() as u32
    }
    
    fn analyze_syn_directory(&mut self, syn_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let possible_paths = [
            syn_path,
            &format!("{}/src", syn_path),
        ];
        
        for path_str in &possible_paths {
            let path = Path::new(path_str);
            if path.exists() {
                self.analyze_directory_recursive(path)?;
                break;
            }
        }
        
        Ok(())
    }
    
    fn analyze_directory_recursive(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.analyze_directory_recursive(&path)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Err(e) = self.analyze_syn_file(&path) {
                    eprintln!("Error analyzing {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Syn Codebase Analysis Report\n\n");
        report.push_str("## Analysis Summary\n");
        report.push_str(&format!("- Files Analyzed: {}\n", self.total_files_analyzed));
        report.push_str(&format!("- Signatures Generated: {}\n", self.signatures.len()));
        
        let total_defids: u32 = self.signatures.iter().map(|s| s.defid_count).sum();
        report.push_str(&format!("- Total DefIds Found: {}\n\n", total_defids));
        
        report.push_str("## File Signatures\n");
        report.push_str("| File | Signature | Syn Patterns | HIR Mappings | DefIds |\n");
        report.push_str("|------|-----------|--------------|--------------|--------|\n");
        
        for sig in &self.signatures {
            let file_name = Path::new(&sig.file_path).file_name()
                .unwrap_or_default().to_string_lossy();
            
            report.push_str(&format!(
                "| `{}` | `0x{:016X}` | {} | {} | {} |\n",
                file_name,
                sig.signature,
                sig.syn_patterns.len(),
                sig.hir_mappings.len(),
                sig.defid_count
            ));
        }
        
        report
    }
}

fn main() {
    let mut analyzer = SynAnalyzer::new();
    
    match analyzer.analyze_syn_directory("./syn") {
        Ok(()) => println!("Analysis complete"),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    let report = analyzer.generate_report();
    
    match fs::write("syn_analysis_report.md", &report) {
        Ok(()) => println!("Report saved: syn_analysis_report.md"),
        Err(e) => eprintln!("Error saving report: {}", e),
    }
    
    println!("Files analyzed: {}", analyzer.total_files_analyzed);
    println!("Signatures generated: {}", analyzer.signatures.len());
}
