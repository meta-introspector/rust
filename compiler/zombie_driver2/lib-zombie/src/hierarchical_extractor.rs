use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LevelSignature {
    level: CompilationLevel,
    hash: String,
    frequency_signature: Vec<f64>,
    structural_features: HashMap<String, f64>,
    dependencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
enum CompilationLevel {
    Byte, // Level 0: Raw bytes and text
    Syn,  // Level 1: Syntax tree
    Span, // Level 2: Source locations
    Hir,  // Level 3: High-level IR
    Ty,   // Level 4: Type analysis
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HierarchicalSignatureExtractor {
    signatures: HashMap<String, Vec<LevelSignature>>, // file -> level signatures
    level_transitions: HashMap<(CompilationLevel, CompilationLevel), Vec<f64>>, // transition matrices
}

impl HierarchicalSignatureExtractor {
    pub fn new() -> Self {
        Self { signatures: HashMap::new(), level_transitions: HashMap::new() }
    }

    pub fn extract_all_levels(
        &mut self,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Extracting 5-level signatures for: {}", file_path);

        let mut file_signatures = Vec::new();

        // Level 0: Byte and text analysis
        let byte_sig = self.extract_byte_signature(file_path)?;
        file_signatures.push(byte_sig.clone());

        // Level 1: Syntax analysis
        let syn_sig = self.extract_syn_signature(file_path, &byte_sig)?;
        file_signatures.push(syn_sig.clone());

        // Level 2: Span analysis
        let span_sig = self.extract_span_signature(file_path, &syn_sig)?;
        file_signatures.push(span_sig.clone());

        // Level 3: HIR analysis
        let hir_sig = self.extract_hir_signature(file_path, &span_sig)?;
        file_signatures.push(hir_sig.clone());

        // Level 4: Type analysis
        let ty_sig = self.extract_ty_signature(file_path, &hir_sig)?;
        file_signatures.push(ty_sig);

        // Record level transitions
        self.record_level_transitions(&file_signatures);

        self.signatures.insert(file_path.to_string(), file_signatures);

        Ok(())
    }

    fn extract_byte_signature(
        &self,
        file_path: &str,
    ) -> Result<LevelSignature, Box<dyn std::error::Error>> {
        let content = std::fs::read(file_path)?;

        // Hash the raw bytes
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = format!("{:x}", hasher.finalize());

        // Byte frequency analysis
        let mut byte_freq = vec![0u64; 256];
        for &byte in &content {
            byte_freq[byte as usize] += 1;
        }

        let frequency_signature: Vec<f64> =
            byte_freq.iter().map(|&count| count as f64 / content.len() as f64).collect();

        // Structural features
        let mut features = HashMap::new();
        features.insert("file_size".to_string(), content.len() as f64);
        features.insert("entropy".to_string(), self.calculate_entropy(&frequency_signature));
        features.insert(
            "ascii_ratio".to_string(),
            content.iter().filter(|&&b| b < 128).count() as f64 / content.len() as f64,
        );

        Ok(LevelSignature {
            level: CompilationLevel::Byte,
            hash,
            frequency_signature,
            structural_features: features,
            dependencies: vec![],
        })
    }

    fn extract_syn_signature(
        &self,
        file_path: &str,
        byte_sig: &LevelSignature,
    ) -> Result<LevelSignature, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;

        // Mock syntax analysis - would use syn crate
        let tokens = self.tokenize_rust(&content);
        let token_freq = self.analyze_token_frequencies(&tokens);

        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", tokens).as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        let mut features = HashMap::new();
        features.insert("token_count".to_string(), tokens.len() as f64);
        features.insert("unique_tokens".to_string(), token_freq.len() as f64);
        features.insert(
            "avg_token_length".to_string(),
            tokens.iter().map(|t| t.len()).sum::<usize>() as f64 / tokens.len() as f64,
        );

        Ok(LevelSignature {
            level: CompilationLevel::Syn,
            hash,
            frequency_signature: token_freq.values().cloned().collect(),
            structural_features: features,
            dependencies: vec![byte_sig.hash.clone()],
        })
    }

    fn extract_span_signature(
        &self,
        file_path: &str,
        syn_sig: &LevelSignature,
    ) -> Result<LevelSignature, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        // Span analysis - line/column positions
        let mut span_features = HashMap::new();
        span_features.insert("line_count".to_string(), lines.len() as f64);
        span_features.insert(
            "avg_line_length".to_string(),
            lines.iter().map(|l| l.len()).sum::<usize>() as f64 / lines.len() as f64,
        );
        span_features.insert(
            "max_line_length".to_string(),
            lines.iter().map(|l| l.len()).max().unwrap_or(0) as f64,
        );

        // Line length distribution as frequency signature
        let mut line_length_dist = vec![0u64; 200]; // Up to 200 chars per line
        for line in &lines {
            let len = line.len().min(199);
            line_length_dist[len] += 1;
        }

        let frequency_signature: Vec<f64> =
            line_length_dist.iter().map(|&count| count as f64 / lines.len() as f64).collect();

        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", span_features).as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Ok(LevelSignature {
            level: CompilationLevel::Span,
            hash,
            frequency_signature,
            structural_features: span_features,
            dependencies: vec![syn_sig.hash.clone()],
        })
    }

    fn extract_hir_signature(
        &self,
        _file_path: &str,
        span_sig: &LevelSignature,
    ) -> Result<LevelSignature, Box<dyn std::error::Error>> {
        // Mock HIR analysis - would use rustc_hir
        let mut hir_features = HashMap::new();
        hir_features.insert(
            "estimated_nodes".to_string(),
            span_sig.structural_features.get("line_count").unwrap_or(&0.0) * 2.5,
        );
        hir_features.insert(
            "control_flow_complexity".to_string(),
            span_sig.structural_features.get("line_count").unwrap_or(&0.0) * 0.1,
        );

        // Mock HIR node frequency distribution
        let frequency_signature = vec![0.3, 0.2, 0.15, 0.1, 0.08, 0.07, 0.05, 0.03, 0.02]; // Mock HIR node types

        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", hir_features).as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Ok(LevelSignature {
            level: CompilationLevel::Hir,
            hash,
            frequency_signature,
            structural_features: hir_features,
            dependencies: vec![span_sig.hash.clone()],
        })
    }

    fn extract_ty_signature(
        &self,
        _file_path: &str,
        hir_sig: &LevelSignature,
    ) -> Result<LevelSignature, Box<dyn std::error::Error>> {
        // Mock type analysis - would use rustc_middle::ty
        let mut ty_features = HashMap::new();
        ty_features.insert(
            "estimated_types".to_string(),
            hir_sig.structural_features.get("estimated_nodes").unwrap_or(&0.0) * 0.3,
        );
        ty_features.insert(
            "generic_complexity".to_string(),
            hir_sig.structural_features.get("control_flow_complexity").unwrap_or(&0.0) * 1.5,
        );
        ty_features.insert(
            "trait_bounds".to_string(),
            hir_sig.structural_features.get("estimated_nodes").unwrap_or(&0.0) * 0.1,
        );

        // Mock type frequency distribution
        let frequency_signature = vec![0.4, 0.25, 0.15, 0.08, 0.05, 0.03, 0.02, 0.01, 0.01]; // Mock type categories

        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", ty_features).as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Ok(LevelSignature {
            level: CompilationLevel::Ty,
            hash,
            frequency_signature,
            structural_features: ty_features,
            dependencies: vec![hir_sig.hash.clone()],
        })
    }

    fn tokenize_rust(&self, content: &str) -> Vec<String> {
        // Simple tokenization - would use syn::parse
        content.split_whitespace().filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
    }

    fn analyze_token_frequencies(&self, tokens: &[String]) -> HashMap<String, f64> {
        let mut freq = HashMap::new();
        for token in tokens {
            *freq.entry(token.clone()).or_insert(0.0) += 1.0;
        }

        let total = tokens.len() as f64;
        for count in freq.values_mut() {
            *count /= total;
        }

        freq
    }

    fn calculate_entropy(&self, distribution: &[f64]) -> f64 {
        distribution.iter().filter(|&&p| p > 0.0).map(|&p| -p * p.log2()).sum()
    }

    fn record_level_transitions(&mut self, signatures: &[LevelSignature]) {
        for i in 0..signatures.len() - 1 {
            let from_level = signatures[i].level.clone();
            let to_level = signatures[i + 1].level.clone();

            // Compute transition vector (difference in frequency signatures)
            let transition: Vec<f64> = signatures[i]
                .frequency_signature
                .iter()
                .zip(&signatures[i + 1].frequency_signature)
                .map(|(a, b)| b - a)
                .collect();

            self.level_transitions
                .entry((from_level, to_level))
                .or_insert_with(Vec::new)
                .extend(transition);
        }
    }

    pub fn build_final_signature(&self, file_path: &str) -> Option<Vec<f64>> {
        let signatures = self.signatures.get(file_path)?;

        // Concatenate all level signatures into final system signature
        let mut final_sig = Vec::new();
        for sig in signatures {
            final_sig.extend(&sig.frequency_signature);
            final_sig.extend(sig.structural_features.values());
        }

        Some(final_sig)
    }

    pub fn export_signatures(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        println!("📁 Hierarchical signatures exported to {}", path);
        Ok(())
    }
}
