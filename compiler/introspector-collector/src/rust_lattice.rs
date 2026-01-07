/// Rust Code Lattice - Fill holes by querying all snippets into universal structure
/// Maps every possible Rust construct into a complete lattice with no gaps

use crate::rust_tree_of_life::*;
use crate::reconstruction_engine::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct RustLattice {
    pub nodes: HashMap<String, LatticeNode>,
    pub connections: HashMap<String, Vec<String>>,
    pub holes: HashSet<String>, // Missing connections to fill
    pub completeness_score: f64,
}

#[derive(Debug, Clone)]
pub struct LatticeNode {
    pub id: String,
    pub construct_type: ConstructType,
    pub representations: HashMap<String, String>, // layer -> representation
    pub connections_in: Vec<String>,
    pub connections_out: Vec<String>,
    pub is_hole: bool, // True if this is a gap we need to fill
}

#[derive(Debug, Clone)]
pub enum ConstructType {
    Enum { variants: Vec<String> },
    Function { params: Vec<String>, return_type: String },
    Struct { fields: Vec<String> },
    Module { items: Vec<String> },
    Expression { expr_type: String },
    Type { type_info: String },
    Macro { expansion: String },
    Unknown, // Hole to be filled
}

impl RustLattice {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            holes: HashSet::new(),
            completeness_score: 0.0,
        }
    }
    
    /// Query all Rust snippets and insert into lattice
    pub fn ingest_all_snippets(&mut self, snippets: Vec<&str>) {
        for snippet in snippets {
            self.ingest_snippet(snippet);
        }
        self.identify_holes();
        self.calculate_completeness();
    }
    
    /// Insert a single code snippet into the lattice
    pub fn ingest_snippet(&mut self, snippet: &str) {
        let node_id = format!("snippet_{}", self.nodes.len());
        let construct_type = self.classify_snippet(snippet);
        
        let mut representations = HashMap::new();
        representations.insert("source".to_string(), snippet.to_string());
        representations.insert("syn".to_string(), format!("syn::parse({})", snippet));
        representations.insert("hir".to_string(), format!("hir::lower({})", snippet));
        
        let node = LatticeNode {
            id: node_id.clone(),
            construct_type,
            representations,
            connections_in: vec![],
            connections_out: vec![],
            is_hole: false,
        };
        
        self.nodes.insert(node_id.clone(), node);
        self.connect_to_existing_nodes(&node_id);
    }
    
    /// Classify what type of construct a snippet represents
    fn classify_snippet(&self, snippet: &str) -> ConstructType {
        if snippet.contains("enum") {
            let variants = self.extract_enum_variants(snippet);
            ConstructType::Enum { variants }
        } else if snippet.contains("fn ") {
            ConstructType::Function { 
                params: vec![], 
                return_type: "()".to_string() 
            }
        } else if snippet.contains("struct") {
            ConstructType::Struct { fields: vec![] }
        } else if snippet.contains("mod ") {
            ConstructType::Module { items: vec![] }
        } else if snippet.contains("macro_rules!") {
            ConstructType::Macro { expansion: snippet.to_string() }
        } else {
            ConstructType::Expression { expr_type: "unknown".to_string() }
        }
    }
    
    fn extract_enum_variants(&self, snippet: &str) -> Vec<String> {
        // Simple extraction - would be more sophisticated in practice
        snippet.lines()
            .filter(|line| line.trim().ends_with(',') && !line.contains("enum"))
            .map(|line| line.trim().trim_end_matches(',').to_string())
            .collect()
    }
    
    /// Connect new node to existing nodes based on semantic relationships
    fn connect_to_existing_nodes(&mut self, new_node_id: &str) {
        let new_node = self.nodes.get(new_node_id).unwrap().clone();
        
        for (existing_id, existing_node) in &self.nodes {
            if existing_id == new_node_id { continue; }
            
            if self.should_connect(&new_node, existing_node) {
                self.add_connection(new_node_id, existing_id);
            }
        }
    }
    
    /// Determine if two nodes should be connected
    fn should_connect(&self, node1: &LatticeNode, node2: &LatticeNode) -> bool {
        match (&node1.construct_type, &node2.construct_type) {
            (ConstructType::Enum { .. }, ConstructType::Function { .. }) => true,
            (ConstructType::Function { .. }, ConstructType::Expression { .. }) => true,
            (ConstructType::Struct { .. }, ConstructType::Enum { .. }) => true,
            _ => false,
        }
    }
    
    /// Add bidirectional connection between nodes
    fn add_connection(&mut self, from_id: &str, to_id: &str) {
        self.connections.entry(from_id.to_string())
            .or_insert_with(Vec::new)
            .push(to_id.to_string());
            
        if let Some(from_node) = self.nodes.get_mut(from_id) {
            from_node.connections_out.push(to_id.to_string());
        }
        
        if let Some(to_node) = self.nodes.get_mut(to_id) {
            to_node.connections_in.push(from_id.to_string());
        }
    }
    
    /// Identify holes in the lattice that need to be filled
    pub fn identify_holes(&mut self) {
        self.holes.clear();
        
        // Find missing connections between construct types
        let enum_nodes: Vec<_> = self.nodes.iter()
            .filter(|(_, node)| matches!(node.construct_type, ConstructType::Enum { .. }))
            .map(|(id, _)| id.clone())
            .collect();
            
        let function_nodes: Vec<_> = self.nodes.iter()
            .filter(|(_, node)| matches!(node.construct_type, ConstructType::Function { .. }))
            .map(|(id, _)| id.clone())
            .collect();
        
        // Every enum should connect to at least one function (usage)
        for enum_id in &enum_nodes {
            let has_function_connection = self.connections.get(enum_id)
                .map(|conns| conns.iter().any(|conn| function_nodes.contains(conn)))
                .unwrap_or(false);
                
            if !has_function_connection {
                let hole_id = format!("hole_enum_to_function_{}", enum_id);
                self.holes.insert(hole_id);
            }
        }
        
        // Identify missing layer representations
        for (node_id, node) in &self.nodes {
            let expected_layers = vec!["source", "syn", "hir", "mir", "llvm", "asm"];
            for layer in expected_layers {
                if !node.representations.contains_key(layer) {
                    let hole_id = format!("hole_{}_{}", node_id, layer);
                    self.holes.insert(hole_id);
                }
            }
        }
    }
    
    /// Fill holes by generating missing constructs
    pub fn fill_holes(&mut self, reconstruction_engine: &ReconstructionEngine) {
        let holes_to_fill: Vec<_> = self.holes.iter().cloned().collect();
        
        for hole_id in holes_to_fill {
            if hole_id.starts_with("hole_enum_to_function_") {
                self.fill_enum_function_hole(&hole_id);
            } else if hole_id.contains("_mir") || hole_id.contains("_llvm") {
                self.fill_layer_hole(&hole_id, reconstruction_engine);
            }
        }
        
        self.calculate_completeness();
    }
    
    /// Generate missing enum-to-function connection
    fn fill_enum_function_hole(&mut self, hole_id: &str) {
        let enum_id = hole_id.replace("hole_enum_to_function_", "");
        
        if let Some(enum_node) = self.nodes.get(&enum_id) {
            if let ConstructType::Enum { variants } = &enum_node.construct_type {
                // Generate a function that uses this enum
                let function_snippet = format!(
                    "fn use_{}(value: SomeEnum) -> String {{\n    match value {{\n{}\n    }}\n}}",
                    enum_id,
                    variants.iter()
                        .map(|v| format!("        {} => \"{}\",", v, v))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                
                self.ingest_snippet(&function_snippet);
                self.holes.remove(hole_id);
            }
        }
    }
    
    /// Fill missing layer representation
    fn fill_layer_hole(&mut self, hole_id: &str, reconstruction_engine: &ReconstructionEngine) {
        let parts: Vec<&str> = hole_id.split('_').collect();
        if parts.len() >= 3 {
            let node_id = parts[1];
            let missing_layer = parts[2];
            
            if let Some(node) = self.nodes.get_mut(node_id) {
                // Try to reconstruct missing layer from existing representations
                for (existing_layer, content) in &node.representations {
                    if let Some(reconstructed) = reconstruction_engine.query_reconstruct(content, missing_layer) {
                        node.representations.insert(missing_layer.to_string(), reconstructed);
                        self.holes.remove(hole_id);
                        break;
                    }
                }
            }
        }
    }
    
    /// Calculate how complete the lattice is (0.0 to 1.0)
    fn calculate_completeness(&mut self) {
        let total_possible_connections = self.nodes.len() * self.nodes.len();
        let actual_connections: usize = self.connections.values().map(|v| v.len()).sum();
        let hole_penalty = self.holes.len() as f64 * 0.1;
        
        self.completeness_score = if total_possible_connections > 0 {
            (actual_connections as f64 / total_possible_connections as f64) - hole_penalty
        } else {
            0.0
        }.max(0.0).min(1.0);
    }
    
    /// Query the lattice for specific patterns
    pub fn query_pattern(&self, pattern: &str) -> Vec<String> {
        self.nodes.iter()
            .filter(|(_, node)| {
                node.representations.values().any(|repr| repr.contains(pattern))
            })
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Generate summary of lattice structure
    pub fn generate_summary(&self) -> String {
        format!(
            "Rust Lattice Summary:\n\
             - Nodes: {}\n\
             - Connections: {}\n\
             - Holes: {}\n\
             - Completeness: {:.2}%\n\
             - Construct Types: {:?}",
            self.nodes.len(),
            self.connections.values().map(|v| v.len()).sum::<usize>(),
            self.holes.len(),
            self.completeness_score * 100.0,
            self.nodes.values()
                .map(|n| std::mem::discriminant(&n.construct_type))
                .collect::<HashSet<_>>()
                .len()
        )
    }
}
