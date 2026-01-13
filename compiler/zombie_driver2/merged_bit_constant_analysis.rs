use goblin::elf::Elf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 TEXT BIT ANALYSIS + CONSTANT PATTERN SEARCH");
    println!("==============================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    // PHASE 1: TEXT SEGMENT BIT ANALYSIS
    println!("\n📝 PHASE 1: TEXT SEGMENT ANALYSIS");
    println!("=================================");

    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("No .text section found")?;

    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_bytes = &binary[text_start..text_start + text_size.min(10000)];

    println!("📦 Text section: {} bytes (analyzing first 10KB)", text_size);

    let mut instruction_patterns = std::collections::HashMap::new();
    let mut found_patterns = Vec::new();

    for (i, chunk) in text_bytes.chunks_exact(4).enumerate().take(100) {
        let instruction = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let opcode = instruction & 0xFF;
        *instruction_patterns.entry(opcode).or_insert(0) += 1;
        found_patterns.push(instruction);

        if i < 5 {
            println!(
                "  {:08x}: {:08x} = opcode {:02x}",
                text_section.sh_addr + (i * 4) as u64,
                instruction,
                opcode
            );
        }
    }

    let mut sorted_patterns: Vec<_> = instruction_patterns.iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n🎯 TOP OPCODES:");
    let target_opcodes: Vec<u8> =
        sorted_patterns.iter().take(10).map(|(opcode, _)| **opcode as u8).collect();
    for (opcode, count) in sorted_patterns.iter().take(10) {
        println!("  {:02x}: {} times", opcode, count);
    }

    // PHASE 2: CONSTANT SECTION SEARCH
    println!("\n📝 PHASE 2: CONSTANT SECTION SEARCH");
    println!("===================================");

    let sections = [".rodata", ".data", ".bss", ".got"];

    for section_name in &sections {
        if let Some(section) = elf
            .section_headers
            .iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == *section_name)
        {
            let start = section.sh_offset as usize;
            let size = section.sh_size as usize;

            if start + size <= binary.len() && size > 0 {
                let section_bytes = &binary[start..start + size.min(20000)];

                println!("\n📦 {} section: {} bytes", section_name, size);

                // Search for opcode clusters
                let mut clusters = 0;
                let mut exact_matches = 0;

                for (i, window) in section_bytes.windows(16).enumerate().step_by(4) {
                    let mut matches = 0;
                    for &byte in window {
                        if target_opcodes.contains(&byte) {
                            matches += 1;
                        }
                    }
                    if matches >= 3 {
                        clusters += 1;
                        if clusters <= 3 {
                            println!(
                                "  🎯 Cluster at {:08x}: {} opcode matches",
                                start + i,
                                matches
                            );
                        }
                    }
                }

                // Search for exact instruction patterns from text
                for (i, chunk) in section_bytes.chunks_exact(4).enumerate() {
                    let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    if found_patterns.contains(&value) {
                        exact_matches += 1;
                        if exact_matches <= 5 {
                            println!("  🔍 Exact match at {:08x}: {:08x}", start + i * 4, value);
                        }
                    }
                }

                println!(
                    "  📊 Total: {} opcode clusters, {} exact matches",
                    clusters, exact_matches
                );
            }
        }
    }

    Ok(())
}
