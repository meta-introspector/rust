use std::fs;
use std::collections::{HashMap, HashSet};
use anyhow::Result;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;

pub struct SymbolMapDriver {
    symbol_map: Value,
    resolved_deps: HashSet<String>,
    base_lib: String,
}

impl SymbolMapDriver {
    pub fn new() -> Result<Self> {
        let base_lib = r#"
#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

include!("wrap_types.rs");
"#.to_string();

        // Load symbol map
        let symbol_map = Self::load_symbol_map()?;
        
        Ok(Self {
            symbol_map,
            resolved_deps: HashSet::new(),
            base_lib,
        })
    }

    fn load_symbol_map() -> Result<Value> {
        println!("📖 Loading symbol map...");
        let file = fs::File::open("symbol_map_original.json.gz")?;
        let mut decoder = GzDecoder::new(file);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents)?;
        
        let json: Value = serde_json::from_str(&contents)?;
        println!("✅ Symbol map loaded with {} entries", 
                 json.as_object().map(|o| o.len()).unwrap_or(0));
        Ok(json)
    }

    pub fn resolve_and_compile(&mut self, target: &str) -> Result<()> {
        println!("🎯 Resolving dependencies for: {}", target);
        
        // Resolve all dependencies recursively
        let all_deps = self.resolve_all_dependencies(target)?;
        println!("📦 Found {} total dependencies", all_deps.len());
        
        // Generate code for all dependencies
        let mut complete_code = self.base_lib.clone();
        
        for dep in &all_deps {
            if let Some(code) = self.generate_code_for_symbol(dep)? {
                complete_code.push_str(&format!("\n// === {} ===\n", dep));
                complete_code.push_str(&code);
            }
        }
        
        // Write and test compilation
        fs::write("src/current.rs", &complete_code)?;
        
        println!("🔨 Testing compilation...");
        let output = std::process::Command::new("cargo")
            .args(&["check", "--lib"])
            .output()?;
            
        if output.status.success() {
            println!("✅ SUCCESS: {} compiles with all dependencies!", target);
        } else {
            println!("❌ FAILED: Compilation errors:");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
        
        Ok(())
    }

    fn resolve_all_dependencies(&mut self, target: &str) -> Result<Vec<String>> {
        let mut resolved = HashSet::new();
        let mut to_process = vec![target.to_string()];
        
        while let Some(current) = to_process.pop() {
            if resolved.contains(&current) {
                continue;
            }
            
            resolved.insert(current.clone());
            
            if let Some(entry) = self.symbol_map.get(&current) {
                if let Some(deps) = entry.get("dependencies").and_then(|d| d.as_array()) {
                    for dep in deps {
                        if let Some(dep_str) = dep.as_str() {
                            to_process.push(dep_str.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(resolved.into_iter().collect())
    }

    fn generate_code_for_symbol(&self, symbol: &str) -> Result<Option<String>> {
        if let Some(entry) = self.symbol_map.get(symbol) {
            if let Some(source_file) = entry.get("source_file").and_then(|s| s.as_str()) {
                // Map to our processed file
                let processed_path = format!("submodules/rust/{}", source_file);
                
                if std::path::Path::new(&processed_path).exists() {
                    let content = fs::read_to_string(&processed_path)?;
                    return Ok(Some(content));
                }
            }
        }
        Ok(None)
    }
}

fn main() -> Result<()> {
    let mut driver = SymbolMapDriver::new()?;
    
    // Try to resolve and compile rustc_driver::main
    driver.resolve_and_compile("rustc_driver_impl::lib::main")?;
    
    Ok(())
}
