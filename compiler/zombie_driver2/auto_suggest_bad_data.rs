use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

macro_rules! auto_suggest {
    ($opcode:expr, $context:expr, $frequency:expr) => {{
        let suggestion = match $opcode {
            0x7e => "JLE rel8 (jump if less/equal)",
            0xc9 => "LEAVE (restore stack frame)",
            0x34 => "XOR AL, imm8 (exclusive or)",
            0xf2 => "REPNE prefix (repeat while not equal)",
            0xb0 => "MOV AL, imm8 (move immediate to AL)",
            0x1d => "SBB EAX, imm32 (subtract with borrow)",
            0x10 => "ADC r/m8, r8 (add with carry)",
            0x15 => "ADC EAX, imm32 (add with carry)",
            0x3d => "CMP EAX, imm32 (compare immediate)",
            0x70 => "JO rel8 (jump if overflow)",
            0x7c => "JL rel8 (jump if less)",
            0x97 => "XCHG EAX, EDI (exchange registers)",
            0x98 => "CBW/CWDE/CDQE (convert byte/word/dword)",
            0xba => "MOV EDX, imm32 (move immediate to EDX)",
            0xbc => "MOV ESP, imm32 (move immediate to ESP)",
            0xc0 => "Shift group (rotate/shift operations)",
            0xc1 => "Shift group (rotate/shift operations)",
            0xc7 => "MOV r/m32, imm32 (move immediate to memory)",
            0xc8 => "ENTER (create stack frame)",
            0xd0 => "Shift group (single bit operations)",
            0xeb => "JMP rel8 (short jump)",
            0xff => "INC/DEC/CALL/JMP group (increment/decrement/call/jump)",
            _ => "NEEDS_MANUAL_CLASSIFICATION",
        };

        if suggestion != "NEEDS_MANUAL_CLASSIFICATION" {
            println!(
                "  💡 AUTO-SUGGEST: {:02x} = {} (freq: {}, context: {})",
                $opcode, suggestion, $frequency, $context
            );
            true
        } else {
            println!(
                "  ❌ BAD_DATA: {:02x} requires manual classification (freq: {}, context: {})",
                $opcode, $frequency, $context
            );
            false
        }
    }};
}

macro_rules! extract_bad_data {
    ($bad_opcodes:expr) => {{
        println!("\n🚨 BAD DATA EXTRACTION:");
        println!("=======================");

        for (opcode, (freq, contexts)) in $bad_opcodes.iter() {
            println!("  ❌ Opcode {:02x}:", opcode);
            println!("     Frequency: {} occurrences", freq);
            println!("     Contexts: {:?}", contexts);
            println!("     Action: Manual classification required");

            // Suggest possible fixes based on context
            if contexts.contains(&"function_start".to_string()) {
                println!("     💡 Hint: Likely function prologue instruction");
            } else if contexts.contains(&"function_end".to_string()) {
                println!("     💡 Hint: Likely function epilogue instruction");
            } else if contexts.contains(&"arithmetic".to_string()) {
                println!("     💡 Hint: Likely arithmetic/logic instruction");
            } else if contexts.contains(&"control_flow".to_string()) {
                println!("     💡 Hint: Likely jump/branch instruction");
            }
            println!();
        }
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AUTO-SUGGEST + BAD DATA EXTRACTION SYSTEM");
    println!("=============================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("No .text section found")?;

    let text_start = text_section.sh_offset as usize;
    let text_bytes = &binary[text_start..text_start + text_section.sh_size as usize];

    let mut opcode_stats = HashMap::new();
    let mut bad_data = HashMap::new();
    let mut suggestions_made = 0;

    // Analyze decoder functions
    for sym in elf.syms.iter().take(200) {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            let demangled = rust_demangle(name);

            if demangled.contains("fma")
                || demangled.contains("backend")
                || demangled.contains("codegen")
            {
                let func_start = (sym.st_value - text_section.sh_addr) as usize;
                let func_size = sym.st_size as usize;

                if func_start + func_size <= text_bytes.len() && func_size > 0 {
                    let func_bytes = &text_bytes[func_start..func_start + func_size];

                    println!("\n🔍 ANALYZING: {}", &demangled[..60.min(demangled.len())]);

                    for (i, chunk) in func_bytes.chunks_exact(4).enumerate().take(15) {
                        let instruction =
                            u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                        let opcode = (instruction & 0xFF) as u8;
                        let address = text_section.sh_addr + func_start as u64 + (i * 4) as u64;

                        // Determine context
                        let context = if i == 0 {
                            "function_start"
                        } else if i > 10 {
                            "function_end"
                        } else if opcode & 0xF0 == 0x70 {
                            "control_flow"
                        } else if opcode & 0xF8 == 0x80 {
                            "arithmetic"
                        } else {
                            "general"
                        };

                        // Track statistics
                        let entry = opcode_stats.entry(opcode).or_insert((0, Vec::new()));
                        entry.0 += 1;
                        if !entry.1.contains(&context.to_string()) {
                            entry.1.push(context.to_string());
                        }

                        // Try auto-suggestion
                        if !auto_suggest!(opcode, context, entry.0) {
                            bad_data.insert(opcode, entry.clone());
                        } else {
                            suggestions_made += 1;
                        }
                    }
                }
            }
        }
    }

    // Extract and report bad data
    extract_bad_data!(bad_data);

    // Summary
    println!("\n📊 AUTO-SUGGESTION SUMMARY:");
    println!("===========================");
    println!("   Total suggestions made: {}", suggestions_made);
    println!("   Bad data entries: {}", bad_data.len());
    println!(
        "   Classification success rate: {:.1}%",
        (suggestions_made as f64 / (suggestions_made + bad_data.len()) as f64) * 100.0
    );

    // Top unclassified opcodes
    println!("\n🎯 TOP UNCLASSIFIED OPCODES:");
    let mut bad_sorted: Vec<_> = bad_data.iter().collect();
    bad_sorted.sort_by(|a, b| b.1.0.cmp(&a.1.0));

    for (opcode, (freq, _)) in bad_sorted.iter().take(5) {
        println!("   {:02x}: {} occurrences - NEEDS MANUAL FIX", opcode, freq);
    }

    println!("\n✅ AUTO-SUGGESTION SYSTEM COMPLETE");
    println!("   System automatically suggests new instructions");
    println!("   Bad data extracted for manual classification");

    Ok(())
}

fn rust_demangle(mangled: &str) -> String {
    if !mangled.starts_with("_ZN") {
        return mangled.to_string();
    }

    let core = &mangled[3..];
    let mut result = String::new();
    let mut chars = core.chars().peekable();
    let mut current_len = String::new();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            current_len.push(ch);
        } else if ch == 'E' {
            break;
        } else {
            if let Ok(len) = current_len.parse::<usize>() {
                if len > 0 && len < 200 {
                    let mut segment = String::new();
                    segment.push(ch);

                    for _ in 1..len {
                        if let Some(next_ch) = chars.next() {
                            segment.push(next_ch);
                        }
                    }

                    if !result.is_empty() {
                        result.push_str("::");
                    }
                    result.push_str(&segment);
                }
            }
            current_len.clear();
        }
    }

    if result.is_empty() { mangled.to_string() } else { result }
}
