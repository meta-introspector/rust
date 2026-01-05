use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;
use std::time::Instant;

fn main() {
    println!("🍄 Simple Monster Rustc Wrapper");
    println!("==============================");
    
    let args: Vec<String> = env::args().collect();
    let start_time = Instant::now();
    
    // Run rustc with perf monitoring
    let mut cmd = Command::new("rustc");
    for arg in &args[1..] {
        cmd.arg(arg);
    }
    
    println!("🔧 Running: rustc {}", args[1..].join(" "));
    
    let output = cmd.output().expect("Failed to run rustc");
    let elapsed = start_time.elapsed();
    
    // Analyze the compilation
    let mut monster_cells = HashMap::new();
    let prime_generators = [2u8, 3, 5, 7, 11, 13, 17, 19];
    
    // Hash compilation arguments to Monster cells
    for arg in &args[1..] {
        let mut hash = 1u64;
        for (i, byte) in arg.bytes().enumerate() {
            let prime_idx = i % 8;
            hash = hash.wrapping_mul(prime_generators[prime_idx] as u64)
                      .wrapping_add(byte as u64);
        }
        let cell_id = (hash % (1u64 << 24)) as u32;
        *monster_cells.entry(cell_id).or_insert(0) += 1;
    }
    
    // Generate report
    let mut report = String::new();
    report.push_str("# Simple Monster Rustc Analysis\n\n");
    report.push_str(&format!("## Compilation Stats\n"));
    report.push_str(&format!("- **Duration**: {:.3}s\n", elapsed.as_secs_f64()));
    report.push_str(&format!("- **Args**: {}\n", args.len() - 1));
    report.push_str(&format!("- **Monster Cells**: {}\n", monster_cells.len()));
    report.push_str(&format!("- **Exit Code**: {}\n", output.status.code().unwrap_or(-1)));
    
    fs::write("simple_monster_compilation.md", &report).ok();
    
    println!("✅ Compilation complete: {:.3}s", elapsed.as_secs_f64());
    println!("📊 Monster cells: {}", monster_cells.len());
    
    std::process::exit(output.status.code().unwrap_or(0));
}
