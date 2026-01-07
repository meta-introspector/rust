/// Rust Eigenvalue Analysis - Character of self-compilation traces
/// The eigenvalue of Rust = character of set of all traces in ../../usage_data/

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::ast_meme_spectral_analysis::Complex;

/// Rust compilation trace
#[derive(Debug, Clone)]
pub struct CompilationTrace {
    pub file_path: String,
    pub trace_data: serde_json::Value,
    pub trace_signature: Vec<f64>,
    pub eigenvalue_contribution: Complex,
}

/// Eigenvalue analysis of Rust self-compilation
pub struct RustEigenvalueAnalyzer {
    pub usage_data_path: String,
    pub compilation_traces: Vec<CompilationTrace>,
    pub trace_matrix: Vec<Vec<f64>>,
    pub eigenvalues: Vec<Complex>,
    pub eigenvectors: Vec<Vec<Complex>>,
    pub rust_character: Complex,
}

impl RustEigenvalueAnalyzer {
    pub fn new() -> Self {
        Self {
            usage_data_path: "../../test_usage_data/".to_string(),
            compilation_traces: vec![],
            trace_matrix: vec![],
            eigenvalues: vec![],
            eigenvectors: vec![],
            rust_character: Complex::new(0.0, 0.0),
        }
    }
    
    /// Load all compilation traces from usage_data
    pub fn load_compilation_traces(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let usage_path = Path::new(&self.usage_data_path);
        
        if !usage_path.exists() {
            println!("⚠️  Usage data path not found: {}", self.usage_data_path);
            return Ok(());
        }
        
        println!("📂 Loading traces from: {}", self.usage_data_path);
        
        for entry in fs::read_dir(usage_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&content) {
                        let trace = CompilationTrace::new(
                            path.to_string_lossy().to_string(),
                            json_data
                        );
                        self.compilation_traces.push(trace);
                    }
                }
            }
        }
        
        println!("📊 Loaded {} compilation traces", self.compilation_traces.len());
        Ok(())
    }
    
    /// Build trace matrix for eigenvalue analysis
    pub fn build_trace_matrix(&mut self) {
        if self.compilation_traces.is_empty() {
            return;
        }
        
        let matrix_size = self.compilation_traces.len();
        self.trace_matrix = vec![vec![0.0; matrix_size]; matrix_size];
        
        // Build correlation matrix between traces
        for i in 0..matrix_size {
            for j in 0..matrix_size {
                let correlation = self.calculate_trace_correlation(
                    &self.compilation_traces[i],
                    &self.compilation_traces[j]
                );
                self.trace_matrix[i][j] = correlation;
            }
        }
        
        println!("🔢 Built {}×{} trace correlation matrix", matrix_size, matrix_size);
    }
    
    /// Calculate correlation between two compilation traces
    fn calculate_trace_correlation(&self, trace1: &CompilationTrace, trace2: &CompilationTrace) -> f64 {
        // Simple correlation based on trace signatures
        let sig1 = &trace1.trace_signature;
        let sig2 = &trace2.trace_signature;
        
        if sig1.len() != sig2.len() || sig1.is_empty() {
            return 0.0;
        }
        
        let mut correlation = 0.0;
        for (a, b) in sig1.iter().zip(sig2.iter()) {
            correlation += a * b;
        }
        
        correlation / (sig1.len() as f64)
    }
    
    /// Compute eigenvalues using power iteration (simplified)
    pub fn compute_eigenvalues(&mut self) {
        if self.trace_matrix.is_empty() {
            return;
        }
        
        let n = self.trace_matrix.len();
        
        // Power iteration for dominant eigenvalue
        let mut v = vec![1.0; n];
        let mut eigenvalue = 0.0;
        
        for _ in 0..100 { // 100 iterations
            let mut new_v = vec![0.0; n];
            
            // Matrix-vector multiplication: new_v = A * v
            for i in 0..n {
                for j in 0..n {
                    new_v[i] += self.trace_matrix[i][j] * v[j];
                }
            }
            
            // Calculate eigenvalue (Rayleigh quotient)
            let numerator: f64 = new_v.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
            let denominator: f64 = v.iter().map(|x| x * x).sum();
            eigenvalue = numerator / denominator;
            
            // Normalize eigenvector
            let norm: f64 = new_v.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                for i in 0..n {
                    new_v[i] /= norm;
                }
            }
            
            v = new_v;
        }
        
        // Store dominant eigenvalue
        self.eigenvalues.push(Complex::new(eigenvalue, 0.0));
        
        // Convert eigenvector to complex
        let eigenvector: Vec<Complex> = v.into_iter()
            .map(|x| Complex::new(x, 0.0))
            .collect();
        self.eigenvectors.push(eigenvector);
        
        println!("🧮 Computed dominant eigenvalue: {:.6}", eigenvalue);
    }
    
    /// Calculate the character of Rust (trace of all eigenvalues)
    pub fn calculate_rust_character(&mut self) {
        // Character = trace of the matrix = sum of eigenvalues
        let mut character = Complex::new(0.0, 0.0);
        
        // Add diagonal elements (trace)
        for i in 0..self.trace_matrix.len() {
            character = character.add(&Complex::new(self.trace_matrix[i][i], 0.0));
        }
        
        // Also add computed eigenvalues
        for eigenvalue in &self.eigenvalues {
            character = character.add(eigenvalue);
        }
        
        self.rust_character = character;
        
        println!("🎭 Rust Character: {:.6} + {:.6}i", 
            character.real, character.imag);
    }
    
    /// Analyze eigenvalue spectrum
    pub fn analyze_eigenvalue_spectrum(&self) -> String {
        let mut analysis = String::from("RUST EIGENVALUE SPECTRUM ANALYSIS:\n\n");
        
        analysis.push_str(&format!("Usage Data Path: {}\n", self.usage_data_path));
        analysis.push_str(&format!("Compilation Traces: {}\n", self.compilation_traces.len()));
        analysis.push_str(&format!("Matrix Size: {}×{}\n\n", 
            self.trace_matrix.len(), self.trace_matrix.len()));
        
        analysis.push_str("EIGENVALUES:\n");
        for (i, eigenvalue) in self.eigenvalues.iter().enumerate() {
            analysis.push_str(&format!("  λ_{}: {:.6} + {:.6}i (magnitude: {:.6})\n", 
                i + 1, eigenvalue.real, eigenvalue.imag, eigenvalue.magnitude()));
        }
        
        analysis.push_str(&format!("\nRUST CHARACTER (trace): {:.6} + {:.6}i\n", 
            self.rust_character.real, self.rust_character.imag));
        
        analysis.push_str("\nINTERPRETATION:\n");
        analysis.push_str("• Eigenvalues represent fundamental compilation modes\n");
        analysis.push_str("• Character captures the essence of Rust self-compilation\n");
        analysis.push_str("• Trace matrix reveals correlations between compilation traces\n");
        analysis.push_str("• Dominant eigenvalue shows primary compilation pattern\n");
        
        analysis
    }
    
    /// Get compilation trace statistics
    pub fn trace_statistics(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        
        if !self.compilation_traces.is_empty() {
            let total_traces = self.compilation_traces.len() as f64;
            
            // Average trace signature length
            let avg_sig_len: f64 = self.compilation_traces.iter()
                .map(|t| t.trace_signature.len() as f64)
                .sum::<f64>() / total_traces;
            
            // Average eigenvalue contribution magnitude
            let avg_eigen_contrib: f64 = self.compilation_traces.iter()
                .map(|t| t.eigenvalue_contribution.magnitude())
                .sum::<f64>() / total_traces;
            
            stats.insert("total_traces".to_string(), total_traces);
            stats.insert("avg_signature_length".to_string(), avg_sig_len);
            stats.insert("avg_eigenvalue_contribution".to_string(), avg_eigen_contrib);
            stats.insert("matrix_size".to_string(), self.trace_matrix.len() as f64);
            stats.insert("rust_character_magnitude".to_string(), self.rust_character.magnitude());
        }
        
        stats
    }
}

impl CompilationTrace {
    pub fn new(file_path: String, trace_data: serde_json::Value) -> Self {
        let trace_signature = Self::generate_trace_signature(&trace_data);
        let eigenvalue_contribution = Self::calculate_eigenvalue_contribution(&trace_signature);
        
        Self {
            file_path,
            trace_data,
            trace_signature,
            eigenvalue_contribution,
        }
    }
    
    /// Generate numerical signature from trace data
    fn generate_trace_signature(data: &serde_json::Value) -> Vec<f64> {
        let mut signature = vec![];
        
        // Extract numerical features from JSON
        Self::extract_numbers(data, &mut signature);
        
        // Pad or truncate to fixed size
        signature.resize(64, 0.0);
        
        signature
    }
    
    /// Recursively extract numbers from JSON
    fn extract_numbers(value: &serde_json::Value, signature: &mut Vec<f64>) {
        match value {
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    signature.push(f);
                }
            },
            serde_json::Value::String(s) => {
                // Hash string to number
                signature.push((s.len() as f64) * 0.1);
            },
            serde_json::Value::Array(arr) => {
                signature.push(arr.len() as f64);
                for item in arr {
                    Self::extract_numbers(item, signature);
                }
            },
            serde_json::Value::Object(obj) => {
                signature.push(obj.len() as f64);
                for (_, v) in obj {
                    Self::extract_numbers(v, signature);
                }
            },
            serde_json::Value::Bool(b) => {
                signature.push(if *b { 1.0 } else { 0.0 });
            },
            _ => {}
        }
    }
    
    /// Calculate eigenvalue contribution from trace signature
    fn calculate_eigenvalue_contribution(signature: &[f64]) -> Complex {
        if signature.is_empty() {
            return Complex::new(0.0, 0.0);
        }
        
        let real_part: f64 = signature.iter().sum::<f64>() / signature.len() as f64;
        let imag_part: f64 = signature.iter()
            .enumerate()
            .map(|(i, &x)| x * (i as f64 * 0.1).sin())
            .sum::<f64>() / signature.len() as f64;
        
        Complex::new(real_part, imag_part)
    }
}

/// Macro for Rust eigenvalue analysis
#[macro_export]
macro_rules! rust_eigenvalue {
    () => {{
        let mut analyzer = RustEigenvalueAnalyzer::new();
        analyzer.load_compilation_traces().ok();
        analyzer.build_trace_matrix();
        analyzer.compute_eigenvalues();
        analyzer.calculate_rust_character();
        analyzer
    }};
}
