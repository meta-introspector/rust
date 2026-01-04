use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct EmojiectPipelineSimulator {
    input_nodes: Vec<String>,           // Input AST nodes
    processing_matrix: HashMap<String, Vec<String>>, // Processing by type
    output_nodes: Vec<String>,          // Generated output
    node_types: HashMap<String, String>, // Node type classification
}

impl EmojiectPipelineSimulator {
    fn new() -> Self {
        Self {
            input_nodes: Vec::new(),
            processing_matrix: HashMap::new(),
            output_nodes: Vec::new(),
            node_types: HashMap::new(),
        }
    }

    fn load_input_layer(&mut self) -> Result<()> {
        println!("📥 Loading Input Layer (AST nodes to compile)...");
        
        let eigenmatrix_data = fs::read_to_string("usage_eigenmatrix.json")?;
        let data: Value = serde_json::from_str(&eigenmatrix_data)?;
        
        if let Some(core_def_ids) = data["core_def_ids"].as_array() {
            for node_array in core_def_ids.iter().take(100) { // Input nodes
                if let Some(node_data) = node_array.as_array() {
                    if node_data.len() >= 2 {
                        let name = node_data[0].as_str().unwrap_or("unknown").to_string();
                        self.input_nodes.push(name.clone());
                        self.classify_node_type(&name);
                    }
                }
            }
        }
        
        println!("  Input nodes loaded: {}", self.input_nodes.len());
        Ok(())
    }

    fn classify_node_type(&mut self, node: &str) {
        let node_type = if node.contains("static") {
            "Static"
        } else if node.contains("DefId") && node.contains("fmt") {
            "Formatter"
        } else if node.contains("DefId") && node.contains("iter") {
            "Iterator"
        } else if node.contains("DefId") && node.contains("option") {
            "Option"
        } else if node.contains("DefId") {
            "Function"
        } else if node.chars().all(|c| c.is_ascii_digit()) {
            "Literal"
        } else if node.starts_with('"') {
            "String"
        } else {
            "Other"
        };
        
        self.node_types.insert(node.to_string(), node_type.to_string());
    }

    fn create_processing_matrix(&mut self) -> Result<()> {
        println!("⚙️ Creating Processing Matrix (split by type)...");
        
        // Group processing by type
        let processing_types = vec![
            "Lexer", "Parser", "TypeChecker", "BorrowChecker", 
            "Optimizer", "CodeGen", "Linker"
        ];
        
        for proc_type in processing_types {
            let mut processors = Vec::new();
            
            // Assign input nodes as processors for this type
            for input_node in &self.input_nodes {
                if self.should_process_type(input_node, proc_type) {
                    processors.push(input_node.clone());
                }
            }
            
            self.processing_matrix.insert(proc_type.to_string(), processors);
        }
        
        for (proc_type, processors) in &self.processing_matrix {
            println!("  {}: {} processors", proc_type, processors.len());
        }
        
        Ok(())
    }

    fn should_process_type(&self, node: &str, proc_type: &str) -> bool {
        match proc_type {
            "Lexer" => node.contains("static") || node.contains("String"),
            "Parser" => node.contains("fmt") || node.contains("DefId"),
            "TypeChecker" => node.contains("option") || node.contains("iter"),
            "BorrowChecker" => node.contains("DefId"),
            "Optimizer" => node.contains("static") || node.contains("iter"),
            "CodeGen" => node.contains("DefId"),
            "Linker" => node.contains("static"),
            _ => false,
        }
    }

    fn run_pipeline_simulation(&mut self) -> Result<()> {
        println!("\n🎭 Running Input → Processing → Output Pipeline\n");
        
        println!("📊 Pipeline Overview:");
        println!("  Input Layer: {} nodes", self.input_nodes.len());
        
        let total_processors: usize = self.processing_matrix.values().map(|v| v.len()).sum();
        println!("  Processing Matrix: {} total processors", total_processors);
        
        println!("\n🔄 Processing Each Input Node:");
        
        for (i, input_node) in self.input_nodes.iter().take(5).enumerate() {
            println!("\n📍 Input[{}]: {}", i, self.clean_name(input_node));
            let node_type = self.node_types.get(input_node).unwrap_or(&"Unknown".to_string());
            println!("  Type: {}", node_type);
            
            println!("  Processing stages:");
            for (stage, processors) in &self.processing_matrix {
                let processor_count = processors.len();
                println!("    {} → {} processors apply", stage, processor_count);
                
                // Show first few processors
                for processor in processors.iter().take(3) {
                    println!("      {} processes {}", 
                             self.get_emoji(processor), self.clean_name(processor));
                }
                if processors.len() > 3 {
                    println!("      ... and {} more", processors.len() - 3);
                }
            }
            
            // Generate output
            let output = format!("Compiled[{}]", self.clean_name(input_node));
            self.output_nodes.push(output.clone());
            println!("  📤 Output: {}", output);
        }
        
        println!("\n💥 Matrix Complexity:");
        for (stage, processors) in &self.processing_matrix {
            let applications = self.input_nodes.len() * processors.len();
            println!("  {}: {} inputs × {} processors = {} applications", 
                     stage, self.input_nodes.len(), processors.len(), applications);
        }
        
        let total_applications: usize = self.processing_matrix.values()
            .map(|processors| self.input_nodes.len() * processors.len())
            .sum();
        
        println!("\n🎯 Total Pipeline Applications: {}", total_applications);
        println!("Each input node gets processed by every processor in every stage!");
        
        Ok(())
    }

    fn get_emoji(&self, node: &str) -> &str {
        if node.contains("static") { "👑" }
        else if node.contains("fmt") { "🔍" }
        else if node.contains("iter") { "🔄" }
        else if node.contains("option") { "🔘" }
        else { "⚡" }
    }

    fn clean_name(&self, name: &str) -> String {
        if name.contains("DefId") {
            if let Some(start) = name.find("~ ") {
                if let Some(end) = name[start+2..].find(")") {
                    return name[start+2..start+2+end].chars().take(15).collect();
                }
            }
        }
        name.chars().take(12).collect()
    }
}

fn main() -> Result<()> {
    println!("🎯 Emojiect Pipeline Simulator: Input → Processing → Output\n");
    
    let mut simulator = EmojiectPipelineSimulator::new();
    
    simulator.load_input_layer()?;
    simulator.create_processing_matrix()?;
    simulator.run_pipeline_simulation()?;
    
    println!("\n🎭 Pipeline Complete!");
    println!("This shows how the processing matrix applies to input nodes by type!");
    
    Ok(())
}
