use std::collections::HashMap;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Expr, Lit};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueUsage {
    file_path: String,
    context: String,
    usage_type: String,
    complexity: u32,
}

struct StreamingIndexer {
    base_dir: String,
    batch_size: usize,
    current_batch: HashMap<String, Vec<ValueUsage>>,
}

impl StreamingIndexer {
    fn new(base_dir: String) -> Self {
        fs::create_dir_all(&base_dir).unwrap();
        Self {
            base_dir,
            batch_size: 1000, // Process 1000 files then flush
            current_batch: HashMap::new(),
        }
    }
    
    fn process_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let source = fs::read_to_string(file_path)?;
        let ast = parse_file(&source)?;
        
        let mut visitor = ConstantVisitor::new(file_path.to_string());
        visitor.visit_file(&ast);
        
        for (value, context, usage_type) in visitor.constants {
            let usage = ValueUsage {
                file_path: file_path.to_string(),
                context,
                usage_type,
                complexity: 1,
            };
            
            self.current_batch.entry(value).or_default().push(usage);
        }
        
        // Flush if batch is getting large
        if self.current_batch.len() > self.batch_size {
            self.flush_batch()?;
        }
        
        Ok(())
    }
    
    fn flush_batch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for (value, usages) in self.current_batch.drain() {
            self.append_to_value_file(&value, &usages)?;
        }
        Ok(())
    }
    
    fn append_to_value_file(&self, value: &str, usages: &[ValueUsage]) -> Result<(), Box<dyn std::error::Error>> {
        let length = value.len();
        let length_dir = format!("{}/length-{}", self.base_dir, length);
        fs::create_dir_all(&length_dir)?;
        
        let safe_filename = value.chars()
            .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
            .collect::<String>();
        
        let file_path = format!("{}/{}.jsonl", length_dir, safe_filename);
        
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
            
        for usage in usages {
            let line = serde_json::to_string(usage)?;
            writeln!(file, "{}", line)?;
        }
        
        Ok(())
    }
}

struct ConstantVisitor {
    file_path: String,
    constants: Vec<(String, String, String)>,
}

impl ConstantVisitor {
    fn new(file_path: String) -> Self {
        Self { file_path, constants: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ConstantVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Lit(lit_expr) => {
                let (value, usage_type) = match &lit_expr.lit {
                    Lit::Int(int_lit) => (int_lit.base10_digits().to_string(), "integer_literal".to_string()),
                    Lit::Float(float_lit) => (float_lit.base10_digits().to_string(), "float_literal".to_string()),
                    Lit::Str(str_lit) => (str_lit.value(), "string_literal".to_string()),
                    Lit::Bool(bool_lit) => (bool_lit.value.to_string(), "boolean_literal".to_string()),
                    _ => return,
                };
                
                self.constants.push((value, "literal_usage".to_string(), usage_type));
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }
}

fn main() {
    println!("🔢 STREAMING VALUE LATTICE INDEXER");
    println!("==================================");

    let base_dir = "/mnt/data1/meta-introspector/value-lattice-streaming";
    let mut indexer = StreamingIndexer::new(base_dir.to_string());
    
    // Process only current directory to start
    let current_dir = std::env::current_dir().unwrap();
    let mut rust_files = Vec::new();
    collect_rust_files(&current_dir, &mut rust_files);
    
    println!("🦀 Found {} Rust files in current directory", rust_files.len());
    
    for (i, file_path) in rust_files.iter().enumerate() {
        if i % 100 == 0 {
            println!("📊 Processed {}/{} files", i, rust_files.len());
        }
        
        if let Err(e) = indexer.process_file(file_path) {
            eprintln!("❌ Error processing {}: {}", file_path, e);
        }
    }
    
    // Final flush
    if let Err(e) = indexer.flush_batch() {
        eprintln!("❌ Error flushing final batch: {}", e);
    }
    
    println!("✅ Streaming indexer complete!");
}

fn collect_rust_files(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
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
