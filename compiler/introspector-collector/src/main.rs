use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CompilationBlock {
    block_number: u64,
    timestamp: u64,
    previous_hash: String,
    fibonacci_level: usize,
    complexity: usize,
    successful_crates: Vec<String>,
    failed_crates: Vec<String>,
    performance_hash: String,
    block_hash: String,
}

impl CompilationBlock {
    fn new(block_number: u64, previous_hash: String, fib_level: usize, complexity: usize) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            block_number,
            timestamp,
            previous_hash,
            fibonacci_level: fib_level,
            complexity,
            successful_crates: Vec::new(),
            failed_crates: Vec::new(),
            performance_hash: String::new(),
            block_hash: String::new(),
        }
    }
    
    fn calculate_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        self.block_number.hash(&mut hasher);
        self.timestamp.hash(&mut hasher);
        self.previous_hash.hash(&mut hasher);
        self.fibonacci_level.hash(&mut hasher);
        self.complexity.hash(&mut hasher);
        self.successful_crates.hash(&mut hasher);
        self.failed_crates.hash(&mut hasher);
        self.performance_hash.hash(&mut hasher);
        self.block_hash = format!("{:x}", hasher.finish());
    }
}

fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⛓️ FIBONACCI BLOCKCHAIN RUSTC DECOMPOSITION");
    println!("♾️ ETERNAL COMPILATION BLOCKCHAIN...");
    
    fs::create_dir_all("blockchain")?;
    fs::create_dir_all("fibonacci_traces")?;
    
    let test_crates = vec![".", "../usage_eigenmatrix", "../../../syn", "../../../burn"];
    let mut blockchain: Vec<CompilationBlock> = Vec::new();
    let mut block_number = 0u64;
    
    loop {
        for fib_level in 0..=10 {
            let complexity = fibonacci(fib_level);
            let previous_hash = blockchain.last()
                .map(|b| b.block_hash.clone())
                .unwrap_or_else(|| "genesis".to_string());
                
            let mut block = CompilationBlock::new(block_number, previous_hash, fib_level, complexity);
            
            println!("\n⛓️ BLOCK {} - Fib:{} C:{}", block_number, fib_level, complexity);
            
            env::set_var("MAX_COMPLEXITY", complexity.to_string());
            
            // Mine the block by compiling crates
            for crate_path in &test_crates {
                if !Path::new(crate_path).exists() { continue; }
                
                let output = Command::new("cargo")
                    .args(&["build", "--release"])
                    .current_dir(crate_path)
                    .env("MAX_COMPLEXITY", complexity.to_string())
                    .output()?;
                    
                let crate_name = Path::new(crate_path).file_name()
                    .unwrap_or_default().to_string_lossy().to_string();
                    
                if output.status.success() {
                    block.successful_crates.push(crate_name);
                    print!("✅");
                } else {
                    block.failed_crates.push(crate_name);
                    print!("❌");
                }
            }
            
            // Calculate performance hash from traces
            let mut perf_hasher = DefaultHasher::new();
            if let Ok(entries) = fs::read_dir("fibonacci_traces") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(metadata) = entry.metadata() {
                            metadata.len().hash(&mut perf_hasher);
                        }
                    }
                }
            }
            block.performance_hash = format!("{:x}", perf_hasher.finish());
            
            // Finalize block
            block.calculate_hash();
            
            // Write block to blockchain
            let block_json = serde_json::to_string_pretty(&block)?;
            fs::write(format!("blockchain/block_{:06}.json", block_number), &block_json)?;
            
            println!("\n⛓️ Block {} mined: {}", block_number, &block.block_hash[..8]);
            println!("   ✅ Success: {}, ❌ Failed: {}", 
                block.successful_crates.len(), block.failed_crates.len());
            
            blockchain.push(block);
            block_number += 1;
            
            // Keep only last 100 blocks in memory
            if blockchain.len() > 100 {
                blockchain.remove(0);
            }
        }
        
        println!("\n⛓️ Blockchain height: {} blocks", block_number);
        thread::sleep(Duration::from_secs(30));
    }
}
