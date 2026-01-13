use std::fs;
use goblin::elf::Elf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 CONSTANT SECTION INSTRUCTION PATTERN SEARCH");
    println!("==============================================");
    
    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;
    
    // Target opcodes from text analysis
    let target_opcodes = [0x00, 0x48, 0x24, 0x89, 0x8b, 0x84, 0x01, 0x44, 0x7c, 0x0f];
    let target_patterns = [0xe5894855u32, 0x2e660b0f, 0x00841f0f, 0x00000000];
    
    // Search .rodata, .data, .bss sections
    let sections = [".rodata", ".data", ".bss", ".got", ".plt"];
    
    for section_name in &sections {
        if let Some(section) = elf.section_headers.iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == *section_name) {
            
            let start = section.sh_offset as usize;
            let size = section.sh_size as usize;
            
            if start + size <= binary.len() && size > 0 {
                let section_bytes = &binary[start..start + size.min(50000)]; // First 50KB
                
                println!("\n📦 {} section: {} bytes", section_name, size);
                
                // Search for opcode clusters
                let mut opcode_clusters = Vec::new();
                for (i, window) in section_bytes.windows(16).enumerate().step_by(4) {
                    let mut matches = 0;
                    for &byte in window {
                        if target_opcodes.contains(&byte) {
                            matches += 1;
                        }
                    }
                    if matches >= 4 { // At least 4 target opcodes in 16 bytes
                        opcode_clusters.push((start + i, matches, window.to_vec()));
                    }
                }
                
                // Search for exact instruction patterns
                let mut pattern_matches = Vec::new();
                for (i, chunk) in section_bytes.chunks_exact(4).enumerate() {
                    let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    if target_patterns.contains(&value) {
                        pattern_matches.push((start + i * 4, value));
                    }
                }
                
                if !opcode_clusters.is_empty() {
                    println!("  🎯 Found {} opcode clusters:", opcode_clusters.len());
                    for (addr, matches, bytes) in opcode_clusters.iter().take(5) {
                        println!("    {:08x}: {} matches - {:02x?}", addr, matches, &bytes[..8]);
                    }
                }
                
                if !pattern_matches.is_empty() {
                    println!("  🔍 Found {} exact pattern matches:", pattern_matches.len());
                    for (addr, pattern) in pattern_matches.iter().take(10) {
                        println!("    {:08x}: {:08x}", addr, pattern);
                    }
                }
            }
        }
    }
    
    Ok(())
}
