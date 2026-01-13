use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 SYMBOL N-GRAM CLUSTERING FOR CODEGEN ANALYSIS");
    println!("================================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let mut all_symbols = Vec::new();
    let mut demangled_symbols = Vec::new();

    // Collect and demangle symbols
    for sym in elf.syms.iter().take(5000) {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_size > 0 {
                all_symbols.push(name.to_string());

                // Simple Rust demangle for _ZN patterns
                let demangled = if name.starts_with("_ZN") {
                    simple_rust_demangle(name)
                } else {
                    name.to_string()
                };
                demangled_symbols.push(demangled);
            }
        }
    }

    println!("📦 Collected {} symbols", all_symbols.len());

    // Generate n-grams from demangled symbols
    let mut ngram_clusters = HashMap::new();
    let target_terms =
        ["codegen", "demangle", "llvm", "backend", "target", "arch", "instruction", "opcode"];

    for symbol in &demangled_symbols {
        let lower_symbol = symbol.to_lowercase();

        // Generate 3-grams, 4-grams, 5-grams
        for n in 3..=5 {
            for window in lower_symbol.chars().collect::<Vec<_>>().windows(n) {
                let ngram: String = window.iter().collect();

                // Check if n-gram contains target terms
                for &term in &target_terms {
                    if ngram.contains(term) {
                        ngram_clusters
                            .entry(term.to_string())
                            .or_insert_with(Vec::new)
                            .push(symbol.clone());
                        break;
                    }
                }
            }
        }
    }

    // Analyze clusters
    println!("\n🎯 CODEGEN-RELATED SYMBOL CLUSTERS:");
    println!("===================================");

    for (term, symbols) in &ngram_clusters {
        if !symbols.is_empty() {
            println!("\n📦 {} cluster ({} symbols):", term.to_uppercase(), symbols.len());

            // Deduplicate and show top matches
            let mut unique_symbols: Vec<_> = symbols.iter().collect();
            unique_symbols.sort();
            unique_symbols.dedup();

            for symbol in unique_symbols.iter().take(10) {
                if symbol.len() > 20 {
                    println!("  {}", if symbol.len() > 80 { &symbol[..80] } else { symbol });
                }
            }

            if unique_symbols.len() > 10 {
                println!("  ... and {} more", unique_symbols.len() - 10);
            }
        }
    }

    // Look for decoder patterns in symbol names
    println!("\n🔍 POTENTIAL DECODER FUNCTIONS:");
    println!("===============================");

    let decoder_patterns = ["decode", "parse", "interpret", "switch", "dispatch", "handle"];

    for symbol in &demangled_symbols {
        let lower = symbol.to_lowercase();
        for &pattern in &decoder_patterns {
            if lower.contains(pattern)
                && (lower.contains("opcode")
                    || lower.contains("instruction")
                    || lower.contains("bytecode"))
            {
                println!("  {}", if symbol.len() > 100 { &symbol[..100] } else { symbol });
                break;
            }
        }
    }

    // Architecture-specific patterns
    println!("\n🏗️  ARCHITECTURE-SPECIFIC SYMBOLS:");
    println!("==================================");

    let arch_patterns = ["x86", "x64", "amd64", "intel", "sse", "avx", "target_feature"];

    for symbol in &demangled_symbols {
        let lower = symbol.to_lowercase();
        for &arch in &arch_patterns {
            if lower.contains(arch) {
                println!("  {}", if symbol.len() > 100 { &symbol[..100] } else { symbol });
                break;
            }
        }
    }

    Ok(())
}

fn simple_rust_demangle(mangled: &str) -> String {
    // Very basic Rust demangling for _ZN patterns
    if !mangled.starts_with("_ZN") {
        return mangled.to_string();
    }

    // Extract the middle part and look for common patterns
    let middle = &mangled[3..mangled.len().saturating_sub(3)];

    // Look for common Rust patterns
    if middle.contains("rustc") {
        return format!("rustc::{}", middle);
    } else if middle.contains("core") {
        return format!("core::{}", middle);
    } else if middle.contains("std") {
        return format!("std::{}", middle);
    }

    // Return simplified version
    format!("rust::{}", middle)
}
