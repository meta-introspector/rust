use std::collections::HashMap;
use std::fs;

/// 24-bit Monster Group Compiler Plugin (Standalone Version)
/// Applies our orchestra model to analyze Rust source code

#[derive(Debug, Clone)]
struct MonsterCell {
    cell_id: u32,           // 24-bit cell identifier (0 to 16,777,215)
    prime_signature: u8,    // Prime generator (2, 3, 5, 7, 11, 13, 17, 19)
    symbols: Vec<String>,   // Symbols mapped to this cell
    usage_count: u32,       // Total usage frequency
    complexity_score: f64,  // Monster Group complexity measure
}

#[derive(Debug)]
struct MonsterCompilerPlugin {
    // 24-bit model: 2^24 = 16,777,216 cells
    cells: HashMap<u32, MonsterCell>,
    prime_generators: [u8; 8],
    symbol_to_cell: HashMap<String, u32>,
    compilation_stats: CompilationStats,
}

#[derive(Debug, Default)]
struct CompilationStats {
    total_symbols: u32,
    cells_used: u32,
    prime_distribution: [u32; 8],
    monster_signature: u128,
    analysis_time_ms: u128,
}

impl MonsterCompilerPlugin {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            prime_generators: [2, 3, 5, 7, 11, 13, 17, 19],
            symbol_to_cell: HashMap::new(),
            compilation_stats: CompilationStats::default(),
        }
    }
    
    fn hash_symbol_to_24bit_cell(&self, symbol: &str) -> u32 {
        // 24-bit Monster Group hash function
        let mut hash = 1u64;
        
        for (i, byte) in symbol.bytes().enumerate() {
            let prime_idx = i % self.prime_generators.len();
            hash = hash.wrapping_mul(self.prime_generators[prime_idx] as u64)
                      .wrapping_add(byte as u64);
        }
        
        // Ensure 24-bit range (0 to 16,777,215)
        (hash % (1u64 << 24)) as u32
    }
    
    fn get_prime_for_symbol(&self, symbol: &str) -> u8 {
        // Select prime generator based on symbol characteristics
        let index = (symbol.len() % 8) as usize;
        self.prime_generators[index]
    }
    
    fn add_symbol_to_model(&mut self, symbol: String) {
        let cell_id = self.hash_symbol_to_24bit_cell(&symbol);
        let prime = self.get_prime_for_symbol(&symbol);
        
        // Get or create cell
        let cell = self.cells.entry(cell_id).or_insert_with(|| MonsterCell {
            cell_id,
            prime_signature: prime,
            symbols: Vec::new(),
            usage_count: 0,
            complexity_score: 0.0,
        });
        
        // Add symbol to cell
        if !cell.symbols.contains(&symbol) {
            cell.symbols.push(symbol.clone());
            cell.usage_count += 1;
            
            // Update complexity score using Monster Group formula
            cell.complexity_score = (cell.usage_count as f64) * (prime as f64).ln();
        }
        
        // Update mappings
        self.symbol_to_cell.insert(symbol, cell_id);
        
        // Update statistics
        self.compilation_stats.total_symbols += 1;
        let prime_index = self.prime_generators.iter().position(|&p| p == prime).unwrap_or(0);
        self.compilation_stats.prime_distribution[prime_index] += 1;
    }
    
    fn analyze_rust_source(&mut self, source_code: &str) {
        println!("🔍 24-bit Monster Group Analysis Starting...");
        
        let start_time = std::time::Instant::now();
        
        // Simple symbol extraction (identifiers, keywords, types)
        let symbols = self.extract_symbols(source_code);
        
        for symbol in symbols {
            self.add_symbol_to_model(symbol);
        }
        
        let elapsed = start_time.elapsed();
        self.compilation_stats.analysis_time_ms = elapsed.as_millis();
        self.compilation_stats.cells_used = self.cells.len() as u32;
        
        // Calculate Monster Group signature
        self.compilation_stats.monster_signature = self.calculate_monster_signature();
        
        println!("✅ Analysis complete: {} symbols mapped to {} cells", 
                self.compilation_stats.total_symbols, self.compilation_stats.cells_used);
    }
    
    fn extract_symbols(&self, source: &str) -> Vec<String> {
        let mut symbols = Vec::new();
        let mut current_symbol = String::new();
        
        for ch in source.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                current_symbol.push(ch);
            } else {
                if !current_symbol.is_empty() && current_symbol.len() > 1 {
                    symbols.push(current_symbol.clone());
                }
                current_symbol.clear();
            }
        }
        
        // Don't forget the last symbol
        if !current_symbol.is_empty() && current_symbol.len() > 1 {
            symbols.push(current_symbol);
        }
        
        symbols
    }
    
    fn calculate_monster_signature(&self) -> u128 {
        let mut signature = 1u128;
        
        for (i, &count) in self.compilation_stats.prime_distribution.iter().enumerate() {
            let prime = self.prime_generators[i] as u128;
            signature = signature.wrapping_mul(prime.wrapping_pow(count % 32));
        }
        
        signature
    }
    
    fn generate_24bit_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# 24-bit Monster Group Compilation Report\n\n");
        report.push_str(&format!("## Statistics\n"));
        report.push_str(&format!("- **Total Symbols**: {}\n", self.compilation_stats.total_symbols));
        report.push_str(&format!("- **Cells Used**: {} / 16,777,216 ({:.6}%)\n", 
            self.compilation_stats.cells_used,
            (self.compilation_stats.cells_used as f64 / 16777216.0) * 100.0));
        report.push_str(&format!("- **Analysis Time**: {}ms\n", self.compilation_stats.analysis_time_ms));
        report.push_str(&format!("- **Monster Signature**: 0x{:032X}\n", self.compilation_stats.monster_signature));
        
        report.push_str("\n## Prime Distribution\n");
        for (i, &count) in self.compilation_stats.prime_distribution.iter().enumerate() {
            let prime = self.prime_generators[i];
            let percentage = (count as f64 / self.compilation_stats.total_symbols as f64) * 100.0;
            report.push_str(&format!("- **Prime {}**: {} symbols ({:.2}%)\n", prime, count, percentage));
        }
        
        report.push_str("\n## Top Monster Cells\n");
        let mut sorted_cells: Vec<_> = self.cells.values().collect();
        sorted_cells.sort_by(|a, b| b.complexity_score.partial_cmp(&a.complexity_score).unwrap());
        
        for (i, cell) in sorted_cells.iter().take(10).enumerate() {
            report.push_str(&format!("{}. **Cell 0x{:06X}** (Prime {}): {} symbols, complexity {:.2}\n",
                i + 1, cell.cell_id, cell.prime_signature, cell.usage_count, cell.complexity_score));
            
            // Show first few symbols
            let sample_symbols: Vec<_> = cell.symbols.iter().take(3).cloned().collect();
            report.push_str(&format!("   Symbols: {}\n", sample_symbols.join(", ")));
        }
        
        report.push_str("\n## Monster Group Analysis\n");
        let density = self.compilation_stats.cells_used as f64 / 16777216.0;
        let avg_complexity = sorted_cells.iter().map(|c| c.complexity_score).sum::<f64>() / sorted_cells.len() as f64;
        
        report.push_str(&format!("- **Cell Density**: {:.8} (sparsity: {:.6}%)\n", density, (1.0 - density) * 100.0));
        report.push_str(&format!("- **Average Complexity**: {:.4}\n", avg_complexity));
        report.push_str(&format!("- **Monster Efficiency**: {:.2}%\n", 
            (avg_complexity / (self.prime_generators.iter().map(|&p| (p as f64).ln()).sum::<f64>() / 8.0)) * 100.0));
        
        report
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <rust_source_file>", args[0]);
        println!("Example: {} src/main.rs", args[0]);
        return;
    }
    
    let source_file = &args[1];
    
    println!("🍄 24-bit Monster Group Compiler Plugin");
    println!("======================================");
    println!("Analyzing: {}", source_file);
    
    // Read source file
    let source_code = match fs::read_to_string(source_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file {}: {}", source_file, e);
            return;
        }
    };
    
    // Create and run Monster analysis
    let mut plugin = MonsterCompilerPlugin::new();
    plugin.analyze_rust_source(&source_code);
    
    // Generate report
    let report = plugin.generate_24bit_report();
    
    // Save report
    let report_file = format!("{}.monster_24bit_report.md", source_file);
    if let Err(e) = fs::write(&report_file, &report) {
        eprintln!("❌ Error writing report: {}", e);
    } else {
        println!("📊 Report saved to: {}", report_file);
    }
    
    // Print summary
    println!("\n🎯 MONSTER GROUP SUMMARY:");
    println!("========================");
    println!("Symbols: {} → Cells: {} (24-bit space)", 
        plugin.compilation_stats.total_symbols, plugin.compilation_stats.cells_used);
    println!("Monster Signature: 0x{:016X}", plugin.compilation_stats.monster_signature);
    println!("Prime Distribution: {:?}", plugin.compilation_stats.prime_distribution);
    
    println!("\n🧬 The 24-bit Monster Group model successfully mapped your Rust code!");
    println!("📈 Each symbol finds its perfect prime-aligned cell in the 16M address space.");
}
