use rustc_demangle;
use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 DECODER SWITCH ANALYSIS: WHO USES OPCODE DATA");
    println!("================================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let target_opcodes = [0x0f, 0x00, 0x48, 0x55, 0xf8, 0xc3, 0x8b, 0x89, 0x84, 0x8d];

    // Get text section
    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("No .text section found")?;

    let text_start = text_section.sh_offset as usize;
    let text_bytes = &binary[text_start..text_start + text_section.sh_size as usize];

    println!("\n🎯 FUNCTIONS WITH OPCODE DATA (POTENTIAL DECODERS):");
    println!("===================================================");

    for sym in elf.syms.iter().take(100) {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 50 && sym.st_value > 0 {
                let func_start = (sym.st_value - text_section.sh_addr) as usize;
                let func_size = sym.st_size as usize;

                if func_start + func_size <= text_bytes.len() {
                    let func_bytes = &text_bytes[func_start..func_start + func_size];

                    // Count opcodes as DATA (any byte position)
                    let mut opcode_data_count = 0;
                    let mut opcode_breakdown = HashMap::new();

                    for &byte in func_bytes {
                        if target_opcodes.contains(&byte) {
                            opcode_data_count += 1;
                            *opcode_breakdown.entry(byte).or_insert(0) += 1;
                        }
                    }

                    // Only show functions with significant opcode data
                    if opcode_data_count >= 20 {
                        // Demangle name
                        let demangled: String = if name.starts_with("_Z") {
                            rustc_demangle::demangle(name).to_string()
                        } else {
                            name.to_string()
                        };

                        println!(
                            "\n📦 {} ({} opcode bytes)",
                            if demangled.len() > 60 { &demangled[..60] } else { &demangled },
                            opcode_data_count
                        );

                        // Show opcode breakdown
                        let mut sorted_opcodes: Vec<_> = opcode_breakdown.iter().collect();
                        sorted_opcodes.sort_by(|a, b| b.1.cmp(a.1));

                        print!("   Opcodes: ");
                        for (opcode, count) in sorted_opcodes.iter().take(5) {
                            print!("{:02x}({}) ", opcode, count);
                        }
                        println!();

                        // Look for switch/branch patterns (common decoder patterns)
                        let switch_patterns = count_switch_patterns(func_bytes);
                        if switch_patterns > 0 {
                            println!("   🔀 {} potential switch/branch patterns", switch_patterns);
                        }

                        // Look for comparison patterns with opcodes
                        let cmp_patterns = count_comparison_patterns(func_bytes, &target_opcodes);
                        if cmp_patterns > 0 {
                            println!("   ⚖️  {} opcode comparison patterns", cmp_patterns);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn count_switch_patterns(bytes: &[u8]) -> usize {
    let mut count = 0;
    // Look for jump table patterns (common in switch statements)
    for window in bytes.windows(4) {
        // Look for patterns like: cmp, jmp, jne, je
        if (window[0] == 0x3c || window[0] == 0x83) && // CMP instructions
           (window[2] == 0x74 || window[2] == 0x75 || window[2] == 0xeb)
        {
            // JE/JNE/JMP
            count += 1;
        }
    }
    count
}

fn count_comparison_patterns(bytes: &[u8], target_opcodes: &[u8]) -> usize {
    let mut count = 0;
    // Look for immediate comparisons with our target opcodes
    for window in bytes.windows(3) {
        if window[0] == 0x3c || window[0] == 0x80 || window[0] == 0x83 {
            // CMP immediate
            if target_opcodes.contains(&window[1]) || target_opcodes.contains(&window[2]) {
                count += 1;
            }
        }
    }
    count
}
