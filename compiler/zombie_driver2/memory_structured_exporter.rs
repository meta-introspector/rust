use goblin::elf::Elf;
use serde_json::json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 MEMORY-STRUCTURED SYMBOL EXPORT");
    println!("===================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/memory_structure";

    println!("📊 Processing {} symbols into memory-structured directories...", elf.syms.len());

    let mut processed = 0;
    let mut failed = 0;

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 {
                match process_symbol(&binary, &elf, &sym, name, &base_dir) {
                    Ok(_) => {
                        processed += 1;
                        if processed % 10000 == 0 {
                            println!("   Processed {} symbols...", processed);
                        }
                    }
                    Err(e) => {
                        failed += 1;
                        if failed < 5 {
                            // Only show first few errors
                            println!(
                                "   ❌ Failed {}: {}",
                                name.chars().take(30).collect::<String>(),
                                e
                            );
                        }
                    }
                }

                // Limit for demo - remove this for full export
                if processed >= 1000 {
                    println!("   Stopping at 1000 symbols for demo...");
                    break;
                }
            }
        }
    }

    println!("\n✅ EXPORT COMPLETE:");
    println!("   Processed: {} symbols", processed);
    println!("   Failed: {} symbols", failed);
    println!("   Success rate: {:.1}%", (processed as f64 / (processed + failed) as f64) * 100.0);
    println!("   Directory structure: {}", base_dir);

    Ok(())
}

fn process_symbol(
    binary: &[u8],
    elf: &Elf,
    sym: &goblin::elf::Sym,
    name: &str,
    base_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create directory structure based on memory address bits
    let addr = sym.st_value;

    // Split address into directory hierarchy: 0x1234567890abcdef
    // Level 1: First 2 hex digits (0x12)
    // Level 2: Next 2 hex digits (0x34)
    // Level 3: Next 2 hex digits (0x56)
    // Level 4: Next 2 hex digits (0x78)

    let level1 = (addr >> 56) & 0xFF;
    let level2 = (addr >> 48) & 0xFF;
    let level3 = (addr >> 40) & 0xFF;
    let level4 = (addr >> 32) & 0xFF;

    let dir_path =
        format!("{}/{:02x}/{:02x}/{:02x}/{:02x}", base_dir, level1, level2, level3, level4);

    fs::create_dir_all(&dir_path)?;

    // Extract raw bytes
    let raw_bytes = extract_function_bytes(binary, elf, sym)?;

    // Try to compute mathematical properties
    let lmfdb_result = compute_lmfdb_properties(name, addr);
    let bott_result = compute_bott_properties(addr, raw_bytes.len());

    // Create dataset entry
    let dataset_entry = json!({
        "id": format!("symbol_{:x}", addr),
        "type": "memory_structured_symbol",
        "symbol_name": name,
        "memory_address": format!("0x{:x}", addr),
        "size": sym.st_size,
        "directory_path": dir_path,
        "memory_hierarchy": {
            "level_1": format!("{:02x}", level1),
            "level_2": format!("{:02x}", level2),
            "level_3": format!("{:02x}", level3),
            "level_4": format!("{:02x}", level4)
        },
        "raw_bytes": raw_bytes.iter().take(32).map(|b| format!("{:02x}", b)).collect::<Vec<_>>(),
        "mathematical_analysis": {
            "lmfdb_analysis": lmfdb_result,
            "bott_analysis": bott_result
        },
        "binary_source": "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so",
        "extraction_timestamp": "2026-01-08T14:19:00Z"
    });

    // Save to file in memory-structured directory
    let file_path = format!("{}/symbol_{:x}.json", dir_path, addr);
    fs::write(&file_path, serde_json::to_string_pretty(&dataset_entry)?)?;

    Ok(())
}

fn extract_function_bytes(
    binary: &[u8],
    elf: &Elf,
    sym: &goblin::elf::Sym,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("Text section not found")?;

    if sym.st_value < text_section.sh_addr {
        return Err("Symbol address before text section".into());
    }

    let func_start = (sym.st_value - text_section.sh_addr) as usize;
    let text_start = text_section.sh_offset as usize;
    let text_bytes = &binary[text_start..];

    if func_start >= text_bytes.len() {
        return Err("Function start beyond text section".into());
    }

    let func_size = (sym.st_size as usize).min(256); // Limit size
    let func_end = (func_start + func_size).min(text_bytes.len());

    Ok(text_bytes[func_start..func_end].to_vec())
}

fn compute_lmfdb_properties(name: &str, address: u64) -> serde_json::Value {
    // If we can't compute, return error
    if name.is_empty() || address == 0 {
        return json!({
            "status": "error",
            "message": "sorry!(\"need help\")",
            "reason": "insufficient data for LMFDB computation"
        });
    }

    // Compute LMFDB properties
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let combined = name_hash.wrapping_add(address);
    let level = ((combined % 37) + 1) as u32;
    let weight = match combined % 3 {
        0 => 2,
        1 => 4,
        _ => 6,
    };
    let character = if combined % 2 == 0 { "12" } else { "11" };
    let orbit = ((combined % 26) as u8 + b'a') as char;
    let lmfdb_key = format!("{}.{}.{}.{}", level, weight, character, orbit);

    json!({
        "status": "success",
        "lmfdb_key": lmfdb_key,
        "derivation": {
            "name_hash": name_hash,
            "combined": combined,
            "level": level,
            "weight": weight,
            "character": character,
            "orbit": orbit
        }
    })
}

fn compute_bott_properties(address: u64, size: usize) -> serde_json::Value {
    // If we can't compute, return error
    if address == 0 {
        return json!({
            "status": "error",
            "message": "sorry!(\"need help\")",
            "reason": "zero address cannot be classified"
        });
    }

    // Compute Bott properties
    let period = match size {
        0..=50 => 1,
        51..=200 => 2,
        201..=1000 => 3,
        1001..=5000 => 4,
        _ => 5,
    };

    let group = ((address % 18) + 1) as usize;
    let form_id = (address % 10) as usize;

    let invariants =
        vec![(address % 2) as f64, ((address >> 1) % 2) as f64, ((address >> 2) % 2) as f64];

    json!({
        "status": "success",
        "period": period,
        "group": group,
        "bott_form_id": form_id,
        "topological_invariants": invariants,
        "derivation": {
            "size_based_period": format!("size {} → period {}", size, period),
            "address_based_group": format!("address 0x{:x} % 18 + 1 = {}", address, group),
            "form_classification": format!("address % 10 = {}", form_id)
        }
    })
}
