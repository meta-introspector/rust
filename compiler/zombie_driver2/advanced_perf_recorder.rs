use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use goblin::elf::Elf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub demangled: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionStats {
    pub count: u64,
    pub samples: u64,
    pub total_time: f64,
}

impl Default for FunctionStats {
    fn default() -> Self {
        Self {
            count: 0,
            samples: 0,
            total_time: 0.0,
        }
    }
}

pub struct PerfRecorder {
    rustc_path: PathBuf,
    output_dir: PathBuf,
}

impl PerfRecorder {
    pub fn new() -> Self {
        Self {
            rustc_path: PathBuf::from("rustc"),
            output_dir: PathBuf::from("./perf_data"),
        }
    }

    pub fn record_compilation(&self, source_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;
        
        let perf_data = self.output_dir.join("perf.data");
        
        println!("Recording perf data for: {}", source_file);
        
        let status = Command::new("perf")
            .args(&[
                "record",
                "-e", "cycles:u,instructions:u,cache-misses:u",
                "-g", "--call-graph", "dwarf",
                "-F", "999",
                "-o", perf_data.to_str().unwrap(),
                "--",
                "rustc",
                source_file,
                "--emit=llvm-ir,obj",
                "-O",
            ])
            .status()?;
        
        if !status.success() {
            return Err("perf record failed".into());
        }
        
        println!("Recorded to {:?}", perf_data);
        Ok(())
    }

    pub fn extract_script(&self) -> Result<String, Box<dyn std::error::Error>> {
        let perf_data = self.output_dir.join("perf.data");
        
        let output = Command::new("perf")
            .args(&[
                "script",
                "-i", perf_data.to_str().unwrap(),
                "--fields", "comm,pid,tid,time,event,ip,sym,dso",
            ])
            .output()?;
        
        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn parse_symbols(&self) -> Result<Vec<Symbol>, Box<dyn std::error::Error>> {
        let sysroot_output = Command::new("rustc")
            .args(&["--print", "sysroot"])
            .output()?;
        
        let sysroot = String::from_utf8(sysroot_output.stdout)?
            .trim()
            .to_string();
        
        let lib_dir = PathBuf::from(sysroot).join("lib");
        
        let entries = fs::read_dir(&lib_dir)?;
        let rustc_driver = entries
            .filter_map(|e| e.ok())
            .find(|e| {
                e.file_name()
                    .to_str()
                    .map(|s| s.starts_with("librustc_driver") && s.ends_with(".so"))
                    .unwrap_or(false)
            })
            .ok_or("librustc_driver.so not found")?;
        
        println!("Found: {:?}", rustc_driver.path());
        
        self.parse_elf(&rustc_driver.path())
    }

    fn parse_elf(&self, path: &PathBuf) -> Result<Vec<Symbol>, Box<dyn std::error::Error>> {
        let buffer = fs::read(path)?;
        let elf = Elf::parse(&buffer)?;
        
        let mut symbols = Vec::new();
        
        for sym in &elf.dynsyms {
            if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                if !name.is_empty() && sym.st_value > 0 {
                    symbols.push(Symbol {
                        name: name.to_string(),
                        address: sym.st_value,
                        size: sym.st_size,
                        demangled: rustc_demangle::demangle(name).to_string(),
                    });
                }
            }
        }
        
        // Also parse regular symbol table
        for sym in &elf.syms {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if !name.is_empty() && sym.st_value > 0 {
                    symbols.push(Symbol {
                        name: name.to_string(),
                        address: sym.st_value,
                        size: sym.st_size,
                        demangled: rustc_demangle::demangle(name).to_string(),
                    });
                }
            }
        }
        
        symbols.sort_by(|a, b| a.address.cmp(&b.address));
        symbols.dedup_by(|a, b| a.address == b.address);
        
        Ok(symbols)
    }

    pub fn save_symbols(&self, symbols: &[Symbol]) -> Result<(), Box<dyn std::error::Error>> {
        let symbols_file = self.output_dir.join("symbols.json");
        let json = serde_json::to_string_pretty(symbols)?;
        fs::write(symbols_file, json)?;
        Ok(())
    }
}

pub struct PerfAnalyzer {
    functions: HashMap<String, FunctionStats>,
}

impl PerfAnalyzer {
    pub fn parse_script(script: &str) -> Self {
        let mut functions = HashMap::new();
        
        for line in script.lines() {
            if let Some((symbol, time)) = Self::extract_symbol_and_time(line) {
                let stats = functions.entry(symbol).or_insert_with(FunctionStats::default);
                stats.samples += 1;
                stats.total_time += time;
            }
        }
        
        Self { functions }
    }

    fn extract_symbol_and_time(line: &str) -> Option<(String, f64)> {
        // Parse perf script format: rustc 12345 [000] 123.456: cycles:u: 7f1234abcd symbol+0x123
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let time_str = parts[3].trim_end_matches(':');
            let time = time_str.parse::<f64>().unwrap_or(0.0);
            
            if let Some(symbol_part) = parts.last() {
                let symbol = if symbol_part.contains('+') {
                    symbol_part.split('+').next().unwrap_or(symbol_part)
                } else {
                    symbol_part
                };
                return Some((symbol.to_string(), time));
            }
        }
        None
    }

    pub fn top_functions(&self, n: usize) -> Vec<(String, u64, f64)> {
        let mut funcs: Vec<_> = self.functions
            .iter()
            .map(|(name, stats)| (name.clone(), stats.samples, stats.total_time))
            .collect();
        
        funcs.sort_by(|a, b| b.1.cmp(&a.1));
        funcs.into_iter().take(n).collect()
    }

    pub fn parser_functions(&self) -> Vec<(String, u64, f64)> {
        self.functions
            .iter()
            .filter(|(name, _)| {
                let lower = name.to_lowercase();
                lower.contains("parse") || 
                lower.contains("token") || 
                lower.contains("lexer") ||
                lower.contains("syntax")
            })
            .map(|(name, stats)| (name.clone(), stats.samples, stats.total_time))
            .collect()
    }

    pub fn save_analysis(&self, output_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let analysis_file = output_dir.join("function_analysis.json");
        let json = serde_json::to_string_pretty(&self.functions)?;
        fs::write(analysis_file, json)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recorder = PerfRecorder::new();
    
    // Create test file if it doesn't exist
    if !std::path::Path::new("test.rs").exists() {
        fs::write("test.rs", r#"
fn main() {
    println!("Hello, world!");
    let x = vec![1, 2, 3, 4, 5];
    let sum: i32 = x.iter().sum();
    println!("Sum: {}", sum);
}
"#)?;
    }
    
    println!("=== Recording rustc compilation ===");
    recorder.record_compilation("test.rs")?;
    
    println!("=== Extracting perf script ===");
    let script = recorder.extract_script()?;
    
    fs::write("perf_data/script.txt", &script)?;
    println!("Script saved to perf_data/script.txt ({} lines)", script.lines().count());
    
    println!("=== Parsing symbols from librustc_driver.so ===");
    let symbols = recorder.parse_symbols()?;
    recorder.save_symbols(&symbols)?;
    
    println!("Found {} symbols", symbols.len());
    
    println!("=== Analyzing performance data ===");
    let analyzer = PerfAnalyzer::parse_script(&script);
    analyzer.save_analysis(&recorder.output_dir)?;
    
    println!("\n=== Top 20 Functions by Sample Count ===");
    for (i, (name, samples, time)) in analyzer.top_functions(20).iter().enumerate() {
        println!("{:2}: {:6} samples, {:8.3}ms - {}", i+1, samples, time, name);
    }
    
    println!("\n=== Parser-Related Functions ===");
    let parser_funcs = analyzer.parser_functions();
    for (name, samples, time) in parser_funcs.iter().take(10) {
        println!("{:6} samples, {:8.3}ms - {}", samples, time, name);
    }
    
    println!("\n=== Top 10 Demangled Symbols ===");
    for (i, sym) in symbols.iter()
        .filter(|s| s.demangled.contains("rustc") || s.demangled.contains("parse"))
        .take(10)
        .enumerate() 
    {
        println!("{}: {} @ 0x{:x} (size: {})", i+1, sym.demangled, sym.address, sym.size);
    }
    
    // Generate target functions for interception
    let target_functions: Vec<_> = analyzer.parser_functions()
        .into_iter()
        .take(5)
        .map(|(name, samples, _)| serde_json::json!({
            "name": name,
            "samples": samples,
            "priority": "high"
        }))
        .collect();
    
    let targets_json = serde_json::to_string_pretty(&target_functions)?;
    fs::write("parser_target_functions.json", targets_json)?;
    
    println!("\nTarget functions saved to parser_target_functions.json");
    println!("Analysis complete! Check perf_data/ directory for results.");
    
    Ok(())
}
