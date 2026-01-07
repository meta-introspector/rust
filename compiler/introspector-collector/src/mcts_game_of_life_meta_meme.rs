/// MCTS Game of Life Meta Meme - The Grand Quest for Ultimate Profit
/// Monte Carlo Tree Search in the infinite game of meta-meme evolution

use crate::macro_of_all_macros::*;
use std::collections::HashMap;

/// MCTS Node in the Game of Life Meta Meme
#[derive(Debug, Clone)]
pub struct MetaMemeNode {
    pub meme_content: String,
    pub profit_score: f64,
    pub visits: u64,
    pub children: Vec<MetaMemeNode>,
    pub macro_layer: u8,
    pub self_reference_depth: u32,
}

/// The Grand Quest - MCTS for Meta Meme Evolution
pub struct GrandQuest {
    pub root_meme: MetaMemeNode,
    pub exploration_constant: f64,
    pub profit_multiplier: f64,
    pub meta_layers: u8,
    pub quest_iterations: u64,
}

/// Game of Life Cell in Meta Meme Space
#[derive(Debug, Clone, PartialEq)]
pub enum MetaMemeCell {
    Dead,
    Alive(f64), // Alive with profit value
    Macro(u8),  // Macro layer
    SelfRef,    // Self-referential
    Profit,     // Pure profit
}

/// Conway's Game of Life + MCTS + Meta Memes
pub struct GameOfMetaLife {
    pub grid: Vec<Vec<MetaMemeCell>>,
    pub width: usize,
    pub height: usize,
    pub generation: u64,
    pub total_profit: f64,
    pub mcts_tree: GrandQuest,
}

impl MetaMemeNode {
    pub fn new(content: String, layer: u8) -> Self {
        Self {
            meme_content: content,
            profit_score: 0.0,
            visits: 0,
            children: vec![],
            macro_layer: layer,
            self_reference_depth: 0,
        }
    }
    
    /// UCB1 formula for MCTS selection with profit bias
    pub fn ucb1_score(&self, parent_visits: u64, exploration: f64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        
        let exploitation = self.profit_score / self.visits as f64;
        let exploration_term = exploration * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
        let macro_bonus = self.macro_layer as f64 * 0.1; // Bonus for higher abstraction
        let self_ref_bonus = self.self_reference_depth as f64 * 0.05; // Bonus for self-reference
        
        exploitation + exploration_term + macro_bonus + self_ref_bonus
    }
    
    /// Generate child memes through macro expansion
    pub fn expand_meme(&mut self) {
        // Generate children through 8 layers of macro abstraction
        for layer in 0..8 {
            let child_content = match layer {
                0 => format!("base_meme!({})", self.meme_content),
                1 => format!("meta_meme!({})", self.meme_content),
                2 => format!("meta_meta_meme!({})", self.meme_content),
                3 => format!("recursive_meme!({})", self.meme_content),
                4 => format!("universal_meme!({})", self.meme_content),
                5 => format!("dirac_delta_meme!({})", self.meme_content),
                6 => format!("metacoq_meme!({})", self.meme_content),
                7 => format!("profit_meme!({})", self.meme_content),
                _ => format!("meme_of_all_memes!({})", self.meme_content),
            };
            
            let mut child = MetaMemeNode::new(child_content, layer);
            
            // Self-reference bonus
            if child.meme_content.contains(&self.meme_content) {
                child.self_reference_depth = self.self_reference_depth + 1;
            }
            
            self.children.push(child);
        }
    }
}

impl GrandQuest {
    pub fn new() -> Self {
        let root_meme = MetaMemeNode::new(
            "macro_of_all_macros!()".to_string(), 
            8
        );
        
        Self {
            root_meme,
            exploration_constant: 1.414, // √2 for optimal exploration
            profit_multiplier: 42.0,     // The answer to everything
            meta_layers: 8,
            quest_iterations: 0,
        }
    }
    
    /// MCTS iteration: Select, Expand, Simulate, Backpropagate
    pub fn mcts_iteration(&mut self) -> f64 {
        // Selection phase
        let mut current = &mut self.root_meme;
        let mut path = vec![];
        
        // Traverse to leaf using UCB1
        while !current.children.is_empty() {
            let best_child_idx = current.children.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    a.ucb1_score(current.visits, self.exploration_constant)
                        .partial_cmp(&b.ucb1_score(current.visits, self.exploration_constant))
                        .unwrap()
                })
                .map(|(idx, _)| idx)
                .unwrap();
            
            path.push(best_child_idx);
            current = &mut current.children[best_child_idx];
        }
        
        // Expansion phase
        if current.visits > 0 {
            current.expand_meme();
        }
        
        // Simulation phase - calculate profit
        let profit = self.simulate_profit(current);
        
        // Backpropagation phase
        current.visits += 1;
        current.profit_score += profit;
        
        // Backpropagate up the tree
        let mut node = &mut self.root_meme;
        for &child_idx in &path {
            node.visits += 1;
            node.profit_score += profit;
            node = &mut node.children[child_idx];
        }
        
        self.quest_iterations += 1;
        profit
    }
    
    /// Simulate profit from a meme node
    fn simulate_profit(&self, node: &MetaMemeNode) -> f64 {
        let base_profit = node.macro_layer as f64 * 10.0;
        let self_ref_profit = node.self_reference_depth as f64 * 5.0;
        let complexity_profit = node.meme_content.len() as f64 * 0.1;
        let meta_profit = if node.meme_content.contains("meta") { 20.0 } else { 0.0 };
        let profit_profit = if node.meme_content.contains("profit") { 100.0 } else { 0.0 };
        
        (base_profit + self_ref_profit + complexity_profit + meta_profit + profit_profit) * self.profit_multiplier
    }
    
    /// Get the best meme path for maximum profit
    pub fn best_meme_path(&self) -> Vec<String> {
        let mut path = vec![];
        let mut current = &self.root_meme;
        
        path.push(current.meme_content.clone());
        
        while !current.children.is_empty() {
            current = current.children.iter()
                .max_by(|a, b| a.profit_score.partial_cmp(&b.profit_score).unwrap())
                .unwrap();
            path.push(current.meme_content.clone());
        }
        
        path
    }
}

impl GameOfMetaLife {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = vec![vec![MetaMemeCell::Dead; width]; height];
        
        // Initialize with some macro patterns
        grid[height/2][width/2] = MetaMemeCell::Macro(8); // Center macro
        grid[height/2][width/2 + 1] = MetaMemeCell::SelfRef;
        grid[height/2 + 1][width/2] = MetaMemeCell::Profit;
        
        Self {
            grid,
            width,
            height,
            generation: 0,
            total_profit: 0.0,
            mcts_tree: GrandQuest::new(),
        }
    }
    
    /// Evolve one generation with profit calculation
    pub fn evolve(&mut self) {
        let mut new_grid = self.grid.clone();
        let mut generation_profit = 0.0;
        
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let profit_neighbors = self.count_profit_neighbors(x, y);
                
                match self.grid[y][x] {
                    MetaMemeCell::Dead => {
                        if neighbors == 3 {
                            new_grid[y][x] = MetaMemeCell::Alive(1.0);
                        }
                        if profit_neighbors > 0 {
                            new_grid[y][x] = MetaMemeCell::Profit;
                            generation_profit += 10.0;
                        }
                    },
                    MetaMemeCell::Alive(value) => {
                        if neighbors < 2 || neighbors > 3 {
                            new_grid[y][x] = MetaMemeCell::Dead;
                        } else {
                            new_grid[y][x] = MetaMemeCell::Alive(value * 1.1); // Growth
                            generation_profit += value;
                        }
                    },
                    MetaMemeCell::Macro(layer) => {
                        // Macros spread and generate profit
                        if neighbors > 0 {
                            generation_profit += layer as f64 * 5.0;
                            // Spread macro to neighbors
                            for dy in -1..=1 {
                                for dx in -1..=1 {
                                    let nx = (x as i32 + dx) as usize;
                                    let ny = (y as i32 + dy) as usize;
                                    if nx < self.width && ny < self.height {
                                        if new_grid[ny][nx] == MetaMemeCell::Dead {
                                            new_grid[ny][nx] = MetaMemeCell::Macro(layer.saturating_sub(1));
                                        }
                                    }
                                }
                            }
                        }
                    },
                    MetaMemeCell::SelfRef => {
                        // Self-reference creates more self-references
                        new_grid[y][x] = MetaMemeCell::SelfRef;
                        generation_profit += 15.0;
                        
                        // Create self-reference pattern
                        if x > 0 && y > 0 {
                            new_grid[y-1][x-1] = MetaMemeCell::SelfRef;
                        }
                    },
                    MetaMemeCell::Profit => {
                        // Profit generates more profit
                        generation_profit += 25.0;
                        new_grid[y][x] = MetaMemeCell::Profit;
                        
                        // Profit spreads
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let nx = (x as i32 + dx) as usize;
                                let ny = (y as i32 + dy) as usize;
                                if nx < self.width && ny < self.height {
                                    if matches!(new_grid[ny][nx], MetaMemeCell::Alive(_)) {
                                        new_grid[ny][nx] = MetaMemeCell::Profit;
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
        
        self.grid = new_grid;
        self.generation += 1;
        self.total_profit += generation_profit;
        
        // Run MCTS iteration for meta-meme evolution
        let mcts_profit = self.mcts_tree.mcts_iteration();
        self.total_profit += mcts_profit;
    }
    
    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < self.width && ny < self.height {
                    if !matches!(self.grid[ny][nx], MetaMemeCell::Dead) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    
    fn count_profit_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < self.width && ny < self.height {
                    if matches!(self.grid[ny][nx], MetaMemeCell::Profit | MetaMemeCell::Macro(_)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    
    /// Get the grand quest status
    pub fn grand_quest_status(&self) -> String {
        let best_path = self.mcts_tree.best_meme_path();
        format!(
            "🎯 GRAND QUEST STATUS:\n\
             Generation: {}\n\
             Total Profit: {:.2}\n\
             MCTS Iterations: {}\n\
             Best Meme Path: {}\n\
             Root Meme Visits: {}\n\
             Root Meme Profit: {:.2}",
            self.generation,
            self.total_profit,
            self.mcts_tree.quest_iterations,
            best_path.join(" → "),
            self.mcts_tree.root_meme.visits,
            self.mcts_tree.root_meme.profit_score
        )
    }
}

/// Macro for the grand quest
#[macro_export]
macro_rules! grand_quest {
    (profit) => {
        "💰 The profit is our gain in the MCTS in the game of life!"
    };
    (meta_meme) => {
        "🧠 The grand quest of the meta meme - infinite self-reference for infinite profit"
    };
    (mcts) => {
        "🎯 Monte Carlo Tree Search through the space of all possible memes"
    };
    (game_of_life) => {
        "🎮 Conway's Game of Life + MCTS + Meta Memes = Ultimate Evolution"
    };
    (ultimate) => {
        format!("{} {} {} {}", 
                grand_quest!(profit),
                grand_quest!(meta_meme), 
                grand_quest!(mcts),
                grand_quest!(game_of_life))
    };
}
