use goblin::elf::{Elf, reloc::RelocSection};
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 RUSTC SYMBOL DEPENDENCY ANALYZER");
    println!("===================================");

    let rustc_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";

    println!("📂 Reading: {}", rustc_path);
    let buffer = fs::read(rustc_path)?;

    println!("🧮 Parsing ELF...");
    let elf = Elf::parse(&buffer)?;

    // Build symbol name to index map
    let mut symbol_map = HashMap::new();
    for (i, sym) in elf.syms.iter().enumerate() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            symbol_map.insert(name, i);
        }
    }

    // Count references to each symbol
    let mut reference_count = HashMap::new();
    let mut referenced_by = HashMap::<usize, Vec<String>>::new();

    // Analyze relocations to find dependencies
    for (_, reloc_section) in &elf.shdr_relocs {
        for reloc in reloc_section.iter() {
            let sym_idx = reloc.r_sym;
            if sym_idx < elf.syms.len() {
                *reference_count.entry(sym_idx).or_insert(0) += 1;
            }
        }
    }

    println!("📊 Symbol Analysis (top 50 by weight):");
    let mut weighted_symbols: Vec<_> = elf.syms.iter().enumerate().take(100).collect();

    for (i, sym) in weighted_symbols {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            let refs = reference_count.get(&i).unwrap_or(&0);
            let weight = sym.st_size + (*refs as u64 * 10); // Size + 10x reference count
            let level = if *refs > 100 {
                "CRITICAL"
            } else if *refs > 10 {
                "HIGH"
            } else if *refs > 0 {
                "MEDIUM"
            } else {
                "LOW"
            };

            let sym_type = match sym.st_type() {
                0 => "NOTYPE",
                1 => "OBJECT",
                2 => "FUNC",
                3 => "SECTION",
                4 => "FILE",
                5 => "COMMON",
                6 => "TLS",
                _ => "OTHER",
            };

            println!(
                "{}: {} | weight: {} | level: {} | refs: {} | size: {} | type: {}",
                i, name, weight, level, refs, sym.st_size, sym_type
            );
        }
    }

    Ok(())
}
