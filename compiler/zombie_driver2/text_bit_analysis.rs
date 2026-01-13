use goblin::elf::Elf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 TEXT SEGMENT BIT PATTERN ANALYSIS");
    println!("===================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    // Find .text section
    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("No .text section found")?;

    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_bytes = &binary[text_start..text_start + text_size.min(10000)]; // First 10KB

    println!("📦 Text section: {} bytes (analyzing first 10KB)", text_size);

    // Analyze bit patterns in 4-byte chunks (typical instruction size)
    let mut instruction_patterns = std::collections::HashMap::new();

    for (i, chunk) in text_bytes.chunks_exact(4).enumerate().take(2500) {
        let instruction = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let address = text_section.sh_addr + (i * 4) as u64;

        // Extract bit patterns
        let opcode = instruction & 0xFF; // First byte
        let modrm = (instruction >> 8) & 0xFF; // Second byte  
        let high_bits = instruction >> 24; // High byte

        *instruction_patterns.entry(opcode).or_insert(0) += 1;

        // Show first 20 instructions with bit breakdown
        if i < 20 {
            println!("  {:08x}: {:08x} = {:032b}", address, instruction, instruction);
            println!(
                "    opcode: {:02x} ({:08b}), modrm: {:02x} ({:08b})",
                opcode, opcode, modrm, modrm
            );
        }
    }

    println!("\n🎯 MOST COMMON OPCODES:");
    let mut sorted_patterns: Vec<_> = instruction_patterns.iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));

    for (opcode, count) in sorted_patterns.iter().take(10) {
        println!("  {:02x} ({:08b}): {} occurrences", opcode, opcode, count);
    }

    Ok(())
}
