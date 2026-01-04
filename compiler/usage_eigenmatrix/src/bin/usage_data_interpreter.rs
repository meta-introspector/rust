use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct UsageDataInterpreter {
    // Primary data from usage collection
    call_graph: HashMap<String, Vec<String>>,
    symbol_table: HashMap<String, String>,
    def_paths: HashMap<String, String>,
    
    // CPU state
    instruction_pointer: String,
    registers: [String; 8],
    stack: Vec<String>,
    
    // Source cache (loaded on demand)
    source_cache: HashMap<String, String>,
    
    cycle_count: usize,
}

impl UsageDataInterpreter {
    fn new() -> Self {
        Self {
            call_graph: HashMap::new(),
            symbol_table: HashMap::new(),
            def_paths: HashMap::new(),
            instruction_pointer: String::new(),
            registers: Default::default(),
            stack: Vec::new(),
            source_cache: HashMap::new(),
            cycle_count: 0,
        }
    }
    
    fn load_usage_data(&mut self) -> Result<()> {
        println!("📊 Loading usage data...");
        
        // Load clean graph data we created
        if fs::metadata("clean_graph_data.json").is_ok() {
            let clean_data = fs::read_to_string("clean_graph_data.json")?;
            let data: Value = serde_json::from_str(&clean_data)?;
            
            // Load call graph
            if let Some(call_graph) = data["call_graph"].as_object() {
                for (caller, callees) in call_graph {
                    if let Some(callees_array) = callees.as_array() {
                        let mut callee_list = Vec::new();
                        for callee in callees_array {
                            if let Some(callee_str) = callee.as_str() {
                                callee_list.push(callee_str.to_string());
                            }
                        }
                        self.call_graph.insert(caller.clone(), callee_list);
                    }
                }
            }
            
            // Load symbol table
            if let Some(symbol_table) = data["symbol_table"].as_object() {
                for (def_id, symbol) in symbol_table {
                    if let Some(symbol_str) = symbol.as_str() {
                        self.symbol_table.insert(def_id.clone(), symbol_str.to_string());
                    }
                }
            }
            
            // Load def paths
            if let Some(def_paths) = data["def_paths"].as_object() {
                for (def_id, path) in def_paths {
                    if let Some(path_str) = path.as_str() {
                        self.def_paths.insert(def_id.clone(), path_str.to_string());
                    }
                }
            }
        } else {
            // Fallback: load from original usage data
            self.load_from_original_usage_data()?;
        }
        
        println!("  Call graph: {} nodes", self.call_graph.len());
        println!("  Symbol table: {} entries", self.symbol_table.len());
        println!("  Def paths: {} entries", self.def_paths.len());
        
        Ok(())
    }
    
    fn load_from_original_usage_data(&mut self) -> Result<()> {
        let usage_dir = "../../usage_data";
        
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage in usages {
                        let caller = usage["user_def_id"].as_str().unwrap_or("unknown");
                        let callee = usage["used_def_id"].as_str().unwrap_or("unknown");
                        
                        // Only DefId -> DefId relationships
                        if caller.contains("DefId") && callee.contains("DefId") {
                            self.call_graph.entry(caller.to_string())
                                .or_insert_with(Vec::new)
                                .push(callee.to_string());
                            
                            // Extract symbols
                            self.extract_symbol_from_def_id(caller);
                            self.extract_symbol_from_def_id(callee);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_symbol_from_def_id(&mut self, def_id: &str) {
        if let Some(start) = def_id.find("~ ") {
            if let Some(end) = def_id[start+2..].find(")") {
                let full_path = &def_id[start+2..start+2+end];
                
                if let Some(last_colon) = full_path.rfind("::") {
                    let module_path = &full_path[..last_colon];
                    let symbol_name = &full_path[last_colon+2..];
                    
                    self.symbol_table.insert(def_id.to_string(), symbol_name.to_string());
                    self.def_paths.insert(def_id.to_string(), module_path.to_string());
                }
            }
        }
    }
    
    fn find_entry_point(&self) -> String {
        // Find node with most outgoing calls
        self.call_graph.iter()
            .max_by_key(|(_, callees)| callees.len())
            .map(|(node, _)| node.clone())
            .unwrap_or_else(|| self.call_graph.keys().next().unwrap_or(&"unknown".to_string()).clone())
    }
    
    fn load_source_if_needed(&mut self, def_id: &str) -> Option<String> {
        // Check cache first
        if let Some(cached) = self.source_cache.get(def_id) {
            return Some(cached.clone());
        }
        
        // Try to load source based on def_path
        if let Some(module_path) = self.def_paths.get(def_id) {
            // Convert module path to potential file path
            let potential_paths = vec![
                format!("src/{}.rs", module_path.replace("::", "/")),
                format!("compiler/{}/src/lib.rs", module_path.split("::").next().unwrap_or("")),
                format!("{}.rs", module_path.replace("::", "_")),
            ];
            
            for path in potential_paths {
                if let Ok(content) = fs::read_to_string(&path) {
                    println!("    📁 Loaded source: {}", path);
                    self.source_cache.insert(def_id.to_string(), content.clone());
                    return Some(content);
                }
            }
        }
        
        // Generate placeholder if no source found
        let default_symbol = "unknown".to_string();
        let symbol = self.symbol_table.get(def_id).unwrap_or(&default_symbol);
        let placeholder = format!("// Generated placeholder for {}\nfn {}() {{\n    // Implementation not available\n}}", symbol, symbol);
        self.source_cache.insert(def_id.to_string(), placeholder.clone());
        Some(placeholder)
    }
    
    fn execute_cycle(&mut self) -> bool {
        self.cycle_count += 1;
        
        let symbol = self.symbol_table.get(&self.instruction_pointer)
            .unwrap_or(&"unknown".to_string()).clone();
        
        println!("\n🔄 Cycle {}: Executing {}", self.cycle_count, symbol);
        println!("  DefId: {}", self.clean_def_id(&self.instruction_pointer));
        
        // Load source if this is a function we want to inspect
        if self.cycle_count <= 3 {
            let current_ip = self.instruction_pointer.clone();
            if let Some(source) = self.load_source_if_needed(&current_ip) {
                let preview = source.lines().take(3).collect::<Vec<_>>().join("\n");
                println!("  Source preview:\n{}", preview);
            }
        }
        
        // Execute: call first callee
        if let Some(callees) = self.call_graph.get(&self.instruction_pointer) {
            if !callees.is_empty() {
                self.stack.push(self.instruction_pointer.clone());
                let next = &callees[0];
                let default_symbol = "unknown".to_string();
                let next_symbol = self.symbol_table.get(next).unwrap_or(&default_symbol);
                println!("  → Calling: {}", next_symbol);
                self.instruction_pointer = next.clone();
            } else {
                // No callees, return from stack
                if let Some(return_addr) = self.stack.pop() {
                    println!("  ← Returning");
                    self.instruction_pointer = return_addr;
                } else {
                    println!("  🛑 Execution complete (no more calls)");
                    return false;
                }
            }
        } else {
            println!("  🛑 No call graph entry");
            return false;
        }
        
        self.cycle_count < 10
    }
    
    fn run(&mut self) -> Result<()> {
        println!("🚀 Starting Usage Data Interpreter\n");
        
        self.load_usage_data()?;
        
        self.instruction_pointer = self.find_entry_point();
        let default_symbol = "unknown".to_string();
        let entry_symbol = self.symbol_table.get(&self.instruction_pointer)
            .unwrap_or(&default_symbol);
        
        println!("🎯 Entry point: {}", entry_symbol);
        println!("  Calls {} functions", 
                 self.call_graph.get(&self.instruction_pointer)
                     .map(|v| v.len()).unwrap_or(0));
        
        while self.execute_cycle() {}
        
        println!("\n✅ Execution complete!");
        println!("  Cycles: {}", self.cycle_count);
        println!("  Source files loaded: {}", self.source_cache.len());
        
        Ok(())
    }
    
    fn clean_def_id(&self, def_id: &str) -> String {
        if let Some(start) = def_id.find("~ ") {
            if let Some(end) = def_id[start+2..].find(")") {
                return def_id[start+2..start+2+end].chars().take(40).collect();
            }
        }
        def_id.chars().take(30).collect()
    }
}

fn main() -> Result<()> {
    let mut interpreter = UsageDataInterpreter::new();
    interpreter.run()
}
