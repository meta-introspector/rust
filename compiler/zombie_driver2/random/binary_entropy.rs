use std::fs;
use goblin::elf::Elf;

pub fn extract_function_entropy(binary_path: &str, arg: u64) -> Option<f64> {
    let binary = fs::read(binary_path).ok()?;
    let elf = Elf::parse(&binary).ok()?;
    
    // Hunt for rand-like functions in symbols
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            let demangled = rustc_demangle::demangle(name).to_string();
            if (demangled.contains("rand") || demangled.contains("random")) && sym.st_size > 0 {
                if let Some(entropy) = extract_symbol_entropy(&binary, &elf, &sym, arg) {
                    println!("🎲 GENERATED: {} from function entropy", entropy);
                    return Some(entropy);
                }
            }
        }
    }
    None
}

fn extract_symbol_entropy(binary: &[u8], elf: &Elf, sym: &goblin::elf::Sym, arg: u64) -> Option<f64> {
    // Find the .text section
    for section in elf.section_headers.iter() {
        if let Some(section_name) = elf.shdr_strtab.get_at(section.sh_name) {
            if section_name == ".text" {
                let text_section = section;
                let func_start = (sym.st_value - text_section.sh_addr) as usize;
                let func_size = sym.st_size as usize;
                let text_start = text_section.sh_offset as usize;
                let text_bytes = &binary[text_start..text_start + text_section.sh_size as usize];
                
                if func_start + func_size <= text_bytes.len() {
                    let func_bytes = &text_bytes[func_start..func_start + func_size];
                    
                    // Use function bytecode as random seed
                    let mut entropy = 0u64;
                    for (i, &byte) in func_bytes.iter().enumerate().take(8) {
                        entropy ^= (byte as u64) << (i * 8);
                    }
                    
                    // Apply argument as modifier
                    entropy = entropy.wrapping_mul(arg).wrapping_add(sym.st_value);
                    
                    // Generate float from entropy
                    return Some(((entropy >> 16) & 0xFFFFFFFF) as f64 / 4294967296.0);
                }
            }
        }
    }
    None
}

pub fn fallback_random(arg: u64) -> f64 {
    let hash = arg.wrapping_mul(1103515245).wrapping_add(12345);
    ((hash >> 16) & 0x7fff) as f64 / 32768.0
}
