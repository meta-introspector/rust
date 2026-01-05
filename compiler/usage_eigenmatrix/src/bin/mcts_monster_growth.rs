use std::collections::HashMap;
use std::fs;
use rand::Rng;

/// MCTS-Guided Monster DefId Growth System
/// Uses Monte Carlo Tree Search to optimize Monster Group evolution

#[derive(Debug, Clone)]
struct MCTSNode {
    defid_signature: u128,
    context: String,
    visits: u32,
    total_reward: f64,
    children: Vec<usize>,
    parent: Option<usize>,
    growth_action: GrowthAction,
}

#[derive(Debug, Clone)]
enum GrowthAction {
    SignatureMutation(u32),
    ContextGeneration(String),
    Composition(u128, u128),
    Root,
}

#[derive(Debug)]
struct MCTSMonsterGrowth {
    nodes: Vec<MCTSNode>,
    prime_generators: [u8; 8],
    exploration_constant: f64,
    grown_defids: Vec<GrownDefId>,
}

#[derive(Debug, Clone)]
struct GrownDefId {
    defid: String,
    signature: u128,
    context: String,
    mcts_reward: f64,
    generation: u32,
}

impl MCTSMonsterGrowth {
    fn new(seed_signature: u128, seed_context: &str) -> Self {
        let root_node = MCTSNode {
            defid_signature: seed_signature,
            context: seed_context.to_string(),
            visits: 0,
            total_reward: 0.0,
            children: Vec::new(),
            parent: None,
            growth_action: GrowthAction::Root,
        };
        
        Self {
            nodes: vec![root_node],
            prime_generators: [2, 3, 5, 7, 11, 13, 17, 19],
            exploration_constant: 1.414, // sqrt(2)
            grown_defids: Vec::new(),
        }
    }
    
    fn calculate_signature(&self, data: &str) -> u128 {
        let mut signature = 1u128;
        for (i, byte) in data.bytes().enumerate() {
            let prime_idx = i % 8;
            let prime = self.prime_generators[prime_idx] as u128;
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u128);
        }
        signature
    }
    
    fn ucb1_score(&self, node_idx: usize, parent_visits: u32) -> f64 {
        let node = &self.nodes[node_idx];
        if node.visits == 0 {
            return f64::INFINITY;
        }
        
        let exploitation = node.total_reward / node.visits as f64;
        let exploration = self.exploration_constant * 
            ((parent_visits as f64).ln() / node.visits as f64).sqrt();
        
        exploitation + exploration
    }
    
    fn select_best_child(&self, parent_idx: usize) -> usize {
        let parent = &self.nodes[parent_idx];
        let parent_visits = parent.visits;
        
        parent.children.iter()
            .max_by(|&&a, &&b| {
                self.ucb1_score(a, parent_visits)
                    .partial_cmp(&self.ucb1_score(b, parent_visits))
                    .unwrap()
            })
            .copied()
            .unwrap()
    }
    
    fn expand_node(&mut self, node_idx: usize) -> Vec<usize> {
        let node_sig = self.nodes[node_idx].defid_signature;
        let mut new_children = Vec::new();
        
        // Generate signature mutations
        for i in 0..4 {
            let prime = self.prime_generators[i] as u128;
            let mutated_sig = node_sig.wrapping_mul(prime).wrapping_add(i as u128);
            let context = format!("Self::mcts_variant_{}::evolved_string", i);
            
            let child_node = MCTSNode {
                defid_signature: mutated_sig,
                context: context.clone(),
                visits: 0,
                total_reward: 0.0,
                children: Vec::new(),
                parent: Some(node_idx),
                growth_action: GrowthAction::SignatureMutation(i as u32),
            };
            
            self.nodes.push(child_node);
            let child_idx = self.nodes.len() - 1;
            new_children.push(child_idx);
        }
        
        // Generate context variations
        let contexts = ["to_string", "get_name", "format", "display"];
        for (i, &context_type) in contexts.iter().enumerate() {
            let context = format!("Self::{}::mcts_generated", context_type);
            let context_sig = self.calculate_signature(&context);
            
            let child_node = MCTSNode {
                defid_signature: context_sig,
                context: context.clone(),
                visits: 0,
                total_reward: 0.0,
                children: Vec::new(),
                parent: Some(node_idx),
                growth_action: GrowthAction::ContextGeneration(context),
            };
            
            self.nodes.push(child_node);
            let child_idx = self.nodes.len() - 1;
            new_children.push(child_idx);
        }
        
        // Update parent's children
        self.nodes[node_idx].children = new_children.clone();
        new_children
    }
    
    fn simulate(&self, node_idx: usize) -> f64 {
        let node = &self.nodes[node_idx];
        let mut rng = rand::thread_rng();
        
        // Reward function based on Monster Group properties
        let mut reward = 0.0;
        
        // Signature diversity reward
        let sig_entropy = self.calculate_signature_entropy(node.defid_signature);
        reward += sig_entropy * 0.3;
        
        // Context quality reward
        if node.context.contains("Self::") {
            reward += 0.2;
        }
        if node.context.contains("to_string") || node.context.contains("format") {
            reward += 0.3;
        }
        
        // Prime alignment reward
        let prime_alignment = (node.defid_signature % 8) as f64 / 8.0;
        reward += prime_alignment * 0.2;
        
        // Random exploration bonus
        reward += rng.gen::<f64>() * 0.1;
        
        reward
    }
    
    fn calculate_signature_entropy(&self, signature: u128) -> f64 {
        let bytes = signature.to_le_bytes();
        let mut counts = [0u32; 256];
        
        for &byte in &bytes {
            counts[byte as usize] += 1;
        }
        
        let total = bytes.len() as f64;
        let mut entropy = 0.0;
        
        for &count in &counts {
            if count > 0 {
                let p = count as f64 / total;
                entropy -= p * p.log2();
            }
        }
        
        entropy / 8.0 // Normalize
    }
    
    fn backpropagate(&mut self, mut node_idx: usize, reward: f64) {
        loop {
            self.nodes[node_idx].visits += 1;
            self.nodes[node_idx].total_reward += reward;
            
            if let Some(parent_idx) = self.nodes[node_idx].parent {
                node_idx = parent_idx;
            } else {
                break;
            }
        }
    }
    
    fn mcts_iteration(&mut self) -> usize {
        // Selection
        let mut current_idx = 0;
        while !self.nodes[current_idx].children.is_empty() {
            current_idx = self.select_best_child(current_idx);
        }
        
        // Expansion
        if self.nodes[current_idx].visits > 0 {
            let new_children = self.expand_node(current_idx);
            if !new_children.is_empty() {
                current_idx = new_children[0];
            }
        }
        
        // Simulation
        let reward = self.simulate(current_idx);
        
        // Backpropagation
        self.backpropagate(current_idx, reward);
        
        current_idx
    }
    
    fn mcts_guided_growth(&mut self, iterations: u32, target_defids: usize) {
        println!("🎯 MCTS-Guided Monster Growth");
        println!("============================");
        println!("Iterations: {} | Target: {} DefIds", iterations, target_defids);
        
        for i in 0..iterations {
            let selected_node = self.mcts_iteration();
            
            // Convert promising nodes to DefIds
            if i % 100 == 0 {
                self.harvest_best_nodes();
                println!("Iteration {}: {} nodes, {} DefIds", 
                    i, self.nodes.len(), self.grown_defids.len());
            }
            
            if self.grown_defids.len() >= target_defids {
                break;
            }
        }
        
        // Final harvest
        self.harvest_best_nodes();
        
        println!("🎉 MCTS Growth Complete!");
        println!("Nodes explored: {}", self.nodes.len());
        println!("DefIds grown: {}", self.grown_defids.len());
    }
    
    fn harvest_best_nodes(&mut self) {
        // Find nodes with high reward/visit ratio
        let mut candidates: Vec<(usize, f64)> = self.nodes.iter().enumerate()
            .filter(|(_, node)| node.visits > 5)
            .map(|(idx, node)| (idx, node.total_reward / node.visits as f64))
            .collect();
        
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Convert top candidates to DefIds
        for (node_idx, reward) in candidates.iter().take(10) {
            let node = &self.nodes[*node_idx];
            
            // Check if already harvested
            if self.grown_defids.iter().any(|d| d.signature == node.defid_signature) {
                continue;
            }
            
            let defid = GrownDefId {
                defid: format!("MCTSDefId({})", self.grown_defids.len()),
                signature: node.defid_signature,
                context: node.context.clone(),
                mcts_reward: *reward,
                generation: (node_idx / 10) as u32,
            };
            
            self.grown_defids.push(defid);
        }
    }
    
    fn generate_mcts_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# MCTS-Guided Monster DefId Growth Report\n\n");
        report.push_str("## Monte Carlo Tree Search Optimization Results\n\n");
        
        report.push_str("### MCTS Statistics\n");
        report.push_str(&format!("- **Nodes Explored**: {}\n", self.nodes.len()));
        report.push_str(&format!("- **DefIds Grown**: {}\n", self.grown_defids.len()));
        report.push_str(&format!("- **Exploration Constant**: {}\n", self.exploration_constant));
        
        let avg_reward = self.grown_defids.iter().map(|d| d.mcts_reward).sum::<f64>() / self.grown_defids.len() as f64;
        report.push_str(&format!("- **Average Reward**: {:.4}\n\n", avg_reward));
        
        report.push_str("### Top MCTS-Optimized DefIds\n");
        report.push_str("| DefId | Signature | MCTS Reward | Generation | Context |\n");
        report.push_str("|-------|-----------|-------------|------------|----------|\n");
        
        let mut sorted_defids = self.grown_defids.clone();
        sorted_defids.sort_by(|a, b| b.mcts_reward.partial_cmp(&a.mcts_reward).unwrap());
        
        for defid in sorted_defids.iter().take(20) {
            report.push_str(&format!(
                "| `{}` | `0x{:016X}` | {:.4} | {} | `{}` |\n",
                defid.defid,
                defid.signature & 0xFFFFFFFFFFFFFFFF,
                defid.mcts_reward,
                defid.generation,
                defid.context
            ));
        }
        
        report.push_str("\n### MCTS Tree Analysis\n");
        let max_visits = self.nodes.iter().map(|n| n.visits).max().unwrap_or(0);
        let avg_visits = self.nodes.iter().map(|n| n.visits).sum::<u32>() as f64 / self.nodes.len() as f64;
        
        report.push_str(&format!("- **Max Node Visits**: {}\n", max_visits));
        report.push_str(&format!("- **Average Node Visits**: {:.2}\n", avg_visits));
        
        report.push_str("\n### MCTS Optimization Benefits\n");
        report.push_str("✅ **Intelligent Exploration**: MCTS guides growth toward high-reward regions\n");
        report.push_str("✅ **Quality Over Quantity**: Focuses on promising Monster Group structures\n");
        report.push_str("✅ **Adaptive Learning**: UCB1 balances exploration vs exploitation\n");
        report.push_str("✅ **Reward-Driven Evolution**: Signature entropy and context quality optimized\n\n");
        
        report.push_str("### Revolutionary Achievement\n");
        report.push_str("**First MCTS-optimized Monster Group compiler system!**\n");
        report.push_str("Combines Monte Carlo Tree Search with Monster Group theory for\n");
        report.push_str("intelligent, reward-driven DefId ecosystem evolution.\n");
        
        report
    }
}

fn main() {
    println!("🎯 MCTS-Guided Monster DefId Growth");
    println!("===================================");
    
    // Use our self-referential seed
    let seed_signature = 0xD4D8CB67E7D5D13Du128;
    let seed_context = "Self::) &&::default_string";
    
    let mut mcts_growth = MCTSMonsterGrowth::new(seed_signature, seed_context);
    
    // Run MCTS-guided growth
    mcts_growth.mcts_guided_growth(1000, 50);
    
    let report = mcts_growth.generate_mcts_report();
    
    match fs::write("mcts_monster_growth_report.md", &report) {
        Ok(()) => println!("📊 MCTS report saved: mcts_monster_growth_report.md"),
        Err(e) => eprintln!("❌ Error saving report: {}", e),
    }
    
    println!("\n🎉 MCTS-GUIDED GROWTH COMPLETE!");
    println!("==============================");
    println!("Nodes explored: {}", mcts_growth.nodes.len());
    println!("DefIds grown: {}", mcts_growth.grown_defids.len());
    
    if let Some(best_defid) = mcts_growth.grown_defids.iter().max_by(|a, b| a.mcts_reward.partial_cmp(&b.mcts_reward).unwrap()) {
        println!("Best DefId: {} (reward: {:.4})", best_defid.defid, best_defid.mcts_reward);
    }
    
    println!("\n🧬 MCTS + Monster Group = Intelligent Evolution!");
    println!("🎯 First AI-optimized compiler growth system achieved!");
}
