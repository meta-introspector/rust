use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

macro_rules! search_pattern {
    ($symbols:expr, $pattern:expr, $name:expr) => {{
        let mut matches = Vec::new();
        for symbol in $symbols {
            if symbol.to_lowercase().contains($pattern) {
                matches.push(symbol.clone());
            }
        }
        if !matches.is_empty() {
            println!("\n🎯 {} PATTERN ({} matches):", $name, matches.len());
            for m in matches.iter().take(5) {
                println!("  {}", if m.len() > 80 { &m[..80] } else { m });
            }
            if matches.len() > 5 {
                println!("  ... and {} more", matches.len() - 5);
            }
        }
        matches
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 CUSTOM RUST DEMANGLER + PATTERN SEARCH");
    println!("=========================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let mut demangled_symbols = Vec::new();

    // Collect and demangle symbols
    for sym in elf.syms.iter().take(3000) {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_size > 10 {
                let demangled = rust_demangle(name);
                demangled_symbols.push(demangled);
            }
        }
    }

    println!("📦 Demangled {} symbols", demangled_symbols.len());

    // Use macros to search for patterns
    let codegen_matches = search_pattern!(&demangled_symbols, "codegen", "CODEGEN");
    let decoder_matches = search_pattern!(&demangled_symbols, "decode", "DECODER");
    let arch_matches = search_pattern!(&demangled_symbols, "arch", "ARCHITECTURE");
    let llvm_matches = search_pattern!(&demangled_symbols, "llvm", "LLVM");
    let target_matches = search_pattern!(&demangled_symbols, "target", "TARGET");
    let instruction_matches = search_pattern!(&demangled_symbols, "instruction", "INSTRUCTION");
    let opcode_matches = search_pattern!(&demangled_symbols, "opcode", "OPCODE");
    let backend_matches = search_pattern!(&demangled_symbols, "backend", "BACKEND");
    let switch_matches = search_pattern!(&demangled_symbols, "switch", "SWITCH");
    let dispatch_matches = search_pattern!(&demangled_symbols, "dispatch", "DISPATCH");

    // Cross-reference patterns
    println!("\n🔍 CROSS-PATTERN ANALYSIS:");
    println!("==========================");

    let all_patterns = [
        ("codegen", &codegen_matches),
        ("decoder", &decoder_matches),
        ("arch", &arch_matches),
        ("llvm", &llvm_matches),
        ("target", &target_matches),
        ("instruction", &instruction_matches),
        ("opcode", &opcode_matches),
        ("backend", &backend_matches),
        ("switch", &switch_matches),
        ("dispatch", &dispatch_matches),
    ];

    // Find symbols matching multiple patterns
    for symbol in &demangled_symbols {
        let mut pattern_count = 0;
        let mut matched_patterns = Vec::new();

        for (pattern_name, _) in &all_patterns {
            if symbol.to_lowercase().contains(pattern_name) {
                pattern_count += 1;
                matched_patterns.push(pattern_name);
            }
        }

        if pattern_count >= 2 {
            println!(
                "  🎯 {} patterns: {} - {}",
                pattern_count,
                matched_patterns.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("+"),
                if symbol.len() > 60 { &symbol[..60] } else { symbol }
            );
        }
    }

    Ok(())
}

fn rust_demangle(mangled: &str) -> String {
    if !mangled.starts_with("_ZN") {
        return mangled.to_string();
    }

    // Extract the core part
    let core = &mangled[3..];
    let mut result = String::new();
    let mut chars = core.chars().peekable();
    let mut current_len = String::new();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            current_len.push(ch);
        } else if ch == 'E' {
            break; // End marker
        } else {
            // We have a length, now read that many characters
            if let Ok(len) = current_len.parse::<usize>() {
                if len > 0 && len < 200 {
                    // Sanity check
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
