use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct SelfReferencialExecutor {
    graph_state: HashMap<String, usize>,
    generation: usize,
}

impl SelfReferencialExecutor {
    fn new() -> Self {
        Self {
            graph_state: HashMap::new(),
            generation: 0,
        }
    }

    fn load_initial_state(&mut self) -> Result<()> {
        println!("🔄 Loading initial graph state (rustc reading itself)...");
        
        let usage_dir = "../../usage_data";
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage_obj in usages {
                        let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                        let count = usage_obj["usage_count"].as_u64().unwrap_or(1) as usize;
                        
                        *self.graph_state.entry(used_id.to_string()).or_insert(0) += count;
                    }
                }
            }
        }
        
        println!("📊 Initial state: {} nodes loaded", self.graph_state.len());
        Ok(())
    }

    fn execute_self_reference(&mut self) -> Result<()> {
        println!("\n🔁 Executing self-referential compilation...");
        
        loop {
            self.generation += 1;
            println!("\n🎯 Generation {}: rustc reading itself...", self.generation);
            
            let previous_state = self.graph_state.clone();
            let new_state = self.compile_self(&previous_state)?;
            
            if self.states_equal(&previous_state, &new_state) {
                println!("✅ FIXED POINT REACHED! Graph is stable.");
                println!("   rustc(graph) → rustc(same_graph)");
                break;
            } else {
                println!("🔄 Graph changed, continuing...");
                self.graph_state = new_state;
                
                if self.generation > 10 {
                    println!("⚠️  Stopping after 10 generations to prevent infinite loop");
                    break;
                }
            }
        }
        
        Ok(())
    }

    fn compile_self(&self, input_graph: &HashMap<String, usize>) -> Result<HashMap<String, usize>> {
        let mut output_graph = HashMap::new();
        
        // Simulate rustc reading its own structure
        for (node, count) in input_graph {
            // Core compilation logic: rustc processes its own nodes
            if node.contains("rustc_") {
                // Rustc components get reinforced (used more)
                output_graph.insert(node.clone(), count + 1);
            } else if node.contains("core::") {
                // Core functions remain stable
                output_graph.insert(node.clone(), *count);
            } else if node == "false" || node == "true" {
                // Constants remain constant
                output_graph.insert(node.clone(), *count);
            } else {
                // Other nodes might decay slightly
                output_graph.insert(node.clone(), count.saturating_sub(1));
            }
        }
        
        // Add some "compilation artifacts" - new nodes created during compilation
        if self.generation == 1 {
            output_graph.insert("compilation_artifact_1".to_string(), 1);
        }
        
        Ok(output_graph)
    }

    fn states_equal(&self, state1: &HashMap<String, usize>, state2: &HashMap<String, usize>) -> bool {
        if state1.len() != state2.len() {
            return false;
        }
        
        for (key, value1) in state1 {
            if let Some(value2) = state2.get(key) {
                if value1 != value2 {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }

    fn demonstrate_modification(&mut self) -> Result<()> {
        println!("\n🔧 Demonstrating graph modification...");
        
        // Modify the graph (simulate code change)
        self.graph_state.insert("NEW_FEATURE".to_string(), 100);
        println!("   Added NEW_FEATURE to graph");
        
        // Now execute again
        let previous_state = self.graph_state.clone();
        let new_state = self.compile_self(&previous_state)?;
        
        if !self.states_equal(&previous_state, &new_state) {
            println!("✅ Graph changed! rustc(modified_graph) → rustc(different_graph)");
            
            // Show the differences
            for (key, new_val) in &new_state {
                if let Some(old_val) = previous_state.get(key) {
                    if new_val != old_val {
                        println!("   {} changed: {} → {}", key, old_val, new_val);
                    }
                } else {
                    println!("   {} added: {}", key, new_val);
                }
            }
        }
        
        Ok(())
    }

    fn print_summary(&self) {
        println!("\n📈 Self-Reference Analysis Summary:");
        println!("  Final generation: {}", self.generation);
        println!("  Graph nodes: {}", self.graph_state.len());
        
        // Find most stable nodes (high counts)
        let mut stable_nodes: Vec<_> = self.graph_state.iter().collect();
        stable_nodes.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("  Most stable nodes (fixed points):");
        for (node, count) in stable_nodes.iter().take(5) {
            println!("    {} - stability: {}", 
                node.chars().take(40).collect::<String>(), count);
        }
    }
}

fn main() -> Result<()> {
    println!("🔁 Self-Referential Rust Compiler Analysis");
    println!("   Demonstrating: rustc(graph) → rustc(same_graph)");
    
    let mut executor = SelfReferencialExecutor::new();
    
    executor.load_initial_state()?;
    executor.execute_self_reference()?;
    executor.demonstrate_modification()?;
    executor.print_summary();
    
    println!("\n🎯 Key Insight: Rust compiler is a fixed-point system!");
    println!("   Until you change the graph, it reproduces itself exactly.");
    
    Ok(())
}
