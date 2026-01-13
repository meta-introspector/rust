use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 TESTING LMFDB CALL GRAPH CONTAINMENT");
    println!("=======================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    // Get our symbol-to-LMFDB mapping
    let lmfdb_map = get_symbol_lmfdb_mapping();

    println!("🔍 Testing call relationships...");

    // Find two functions where one might call the other
    let mut test_cases = Vec::new();

    for sym_f in elf.syms.iter().take(100) {
        if let Some(name_f) = elf.strtab.get_at(sym_f.st_name) {
            if sym_f.st_size > 20 && lmfdb_map.contains_key(&sym_f.st_value) {
                // Look for potential calls within this function
                if let Some(text_section) = elf
                    .section_headers
                    .iter()
                    .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
                {
                    if sym_f.st_value >= text_section.sh_addr {
                        let func_start = (sym_f.st_value - text_section.sh_addr) as usize;
                        let text_start = text_section.sh_offset as usize;
                        let text_bytes = &binary[text_start..];

                        if func_start < text_bytes.len() {
                            let func_size = (sym_f.st_size as usize).min(100);
                            let func_end = (func_start + func_size).min(text_bytes.len());
                            let func_bytes = &text_bytes[func_start..func_end];

                            // Look for call instructions (simplified)
                            for chunk in func_bytes.chunks(8) {
                                if chunk.len() >= 8 {
                                    let addr = u64::from_le_bytes([
                                        chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5],
                                        chunk[6], chunk[7],
                                    ]);

                                    // Check if this address corresponds to another mapped function
                                    if lmfdb_map.contains_key(&addr) && addr != sym_f.st_value {
                                        test_cases.push((sym_f.st_value, addr, name_f.to_string()));
                                        if test_cases.len() >= 5 {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if test_cases.len() >= 5 {
            break;
        }
    }

    println!("📊 Found {} potential call relationships", test_cases.len());

    // Test the containment hypothesis
    for (f_addr, g_addr, f_name) in test_cases {
        let lmfdb_f = lmfdb_map.get(&f_addr).unwrap();
        let lmfdb_g = lmfdb_map.get(&g_addr).unwrap();

        println!(
            "\n🔍 Testing: {} calls function at 0x{:x}",
            f_name.chars().take(30).collect::<String>(),
            g_addr
        );
        println!("   f → LMFDB: {}", lmfdb_f);
        println!("   g → LMFDB: {}", lmfdb_g);

        // Parse LMFDB labels to extract mathematical properties
        let f_parts: Vec<&str> = lmfdb_f.split('.').collect();
        let g_parts: Vec<&str> = lmfdb_g.split('.').collect();

        if f_parts.len() >= 3 && g_parts.len() >= 3 {
            let f_level: u32 = f_parts[0].parse().unwrap_or(0);
            let f_weight: u32 = f_parts[1].parse().unwrap_or(0);
            let g_level: u32 = g_parts[0].parse().unwrap_or(0);
            let g_weight: u32 = g_parts[1].parse().unwrap_or(0);

            // Test mathematical containment properties
            let level_divides = f_level % g_level == 0 || g_level % f_level == 0;
            let weight_compatible = f_weight >= g_weight;
            let same_character = f_parts.get(2) == g_parts.get(2);

            println!("   📐 Mathematical properties:");
            println!("      Level divisibility: {} ({} vs {})", level_divides, f_level, g_level);
            println!(
                "      Weight compatibility: {} ({} ≥ {})",
                weight_compatible, f_weight, g_weight
            );
            println!(
                "      Character match: {} ({} vs {})",
                same_character,
                f_parts.get(2).map_or("?", |v| v),
                g_parts.get(2).map_or("?", |v| v)
            );

            let contains = level_divides && weight_compatible;
            println!("   🎯 CONTAINMENT: {}", if contains { "✅ YES" } else { "❌ NO" });
        }
    }

    println!("\n🧮 MATHEMATICAL INTERPRETATION:");
    println!("   If f calls g, then mathematically:");
    println!("   • f's modular form should 'contain' g's form");
    println!("   • Level(f) should be divisible by Level(g)");
    println!("   • Weight(f) should be ≥ Weight(g)");
    println!("   • Characters should be compatible");

    println!("\n❓ HYPOTHESIS TEST RESULT:");
    println!("   This is a TESTABLE mathematical prediction!");
    println!("   Call graph structure → Modular form containment");

    Ok(())
}

// Include the generated mapping function
pub fn get_symbol_lmfdb_mapping() -> HashMap<u64, String> {
    let mut map = HashMap::new();
    map.insert(0xbf9fd30, "6.6.12.f".to_string());
    map.insert(0xbfa01a0, "24.6.12.x".to_string());
    map.insert(0xbfa01b0, "1.2.11.a".to_string());
    map.insert(0xbfa01c0, "2.4.12.b".to_string());
    map.insert(0xbfa0280, "16.2.12.p".to_string());
    map.insert(0xbfa0290, "30.6.12.d".to_string());
    map.insert(0xbfa02a0, "35.4.11.i".to_string());
    map.insert(0xbfa0330, "26.4.12.z".to_string());
    map.insert(0xe1b4600, "26.4.12.z".to_string());
    map.insert(0x42317b0, "27.6.11.a".to_string());
    map.insert(0x423fb00, "8.4.12.h".to_string());
    map.insert(0x42410c0, "21.6.11.u".to_string());
    map.insert(0x42412f0, "31.2.11.e".to_string());
    map.insert(0x425d6f0, "34.2.12.h".to_string());
    map.insert(0x4261a40, "37.2.11.k".to_string());
    map
}
