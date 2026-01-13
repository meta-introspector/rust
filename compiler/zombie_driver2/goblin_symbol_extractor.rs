use goblin::elf::Elf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 GOBLIN SYMBOL EXTRACTOR FOR PARSER FUNCTIONS");
    println!("===============================================");

    let so_files = vec![
        "target/debug/libsyn_analyzer.so",
        "target/release/libsyn_analyzer.so",
        "target/debug/deps/libsyn_analyzer.so",
        "target/release/deps/libsyn_analyzer.so",
    ];

    for so_file in so_files {
        if let Ok(buffer) = fs::read(so_file) {
            println!("📦 Analyzing: {}", so_file);

            match Elf::parse(&buffer) {
                Ok(elf) => {
                    println!("✅ ELF parsed successfully");

                    // Extract all symbol names
                    let mut all_symbols = Vec::new();

                    // Check dynamic symbols
                    for sym in &elf.dynsyms {
                        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                            all_symbols.push(name.to_string());
                        }
                    }

                    // Check regular symbols
                    for sym in &elf.syms {
                        if let Some(name) = elf.strtab.get_at(sym.st_name) {
                            all_symbols.push(name.to_string());
                        }
                    }

                    println!("📊 Found {} total symbols", all_symbols.len());

                    // Filter for parser-related symbols
                    let parser_symbols: Vec<String> = all_symbols
                        .into_iter()
                        .filter(|name| {
                            name.contains("parse")
                                || name.contains("Parse")
                                || name.contains("parser")
                                || name.contains("Parser")
                                || name.contains("rustc_parse")
                                || name.contains("tokenstream")
                                || name.contains("lexer")
                        })
                        .collect();

                    println!("🎯 Found {} parser-related symbols:", parser_symbols.len());
                    for (i, symbol) in parser_symbols.iter().enumerate() {
                        println!("  {}: {}", i + 1, symbol);
                    }

                    // Save to file
                    let output_file = format!(
                        "{}_parser_symbols.txt",
                        so_file.replace("/", "_").replace(".", "_")
                    );
                    fs::write(&output_file, parser_symbols.join("\n"))?;
                    println!("💾 Saved to: {}", output_file);

                    break; // Use first successful file
                }
                Err(e) => {
                    println!("❌ Failed to parse ELF: {}", e);
                }
            }
        } else {
            println!("❌ Could not read: {}", so_file);
        }
    }

    Ok(())
}
