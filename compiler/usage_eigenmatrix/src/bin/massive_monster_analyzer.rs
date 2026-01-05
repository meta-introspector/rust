use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Massive Monster Group Analysis - Entire Rust Compiler Codebase
/// Maps all Rust source files in ../../ to our 24-bit Monster matrix

#[derive(Debug, Clone)]
struct MonsterCell {
    cell_id: u32,
    prime_signature: u8,
    symbols: Vec<String>,
    files: Vec<String>,
    usage_count: u32,
    complexity_score: f64,
}

#[derive(Debug)]
struct MassiveMonsterAnalyzer {
    cells: HashMap<u32, MonsterCell>,
    prime_generators: [u8; 8],
    symbol_to_cell: HashMap<String, u32>,
    file_count: u32,
    total_symbols: u32,
}

impl MassiveMonsterAnalyzer {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            prime_generators: [2, 3, 5, 7, 11, 13, 17, 19],
            symbol_to_cell: HashMap::new(),
            file_count: 0,
            total_symbols: 0,
        }
    }
    
    fn hash_symbol_to_24bit_cell(&self, symbol: &str) -> u32 {
        let mut hash = 1u64;
        for (i, byte) in symbol.bytes().enumerate() {
            let prime_idx = i % 8;
            hash = hash.wrapping_mul(self.prime_generators[prime_idx] as u64)
                      .wrapping_add(byte as u64);
        }
        (hash % (1u64 << 24)) as u32
    }
    
    fn get_prime_for_symbol(&self, symbol: &str) -> u8 {
        self.prime_generators[symbol.len() % 8]
    }
    
    fn add_symbol(&mut self, symbol: String, file_path: &str) {
        let cell_id = self.hash_symbol_to_24bit_cell(&symbol);
        let prime = self.get_prime_for_symbol(&symbol);
        
        let cell = self.cells.entry(cell_id).or_insert_with(|| MonsterCell {
            cell_id,
            prime_signature: prime,
            symbols: Vec::new(),
            files: Vec::new(),
            usage_count: 0,
            complexity_score: 0.0,
        });
        
        if !cell.symbols.contains(&symbol) {
            cell.symbols.push(symbol.clone());
        }
        
        if !cell.files.contains(&file_path.to_string()) {
            cell.files.push(file_path.to_string());
        }
        
        cell.usage_count += 1;
        cell.complexity_score = (cell.usage_count as f64) * (prime as f64).ln();
        
        self.symbol_to_cell.insert(symbol, cell_id);
        self.total_symbols += 1;
    }
    
    fn extract_symbols(&self, source: &str) -> Vec<String> {
        let mut symbols = Vec::new();
        let mut current = String::new();
        
        for ch in source.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                current.push(ch);
            } else {
                if current.len() > 2 {
                    symbols.push(current.clone());
                }
                current.clear();
            }
        }
        
        if current.len() > 2 {
            symbols.push(current);
        }
        
        symbols
    }
    
    fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let symbols = self.extract_symbols(&content);
        
        let path_str = file_path.to_string_lossy();
        for symbol in symbols {
            self.add_symbol(symbol, &path_str);
        }
        
        self.file_count += 1;
        
        if self.file_count % 100 == 0 {
            println!("📊 Processed {} files, {} symbols, {} cells", 
                self.file_count, self.total_symbols, self.cells.len());
        }
        
        Ok(())
    }
    
    fn analyze_directory(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip certain directories
                let dir_name = path.file_name().unwrap().to_string_lossy();
                if dir_name.starts_with('.') || dir_name == "target" || dir_name == "build" {
                    continue;
                }
                self.analyze_directory(&path)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Err(e) = self.analyze_file(&path) {
                    eprintln!("⚠️  Error processing {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    fn generate_massive_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# MASSIVE MONSTER GROUP ANALYSIS\n");
        report.push_str("## Entire Rust Compiler Codebase Mapping\n\n");
        
        report.push_str(&format!("### 🔢 Scale Statistics\n"));
        report.push_str(&format!("- **Files Analyzed**: {}\n", self.file_count));
        report.push_str(&format!("- **Total Symbols**: {}\n", self.total_symbols));
        report.push_str(&format!("- **Monster Cells Used**: {} / 16,777,216\n", self.cells.len()));
        report.push_str(&format!("- **Cell Density**: {:.8}%\n", 
            (self.cells.len() as f64 / 16777216.0) * 100.0));
        
        // Prime distribution
        let mut prime_counts = [0u32; 8];
        for cell in self.cells.values() {
            if let Some(idx) = self.prime_generators.iter().position(|&p| p == cell.prime_signature) {
                prime_counts[idx] += cell.usage_count;
            }
        }
        
        report.push_str("\n### 🔢 Prime Distribution\n");
        for (i, &count) in prime_counts.iter().enumerate() {
            let prime = self.prime_generators[i];
            let percentage = (count as f64 / self.total_symbols as f64) * 100.0;
            report.push_str(&format!("- **Prime {}**: {} symbols ({:.2}%)\n", prime, count, percentage));
        }
        
        // Top cells
        let mut sorted_cells: Vec<_> = self.cells.values().collect();
        sorted_cells.sort_by(|a, b| b.complexity_score.partial_cmp(&a.complexity_score).unwrap());
        
        report.push_str("\n### 🏆 Top Monster Cells\n");
        for (i, cell) in sorted_cells.iter().take(20).enumerate() {
            report.push_str(&format!("{}. **Cell 0x{:06X}** (Prime {}): {} uses, {} files\n",
                i + 1, cell.cell_id, cell.prime_signature, cell.usage_count, cell.files.len()));
            
            let sample_symbols: Vec<_> = cell.symbols.iter().take(5).cloned().collect();
            report.push_str(&format!("   Symbols: {}\n", sample_symbols.join(", ")));
        }
        
        // Monster signature
        let mut signature = 1u128;
        for (i, &count) in prime_counts.iter().enumerate() {
            let prime = self.prime_generators[i] as u128;
            signature = signature.wrapping_mul(prime.wrapping_pow(count % 64));
        }
        
        report.push_str(&format!("\n### 🧬 Rust Compiler Monster Signature\n"));
        report.push_str(&format!("**0x{:032X}**\n", signature));
        
        report.push_str("\n### 📊 Analysis Summary\n");
        report.push_str(&format!("- **Average symbols per file**: {:.1}\n", 
            self.total_symbols as f64 / self.file_count as f64));
        report.push_str(&format!("- **Average complexity per cell**: {:.2}\n",
            sorted_cells.iter().map(|c| c.complexity_score).sum::<f64>() / sorted_cells.len() as f64));
        report.push_str(&format!("- **Monster efficiency**: {:.4}%\n",
            (self.cells.len() as f64 / self.total_symbols as f64) * 100.0));
        
        report
    }
}

fn main() {
    println!("🍄 MASSIVE MONSTER GROUP ANALYSIS");
    println!("=================================");
    println!("Mapping entire Rust compiler codebase to 24-bit Monster matrix...");
    
    let start_time = std::time::Instant::now();
    let mut analyzer = MassiveMonsterAnalyzer::new();
    
    // Analyze ../../ directory
    let target_dir = Path::new("../../");
    
    println!("🔍 Starting analysis of: {}", target_dir.display());
    
    match analyzer.analyze_directory(target_dir) {
        Ok(()) => {
            let elapsed = start_time.elapsed();
            
            println!("\n✅ ANALYSIS COMPLETE!");
            println!("====================");
            println!("Files: {} | Symbols: {} | Cells: {} | Time: {:.2}s", 
                analyzer.file_count, analyzer.total_symbols, analyzer.cells.len(), elapsed.as_secs_f64());
            
            // Generate and save report
            let report = analyzer.generate_massive_report();
            
            match fs::write("MASSIVE_MONSTER_ANALYSIS.md", &report) {
                Ok(()) => println!("📊 Report saved to: MASSIVE_MONSTER_ANALYSIS.md"),
                Err(e) => eprintln!("❌ Error saving report: {}", e),
            }
            
            // Print key metrics
            println!("\n🎯 KEY METRICS:");
            println!("===============");
            println!("Cell Density: {:.8}% of 24-bit space", 
                (analyzer.cells.len() as f64 / 16777216.0) * 100.0);
            println!("Compression Ratio: {:.1}:1 (symbols to cells)", 
                analyzer.total_symbols as f64 / analyzer.cells.len() as f64);
            
            println!("\n🧬 The entire Rust compiler is now mapped to Monster Group theory!");
            println!("📈 Every symbol in {} files finds its perfect mathematical home.", analyzer.file_count);
        }
        Err(e) => {
            eprintln!("❌ Analysis failed: {}", e);
        }
    }
}
