use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct GraphInterpreterWithCallbacks {
    // CPU State
    instruction_pointer: String,
    registers: [String; 8],
    stack: Vec<String>,
    heap: HashMap<String, String>,
    
    // Program
    graph: HashMap<String, Vec<String>>,
    
    // Symbol table and callbacks
    symbol_table: HashMap<String, String>, // node -> file path
    ast_cache: HashMap<String, String>,    // cached ASTs
    
    cycle_count: usize,
}

impl GraphInterpreterWithCallbacks {
    fn new() -> Self {
        Self {
            instruction_pointer: String::new(),
            registers: Default::default(),
            stack: Vec::new(),
            heap: HashMap::new(),
            graph: HashMap::new(),
            symbol_table: HashMap::new(),
            ast_cache: HashMap::new(),
            cycle_count: 0,
        }
    }

    fn load_program(&mut self) -> Result<()> {
        println!("💾 Loading program and building symbol table...");
        
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
                        
                        self.graph.entry(caller).or_insert_with(Vec::new).push(callee.clone());
                        
                        // Build symbol table - map nodes to potential file paths
                        self.build_symbol_entry(&callee);
                    }
                }
            }
        }
        
        println!("  Program loaded: {} nodes", self.graph.len());
        println!("  Symbol table: {} entries", self.symbol_table.len());
        Ok(())
    }

    fn build_symbol_entry(&mut self, node: &str) {
        // Extract file paths from DefId nodes
        if node.contains("DefId") && node.contains("~ ") {
            if let Some(start) = node.find("~ ") {
                if let Some(end) = node[start+2..].find("[") {
                    let module_path = &node[start+2..start+2+end];
                    
                    // Convert module path to potential file path
                    let file_path = if module_path.contains("::") {
                        let parts: Vec<&str> = module_path.split("::").collect();
                        if parts.len() > 1 {
                            format!("src/{}.rs", parts[0].replace("[", "").replace("]", ""))
                        } else {
                            format!("src/{}.rs", module_path.replace("[", "").replace("]", ""))
                        }
                    } else {
                        format!("src/{}.rs", module_path.replace("[", "").replace("]", ""))
                    };
                    
                    self.symbol_table.insert(node.to_string(), file_path);
                }
            }
        }
    }

    fn load_ast_callback(&mut self, node: &str) -> String {
        // Check if we have this in symbol table
        if let Some(file_path) = self.symbol_table.get(node) {
            // Check cache first
            if let Some(cached_ast) = self.ast_cache.get(node) {
                return cached_ast.clone();
            }
            
            // Try to load actual Rust file
            println!("    📁 Loading AST for {} from {}", self.clean_name(node), file_path);
            
            let ast_content = if Path::new(file_path).exists() {
                match fs::read_to_string(file_path) {
                    Ok(content) => {
                        // Simulate AST parsing - just take first few lines
                        let lines: Vec<&str> = content.lines().take(3).collect();
                        format!("AST[{}]", lines.join("; "))
                    }
                    Err(_) => format!("AST[FILE_NOT_FOUND: {}]", file_path)
                }
            } else {
                // Simulate AST for known patterns
                if node.contains("core") {
                    "AST[pub fn core_function() { /* core logic */ }]".to_string()
                } else if node.contains("rustc") {
                    "AST[fn rustc_function() { /* compiler logic */ }]".to_string()
                } else {
                    format!("AST[fn {}() {{ /* generated */ }}]", self.clean_name(node))
                }
            };
            
            // Cache the AST
            self.ast_cache.insert(node.to_string(), ast_content.clone());
            ast_content
        } else {
            format!("AST[UNKNOWN: {}]", self.clean_name(node))
        }
    }

    fn execute_cycle(&mut self) -> bool {
        self.cycle_count += 1;
        
        println!("\n🔄 Cycle {}", self.cycle_count);
        println!("  IP: {}", self.clean_name(&self.instruction_pointer));
        println!("  R0: {}", self.clean_name(&self.registers[0]));
        
        // CALLBACK: Load AST when we encounter a DefId
        if self.instruction_pointer.contains("DefId") {
            let current_ip = self.instruction_pointer.clone();
            let ast = self.load_ast_callback(&current_ip);
            println!("  🌳 AST Loaded: {}", ast.chars().take(50).collect::<String>());
            
            // Store AST in heap
            let heap_key = format!("ast_{}", self.cycle_count);
            self.heap.insert(heap_key.clone(), ast);
            println!("  💾 AST stored in heap: {}", heap_key);
        }
        
        // Execute instruction
        let instruction = self.read_node_file(&self.instruction_pointer);
        println!("  Instruction: {}", instruction);
        
        match self.get_node_type(&self.instruction_pointer) {
            "CALL" => self.execute_call(),
            "LOAD" => self.execute_load(),
            "LOAD_STR" => self.execute_load_str(),
            "LOAD_NUM" => self.execute_load_num(),
            "LOAD_DATA" => self.execute_load_data(),
            _ => panic!("🚨 UNKNOWN INSTRUCTION: {}", self.clean_name(&self.instruction_pointer)),
        }
        
        self.cycle_count < 20
    }

    fn read_node_file(&self, node: &str) -> String {
        if node.contains("DefId") {
            "CALL_WITH_AST".to_string()
        } else if node.contains("static") {
            "LOAD_CONST".to_string()
        } else {
            "NOP".to_string()
        }
    }

    fn get_node_type(&self, node: &str) -> &str {
        if node.contains("DefId") { "CALL" }
        else if node.contains("static") { "LOAD" }
        else if node.starts_with("\"") { "LOAD_STR" }
        else if node.chars().all(|c| c.is_ascii_digit()) { "LOAD_NUM" }
        else { "LOAD_DATA" }
    }

    fn execute_call(&mut self) {
        self.stack.push(self.instruction_pointer.clone());
        
        if let Some(callees) = self.graph.get(&self.instruction_pointer) {
            if !callees.is_empty() {
                let next = &callees[0];
                println!("  CALL: {} → {}", self.clean_name(&self.instruction_pointer), self.clean_name(next));
                self.instruction_pointer = next.clone();
            } else if !self.stack.is_empty() {
                self.instruction_pointer = self.stack.pop().unwrap();
                println!("  RET: returning to {}", self.clean_name(&self.instruction_pointer));
            }
        }
    }

    fn execute_load(&mut self) {
        self.registers[1] = self.instruction_pointer.clone();
        println!("  LOAD: {} → R1", self.clean_name(&self.instruction_pointer));
        
        if let Some(callees) = self.graph.get(&self.instruction_pointer) {
            if !callees.is_empty() {
                self.instruction_pointer = callees[0].clone();
            }
        }
    }

    fn execute_load_str(&mut self) {
        // Load string into R2
        self.registers[2] = self.instruction_pointer.clone();
        println!("  LOAD_STR: {} → R2", self.clean_name(&self.instruction_pointer));
        
        // Continue or return
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

    fn execute_load_num(&mut self) {
        // Load number into R3
        self.registers[3] = self.instruction_pointer.clone();
        println!("  LOAD_NUM: {} → R3", self.clean_name(&self.instruction_pointer));
        
        // Continue or return
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

    fn execute_load_data(&mut self) {
        // Load data into R4
        self.registers[4] = self.instruction_pointer.clone();
        println!("  LOAD_DATA: {} → R4", self.clean_name(&self.instruction_pointer));
        
        // Continue or return
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

    fn execute_nop(&mut self) {
        panic!("🚨 NOP PANIC: Unhandled instruction at IP: {}", self.clean_name(&self.instruction_pointer));
    }

    fn set_entry_point(&mut self) -> Result<()> {
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
        
        println!("🎯 Entry point: {}", self.clean_name(&self.instruction_pointer));
        Ok(())
    }

    fn run(&mut self, input_node: &str) -> Result<()> {
        println!("🚀 Starting Graph Interpreter with AST Callbacks\n");
        
        self.registers[0] = input_node.to_string();
        println!("📥 Input: {}", input_node);
        
        while self.execute_cycle() {}
        
        println!("\n🎭 Execution Complete!");
        println!("  Cycles: {}", self.cycle_count);
        println!("  ASTs loaded: {}", self.ast_cache.len());
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
    let mut interpreter = GraphInterpreterWithCallbacks::new();
    
    interpreter.load_program()?;
    interpreter.set_entry_point()?;
    
    interpreter.run("false")?;
    
    Ok(())
}
