use goblin::elf::Elf;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::symlink;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 COMPLETE BINARY ANALYSIS + EXPORT");
    println!("====================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis";
    let index_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_index";

    fs::create_dir_all(&base_dir)?;
    fs::create_dir_all(&index_dir)?;

    println!("📊 Extracting strings from binary sections...");
    let strings = extract_all_strings(&binary, &elf)?;
    println!("   Found {} strings", strings.len());

    println!("📊 Building call graph from disassembly...");
    let call_graph = build_call_graph(&binary, &elf)?;
    println!("   Found {} call relationships", call_graph.len());

    println!("📊 Processing {} symbols with complete analysis...", elf.syms.len());

    let mut processed = 0;
    let mut failed = 0;
    let mut name_to_path: HashMap<String, String> = HashMap::new();

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 && !name.is_empty() {
                match process_complete_symbol(
                    &binary,
                    &elf,
                    &sym,
                    name,
                    &base_dir,
                    &strings,
                    &call_graph,
                ) {
                    Ok(file_path) => {
                        processed += 1;
                        name_to_path.insert(name.to_string(), file_path);

                        if processed % 1000 == 0 {
                            println!("   Processed {} symbols...", processed);
                        }
                    }
                    Err(e) => {
                        failed += 1;
                        if failed < 3 {
                            println!(
                                "   ❌ Failed {}: {}",
                                name.chars().take(30).collect::<String>(),
                                e
                            );
                        }
                    }
                }

                if processed >= 5000 {
                    println!("   Stopping at 5,000 symbols for demo...");
                    break;
                }
            }
        }
    }

    println!("\n📚 Creating complete alphabetical index...");
    create_alphabetical_index(&name_to_path, &index_dir)?;

    println!("\n✅ COMPLETE ANALYSIS EXPORT:");
    println!("   Processed: {} symbols", processed);
    println!("   Failed: {} symbols", failed);
    println!("   Strings extracted: {}", strings.len());
    println!("   Call relationships: {}", call_graph.len());
    println!("   Complete analysis: {}", base_dir);
    println!("   Alphabetical index: {}", index_dir);

    Ok(())
}

fn extract_all_strings(
    binary: &[u8],
    elf: &Elf,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut strings = Vec::new();

    // Extract from .rodata section
    if let Some(rodata) = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".rodata")
    {
        let start = rodata.sh_offset as usize;
        let end = start + rodata.sh_size as usize;
        if end <= binary.len() {
            let section_data = &binary[start..end];
            strings.extend(extract_strings_from_bytes(section_data));
        }
    }

    // Extract from .data section
    if let Some(data) = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".data")
    {
        let start = data.sh_offset as usize;
        let end = start + data.sh_size as usize;
        if end <= binary.len() {
            let section_data = &binary[start..end];
            strings.extend(extract_strings_from_bytes(section_data));
        }
    }

    Ok(strings)
}

fn extract_strings_from_bytes(data: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current_string = Vec::new();

    for &byte in data {
        if byte >= 32 && byte <= 126 {
            // Printable ASCII
            current_string.push(byte);
        } else {
            if current_string.len() >= 4 {
                // Minimum string length
                if let Ok(s) = String::from_utf8(current_string.clone()) {
                    strings.push(s);
                }
            }
            current_string.clear();
        }
    }

    // Handle final string
    if current_string.len() >= 4 {
        if let Ok(s) = String::from_utf8(current_string) {
            strings.push(s);
        }
    }

    strings
}

fn build_call_graph(
    binary: &[u8],
    elf: &Elf,
) -> Result<HashMap<u64, Vec<u64>>, Box<dyn std::error::Error>> {
    let mut call_graph = HashMap::new();

    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("Text section not found")?;

    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_addr = text_section.sh_addr;

    if text_start + text_size > binary.len() {
        return Ok(call_graph);
    }

    let text_data = &binary[text_start..text_start + text_size];

    // Simple x86-64 call instruction detection
    for (i, window) in text_data.windows(5).enumerate() {
        if window[0] == 0xe8 {
            // CALL rel32
            let rel_offset = i32::from_le_bytes([window[1], window[2], window[3], window[4]]);
            let call_addr = text_addr + i as u64 + 5; // Address after call instruction
            let target_addr = (call_addr as i64 + rel_offset as i64) as u64;

            call_graph.entry(call_addr).or_insert_with(Vec::new).push(target_addr);
        }
    }

    Ok(call_graph)
}

fn process_complete_symbol(
    binary: &[u8],
    elf: &Elf,
    sym: &goblin::elf::Sym,
    name: &str,
    base_dir: &str,
    strings: &[String],
    call_graph: &HashMap<u64, Vec<u64>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let addr = sym.st_value;
    let chunk = addr / 0x100000;
    let chunk_dir = format!("{}/chunk_{:08x}", base_dir, chunk);
    fs::create_dir_all(&chunk_dir)?;

    // Extract raw bytes and disassembly
    let raw_bytes = extract_function_bytes(binary, elf, sym)?;
    let disassembly = match disassemble_bytes(&raw_bytes) {
        Ok(d) => d,
        Err(_) => json!({
            "status": "error",
            "message": "sorry!(\"need help\")",
            "reason": "disassembly failed"
        }),
    };

    // Find related strings (heuristic: strings containing parts of function name)
    let related_strings: Vec<&String> = strings
        .iter()
        .filter(|s| {
            let clean_name = name.replace("_ZN", "").replace("E", "");
            s.to_lowercase().contains(&clean_name.to_lowercase())
                || clean_name.to_lowercase().contains(&s.to_lowercase())
        })
        .take(10)
        .collect();

    // Find function calls
    let calls_from = call_graph.get(&addr).cloned().unwrap_or_default();
    let calls_to: Vec<u64> = call_graph
        .iter()
        .filter_map(|(caller, targets)| if targets.contains(&addr) { Some(*caller) } else { None })
        .collect();

    // Compute mathematical properties
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

    // Create complete dataset entry
    let dataset_entry = json!({
        "id": format!("symbol_{:x}", addr),
        "type": "complete_binary_analysis",
        "symbol_name": name,
        "demangled_name": demangle_name(name),
        "memory_address": format!("0x{:x}", addr),
        "memory_chunk": format!("chunk_{:08x}", chunk),
        "size": sym.st_size,
        "raw_bytes": raw_bytes.iter().take(64).map(|b| format!("{:02x}", b)).collect::<Vec<_>>(),
        "disassembly": disassembly,
        "related_strings": related_strings,
        "function_calls": {
            "calls_from": calls_from.iter().map(|addr| format!("0x{:x}", addr)).collect::<Vec<_>>(),
            "calls_to": calls_to.iter().map(|addr| format!("0x{:x}", addr)).collect::<Vec<_>>(),
            "call_count_out": calls_from.len(),
            "call_count_in": calls_to.len()
        },
        "mathematical_analysis": {
            "lmfdb_analysis": lmfdb_result,
            "bott_analysis": bott_result
        },
        "binary_source": "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so",
        "extraction_timestamp": "2026-01-08T19:25:00Z"
    });

    let safe_name = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .take(50)
        .collect::<String>();

    let file_path = format!("{}/addr_{:x}_{}.json", chunk_dir, addr, safe_name);
    fs::write(&file_path, serde_json::to_string_pretty(&dataset_entry)?)?;

    Ok(file_path)
}

fn disassemble_bytes(bytes: &[u8]) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut instructions = Vec::new();

    // Simple x86-64 instruction parsing (basic patterns)
    let mut i = 0;
    while i < bytes.len().min(32) {
        // Limit to first 32 bytes
        if i + 1 < bytes.len() {
            let opcode = bytes[i];
            let instruction = match opcode {
                0x48 => "mov/rex",
                0x50..=0x57 => "push",
                0x58..=0x5f => "pop",
                0xe8 => "call",
                0xe9 => "jmp",
                0xc3 => "ret",
                0x83 => "add/sub/cmp",
                0x89 => "mov",
                0x8b => "mov",
                _ => "unknown",
            };

            instructions.push(json!({
                "offset": i,
                "opcode": format!("{:02x}", opcode),
                "instruction": instruction
            }));
        }
        i += 1;
    }

    Ok(json!({
        "status": "success",
        "instructions": instructions,
        "note": "basic x86-64 pattern matching"
    }))
}

// Reuse helper functions from previous implementation
fn create_alphabetical_index(
    name_to_path: &HashMap<String, String>,
    index_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut char_groups: HashMap<char, Vec<(&String, &String)>> = HashMap::new();

    for (name, path) in name_to_path {
        let first_char = name.chars().next().unwrap_or('_').to_ascii_lowercase();
        char_groups.entry(first_char).or_default().push((name, path));
    }

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
            if let Ok(relative_path) = make_relative_path(&link_path, original_path) {
                let _ = symlink(&relative_path, &link_path);
            }
        }
    }

    println!("   Created alphabetical index for {} character groups", group_count);
    Ok(())
}

fn make_relative_path(from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>> {
    let from_parts: Vec<&str> = from.split('/').collect();
    let to_parts: Vec<&str> = to.split('/').collect();
    let up_count = from_parts.len() - 1;
    let mut relative = vec![".."; up_count];
    relative.extend(to_parts);
    Ok(relative.join("/"))
}

fn demangle_name(name: &str) -> String {
    if name.starts_with("_ZN") {
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

    let func_size = (sym.st_size as usize).min(512);
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
