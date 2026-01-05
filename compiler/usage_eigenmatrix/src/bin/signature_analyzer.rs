use std::collections::HashMap;
use std::fs;

/// Code signature analyzer using mathematical group operations
struct SignatureAnalyzer {
    signatures: HashMap<String, u64>,
    prime_basis: [u64; 8],
}

#[derive(Debug, Clone)]
struct AnalysisResult {
    code: String,
    signature: u64,
    structural_elements: Vec<String>,
}

impl SignatureAnalyzer {
    fn new() -> Self {
        Self {
            signatures: HashMap::new(),
            prime_basis: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }

    /// Calculate signature based on code structure using group operations
    fn calculate_signature(&self, code: &str) -> u64 {
        let mut signature = 1u64;
        
        // Analyze syntactic structure
        let tokens = self.tokenize(code);
        
        for (i, token) in tokens.iter().enumerate() {
            let prime_idx = i % self.prime_basis.len();
            let prime = self.prime_basis[prime_idx];
            
            // Map token to group element
            let token_value = self.token_to_group_element(token);
            signature = signature.wrapping_mul(prime).wrapping_add(token_value);
        }
        
        signature
    }

    /// Extract structural tokens from code
    fn tokenize(&self, code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            
            // Extract structural elements
            if trimmed.contains("fn ") {
                tokens.push("function".to_string());
            }
            if trimmed.contains("let ") {
                tokens.push("binding".to_string());
            }
            if trimmed.contains("for ") {
                tokens.push("loop".to_string());
            }
            if trimmed.contains("if ") {
                tokens.push("conditional".to_string());
            }
            if trimmed.contains("struct ") {
                tokens.push("structure".to_string());
            }
            if trimmed.contains("enum ") {
                tokens.push("enumeration".to_string());
            }
        }
        
        tokens
    }

    /// Map token to mathematical group element
    fn token_to_group_element(&self, token: &str) -> u64 {
        match token {
            "function" => 2,
            "binding" => 3,
            "loop" => 5,
            "conditional" => 7,
            "structure" => 11,
            "enumeration" => 13,
            _ => 1,
        }
    }

    /// Analyze code and generate signature
    fn analyze(&mut self, code: &str, name: &str) -> AnalysisResult {
        let signature = self.calculate_signature(code);
        let structural_elements = self.tokenize(code);
        
        self.signatures.insert(name.to_string(), signature);
        
        AnalysisResult {
            code: code.to_string(),
            signature,
            structural_elements,
        }
    }

    /// Generate report of analysis results
    fn generate_report(&self, results: &[AnalysisResult]) -> String {
        let mut report = String::new();
        
        report.push_str("# Code Signature Analysis Report\n\n");
        report.push_str("## Structural Analysis\n\n");
        
        report.push_str("| Code | Signature | Elements |\n");
        report.push_str("|------|-----------|----------|\n");
        
        for result in results {
            let code_preview = result.code.chars().take(30).collect::<String>().replace('\n', " ");
            report.push_str(&format!(
                "| `{}...` | `0x{:016X}` | {} |\n",
                code_preview,
                result.signature,
                result.structural_elements.len()
            ));
        }
        
        report.push_str("\n## Signature Details\n\n");
        
        for (i, result) in results.iter().enumerate() {
            report.push_str(&format!("### Analysis {}\n", i + 1));
            report.push_str(&format!("**Signature**: `0x{:016X}`\n\n", result.signature));
            report.push_str("**Structural Elements**:\n");
            for element in &result.structural_elements {
                report.push_str(&format!("- {}\n", element));
            }
            report.push_str("\n");
        }
        
        report
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = SignatureAnalyzer::new();
    let mut results = Vec::new();
    
    // Analyze simple prime sieve
    if let Ok(code) = fs::read_to_string("simple_prime_sieve.rs") {
        let result = analyzer.analyze(&code, "prime_sieve");
        results.push(result);
    }
    
    // Analyze test expressions
    let test_expressions = [
        "fn main() { let x = 1 + 2; }",
        "fn factorial(n: u32) -> u32 { if n <= 1 { 1 } else { n * factorial(n-1) } }",
        "struct Point { x: i32, y: i32 }",
        "enum Color { Red, Green, Blue }",
    ];
    
    for (i, expr) in test_expressions.iter().enumerate() {
        let result = analyzer.analyze(expr, &format!("test_{}", i));
        results.push(result);
    }
    
    // Generate and save report
    let report = analyzer.generate_report(&results);
    fs::write("signature_analysis_report.md", &report)?;
    
    println!("Analysis complete. {} signatures generated.", results.len());
    println!("Report saved to: signature_analysis_report.md");
    
    Ok(())
}
