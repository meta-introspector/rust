use goblin::elf::Elf;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::symlink;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 CHUNKED MEMORY + ALPHABETICAL INDEX EXPORT");
    println!("==============================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/chunked_memory";
    let index_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/alphabetical_index";

    fs::create_dir_all(&base_dir)?;
    fs::create_dir_all(&index_dir)?;

    println!("📊 Processing {} symbols into chunked memory directories...", elf.syms.len());

    let mut processed = 0;
    let mut failed = 0;
    let mut name_to_path: HashMap<String, String> = HashMap::new();

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 && !name.is_empty() {
                match process_symbol(&binary, &elf, &sym, name, &base_dir) {
                    Ok(file_path) => {
                        processed += 1;
                        name_to_path.insert(name.to_string(), file_path);

                        if processed % 5000 == 0 {
                            println!("   Processed {} symbols...", processed);
                        }
                    }
                    Err(e) => {
                        failed += 1;
                        if failed < 5 {
                            println!(
                                "   ❌ Failed {}: {}",
                                name.chars().take(30).collect::<String>(),
                                e
                            );
                        }
                    }
                }

                // Process more symbols for better demo
                if processed >= 10000 {
                    println!("   Stopping at 10,000 symbols for demo...");
                    break;
                }
            }
        }
    }

    println!("\n📚 Creating alphabetical index with symlinks...");
    create_alphabetical_index(&name_to_path, &index_dir)?;

    println!("\n✅ EXPORT COMPLETE:");
    println!("   Processed: {} symbols", processed);
    println!("   Failed: {} symbols", failed);
    println!("   Success rate: {:.1}%", (processed as f64 / (processed + failed) as f64) * 100.0);
    println!("   Memory structure: {}", base_dir);
    println!("   Alphabetical index: {}", index_dir);

    Ok(())
}

fn process_symbol(
    binary: &[u8],
    elf: &Elf,
    sym: &goblin::elf::Sym,
    name: &str,
    base_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let addr = sym.st_value;

    // Chunk addresses into 1MB ranges (0x100000 = 1MB)
    let chunk = addr / 0x100000;
    let chunk_dir = format!("{}/chunk_{:08x}", base_dir, chunk);
    fs::create_dir_all(&chunk_dir)?;

    // Extract raw bytes
    let raw_bytes = extract_function_bytes(binary, elf, sym)?;

    // Compute mathematical properties - use sorry!() when we can't
    let lmfdb_result = match compute_lmfdb_properties(name, addr) {
        Ok(result) => result,
        Err(_) => json!({
            "status": "error",
            "message": "sorry!(\"need help\")",
            "reason": "LMFDB computation failed"
        }),
    };

    let bott_result = match compute_bott_properties(addr, raw_bytes.len()) {
        Ok(result) => result,
        Err(_) => json!({
            "status": "error",
            "message": "sorry!(\"need help\")",
            "reason": "Bott computation failed"
        }),
    };

    // Create dataset entry with resolved name
    let dataset_entry = json!({
        "id": format!("symbol_{:x}", addr),
        "type": "chunked_memory_symbol",
        "symbol_name": name,
        "demangled_name": demangle_name(name),
        "memory_address": format!("0x{:x}", addr),
        "memory_chunk": format!("chunk_{:08x}", chunk),
        "size": sym.st_size,
        "raw_bytes": raw_bytes.iter().take(32).map(|b| format!("{:02x}", b)).collect::<Vec<_>>(),
        "mathematical_analysis": {
            "lmfdb_analysis": lmfdb_result,
            "bott_analysis": bott_result
        },
        "binary_source": "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so",
        "extraction_timestamp": "2026-01-08T19:20:00Z"
    });

    // Save with both address and name in filename
    let safe_name = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .take(50)
        .collect::<String>();

    let file_path = format!("{}/addr_{:x}_{}.json", chunk_dir, addr, safe_name);
    fs::write(&file_path, serde_json::to_string_pretty(&dataset_entry)?)?;

    Ok(file_path)
}

fn create_alphabetical_index(
    name_to_path: &HashMap<String, String>,
    index_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Group by first character
    let mut char_groups: HashMap<char, Vec<(&String, &String)>> = HashMap::new();

    for (name, path) in name_to_path {
        let first_char = name.chars().next().unwrap_or('_').to_ascii_lowercase();
        char_groups.entry(first_char).or_default().push((name, path));
    }

    // Create alphabetical directories and symlinks
    let group_count = char_groups.len();
    for (first_char, entries) in char_groups {
        let char_dir = format!("{}/{}", index_dir, first_char);
        fs::create_dir_all(&char_dir)?;

        for (name, original_path) in entries {
            let safe_name = name
                .chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .take(100)
                .collect::<String>();

            let link_path = format!("{}/{}.json", char_dir, safe_name);

            // Create relative symlink
            if let Ok(relative_path) = make_relative_path(&link_path, original_path) {
                let _ = symlink(&relative_path, &link_path); // Ignore errors for existing links
            }
        }
    }

    println!("   Created alphabetical index for {} character groups", group_count);
    Ok(())
}

fn make_relative_path(from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Simple relative path calculation
    let from_parts: Vec<&str> = from.split('/').collect();
    let to_parts: Vec<&str> = to.split('/').collect();

    // Count how many directories to go up
    let up_count = from_parts.len() - 1;
    let mut relative = vec![".."; up_count];
    relative.extend(to_parts);

    Ok(relative.join("/"))
}

fn demangle_name(name: &str) -> String {
    // Simple demangling - just remove common prefixes
    if name.starts_with("_ZN") {
        // Rust mangled name - try to extract readable parts
        name.replace("_ZN", "").replace("E", "::").chars().take(100).collect()
    } else {
        name.to_string()
    }
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

    let func_size = (sym.st_size as usize).min(256);
    let func_end = (func_start + func_size).min(text_bytes.len());

    Ok(text_bytes[func_start..func_end].to_vec())
}

fn compute_lmfdb_properties(
    name: &str,
    address: u64,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    if name.is_empty() || address == 0 {
        return Err("Insufficient data".into());
    }

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

    Ok(json!({
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
    }))
}

fn compute_bott_properties(
    address: u64,
    size: usize,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    if address == 0 {
        return Err("Zero address".into());
    }

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

    Ok(json!({
        "status": "success",
        "period": period,
        "group": group,
        "bott_form_id": form_id,
        "topological_invariants": invariants
    }))
}
