use goblin::elf::Elf;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 TOP 10% MOST CALLED FUNCTIONS EXPORT");
    println!("=======================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/top_functions";
    fs::create_dir_all(&base_dir)?;

    println!("📊 Building call frequency map...");
    let call_frequency = build_call_frequency(&binary, &elf)?;
    println!("   Found {} unique call targets", call_frequency.len());

    println!("📊 Extracting strings...");
    let strings = extract_strings(&binary, &elf)?;

    println!("🏆 Finding top 10% most called functions...");
    let mut sorted_calls: Vec<_> = call_frequency.iter().collect();
    sorted_calls.sort_by(|a, b| b.1.cmp(a.1));

    let top_10_percent = (sorted_calls.len() as f64 * 0.1).ceil() as usize;
    let top_functions = &sorted_calls[..top_10_percent.min(sorted_calls.len())];

    println!("   Top 10% = {} functions", top_functions.len());
    for (i, (addr, count)) in top_functions.iter().take(10).enumerate() {
        println!("     #{}: 0x{:x} called {} times", i + 1, addr, count);
    }

    println!("💾 Exporting top functions...");
    let mut exported = 0;

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if let Some(&call_count) = call_frequency.get(&sym.st_value) {
                // Check if this function is in top 10%
                if top_functions.iter().any(|(addr, _)| **addr == sym.st_value) {
                    match export_top_function(
                        &binary, &elf, &sym, name, &base_dir, &strings, call_count,
                    ) {
                        Ok(_) => {
                            exported += 1;
                            if exported % 100 == 0 {
                                println!("   Exported {} functions...", exported);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }

    println!("\n✅ TOP FUNCTIONS EXPORT COMPLETE:");
    println!("   Total call targets: {}", call_frequency.len());
    println!("   Top 10% functions: {}", top_functions.len());
    println!("   Functions exported: {}", exported);
    println!("   Export directory: {}", base_dir);

    Ok(())
}

fn build_call_frequency(
    binary: &[u8],
    elf: &Elf,
) -> Result<HashMap<u64, u32>, Box<dyn std::error::Error>> {
    let mut frequency = HashMap::new();

    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("Text section not found")?;

    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_addr = text_section.sh_addr;

    if text_start + text_size > binary.len() {
        return Ok(frequency);
    }

    let text_data = &binary[text_start..text_start + text_size];

    // Count call targets
    for (i, window) in text_data.windows(5).enumerate() {
        if window[0] == 0xe8 {
            // Direct call
            let rel_offset = i32::from_le_bytes([window[1], window[2], window[3], window[4]]);
            let call_addr = text_addr + i as u64 + 5;
            let target_addr = (call_addr as i64 + rel_offset as i64) as u64;

            *frequency.entry(target_addr).or_insert(0) += 1;
        }
    }

    Ok(frequency)
}

fn extract_strings(binary: &[u8], elf: &Elf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut strings = Vec::new();

    for section_name in &[".rodata", ".data"] {
        if let Some(section) = elf
            .section_headers
            .iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == *section_name)
        {
            let start = section.sh_offset as usize;
            let end = start + section.sh_size as usize;
            if end <= binary.len() {
                let section_data = &binary[start..end];
                strings.extend(extract_strings_from_bytes(section_data));
            }
        }
    }

    Ok(strings)
}

fn extract_strings_from_bytes(data: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current_string = Vec::new();

    for &byte in data {
        if byte >= 32 && byte <= 126 {
            current_string.push(byte);
        } else {
            if current_string.len() >= 4 {
                if let Ok(s) = String::from_utf8(current_string.clone()) {
                    strings.push(s);
                }
            }
            current_string.clear();
        }
    }

    if current_string.len() >= 4 {
        if let Ok(s) = String::from_utf8(current_string) {
            strings.push(s);
        }
    }

    strings
}

fn export_top_function(
    binary: &[u8],
    elf: &Elf,
    sym: &goblin::elf::Sym,
    name: &str,
    base_dir: &str,
    strings: &[String],
    call_count: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = sym.st_value;

    // Extract function bytes
    let raw_bytes = extract_function_bytes(binary, elf, sym)?;

    // Find related strings
    let related_strings: Vec<&String> = strings
        .iter()
        .filter(|s| {
            let clean_name = name.replace("_ZN", "").replace("E", "");
            s.to_lowercase().contains(&clean_name.to_lowercase())
                || clean_name.to_lowercase().contains(&s.to_lowercase())
        })
        .take(3)
        .collect();

    // Mathematical analysis
    let lmfdb_result = compute_lmfdb_properties(name, addr)?;
    let bott_result = compute_bott_properties(addr, raw_bytes.len())?;

    let dataset_entry = json!({
        "id": format!("top_func_{:x}", addr),
        "type": "top_called_function",
        "symbol_name": name,
        "demangled_name": demangle_name(name),
        "memory_address": format!("0x{:x}", addr),
        "size": sym.st_size,
        "call_frequency": call_count,
        "popularity_rank": "top_10_percent",
        "raw_bytes": raw_bytes.iter().take(32).map(|b| format!("{:02x}", b)).collect::<Vec<_>>(),
        "related_strings": related_strings,
        "mathematical_analysis": {
            "lmfdb_analysis": lmfdb_result,
            "bott_analysis": bott_result
        },
        "binary_source": "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so",
        "extraction_timestamp": "2026-01-08T19:42:00Z"
    });

    let safe_name = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .take(50)
        .collect::<String>();

    let file_path =
        format!("{}/top_{:04}calls_{:x}_{}.json", base_dir, call_count, addr, safe_name);
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

    let func_size = (sym.st_size as usize).min(256);
    let func_end = (func_start + func_size).min(text_bytes.len());

    Ok(text_bytes[func_start..func_end].to_vec())
}

fn demangle_name(name: &str) -> String {
    if name.starts_with("_ZN") {
        name.replace("_ZN", "").replace("E", "::").chars().take(100).collect()
    } else {
        name.to_string()
    }
}

fn compute_lmfdb_properties(
    name: &str,
    address: u64,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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
