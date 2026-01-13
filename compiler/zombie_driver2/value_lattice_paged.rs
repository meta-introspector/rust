use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, Seek, SeekFrom};
use std::path::Path;
use serde::{Deserialize, Serialize};

const PAGE_SIZE: usize = 10000; // Values per page
const MAX_PAGES_IN_MEMORY: usize = 10; // Keep max 10 pages in RAM

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueUsage {
    file_path: String,
    usage_type: String,
    complexity: u32,
}

#[derive(Debug)]
struct PagedValueStore {
    base_dir: String,
    pages_in_memory: HashMap<String, Vec<ValueUsage>>, // page_id -> values
    page_access_order: Vec<String>, // LRU tracking
    page_offsets: HashMap<String, (u64, u64)>, // page_id -> (start_offset, end_offset)
    index_file: File,
}

impl PagedValueStore {
    fn new(base_dir: &str) -> std::io::Result<Self> {
        std::fs::create_dir_all(base_dir)?;
        
        let index_path = format!("{}/page_index.txt", base_dir);
        let index_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(index_path)?;
            
        Ok(Self {
            base_dir: base_dir.to_string(),
            pages_in_memory: HashMap::new(),
            page_access_order: Vec::new(),
            page_offsets: HashMap::new(),
            index_file,
        })
    }
    
    fn get_page_id(value: &str) -> String {
        // Hash-based partitioning
        let hash = value.chars().map(|c| c as u32).sum::<u32>();
        let page_num = hash % 1000; // 1000 pages max
        format!("page_{:04}", page_num)
    }
    
    fn add_value(&mut self, value: String, usage: ValueUsage) -> std::io::Result<()> {
        let page_id = Self::get_page_id(&value);
        
        // Load page if not in memory
        if !self.pages_in_memory.contains_key(&page_id) {
            self.load_page(&page_id)?;
        }
        
        // Add to page
        let page = self.pages_in_memory.entry(page_id.clone()).or_default();
        page.push(usage);
        
        // Update LRU
        self.page_access_order.retain(|id| id != &page_id);
        self.page_access_order.push(page_id.clone());
        
        // Evict oldest pages if memory limit exceeded
        while self.pages_in_memory.len() > MAX_PAGES_IN_MEMORY {
            let oldest_page = self.page_access_order.remove(0);
            self.flush_page(&oldest_page)?;
            self.pages_in_memory.remove(&oldest_page);
        }
        
        Ok(())
    }
    
    fn load_page(&mut self, page_id: &str) -> std::io::Result<()> {
        let page_file = format!("{}/{}.jsonl", self.base_dir, page_id);
        
        if !Path::new(&page_file).exists() {
            // New page
            self.pages_in_memory.insert(page_id.to_string(), Vec::new());
            return Ok(());
        }
        
        let file = File::open(&page_file)?;
        let reader = BufReader::new(file);
        let mut usages = Vec::new();
        
        for line in reader.lines() {
            if let Ok(usage) = serde_json::from_str::<ValueUsage>(&line?) {
                usages.push(usage);
            }
        }
        
        self.pages_in_memory.insert(page_id.to_string(), usages);
        println!("📄 Loaded page {} with {} entries", page_id, self.pages_in_memory[page_id].len());
        
        Ok(())
    }
    
    fn flush_page(&mut self, page_id: &str) -> std::io::Result<()> {
        if let Some(usages) = self.pages_in_memory.get(page_id) {
            let page_file = format!("{}/{}.jsonl", self.base_dir, page_id);
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&page_file)?;
                
            for usage in usages {
                let line = serde_json::to_string(usage)?;
                writeln!(file, "{}", line)?;
            }
            
            println!("💾 Flushed page {} with {} entries", page_id, usages.len());
        }
        
        Ok(())
    }
    
    fn flush_all(&mut self) -> std::io::Result<()> {
        let page_ids: Vec<String> = self.pages_in_memory.keys().cloned().collect();
        for page_id in page_ids {
            self.flush_page(&page_id)?;
        }
        Ok(())
    }
    
    fn get_stats(&self) -> (usize, usize) {
        let pages_in_memory = self.pages_in_memory.len();
        let total_entries: usize = self.pages_in_memory.values().map(|v| v.len()).sum();
        (pages_in_memory, total_entries)
    }
}

fn main() -> std::io::Result<()> {
    println!("🗂️  PAGED VALUE LATTICE INDEXER");
    println!("===============================");
    println!("📄 Page size: {} values", PAGE_SIZE);
    println!("🧠 Max pages in memory: {}", MAX_PAGES_IN_MEMORY);

    let base_dir = "/mnt/data1/meta-introspector/value-lattice-paged";
    let mut store = PagedValueStore::new(base_dir)?;
    
    // Process current directory only
    let current_dir = std::env::current_dir()?;
    let mut rust_files = Vec::new();
    collect_rust_files(&current_dir, &mut rust_files);
    
    println!("🦀 Processing {} Rust files", rust_files.len());
    
    for (i, file_path) in rust_files.iter().enumerate() {
        if i % 50 == 0 {
            let (pages, entries) = store.get_stats();
            println!("📊 File {}/{} | Pages in memory: {} | Total entries: {}", 
                     i, rust_files.len(), pages, entries);
        }
        
        if let Err(e) = process_file_paged(&mut store, file_path) {
            eprintln!("❌ Error processing {}: {}", file_path, e);
        }
    }
    
    // Final flush
    store.flush_all()?;
    
    let (pages, entries) = store.get_stats();
    println!("✅ Paged indexer complete!");
    println!("📄 Final pages in memory: {}", pages);
    println!("📊 Total entries processed: {}", entries);
    
    Ok(())
}

fn process_file_paged(store: &mut PagedValueStore, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use syn::{parse_file, visit::Visit, Expr, Lit};
    
    struct ConstantVisitor<'a> {
        store: &'a mut PagedValueStore,
        file_path: String,
    }
    
    impl<'a> ConstantVisitor<'a> {
        fn new(store: &'a mut PagedValueStore, file_path: String) -> Self {
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
