use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::data_structures::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypespaceMapping {
    pub collector_element: String,
    pub element_type: String, // "type", "function", "const", "macro"
    pub rust_matches: Vec<RustMatch>,
    pub similarity_score: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RustMatch {
    pub rust_element: String,
    pub similarity: f64,
    pub match_type: String,
    pub usage_pattern: String,
}

pub struct TypespaceMapper {
    collector_types: HashMap<String, UsageEntry>,
    rust_types: HashMap<String, UsageEntry>,
    mappings: Vec<TypespaceMapping>,
}

impl TypespaceMapper {
    pub fn new() -> Self {
        Self {
            collector_types: HashMap::new(),
            rust_types: HashMap::new(),
            mappings: Vec::new(),
        }
    }
    
    pub fn map_typespace(&mut self) {
        println!("=== TYPESPACE MAPPING: T_collector → T_rust ===");
        
        for (element_name, usage) in &self.collector_types {
            let element_type = self.classify_element(&usage.kind);
            let rust_matches = self.find_closest_3(element_name, usage);
            
            let avg_similarity = rust_matches.iter()
                .map(|m| m.similarity)
                .sum::<f64>() / rust_matches.len().max(1) as f64;
            
            let mapping = TypespaceMapping {
                collector_element: element_name.clone(),
                element_type,
                rust_matches,
                similarity_score: avg_similarity,
            };
            
            self.print_mapping(&mapping);
            self.mappings.push(mapping);
        }
    }
    
    fn classify_element(&self, kind: &str) -> String {
        match kind {
            k if k.contains("struct") => "type".to_string(),
            k if k.contains("enum") => "type".to_string(),
            k if k.contains("function") => "function".to_string(),
            k if k.contains("const") => "const".to_string(),
            k if k.contains("macro") => "macro".to_string(),
            _ => "other".to_string(),
        }
    }
    
    fn find_closest_3(&self, element: &str, usage: &UsageEntry) -> Vec<RustMatch> {
        let mut candidates: Vec<RustMatch> = Vec::new();
        
        for (rust_element, rust_usage) in &self.rust_types {
            let similarity = self.calculate_similarity(usage, rust_usage);
            
            candidates.push(RustMatch {
                rust_element: rust_element.clone(),
                similarity,
                match_type: self.get_match_type(usage, rust_usage),
                usage_pattern: rust_usage.usage_type.clone(),
            });
        }
        
        // Sort by similarity and take top 3
        candidates.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        candidates.truncate(3);
        candidates
    }
    
    fn calculate_similarity(&self, a: &UsageEntry, b: &UsageEntry) -> f64 {
        let mut score = 0.0;
        
        // Exact matches
        if a.kind == b.kind { score += 0.4; }
        if a.usage_type == b.usage_type { score += 0.3; }
        if a.node_type == b.node_type { score += 0.2; }
        
        // Fuzzy matches
        score += self.string_similarity(&a.symbol, &b.symbol) * 0.1;
        
        score
    }
    
    fn string_similarity(&self, a: &str, b: &str) -> f64 {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();
        
        if a_lower == b_lower { return 1.0; }
        if a_lower.contains(&b_lower) || b_lower.contains(&a_lower) { return 0.7; }
        
        // Simple character overlap
        let common: usize = a_lower.chars()
            .filter(|c| b_lower.contains(*c))
            .count();
        
        common as f64 / a_lower.len().max(b_lower.len()) as f64
    }
    
    fn get_match_type(&self, a: &UsageEntry, b: &UsageEntry) -> String {
        if a.kind == b.kind && a.usage_type == b.usage_type {
            "exact".to_string()
        } else if a.kind == b.kind {
            "structural".to_string()
        } else if a.usage_type == b.usage_type {
            "behavioral".to_string()
        } else {
            "fuzzy".to_string()
        }
    }
    
    fn print_mapping(&self, mapping: &TypespaceMapping) {
        println!("\n{} [{}] → Rust typespace:", 
                mapping.collector_element, 
                mapping.element_type);
        
        for (i, rust_match) in mapping.rust_matches.iter().enumerate() {
            println!("  {}. {} ({:.3}) [{}] {}", 
                    i + 1,
                    rust_match.rust_element,
                    rust_match.similarity,
                    rust_match.match_type,
                    rust_match.usage_pattern);
        }
    }
    
    pub fn save_mappings(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.mappings)?;
        std::fs::write(path, json)?;
        println!("\nTypespace mappings saved to: {}", path);
        Ok(())
    }
    
    pub fn analyze_coverage(&self) {
        println!("\n=== TYPESPACE COVERAGE ANALYSIS ===");
        
        let high_similarity = self.mappings.iter()
            .filter(|m| m.similarity_score > 0.8)
            .count();
        
        let medium_similarity = self.mappings.iter()
            .filter(|m| m.similarity_score > 0.5 && m.similarity_score <= 0.8)
            .count();
        
        let low_similarity = self.mappings.iter()
            .filter(|m| m.similarity_score <= 0.5)
            .count();
        
        println!("High similarity (>0.8): {} elements", high_similarity);
        println!("Medium similarity (0.5-0.8): {} elements", medium_similarity);
        println!("Low similarity (<0.5): {} elements (novel)", low_similarity);
        
        let novelty_ratio = low_similarity as f64 / self.mappings.len() as f64;
        println!("Novelty ratio: {:.2}%", novelty_ratio * 100.0);
    }
}
