use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct GraphInterpreter {
    // CPU State
    instruction_pointer: String,    // Current node
    registers: [String; 8],        // R0-R7
    stack: Vec<String>,            // Call stack
    heap: HashMap<String, String>, // Memory
    
    // Program
    graph: HashMap<String, Vec<String>>, // Call graph
    
    // Execution
    cycle_count: usize,
}

impl GraphInterpreter {
    fn new() -> Self {
        Self {
            instruction_pointer: String::new(),
            registers: Default::default(),
            stack: Vec::new(),
            heap: HashMap::new(),
            graph: HashMap::new(),
            cycle_count: 0,
        }
    }

    fn load_program(&mut self) -> Result<()> {
        println!("💾 Loading program (graph) into interpreter...");
        
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
                        
                        self.graph.entry(caller).or_insert_with(Vec::new).push(callee);
                    }
                }
            }
        }
        
        println!("  Program loaded: {} nodes", self.graph.len());
        Ok(())
    }

    fn set_entry_point(&mut self) -> Result<()> {
        // Find main (highest eigenvalue)
        let eigenmatrix_data = fs::read_to_string("usage_eigenmatrix.json")?;
        let data: Value = serde_json::from_str(&eigenmatrix_data)?;
        
        self.instruction_pointer = if let Some(core_def_ids) = data["core_def_ids"].as_array() {
            if let Some(first) = core_def_ids.first() {
                if let Some(node_data) = first.as_array() {
                    node_data[0].as_str().unwrap_or("static META").to_string()
                } else {
                    "static META".to_string()
                }
            } else {
                "static META".to_string()
            }
        } else {
            "static META".to_string()
        };
        
        println!("🎯 Entry point set: {}", self.clean_name(&self.instruction_pointer));
        Ok(())
    }

    fn load_input(&mut self, input_node: &str) {
        // Load input into R0
        self.registers[0] = input_node.to_string();
        println!("📥 Input loaded into R0: {}", self.clean_name(input_node));
    }

    fn execute_cycle(&mut self) -> bool {
        self.cycle_count += 1;
        
        println!("\n🔄 Cycle {}", self.cycle_count);
        println!("  IP: {}", self.clean_name(&self.instruction_pointer));
        println!("  R0: {}", self.clean_name(&self.registers[0]));
        println!("  Stack depth: {}", self.stack.len());
        
        // Read instruction (node file)
        let instruction = self.read_node_file(&self.instruction_pointer);
        println!("  Instruction: {}", instruction);
        
        // Execute based on node type
        match self.get_node_type(&self.instruction_pointer) {
            "CALL" => self.execute_call(),
            "LOAD" => self.execute_load(),
            "STORE" => self.execute_store(),
            "HALT" => return false,
            _ => self.execute_nop(),
        }
        
        // Check termination
        self.cycle_count < 50
    }

    fn read_node_file(&self, node: &str) -> String {
        // Simulate reading node as file
        if node.contains("static") {
            "LOAD_CONST".to_string()
        } else if node.contains("DefId") {
            "CALL_FUNC".to_string()
        } else if node.contains("\"") {
            "LOAD_STR".to_string()
        } else {
            "NOP".to_string()
        }
    }

    fn get_node_type(&self, node: &str) -> &str {
        if node.contains("static") { "LOAD" }
        else if node.contains("DefId") { "CALL" }
        else if self.graph.get(node).map_or(true, |v| v.is_empty()) { "HALT" }
        else { "CALL" }
    }

    fn execute_call(&mut self) {
        // Push current IP to stack
        self.stack.push(self.instruction_pointer.clone());
        
        // Jump to first callee
        if let Some(callees) = self.graph.get(&self.instruction_pointer) {
            if !callees.is_empty() {
                let next = &callees[0];
                println!("  CALL: {} → {}", self.clean_name(&self.instruction_pointer), self.clean_name(next));
                self.instruction_pointer = next.clone();
            } else {
                // No callees, return
                if !self.stack.is_empty() {
                    self.instruction_pointer = self.stack.pop().unwrap();
                    println!("  RET: no callees, returning to {}", self.clean_name(&self.instruction_pointer));
                }
            }
        } else if !self.stack.is_empty() {
            self.instruction_pointer = self.stack.pop().unwrap();
            println!("  RET: no graph entry, returning to {}", self.clean_name(&self.instruction_pointer));
        }
    }

    fn execute_load(&mut self) {
        // Load current node into R1
        self.registers[1] = self.instruction_pointer.clone();
        println!("  LOAD: {} → R1", self.clean_name(&self.instruction_pointer));
        
        // Move to next instruction or return
        if let Some(callees) = self.graph.get(&self.instruction_pointer) {
            if !callees.is_empty() {
                self.instruction_pointer = callees[0].clone();
            } else if !self.stack.is_empty() {
                self.instruction_pointer = self.stack.pop().unwrap();
                println!("  RET: returning to {}", self.clean_name(&self.instruction_pointer));
            }
        } else if !self.stack.is_empty() {
            self.instruction_pointer = self.stack.pop().unwrap();
            println!("  RET: returning to {}", self.clean_name(&self.instruction_pointer));
        }
    }

    fn execute_store(&mut self) {
        // Store R0 to heap
        let key = format!("heap_{}", self.cycle_count);
        self.heap.insert(key.clone(), self.registers[0].clone());
        println!("  STORE: R0 → {}", key);
    }

    fn execute_nop(&mut self) {
        println!("  NOP");
        // Try to continue or return from stack
        if let Some(callees) = self.graph.get(&self.instruction_pointer) {
            if !callees.is_empty() {
                self.instruction_pointer = callees[0].clone();
            } else if !self.stack.is_empty() {
                // Return from call
                self.instruction_pointer = self.stack.pop().unwrap();
                println!("  RET: returning to {}", self.clean_name(&self.instruction_pointer));
            }
        } else if !self.stack.is_empty() {
            self.instruction_pointer = self.stack.pop().unwrap();
            println!("  RET: returning to {}", self.clean_name(&self.instruction_pointer));
        }
    }

    fn run(&mut self, input_node: &str) -> Result<()> {
        println!("🚀 Starting Graph Interpreter\n");
        
        self.load_input(input_node);
        
        while self.execute_cycle() {
            // Continue execution
        }
        
        println!("\n🎭 Execution Complete!");
        println!("  Total cycles: {}", self.cycle_count);
        println!("  Final IP: {}", self.clean_name(&self.instruction_pointer));
        println!("  Heap entries: {}", self.heap.len());
        
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
    let mut interpreter = GraphInterpreter::new();
    
    interpreter.load_program()?;
    interpreter.set_entry_point()?;
    
    // Use a leaf node as input
    let leaf_input = "false"; // Simple leaf node
    
    interpreter.run(leaf_input)?;
    
    Ok(())
}
