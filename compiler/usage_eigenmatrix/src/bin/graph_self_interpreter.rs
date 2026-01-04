use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔄 GRAPH SELF-INTERPRETATION: EVERY NODE → EVERY NODE");
    println!("═══════════════════════════════════════════════════");
    
    // Load the graph
    let graph = load_graph()?;
    let nodes: Vec<String> = graph.keys().cloned().collect();
    println!("📊 Loaded {} nodes", nodes.len());
    
    // Apply each node to each node exactly once
    let mut interpreter = GraphInterpreter::new(graph);
    let result = interpreter.interpret_all_pairs(&nodes)?;
    
    println!("✅ Interpretation complete: {} operations", result.len());
    
    // Save results
    let result_json = serde_json::json!({
        "total_operations": result.len(),
        "operations": result.iter().take(100).map(|op| {
            serde_json::json!({
                "node_a": op.node_a,
                "node_b": op.node_b,
                "result_count": op.result.len(),
                "operation_type": op.operation_type
            })
        }).collect::<Vec<_>>()
    });
    
    fs::write("graph_self_interpretation.json", 
        serde_json::to_string_pretty(&result_json)?)?;
    
    Ok(())
}

struct GraphInterpreter {
    graph: HashMap<String, Vec<String>>,
    operations: Vec<Operation>,
}

#[derive(Clone)]
struct Operation {
    node_a: String,
    node_b: String,
    result: Vec<String>,
    operation_type: String,
}

impl GraphInterpreter {
    fn new(graph: HashMap<String, Vec<String>>) -> Self {
        Self {
            graph,
            operations: Vec::new(),
        }
    }
    
    fn interpret_all_pairs(&mut self, nodes: &[String]) -> Result<Vec<Operation>> {
        println!("🎯 Starting all-pairs interpretation...");
        let total_ops = nodes.len() * nodes.len();
        println!("   Total operations: {}", total_ops);
        
        for (i, node_a) in nodes.iter().enumerate() {
            for (j, node_b) in nodes.iter().enumerate() {
                if i % 10 == 0 && j == 0 {
                    println!("   Progress: {}/{} nodes", i, nodes.len());
                }
                
                let result = self.apply_node_to_node(node_a, node_b);
                
                self.operations.push(Operation {
                    node_a: node_a.clone(),
                    node_b: node_b.clone(),
                    result,
                    operation_type: self.determine_operation_type(node_a, node_b),
                });
            }
        }
        
        Ok(self.operations.clone())
    }
    
    fn apply_node_to_node(&self, node_a: &str, node_b: &str) -> Vec<String> {
        // Get connections for both nodes
        let connections_a = self.graph.get(node_a).cloned().unwrap_or_default();
        let connections_b = self.graph.get(node_b).cloned().unwrap_or_default();
        
        // Apply node_a to node_b: combine their connections
        let mut result = Vec::new();
        
        // If A calls B, result is B's connections
        if connections_a.contains(&node_b.to_string()) {
            result.extend(connections_b.clone());
        }
        
        // If B calls A, result is A's connections  
        if connections_b.contains(&node_a.to_string()) {
            result.extend(connections_a.clone());
        }
        
        // If no direct connection, create intersection
        if result.is_empty() {
            for conn_a in &connections_a {
                if connections_b.contains(conn_a) {
                    result.push(conn_a.clone());
                }
            }
        }
        
        // Remove duplicates
        result.sort();
        result.dedup();
        result
    }
    
    fn determine_operation_type(&self, node_a: &str, node_b: &str) -> String {
        let connections_a = self.graph.get(node_a).cloned().unwrap_or_default();
        let connections_b = self.graph.get(node_b).cloned().unwrap_or_default();
        
        if node_a == node_b {
            "self_application".to_string()
        } else if connections_a.contains(&node_b.to_string()) {
            "direct_call".to_string()
        } else if connections_b.contains(&node_a.to_string()) {
            "reverse_call".to_string()
        } else {
            "intersection".to_string()
        }
    }
}

fn load_graph() -> Result<HashMap<String, Vec<String>>> {
    let mut graph = HashMap::new();
    let usage_dir = "../../usage_data";
    let mut file_count = 0;
    
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
                    
                    graph.entry(user_id.to_string())
                        .or_insert_with(Vec::new)
                        .push(used_id.to_string());
                }
            }
            
            file_count += 1;
            if file_count >= 50 { // Limit for manageable computation
                break;
            }
        }
    }
    
    Ok(graph)
}
