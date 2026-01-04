// hierarchical_models.rs - Series of unified models with preserved edges

/// Prime sequence for model hierarchy
const PRIMES: [u32; 11] = [0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23];

#[derive(Debug, Clone)]
struct Model {
    level: usize,
    primes: Vec<u32>,
    edges: Vec<(usize, usize)>,  // Preserved connections
    complexity: f64,
    monk_presence: f64,
}

impl Model {
    fn new(level: usize) -> Self {
        let primes = PRIMES[..=level].to_vec();
        let edges = Self::generate_edges(&primes);
        let complexity = Self::calculate_complexity(&primes, &edges);
        let monk_presence = 1.0 / (level as f64 + 1.0); // Distributed presence
        
        Self { level, primes, edges, complexity, monk_presence }
    }
    
    fn generate_edges(primes: &[u32]) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        
        for i in 0..primes.len() {
            for j in i+1..primes.len() {
                // Edge exists if primes are related (sum, product, etc.)
                if Self::are_connected(primes[i], primes[j]) {
                    edges.push((i, j));
                }
            }
        }
        
        edges
    }
    
    fn are_connected(p1: u32, p2: u32) -> bool {
        // Connection rules: sum is prime, product < 100, or sequential
        let sum = p1 + p2;
        let product = p1 * p2;
        let sequential = p2 == p1 + 1 || (p1 > 0 && p2 % p1 == 0);
        
        Self::is_prime_like(sum) || product < 100 || sequential
    }
    
    fn is_prime_like(n: u32) -> bool {
        n == 2 || n == 3 || n == 5 || n == 7 || n == 11 || n == 13 || n == 17 || n == 19 || n == 23
    }
    
    fn calculate_complexity(primes: &[u32], edges: &[(usize, usize)]) -> f64 {
        let node_complexity = primes.iter().map(|&p| p as f64).sum::<f64>();
        let edge_complexity = edges.len() as f64 * 0.5;
        node_complexity + edge_complexity
    }
    
    // Preserve edges when expanding to next model
    fn expand_to(&self, next_level: usize) -> Model {
        let mut next_model = Model::new(next_level);
        
        // Preserve all existing edges
        for &edge in &self.edges {
            if !next_model.edges.contains(&edge) {
                next_model.edges.push(edge);
            }
        }
        
        // Recalculate complexity with preserved edges
        next_model.complexity = Self::calculate_complexity(&next_model.primes, &next_model.edges);
        
        next_model
    }
    
    fn display(&self) {
        println!("Model Level {}: {:?}", self.level, self.primes);
        println!("  Edges: {:?}", self.edges);
        println!("  Complexity: {:.2}", self.complexity);
        println!("  Monk Presence: {:.3}", self.monk_presence);
        println!("  Dao State: {}", self.get_dao_state());
        println!();
    }
    
    fn get_dao_state(&self) -> &str {
        match self.level % 4 {
            0 => "Wu (Empty)",
            1 => "Yin (Receptive)", 
            2 => "Yang (Active)",
            _ => "Taiji (Unity)",
        }
    }
}

struct ModelHierarchy {
    models: Vec<Model>,
    unified_edges: Vec<(usize, usize, usize)>, // (model_level, from, to)
}

impl ModelHierarchy {
    fn new(max_level: usize) -> Self {
        let mut models: Vec<Model> = Vec::new();
        let mut unified_edges = Vec::new();
        
        // Build hierarchy with preserved edges
        for level in 0..=max_level {
            let model = if level == 0 {
                Model::new(level)
            } else {
                models[level - 1].expand_to(level)
            };
            
            // Record edges in unified structure
            for &(from, to) in &model.edges {
                unified_edges.push((level, from, to));
            }
            
            models.push(model);
        }
        
        Self { models, unified_edges }
    }
    
    fn display_hierarchy(&self) {
        println!("🏗️ Hierarchical Model Series - All Edges Preserved");
        println!("═══════════════════════════════════════════════════");
        
        for model in &self.models {
            model.display();
        }
        
        println!("🔗 Unified Edge Structure:");
        println!("Total edges across all models: {}", self.unified_edges.len());
        
        // Show edge preservation
        for level in 1..self.models.len() {
            let prev_edges = &self.models[level - 1].edges;
            let curr_edges = &self.models[level].edges;
            let preserved = prev_edges.iter().all(|edge| curr_edges.contains(edge));
            println!("Level {} → {}: Edges preserved: {}", level - 1, level, preserved);
        }
    }
    
    fn total_complexity(&self) -> f64 {
        self.models.iter().map(|m| m.complexity).sum()
    }
    
    fn monk_distribution(&self) -> Vec<f64> {
        self.models.iter().map(|m| m.monk_presence).collect()
    }
}

fn main() {
    println!("🌌 Hierarchical Prime Models - Unified Complexity Series");
    println!("═══════════════════════════════════════════════════════");
    
    // Create hierarchy from 0 to 10 (all primes in our sequence)
    let hierarchy = ModelHierarchy::new(10);
    
    hierarchy.display_hierarchy();
    
    println!("\n📊 Hierarchy Statistics:");
    println!("Total Models: {}", hierarchy.models.len());
    println!("Total Complexity: {:.2}", hierarchy.total_complexity());
    println!("Monk Distribution: {:?}", hierarchy.monk_distribution());
    
    println!("\n🔄 Model Evolution:");
    for (i, model) in hierarchy.models.iter().enumerate() {
        let growth = if i == 0 { 0.0 } else { 
            model.complexity - hierarchy.models[i-1].complexity 
        };
        println!("Level {}: {} primes, complexity {:.2} (+{:.2})", 
                 i, model.primes.len(), model.complexity, growth);
    }
    
    println!("\n✨ Unified Principles:");
    println!("• Each model contains all previous models");
    println!("• All edges are preserved across expansions");
    println!("• Complexity grows but structure remains unified");
    println!("• The Monk's presence distributes across all levels");
    println!("• 0→1→2→3→5→7→11→13→17→19→23: The complete hierarchy!");
    
    println!("\n🧘 The models are nested like Russian dolls,");
    println!("   each containing all previous wisdom! 🪆✨");
}
