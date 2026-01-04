use anyhow::Result;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
struct GraphNode {
    id: String,
    node_type: NodeType,
    connections: Vec<String>,
    execution_count: usize,
    data: Option<String>,
}

#[derive(Debug, Clone)]
enum NodeType {
    Function,
    Constant,
    Literal,
    Static,
    Driver,
}

struct GraphExecutor {
    nodes: HashMap<String, GraphNode>,
    execution_queue: VecDeque<String>,
    execution_log: Vec<String>,
}

impl GraphExecutor {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            execution_queue: VecDeque::new(),
            execution_log: Vec::new(),
        }
    }

    fn load_graph(&mut self) -> Result<()> {
        println!("🔄 Loading graph from usage data...");
        
        let usage_dir = "../../usage_data";
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage_obj in usages {
                        let user_id = usage_obj["user_def_id"].as_str().unwrap_or("unknown");
                        let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                        let count = usage_obj["usage_count"].as_u64().unwrap_or(1) as usize;
                        
                        self.add_node(user_id, used_id, count);
                    }
                }
            }
        }
        
        println!("📊 Loaded {} nodes", self.nodes.len());
        Ok(())
    }

    fn add_node(&mut self, user_id: &str, used_id: &str, _weight: usize) {
        let user_type = self.classify_node(user_id);
        let used_type = self.classify_node(used_id);
        let used_data = self.extract_data(used_id);
        
        // Add user node
        let user_node = self.nodes.entry(user_id.to_string()).or_insert_with(|| {
            GraphNode {
                id: user_id.to_string(),
                node_type: user_type.clone(),
                connections: Vec::new(),
                execution_count: 0,
                data: None,
            }
        });
        
        if !user_node.connections.contains(&used_id.to_string()) {
            user_node.connections.push(used_id.to_string());
        }
        
        // Add used node
        self.nodes.entry(used_id.to_string()).or_insert_with(|| {
            GraphNode {
                id: used_id.to_string(),
                node_type: used_type,
                connections: Vec::new(),
                execution_count: 0,
                data: used_data,
            }
        });
    }

    fn classify_node(&self, node_id: &str) -> NodeType {
        if node_id.contains("rustc_driver") {
            NodeType::Driver
        } else if node_id.starts_with("\"") || node_id.chars().all(|c| c.is_ascii_digit()) {
            NodeType::Literal
        } else if node_id == node_id.to_uppercase() && node_id.len() > 1 {
            NodeType::Static
        } else if node_id == "true" || node_id == "false" || node_id.starts_with("DefId") {
            NodeType::Constant
        } else {
            NodeType::Function
        }
    }

    fn extract_data(&self, node_id: &str) -> Option<String> {
        if node_id.starts_with("\"") && node_id.ends_with("\"") {
            Some(node_id[1..node_id.len()-1].to_string())
        } else if node_id == "true" || node_id == "false" {
            Some(node_id.to_string())
        } else if node_id.chars().all(|c| c.is_ascii_digit()) {
            Some(node_id.to_string())
        } else {
            None
        }
    }

    fn execute_graph(&mut self) -> Result<()> {
        println!("🚀 Starting graph execution simulation...");
        
        // Start with rustc_driver as entry point
        if let Some(driver_node) = self.find_driver_node() {
            self.execution_queue.push_back(driver_node);
        }
        
        let mut steps = 0;
        while !self.execution_queue.is_empty() && steps < 50 {
            let current_node_id = self.execution_queue.pop_front().unwrap();
            self.execute_node(&current_node_id)?;
            steps += 1;
        }
        
        self.print_execution_summary();
        Ok(())
    }

    fn find_driver_node(&self) -> Option<String> {
        self.nodes.keys()
            .find(|id| id.contains("rustc_driver"))
            .cloned()
    }

    fn execute_node(&mut self, node_id: &str) -> Result<()> {
        let simplified_name = self.simplify_name(node_id);
        
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.execution_count += 1;
            
            let execution_msg = match &node.node_type {
                NodeType::Driver => format!("🎯 DRIVER: {} initializing compiler", node_id),
                NodeType::Function => format!("⚙️  FUNCTION: {} executing", simplified_name),
                NodeType::Constant => format!("📊 CONSTANT: {} = {:?}", simplified_name, node.data),
                NodeType::Literal => format!("📝 LITERAL: {:?}", node.data),
                NodeType::Static => format!("🏗️  STATIC: {} accessed", node_id),
            };
            
            self.execution_log.push(execution_msg.clone());
            println!("  {}", execution_msg);
            
            // Queue connected nodes for execution
            let connections = node.connections.clone();
            drop(node); // Release the mutable borrow
            
            for connected_id in connections.iter().take(3) { // Limit to prevent explosion
                if !self.execution_queue.contains(connected_id) {
                    self.execution_queue.push_back(connected_id.clone());
                }
            }
        }
        
        Ok(())
    }

    fn simplify_name(&self, name: &str) -> String {
        if name.contains("::") {
            name.split("::").last().unwrap_or(name).to_string()
        } else {
            name.chars().take(30).collect()
        }
    }

    fn print_execution_summary(&self) {
        println!("\n📈 Execution Summary:");
        println!("  Total execution steps: {}", self.execution_log.len());
        
        let mut node_types = HashMap::new();
        for node in self.nodes.values() {
            if node.execution_count > 0 {
                *node_types.entry(format!("{:?}", node.node_type)).or_insert(0) += 1;
            }
        }
        
        println!("  Executed node types:");
        for (node_type, count) in node_types {
            println!("    {}: {} nodes", node_type, count);
        }
        
        // Save execution trace
        let trace = self.execution_log.join("\n");
        fs::write("graph_execution_trace.txt", trace).unwrap();
        println!("  💾 Execution trace saved to graph_execution_trace.txt");
    }
}

fn main() -> Result<()> {
    println!("🕸️ Graph Execution Engine - Interpreting Rust Ecosystem");
    
    let mut executor = GraphExecutor::new();
    executor.load_graph()?;
    executor.execute_graph()?;
    
    println!("\n✅ Graph execution simulation complete!");
    
    Ok(())
}
