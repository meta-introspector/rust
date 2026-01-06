use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use rand::Rng;

#[derive(Debug, Clone)]
struct UsageWeight {
    function: String,
    weight: f64,
    last_seen: u64,
    improvement_potential: f64,
}

#[derive(Debug)]
struct DrunkenWalker {
    usage_weights: HashMap<String, UsageWeight>,
    current_path: Vec<String>,
    exploration_bias: f64,
    mutation_rate: f64,
    generation: u64,
}

impl DrunkenWalker {
    fn new() -> Self {
        Self {
            usage_weights: HashMap::new(),
            current_path: Vec::new(),
            exploration_bias: 0.3,
            mutation_rate: 0.1,
            generation: 0,
        }
    }

    fn load_usage_patterns(&mut self, usage_dir: &str) {
        if let Ok(entries) = fs::read_dir(usage_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            if let Some(obj) = json.as_object() {
                                for (key, value) in obj {
                                    if let Some(count) = value.as_f64() {
                                        let weight = UsageWeight {
                                            function: key.clone(),
                                            weight: count,
                                            last_seen: self.generation,
                                            improvement_potential: self.calculate_improvement_potential(key, count),
                                        };
                                        self.usage_weights.insert(key.clone(), weight);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn calculate_improvement_potential(&self, function: &str, usage_count: f64) -> f64 {
        // High usage + complex patterns = high improvement potential
        let complexity_bonus = if function.contains("::") { 1.5 } else { 1.0 };
        let frequency_factor = (usage_count / 100.0).min(2.0);
        complexity_bonus * frequency_factor
    }

    fn choose_next_function(&mut self) -> Option<String> {
        let mut rng = rand::thread_rng();
        
        // Weighted random selection based on usage patterns and improvement potential
        let total_weight: f64 = self.usage_weights.values()
            .map(|w| w.weight * w.improvement_potential * self.exploration_bias)
            .sum();
        
        if total_weight == 0.0 {
            return None;
        }
        
        let mut random_point = rng.gen_range(0.0..1.0) * total_weight;
        
        for (func_name, weight) in &self.usage_weights {
            let adjusted_weight = weight.weight * weight.improvement_potential * self.exploration_bias;
            if random_point <= adjusted_weight {
                return Some(func_name.clone());
            }
            random_point -= adjusted_weight;
        }
        
        None
    }

    fn mutate_exploration_strategy(&mut self) {
        let mut rng = rand::thread_rng();
        
        if rng.gen_range(0.0..1.0) < self.mutation_rate {
            // Mutate exploration bias
            self.exploration_bias += rng.gen_range(-0.1..0.1);
            self.exploration_bias = self.exploration_bias.clamp(0.1, 0.9);
            
            // Mutate mutation rate itself (meta-mutation)
            self.mutation_rate += rng.gen_range(-0.01..0.01);
            self.mutation_rate = self.mutation_rate.clamp(0.05, 0.3);
            
            println!("🧬 Mutated: bias={:.3}, mutation_rate={:.3}", 
                     self.exploration_bias, self.mutation_rate);
        }
    }

    fn walk_and_predict(&mut self, steps: usize) -> Vec<String> {
        let mut predictions = Vec::new();
        
        for step in 0..steps {
            self.generation += 1;
            
            if let Some(next_func) = self.choose_next_function() {
                self.current_path.push(next_func.clone());
                predictions.push(next_func.clone());
                
                // Update weights based on path context
                self.update_contextual_weights(&next_func);
                
                // Occasionally mutate strategy
                if step % 10 == 0 {
                    self.mutate_exploration_strategy();
                }
                
                println!("🚶 Step {}: {} (potential: {:.2})", 
                         step, 
                         next_func,
                         self.usage_weights.get(&next_func)
                             .map(|w| w.improvement_potential)
                             .unwrap_or(0.0));
            }
        }
        
        predictions
    }

    fn update_contextual_weights(&mut self, current_func: &str) {
        // Pre-calculate similarities to avoid borrow checker issues
        let similarities: Vec<(String, f64)> = self.usage_weights.keys()
            .filter(|func_name| *func_name != current_func)
            .map(|func_name| (func_name.clone(), self.calculate_similarity(current_func, func_name)))
            .collect();
        
        // Update weights based on similarities
        for (func_name, similarity) in similarities {
            if similarity > 0.5 {
                if let Some(weight) = self.usage_weights.get_mut(&func_name) {
                    weight.improvement_potential *= 1.1;
                    weight.last_seen = self.generation;
                }
            }
        }
    }

    fn calculate_similarity(&self, func1: &str, func2: &str) -> f64 {
        // Simple similarity based on common prefixes and patterns
        let parts1: Vec<&str> = func1.split("::").collect();
        let parts2: Vec<&str> = func2.split("::").collect();
        
        let common_parts = parts1.iter()
            .zip(parts2.iter())
            .take_while(|(a, b)| a == b)
            .count();
        
        common_parts as f64 / parts1.len().max(parts2.len()) as f64
    }

    fn generate_improvement_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Find high-potential, underexplored functions
        let mut candidates: Vec<_> = self.usage_weights.values().collect();
        candidates.sort_by(|a, b| b.improvement_potential.partial_cmp(&a.improvement_potential).unwrap());
        
        for candidate in candidates.iter().take(5) {
            if candidate.improvement_potential > 1.0 {
                suggestions.push(format!(
                    "Focus on {}: {:.1}x improvement potential (used {} times)",
                    candidate.function,
                    candidate.improvement_potential,
                    candidate.weight as u64
                ));
            }
        }
        
        suggestions
    }

    fn evolve_strategy(&mut self) {
        // Decay old weights and boost recent discoveries
        for weight in self.usage_weights.values_mut() {
            let age = self.generation - weight.last_seen;
            if age > 50 {
                weight.improvement_potential *= 0.95; // Decay
            } else if age < 5 {
                weight.improvement_potential *= 1.05; // Boost recent
            }
        }
        
        println!("🧠 Strategy evolved at generation {}", self.generation);
    }
}

fn main() {
    println!("🍺 Drunken Walker: Self-Improving Code Explorer");
    
    let mut walker = DrunkenWalker::new();
    
    // Load usage patterns from current run
    walker.load_usage_patterns("./test_usage_data");
    
    println!("📊 Loaded {} usage patterns", walker.usage_weights.len());
    
    // Walk and predict next exploration targets
    let predictions = walker.walk_and_predict(20);
    
    println!("\n🔮 Predicted exploration path:");
    for (i, pred) in predictions.iter().enumerate() {
        println!("  {}. {}", i + 1, pred);
    }
    
    // Generate improvement suggestions
    let suggestions = walker.generate_improvement_suggestions();
    println!("\n💡 Improvement suggestions:");
    for suggestion in suggestions {
        println!("  • {}", suggestion);
    }
    
    // Evolve strategy for next run
    walker.evolve_strategy();
    
    println!("\n🎯 Next run will use:");
    println!("  • Exploration bias: {:.3}", walker.exploration_bias);
    println!("  • Mutation rate: {:.3}", walker.mutation_rate);
    println!("  • Generation: {}", walker.generation);
}
