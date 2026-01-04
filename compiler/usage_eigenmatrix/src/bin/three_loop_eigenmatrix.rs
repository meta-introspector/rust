use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct ThreeLoopEigenmatrix {
    input_loop: Vec<(String, u64)>,
    processing_loop: Vec<(String, u64)>,
    output_loop: Vec<(String, u64)>,
}

impl ThreeLoopEigenmatrix {
    fn new() -> Self {
        Self {
            input_loop: Vec::new(),
            processing_loop: Vec::new(),
            output_loop: Vec::new(),
        }
    }

    fn build_loops(&mut self) -> Result<()> {
        let mut node_usage = HashMap::new();
        let usage_dir = "../../usage_data";
        
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage in usages {
                        let used_node = usage["used_def_id"].as_str().unwrap_or("unknown");
                        let count = usage["usage_count"].as_u64().unwrap_or(1);
                        *node_usage.entry(used_node.to_string()).or_insert(0) += count;
                    }
                }
            }
        }
        
        // Classify into three loops
        for (node, count) in node_usage {
            match self.classify_node(&node) {
                "Input" => self.input_loop.push((node, count)),
                "Processing" => self.processing_loop.push((node, count)),
                "Output" => self.output_loop.push((node, count)),
                _ => {}
            }
        }
        
        // Sort by usage count
        self.input_loop.sort_by(|a, b| b.1.cmp(&a.1));
        self.processing_loop.sort_by(|a, b| b.1.cmp(&a.1));
        self.output_loop.sort_by(|a, b| b.1.cmp(&a.1));
        
        Ok(())
    }

    fn classify_node(&self, node: &str) -> &str {
        if node.contains("\"") || node.chars().all(|c| c.is_ascii_digit()) || node.contains("static") {
            "Input"
        } else if node.contains("emit") || node.contains("write") || node.contains("output") {
            "Output"
        } else {
            "Processing"
        }
    }

    fn run_three_loop_simulation(&self) {
        println!("🎭 Three-Loop Eigenmatrix Simulation\n");
        
        println!("📊 Loop Sizes:");
        println!("  📥 Input Loop: {} nodes", self.input_loop.len());
        println!("  ⚙️ Processing Loop: {} nodes", self.processing_loop.len());
        println!("  📤 Output Loop: {} nodes", self.output_loop.len());
        
        println!("\n🔄 Pipeline Execution:\n");
        
        // Step 1: Input loop applies to all
        println!("1️⃣ INPUT LOOP → ALL NODES");
        let input_applications = self.input_loop.len() * (self.input_loop.len() + self.processing_loop.len() + self.output_loop.len());
        println!("   {} input nodes × {} total nodes = {} applications", 
                 self.input_loop.len(), 
                 self.input_loop.len() + self.processing_loop.len() + self.output_loop.len(),
                 input_applications);
        
        for (i, (input_node, count)) in self.input_loop.iter().take(3).enumerate() {
            println!("   📥[{}] {} ({}x) → applies to all nodes", i, self.clean_name(input_node), count);
        }
        
        // Step 2: Processing loop applies to input
        println!("\n2️⃣ PROCESSING LOOP → INPUT LOOP");
        let processing_applications = self.processing_loop.len() * self.input_loop.len();
        println!("   {} processing nodes × {} input nodes = {} applications", 
                 self.processing_loop.len(), self.input_loop.len(), processing_applications);
        
        for (i, (proc_node, count)) in self.processing_loop.iter().take(3).enumerate() {
            println!("   ⚙️[{}] {} ({}x) → processes input", i, self.clean_name(proc_node), count);
        }
        
        // Step 3: Output loop applies to processing
        println!("\n3️⃣ OUTPUT LOOP → PROCESSING LOOP");
        let output_applications = self.output_loop.len() * self.processing_loop.len();
        println!("   {} output nodes × {} processing nodes = {} applications", 
                 self.output_loop.len(), self.processing_loop.len(), output_applications);
        
        for (i, (output_node, count)) in self.output_loop.iter().take(3).enumerate() {
            println!("   📤[{}] {} ({}x) → emits processing", i, self.clean_name(output_node), count);
        }
        
        println!("\n💥 Total Pipeline Applications:");
        let total = input_applications + processing_applications + output_applications;
        println!("   Input applications: {}", input_applications);
        println!("   Processing applications: {}", processing_applications);
        println!("   Output applications: {}", output_applications);
        println!("   TOTAL: {} applications", total);
        
        println!("\n🎯 Pipeline Flow:");
        println!("   📥 Input → ⚙️ Processing → 📤 Output");
        println!("   Each stage applies its eigenmatrix to the previous stage!");
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
    let mut eigenmatrix = ThreeLoopEigenmatrix::new();
    eigenmatrix.build_loops()?;
    eigenmatrix.run_three_loop_simulation();
    
    println!("\n🎭 Three-Loop Eigenmatrix Complete!");
    println!("The compiler applies each loop to the appropriate target in pipeline order!");
    
    Ok(())
}
