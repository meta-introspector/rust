use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 RUSTC ENUM & CONSTANT EXTRACTOR");
    println!("==================================");

    let rustc_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";

    println!("📂 Reading: {}", rustc_path);
    let buffer = fs::read(rustc_path)?;

    println!("🧮 Parsing ELF...");
    let elf = Elf::parse(&buffer)?;

    // Extract constant data sections
    let mut constant_data = HashMap::new();
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name.contains("rodata") || name.contains("const") {
                let start = section.sh_offset as usize;
                let size = section.sh_size as usize;
                if start + size <= buffer.len() && size > 0 {
                    constant_data.insert(name.to_string(), &buffer[start..start + size]);
                }
            }
        }
    }

    // Look for enum-related symbols (fmt, Debug, Display functions)
    let mut enum_symbols = Vec::new();
    let mut string_symbols = Vec::new();

    for (i, sym) in elf.syms.iter().enumerate() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            // Look for Debug/Display implementations (enum to string converters)
            if name.contains("fmt") && (name.contains("Debug") || name.contains("Display")) {
                enum_symbols.push((i, name, sym.st_size));
            }

            // Look for string literals and constants
            if sym.st_type() == 1 && sym.st_size > 0 {
                // OBJECT type
                if name.contains("str") || name.contains("STRING") || name.contains("CONST") {
                    string_symbols.push((i, name, sym.st_size));
                }
            }
        }
    }

    println!("📊 Constant Data Sections:");
    for (name, data) in &constant_data {
        println!("  {} | size: {} bytes", name, data.len());

        // Look for string patterns in constant data
        let mut strings = Vec::new();
        let mut i = 0;
        while i < data.len() {
            if data[i] >= 32 && data[i] <= 126 {
                // Printable ASCII
                let start = i;
                while i < data.len() && data[i] >= 32 && data[i] <= 126 {
                    i += 1;
                }
                if i - start > 3 {
                    // Minimum string length
                    if let Ok(s) = std::str::from_utf8(&data[start..i]) {
                        strings.push(s.to_string());
                    }
                }
            } else {
                i += 1;
            }
        }

        if !strings.is_empty() {
            println!("    🔤 Strings found: {}", strings.len());
            for s in strings.iter().take(10) {
                if s.len() > 2 && s.len() < 50 {
                    println!("      \"{}\"", s);
                }
            }
        }
    }

    println!("\n🎯 Enum-related Symbols (Debug/Display functions):");
    for (idx, name, size) in enum_symbols.iter().take(20) {
        println!("  {}: {} | size: {} bytes", idx, name, size);

        // Try to extract enum name from mangled symbol
        if let Some(enum_name) = extract_enum_name(name) {
            println!("    📋 Enum: {}", enum_name);
        }
    }

    println!("\n📝 String/Constant Symbols:");
    for (idx, name, size) in string_symbols.iter().take(15) {
        println!("  {}: {} | size: {} bytes", idx, name, size);
    }

    // Look for enum discriminant patterns in rodata
    if let Some(rodata) = constant_data.get(".rodata") {
        println!("\n🔢 Analyzing .rodata for enum patterns...");
        analyze_enum_patterns(rodata);
    }

    Ok(())
}

fn extract_enum_name(mangled: &str) -> Option<String> {
    // Simple enum name extraction from Rust mangled names
    if mangled.contains("_$LT$") && mangled.contains("$GT$") {
        if let Some(start) = mangled.find("_$LT$") {
            if let Some(end) = mangled.find("$GT$") {
                let inner = &mangled[start + 5..end];
                if let Some(enum_part) = inner.split("..").next() {
                    return Some(enum_part.replace("$u20$", " "));
                }
            }
        }
    }

    // Look for common enum patterns
    if mangled.contains("Kind") || mangled.contains("Type") || mangled.contains("Error") {
        let parts: Vec<&str> = mangled.split("::").collect();
        for part in parts {
            if part.contains("Kind") || part.contains("Type") || part.contains("Error") {
                return Some(part.to_string());
            }
        }
    }

    None
}

fn analyze_enum_patterns(data: &[u8]) {
    let mut enum_candidates = Vec::new();

    // Look for sequential integer patterns (enum discriminants)
    for i in 0..data.len().saturating_sub(16) {
        if i % 4 == 0 {
            // Align to 4-byte boundaries
            let vals: Vec<u32> = (0..4)
                .map(|j| {
                    u32::from_le_bytes([
                        data[i + j * 4],
                        data[i + j * 4 + 1],
                        data[i + j * 4 + 2],
                        data[i + j * 4 + 3],
                    ])
                })
                .collect();

            // Check for sequential pattern (0, 1, 2, 3...)
            if vals[0] == 0 && vals[1] == 1 && vals[2] == 2 && vals[3] == 3 {
                enum_candidates.push((i, vals));
            }
        }
    }

    println!("  🎯 Found {} potential enum discriminant sequences", enum_candidates.len());
    for (offset, vals) in enum_candidates.iter().take(5) {
        println!("    Offset 0x{:x}: {:?}", offset, vals);
    }
}
