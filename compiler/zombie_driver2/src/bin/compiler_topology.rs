use goblin::elf::Elf;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct CompilerNode {
    name: String,
    symbol_type: String,
    size: u64,
    references: usize,
    weight: u64,
    level: String,
    dependencies: Vec<String>,
    data_content: Option<Vec<u8>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️ RUSTC TOPOLOGICAL COMPILER MAP");
    println!("=================================");

    let rustc_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";

    println!("📂 Reading: {}", rustc_path);
    let buffer = fs::read(rustc_path)?;

    println!("🧮 Parsing ELF...");
    let elf = Elf::parse(&buffer)?;

    // Build topology map
    let mut topology = HashMap::new();
    let mut reference_count = HashMap::new();

    // Count references
    for (_, reloc_section) in &elf.shdr_relocs {
        for reloc in reloc_section.iter() {
            let sym_idx = reloc.r_sym;
            if sym_idx < elf.syms.len() {
                *reference_count.entry(sym_idx).or_insert(0) += 1;
            }
        }
    }

    // Extract data sections for constants/tables
    let mut data_sections = HashMap::new();
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name.contains("data") || name.contains("rodata") || name.contains("const") {
                let start = section.sh_offset as usize;
                let size = section.sh_size as usize;
                if start + size <= buffer.len() {
                    data_sections.insert(name.to_string(), buffer[start..start + size].to_vec());
                }
            }
        }
    }

    println!("📊 Building topology for {} symbols...", elf.syms.len().min(200));

    // Build nodes
    for (i, sym) in elf.syms.iter().enumerate().take(200) {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            let refs = reference_count.get(&i).unwrap_or(&0);
            let weight = sym.st_size + (*refs as u64 * 10);
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

            // Extract data if it's an object/data symbol
            let data_content = if sym_type == "OBJECT" && sym.st_size > 0 && sym.st_size < 1024 {
                // Try to extract data from the symbol's location
                Some(vec![0u8; sym.st_size as usize]) // Placeholder for actual data extraction
            } else {
                None
            };

            let node = CompilerNode {
                name: name.to_string(),
                symbol_type: sym_type.to_string(),
                size: sym.st_size,
                references: *refs,
                weight,
                level: level.to_string(),
                dependencies: Vec::new(), // Will be filled by analyzing relocations
                data_content,
            };

            topology.insert(i, node);
        }
    }

    println!("🔍 Data Sections Found:");
    for (name, data) in &data_sections {
        println!("  {} | size: {} bytes", name, data.len());
        if data.len() < 100 {
            println!("    Content preview: {:?}", &data[..data.len().min(20)]);
        }
    }

    println!("\n🗺️ Compiler Topology (Critical Nodes):");
    let mut critical_nodes: Vec<_> = topology.values().collect();
    critical_nodes.sort_by(|a, b| b.weight.cmp(&a.weight));

    for node in critical_nodes.iter().take(20) {
        println!(
            "🔴 {} | type: {} | weight: {} | level: {} | refs: {} | size: {}",
            node.name, node.symbol_type, node.weight, node.level, node.references, node.size
        );

        if let Some(data) = &node.data_content {
            println!("    📊 Data: {} bytes", data.len());
        }
    }

    Ok(())
}
