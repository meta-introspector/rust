use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct EmojiectQuadraticSimulator {
    program_graph: HashMap<String, Vec<String>>,  // Program structure
    ast_data_nodes: HashMap<String, String>,      // AST content for each node
    compiler_driver: Vec<String>,                 // Compiler emojiects
    node_emojis: HashMap<String, String>,         // Visual representation
}

impl EmojiectQuadraticSimulator {
    fn new() -> Self {
        Self {
            program_graph: HashMap::new(),
            ast_data_nodes: HashMap::new(),
            compiler_driver: Vec::new(),
            node_emojis: HashMap::new(),
        }
    }

    fn load_program_graph(&mut self) -> Result<()> {
        println!("📊 Loading program graph...");
        
        let usage_dir = "../../usage_data";
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage in usages {
                        let caller = usage["user_def_id"].as_str().unwrap_or("unknown").to_string();
                        let callee = usage["used_def_id"].as_str().unwrap_or("unknown").to_string();
                        
                        self.program_graph.entry(caller).or_insert_with(Vec::new).push(callee);
                    }
                }
            }
        }
        
        println!("  Graph nodes loaded: {}", self.program_graph.len());
        Ok(())
    }

    fn load_ast_data_nodes(&mut self) -> Result<()> {
        println!("🌳 Loading AST data nodes...");
        
        // Simulate AST content for each node
        for node in self.program_graph.keys() {
            let ast_content = format!("AST[{}]", self.clean_name(node));
            self.ast_data_nodes.insert(node.clone(), ast_content);
        }
        
        println!("  AST nodes loaded: {}", self.ast_data_nodes.len());
        Ok(())
    }

    fn create_compiler_driver(&mut self) -> Result<()> {
        println!("🔧 Creating compiler driver...");
        
        // Load top nodes as compiler emojiects
        let eigenmatrix_data = fs::read_to_string("usage_eigenmatrix.json")?;
        let data: Value = serde_json::from_str(&eigenmatrix_data)?;
        
        if let Some(core_def_ids) = data["core_def_ids"].as_array() {
            for node_array in core_def_ids.iter().take(50) { // Top 50 as compiler
                if let Some(node_data) = node_array.as_array() {
                    if node_data.len() >= 2 {
                        let name = node_data[0].as_str().unwrap_or("unknown").to_string();
                        self.compiler_driver.push(name);
                    }
                }
            }
        }
        
        println!("  Compiler emojiects: {}", self.compiler_driver.len());
        Ok(())
    }

    fn assign_emojis(&mut self) {
        let emojis = vec!["👑", "⚡", "🔍", "🔘", "🔢", "📝", "🔄", "💾", "⚖️", "🎨"];
        
        for (i, node) in self.compiler_driver.iter().enumerate() {
            let emoji = emojis.get(i % emojis.len()).unwrap_or(&"❓");
            self.node_emojis.insert(node.clone(), emoji.to_string());
        }
    }

    fn run_quadratic_simulation(&self) -> Result<()> {
        println!("\n🎭 Running Quadratic Emojiect Simulation");
        println!("Program Graph: {} nodes", self.program_graph.len());
        println!("Compiler Driver: {} emojiects", self.compiler_driver.len());
        println!("Total Applications: {} × {} = {}", 
                 self.program_graph.len(), self.compiler_driver.len(),
                 self.program_graph.len() * self.compiler_driver.len());
        
        println!("\n🔄 Simulation Process:");
        
        let mut applications = 0;
        for (i, program_node) in self.program_graph.keys().enumerate() {
            if i >= 10 { break; } // Show first 10 for demo
            
            println!("\n📍 Processing program node: {}", self.clean_name(program_node));
            
            for compiler_emojiect in &self.compiler_driver {
                let emoji = self.node_emojis.get(compiler_emojiect).unwrap_or(&"❓".to_string());
                let ast_data = self.ast_data_nodes.get(program_node).unwrap_or(&"AST[unknown]".to_string());
                
                println!("  {} {} → {}", emoji, self.clean_name(compiler_emojiect), ast_data);
                applications += 1;
            }
        }
        
        println!("\n💥 Quadratic Explosion Demonstrated:");
        println!("  Applications shown: {}", applications);
        println!("  Full simulation would be: {} applications", 
                 self.program_graph.len() * self.compiler_driver.len());
        println!("  Each compiler emojiect processes each program node!");
        
        Ok(())
    }

    fn clean_name(&self, name: &str) -> String {
        if name.contains("DefId") {
            if let Some(start) = name.find("~ ") {
                if let Some(end) = name[start+2..].find(")") {
                    return name[start+2..start+2+end].chars().take(20).collect();
                }
            }
        }
        name.chars().take(15).collect()
    }
}

fn main() -> Result<()> {
    println!("🎯 Emojiect Quadratic Simulation System\n");
    
    let mut simulator = EmojiectQuadraticSimulator::new();
    
    simulator.load_program_graph()?;
    simulator.load_ast_data_nodes()?;
    simulator.create_compiler_driver()?;
    simulator.assign_emojis();
    simulator.run_quadratic_simulation()?;
    
    println!("\n🎭 Simulation Complete!");
    println!("This demonstrates how every compiler emojiect gets applied to every program node!");
    
    Ok(())
}
