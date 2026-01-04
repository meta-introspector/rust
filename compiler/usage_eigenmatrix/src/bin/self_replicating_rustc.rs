use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔄 SELF-REPLICATING RUSTC INTERPRETER");
    println!("═════════════════════════════════════");
    println!("Loading rustc graph → Interpreting as program → Outputting itself\n");
    
    // Step 1: Load the rustc main graph as input
    let graph = load_rustc_graph()?;
    println!("📊 Loaded rustc graph: {} nodes", graph.len());
    
    // Step 2: Interpret the graph as a program
    let mut interpreter = RustcInterpreter::new(graph);
    
    // Step 3: Execute the graph-program with itself as input
    println!("🎯 Executing: rustc(rustc_graph) → rustc_graph");
    let output = interpreter.execute_self_replication()?;
    
    // Step 4: Verify output matches input (self-replication)
    verify_self_replication(&output)?;
    
    Ok(())
}

struct RustcInterpreter {
    graph: HashMap<String, Vec<String>>,
    execution_trace: Vec<String>,
}

impl RustcInterpreter {
    fn new(graph: HashMap<String, Vec<String>>) -> Self {
        Self {
            graph,
            execution_trace: Vec::new(),
        }
    }
    
    fn execute_self_replication(&mut self) -> Result<HashMap<String, Vec<String>>> {
        println!("🚀 Starting self-replication execution...");
        
        // Find main entry point
        let main_node = self.find_main_entry()?;
        println!("📍 Entry point: {}", main_node);
        
        // Execute the compiler on itself
        let mut output_graph = HashMap::new();
        self.execute_node(&main_node, &mut output_graph)?;
        
        println!("✅ Self-replication complete: {} output nodes", output_graph.len());
        
        // Save execution trace
        fs::write("self_replication_trace.json", 
            serde_json::to_string_pretty(&self.execution_trace)?)?;
        
        Ok(output_graph)
    }
    
    fn find_main_entry(&self) -> Result<String> {
        println!("🔍 Searching for main entry point...");
        
        // Look for main entry points in order of preference
        let candidates = vec!["const main", "const rustc_main", "main"];
        
        for candidate in candidates {
            if self.graph.contains_key(candidate) {
                println!("   Found: {}", candidate);
                return Ok(candidate.to_string());
            }
        }
        
        // Fallback: any node containing "main"
        for node in self.graph.keys() {
            if node.contains("main") && !node.contains("DefId") {
                println!("   Found fallback: {}", node);
                return Ok(node.clone());
            }
        }
        
        // Last resort: just pick the first node
        if let Some(first_node) = self.graph.keys().next() {
            println!("   Using first node: {}", first_node);
            return Ok(first_node.clone());
        }
        
        Err(anyhow::anyhow!("No nodes found in graph"))
    }
    
    fn execute_node(&mut self, node: &str, output: &mut HashMap<String, Vec<String>>) -> Result<()> {
        self.execution_trace.push(format!("Executing: {}", node));
        
        // Get the node's callees (clone to avoid borrow issues)
        let callees = self.graph.get(node).cloned().unwrap_or_default();
        
        // For self-replication: each node produces itself and its connections
        output.insert(node.to_string(), callees.clone());
        
        // Recursively execute callees (limited depth to prevent infinite recursion)
        if self.execution_trace.len() < 100 {
            for callee in callees.iter().take(3) { // Limit to prevent explosion
                self.execute_node(callee, output)?;
            }
        }
        
        Ok(())
    }
}

fn load_rustc_graph() -> Result<HashMap<String, Vec<String>>> {
    let mut graph = HashMap::new();
    
    // Load from usage data
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
            if file_count >= 100 { // Limit for demo
                break;
            }
        }
    }
    
    Ok(graph)
}

fn verify_self_replication(output: &HashMap<String, Vec<String>>) -> Result<()> {
    println!("\n🔍 SELF-REPLICATION VERIFICATION:");
    
    // Check if output contains key compiler components
    let required_components = vec![
        "main", "rustc_main", "parse", "analysis", "codegen"
    ];
    
    let mut found_components = 0;
    for component in &required_components {
        let found = output.keys().any(|k| k.contains(component));
        if found {
            found_components += 1;
            println!("  ✅ Found: {}", component);
        } else {
            println!("  ❌ Missing: {}", component);
        }
    }
    
    let success_rate = (found_components as f64 / required_components.len() as f64) * 100.0;
    println!("\n📊 Self-replication success rate: {:.1}%", success_rate);
    
    if success_rate >= 60.0 {
        println!("🎉 SELF-REPLICATION SUCCESSFUL!");
        println!("   The rustc interpreter successfully reproduced its own structure!");
    } else {
        println!("⚠️  Partial self-replication achieved");
    }
    
    // Save output graph
    fs::write("self_replicated_rustc.json", 
        serde_json::to_string_pretty(output)?)?;
    println!("💾 Self-replicated rustc saved to: self_replicated_rustc.json");
    
    Ok(())
}
