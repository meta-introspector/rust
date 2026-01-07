/// AST Meme Spectral Analysis - Each AST is a meme in frequency domain
/// Spectral decomposition of AST patterns across the meme space

use std::collections::HashMap;
use crate::ast_resource_estimation::{ASTNodeType, AnnotatedASTNode, OperationCost};

/// Complex number for spectral analysis
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

/// AST Meme - Each AST node is a meme with frequency characteristics
#[derive(Debug, Clone)]
pub struct ASTMeme {
    pub ast_node: AnnotatedASTNode,
    pub meme_frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub spectral_signature: Vec<Complex>,
    pub meme_id: String,
}

/// Spectral analysis of AST meme domain
pub struct ASTMemeSpectralAnalyzer {
    pub memes: Vec<ASTMeme>,
    pub frequency_domain: HashMap<String, Vec<Complex>>,
    pub spectral_basis: Vec<f64>,
    pub dominant_frequencies: Vec<f64>,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
    
    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }
    
    pub fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
    
    pub fn multiply(&self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real
        )
    }
}

impl ASTMeme {
    pub fn new(ast_node: AnnotatedASTNode) -> Self {
        let meme_frequency = Self::calculate_meme_frequency(&ast_node);
        let amplitude = Self::calculate_amplitude(&ast_node);
        let phase = Self::calculate_phase(&ast_node);
        let meme_id = Self::generate_meme_id(&ast_node);
        
        Self {
            ast_node,
            meme_frequency,
            amplitude,
            phase,
            spectral_signature: vec![],
            meme_id,
        }
    }
    
    /// Calculate meme frequency based on AST structure
    fn calculate_meme_frequency(ast: &AnnotatedASTNode) -> f64 {
        let base_freq = match &ast.node_type {
            ASTNodeType::Literal(_) => 1.0,
            ASTNodeType::Variable(_) => 2.0,
            ASTNodeType::BinaryOp(_) => 5.0,
            ASTNodeType::FunctionCall(_) => 10.0,
            ASTNodeType::Assignment => 3.0,
            ASTNodeType::IfStatement => 7.0,
            ASTNodeType::Loop => 15.0,
            ASTNodeType::Return => 4.0,
            ASTNodeType::Block => 8.0,
            ASTNodeType::Function(_) => 20.0,
            ASTNodeType::Module(_) => 50.0,
            ASTNodeType::Crate(_) => 100.0,
            ASTNodeType::Repository(_) => 200.0,
            ASTNodeType::Ecosystem(_) => 500.0,
        };
        
        // Modulate frequency by complexity and children
        let complexity_factor = ast.accumulated_cost.complexity_score / 10.0;
        let children_factor = ast.children.len() as f64 * 0.5;
        
        base_freq * (1.0 + complexity_factor) * (1.0 + children_factor)
    }
    
    /// Calculate amplitude based on resource cost
    fn calculate_amplitude(ast: &AnnotatedASTNode) -> f64 {
        let memory_factor = ast.accumulated_cost.memory_bytes as f64 / 1024.0; // KB
        let cpu_factor = ast.accumulated_cost.cpu_cycles as f64 / 1000.0;
        
        (memory_factor + cpu_factor).sqrt()
    }
    
    /// Calculate phase based on AST structure hash
    fn calculate_phase(ast: &AnnotatedASTNode) -> f64 {
        let type_hash = match &ast.node_type {
            ASTNodeType::Literal(s) => s.len(),
            ASTNodeType::Variable(s) => s.len() * 2,
            ASTNodeType::BinaryOp(s) => s.len() * 3,
            ASTNodeType::FunctionCall(s) => s.len() * 4,
            ASTNodeType::Function(s) => s.len() * 5,
            ASTNodeType::Module(s) => s.len() * 6,
            ASTNodeType::Crate(s) => s.len() * 7,
            ASTNodeType::Repository(s) => s.len() * 8,
            ASTNodeType::Ecosystem(s) => s.len() * 9,
            _ => 1,
        };
        
        (type_hash as f64 * 0.1) % (2.0 * std::f64::consts::PI)
    }
    
    /// Generate unique meme ID
    fn generate_meme_id(ast: &AnnotatedASTNode) -> String {
        format!("meme_{:x}", 
            ast.accumulated_cost.cpu_cycles ^ 
            ast.accumulated_cost.memory_bytes ^ 
            (ast.accumulated_cost.complexity_score as u64))
    }
    
    /// Generate spectral signature using DFT
    pub fn generate_spectral_signature(&mut self, sample_points: usize) {
        let mut signature = vec![];
        
        for k in 0..sample_points {
            let mut sum = Complex::new(0.0, 0.0);
            
            for n in 0..sample_points {
                let angle = -2.0 * std::f64::consts::PI * (k as f64) * (n as f64) / (sample_points as f64);
                let twiddle = Complex::new(angle.cos(), angle.sin());
                
                // Sample the meme at different points
                let sample_value = self.sample_meme_at_point(n as f64 / sample_points as f64);
                let sample_complex = Complex::new(sample_value, 0.0);
                
                sum = sum.add(&sample_complex.multiply(&twiddle));
            }
            
            signature.push(sum);
        }
        
        self.spectral_signature = signature;
    }
    
    /// Sample the meme at a specific point in its domain
    fn sample_meme_at_point(&self, t: f64) -> f64 {
        // Generate meme signal: amplitude * sin(frequency * t + phase)
        self.amplitude * (self.meme_frequency * t + self.phase).sin()
    }
}

impl ASTMemeSpectralAnalyzer {
    pub fn new() -> Self {
        Self {
            memes: vec![],
            frequency_domain: HashMap::new(),
            spectral_basis: vec![],
            dominant_frequencies: vec![],
        }
    }
    
    /// Add AST meme to the analysis
    pub fn add_ast_meme(&mut self, ast_node: AnnotatedASTNode) {
        let mut meme = ASTMeme::new(ast_node);
        meme.generate_spectral_signature(64); // 64-point DFT
        
        self.memes.push(meme);
    }
    
    /// Perform spectral analysis of all memes
    pub fn analyze_meme_spectrum(&mut self) {
        // Build frequency domain representation
        for meme in &self.memes {
            self.frequency_domain.insert(
                meme.meme_id.clone(), 
                meme.spectral_signature.clone()
            );
        }
        
        // Find dominant frequencies
        self.find_dominant_frequencies();
        
        // Build spectral basis
        self.build_spectral_basis();
    }
    
    /// Find dominant frequencies across all memes
    fn find_dominant_frequencies(&mut self) {
        let mut frequency_power = HashMap::new();
        
        for meme in &self.memes {
            for (i, complex) in meme.spectral_signature.iter().enumerate() {
                let power = complex.magnitude().powi(2);
                *frequency_power.entry(i).or_insert(0.0) += power;
            }
        }
        
        // Sort by power and take top frequencies
        let mut freq_power_pairs: Vec<_> = frequency_power.into_iter().collect();
        freq_power_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        self.dominant_frequencies = freq_power_pairs.into_iter()
            .take(10)
            .map(|(freq_bin, _)| freq_bin as f64)
            .collect();
    }
    
    /// Build spectral basis functions
    fn build_spectral_basis(&mut self) {
        self.spectral_basis = self.dominant_frequencies.clone();
    }
    
    /// Classify meme by spectral characteristics
    pub fn classify_meme(&self, meme: &ASTMeme) -> String {
        let dominant_freq = meme.spectral_signature.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.magnitude().partial_cmp(&b.magnitude()).unwrap())
            .map(|(i, _)| i as f64)
            .unwrap_or(0.0);
        
        match dominant_freq {
            f if f < 5.0 => "Low-frequency meme (simple structures)".to_string(),
            f if f < 15.0 => "Mid-frequency meme (control flow)".to_string(),
            f if f < 30.0 => "High-frequency meme (complex functions)".to_string(),
            _ => "Ultra-high frequency meme (ecosystem-level)".to_string(),
        }
    }
    
    /// Find similar memes using spectral distance
    pub fn find_similar_memes(&self, target_meme: &ASTMeme, threshold: f64) -> Vec<String> {
        let mut similar = vec![];
        
        for meme in &self.memes {
            if meme.meme_id == target_meme.meme_id { continue; }
            
            let distance = self.spectral_distance(target_meme, meme);
            if distance < threshold {
                similar.push(format!("{} (distance: {:.3})", meme.meme_id, distance));
            }
        }
        
        similar
    }
    
    /// Calculate spectral distance between two memes
    fn spectral_distance(&self, meme1: &ASTMeme, meme2: &ASTMeme) -> f64 {
        let mut sum_squared_diff = 0.0;
        
        for (c1, c2) in meme1.spectral_signature.iter().zip(meme2.spectral_signature.iter()) {
            let diff = c1.magnitude() - c2.magnitude();
            sum_squared_diff += diff * diff;
        }
        
        sum_squared_diff.sqrt()
    }
    
    /// Generate spectral analysis report
    pub fn spectral_report(&self) -> String {
        let mut report = String::from("AST MEME SPECTRAL ANALYSIS REPORT\n\n");
        
        report.push_str(&format!("Total memes analyzed: {}\n", self.memes.len()));
        report.push_str(&format!("Dominant frequencies: {:?}\n\n", self.dominant_frequencies));
        
        report.push_str("MEME CLASSIFICATIONS:\n");
        for meme in &self.memes {
            let classification = self.classify_meme(meme);
            report.push_str(&format!("  {}: {} (freq: {:.2}, amp: {:.2})\n", 
                meme.meme_id, classification, meme.meme_frequency, meme.amplitude));
        }
        
        report.push_str("\nSPECTRAL CHARACTERISTICS:\n");
        for meme in &self.memes {
            let max_magnitude = meme.spectral_signature.iter()
                .map(|c| c.magnitude())
                .fold(0.0, f64::max);
            report.push_str(&format!("  {}: max magnitude {:.3}\n", meme.meme_id, max_magnitude));
        }
        
        report
    }
    
    /// Get meme frequency distribution
    pub fn frequency_distribution(&self) -> HashMap<String, usize> {
        let mut distribution = HashMap::new();
        
        for meme in &self.memes {
            let classification = self.classify_meme(meme);
            *distribution.entry(classification).or_insert(0) += 1;
        }
        
        distribution
    }
}

/// Macro for AST meme spectral analysis
#[macro_export]
macro_rules! ast_meme_spectrum {
    ($($ast:expr),*) => {{
        let mut analyzer = ASTMemeSpectralAnalyzer::new();
        $(analyzer.add_ast_meme($ast);)*
        analyzer.analyze_meme_spectrum();
        analyzer
    }};
}
