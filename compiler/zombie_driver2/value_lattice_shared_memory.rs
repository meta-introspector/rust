use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};

const SHARED_MEMORY_LIMIT_GB: usize = 20;
const BYTES_PER_GB: usize = 1024 * 1024 * 1024;
const MAX_MEMORY_BYTES: usize = SHARED_MEMORY_LIMIT_GB * BYTES_PER_GB;
const ESTIMATED_BYTES_PER_ENTRY: usize = 200; // Conservative estimate
const MAX_ENTRIES_IN_MEMORY: usize = MAX_MEMORY_BYTES / ESTIMATED_BYTES_PER_ENTRY;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueUsage {
    file_path: String,
    usage_type: String,
    complexity: u32,
}

struct SharedMemoryValueStore {
    base_dir: String,
    values_in_memory: HashMap<String, Vec<ValueUsage>>,
    current_memory_usage: usize,
    total_entries_processed: usize,
    flush_counter: usize,
}

impl SharedMemoryValueStore {
    fn new(base_dir: &str) -> std::io::Result<Self> {
        std::fs::create_dir_all(base_dir)?;
        
        println!("🧠 Shared Memory Configuration:");
        println!("   Allocated: {} GB ({} bytes)", SHARED_MEMORY_LIMIT_GB, MAX_MEMORY_BYTES);
        println!("   Max entries in memory: {}", MAX_ENTRIES_IN_MEMORY);
        
        Ok(Self {
            base_dir: base_dir.to_string(),
            values_in_memory: HashMap::new(),
            current_memory_usage: 0,
            total_entries_processed: 0,
            flush_counter: 0,
        })
    }
    
    fn add_value(&mut self, value: String, usage: ValueUsage) -> std::io::Result<()> {
        self.values_in_memory.entry(value).or_default().push(usage);
        self.current_memory_usage += ESTIMATED_BYTES_PER_ENTRY;
        self.total_entries_processed += 1;
        
        // Check if we need to flush to stay within memory limit
        if self.current_memory_usage >= MAX_MEMORY_BYTES {
            self.flush_to_disk()?;
        }
        
        Ok(())
    }
    
    fn flush_to_disk(&mut self) -> std::io::Result<()> {
        self.flush_counter += 1;
        let flush_dir = format!("{}/flush_{:04}", self.base_dir, self.flush_counter);
        std::fs::create_dir_all(&flush_dir)?;
        
        println!("💾 Flushing {} unique values ({} entries) to disk - Flush #{}", 
                 self.values_in_memory.len(), 
                 self.total_entries_processed,
                 self.flush_counter);
        
        for (value, usages) in self.values_in_memory.drain() {
            let safe_filename = value.chars()
                .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
                .collect::<String>();
            
            let file_path = format!("{}/{}.jsonl", flush_dir, safe_filename);
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)?;
                
            for usage in usages {
                let line = serde_json::to_string(&usage)?;
                writeln!(file, "{}", line)?;
            }
        }
        
        self.current_memory_usage = 0;
        println!("✅ Flush complete - Memory reset");
        
        Ok(())
    }
    
    fn get_stats(&self) -> (usize, usize, f64) {
        let memory_gb = self.current_memory_usage as f64 / BYTES_PER_GB as f64;
        (self.values_in_memory.len(), self.total_entries_processed, memory_gb)
    }
}

fn main() -> std::io::Result<()> {
    println!("🚀 SHARED MEMORY VALUE LATTICE INDEXER");
    println!("======================================");

    let base_dir = "/mnt/data1/meta-introspector/value-lattice-shared";
    let mut store = SharedMemoryValueStore::new(base_dir)?;
    
    // Process current directory
    let current_dir = std::env::current_dir()?;
    let mut rust_files = Vec::new();
    collect_rust_files(&current_dir, &mut rust_files);
    
    println!("🦀 Processing {} Rust files with 20GB shared memory", rust_files.len());
    
    for (i, file_path) in rust_files.iter().enumerate() {
        if i % 100 == 0 {
            let (unique_values, total_entries, memory_gb) = store.get_stats();
            println!("📊 File {}/{} | Values: {} | Entries: {} | Memory: {:.2} GB", 
                     i, rust_files.len(), unique_values, total_entries, memory_gb);
        }
        
        if let Err(e) = process_file_shared(&mut store, file_path) {
            eprintln!("❌ Error processing {}: {}", file_path, e);
        }
    }
    
    // Final flush
    store.flush_to_disk()?;
    
    let (unique_values, total_entries, memory_gb) = store.get_stats();
    println!("✅ Shared memory indexer complete!");
    println!("📊 Total unique values: {}", unique_values);
    println!("📊 Total entries: {}", total_entries);
    println!("🧠 Final memory usage: {:.2} GB", memory_gb);
    
    Ok(())
}

fn process_file_shared(store: &mut SharedMemoryValueStore, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use syn::{parse_file, visit::Visit, Expr, Lit};
    
    struct ConstantVisitor<'a> {
        store: &'a mut SharedMemoryValueStore,
        file_path: String,
    }
    
    impl<'a> ConstantVisitor<'a> {
        fn new(store: &'a mut SharedMemoryValueStore, file_path: String) -> Self {
            Self { store, file_path }
        }
    }
    
    impl<'ast, 'a> Visit<'ast> for ConstantVisitor<'a> {
        fn visit_expr(&mut self, expr: &'ast Expr) {
            match expr {
                Expr::Lit(lit_expr) => {
                    let (value, usage_type) = match &lit_expr.lit {
                        Lit::Int(int_lit) => (int_lit.base10_digits().to_string(), "integer_literal"),
                        Lit::Float(float_lit) => (float_lit.base10_digits().to_string(), "float_literal"),
                        Lit::Str(str_lit) => (str_lit.value(), "string_literal"),
                        Lit::Bool(bool_lit) => (bool_lit.value.to_string(), "boolean_literal"),
                        _ => return,
                    };
                    
                    let usage = ValueUsage {
                        file_path: self.file_path.clone(),
                        usage_type: usage_type.to_string(),
                        complexity: 1,
                    };
                    
                    if let Err(e) = self.store.add_value(value, usage) {
                        eprintln!("❌ Error adding value: {}", e);
                    }
                }
                _ => {}
            }
            syn::visit::visit_expr(self, expr);
        }
    }
    
    let source = std::fs::read_to_string(file_path)?;
    let ast = parse_file(&source)?;
    
    let mut visitor = ConstantVisitor::new(store, file_path.to_string());
    visitor.visit_file(&ast);
    
    Ok(())
}

fn collect_rust_files(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !matches!(name, "target" | ".git" | "node_modules" | "build" | "dist") {
                        collect_rust_files(&path, files);
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
}
