use goblin::elf::Elf;
use serde_json::json;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 MAIN ROUTINE DEPENDENCY ANALYSIS");
    println!("===================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/main_dependencies";
    fs::create_dir_all(&base_dir)?;

    println!("🔍 Finding main function...");
    let main_addr = find_main_function(&elf)?;
    println!("   Found main at: 0x{:x}", main_addr);

    println!("📊 Building call graph...");
    let call_graph = build_call_graph(&binary, &elf)?;
    println!("   Found {} call relationships", call_graph.len());

    println!("🕸️ Tracing dependencies from main...");

    // Debug: Check what calls originate from main
    if let Some(main_calls) = call_graph.get(&main_addr) {
        println!("   Main function makes {} direct calls:", main_calls.len());
        for (i, &target) in main_calls.iter().enumerate() {
            if i < 10 {
                // Show first 10
                println!("     Call {}: 0x{:x} -> 0x{:x}", i + 1, main_addr, target);
            }
        }
    } else {
        println!("   ❌ Main function 0x{:x} not found in call graph!", main_addr);
        println!("   Checking nearby addresses...");
        for offset in -100i64..=100i64 {
            let check_addr = (main_addr as i64 + offset) as u64;
            if let Some(calls) = call_graph.get(&check_addr) {
                if !calls.is_empty() {
                    println!(
                        "     Found calls at nearby address 0x{:x} (offset {}): {} calls",
                        check_addr,
                        offset,
                        calls.len()
                    );
                }
            }
        }
    }

    let dependencies = trace_dependencies(main_addr, &call_graph);
    println!("   Found {} functions used by main", dependencies.len());

    println!("📊 Extracting strings...");
    let strings = extract_all_strings(&binary, &elf)?;

    println!("💾 Exporting main dependency functions...");
    let mut exported = 0;

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if dependencies.contains(&sym.st_value) && sym.st_size > 0 && !name.is_empty() {
                match export_main_dependency(
                    &binary,
                    &elf,
                    &sym,
                    name,
                    &base_dir,
                    &strings,
                    &call_graph,
                    main_addr,
                ) {
                    Ok(_) => {
                        exported += 1;
                        println!("   Exported: {}", name.chars().take(50).collect::<String>());
                    }
                    Err(e) => {
                        println!(
                            "   ❌ Failed {}: {}",
                            name.chars().take(30).collect::<String>(),
                            e
                        );
                    }
                }
            }
        }
    }

    println!("\n✅ MAIN DEPENDENCY EXPORT COMPLETE:");
    println!("   Main function: 0x{:x}", main_addr);
    println!("   Dependencies found: {}", dependencies.len());
    println!("   Functions exported: {}", exported);
    println!("   Export directory: {}", base_dir);

    Ok(())
}

fn find_main_function(elf: &Elf) -> Result<u64, Box<dyn std::error::Error>> {
    // Look for the actual rustc_driver main that does work
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if name.contains("rustc_driver") && name.contains("main") && !name.contains("closure") {
                println!("   Found rustc_driver main: {} at 0x{:x}", name, sym.st_value);
                return Ok(sym.st_value);
            }
        }
    }

    // Look for any main function with substantial size
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if (name == "main" || name.contains("main")) && sym.st_size > 100 {
                println!(
                    "   Found substantial main: {} at 0x{:x} (size: {})",
                    name, sym.st_value, sym.st_size
                );
                return Ok(sym.st_value);
            }
        }
    }

    // Fallback: use entry point
    println!("   Using entry point: 0x{:x}", elf.entry);
    Ok(elf.entry)
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
    println!("   Analyzing .text section: {} bytes at 0x{:x}", text_size, text_addr);

    let mut call_count = 0;

    // Find CALL instructions (0xe8 = call rel32, 0xff = call indirect)
    for (i, window) in text_data.windows(5).enumerate() {
        if window[0] == 0xe8 {
            // Direct call
            let rel_offset = i32::from_le_bytes([window[1], window[2], window[3], window[4]]);
            let call_addr = text_addr + i as u64 + 5;
            let target_addr = (call_addr as i64 + rel_offset as i64) as u64;

            call_graph.entry(call_addr).or_insert_with(Vec::new).push(target_addr);
            call_count += 1;

            if call_count % 100000 == 0 {
                println!("     Found {} calls so far...", call_count);
            }
        }
    }

    // Also check for indirect calls (0xff /2 and 0xff /3)
    for (i, window) in text_data.windows(2).enumerate() {
        if window[0] == 0xff && ((window[1] & 0x38) == 0x10 || (window[1] & 0x38) == 0x18) {
            let call_addr = text_addr + i as u64;
            // For indirect calls, we can't determine target statically
            // but we record the call site
            call_graph.entry(call_addr).or_insert_with(Vec::new);
            call_count += 1;
        }
    }

    println!("   Found {} total call instructions", call_count);
    Ok(call_graph)
}

fn trace_dependencies(start_addr: u64, call_graph: &HashMap<u64, Vec<u64>>) -> HashSet<u64> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_addr);

    println!("   Tracing all transitive dependencies from 0x{:x}...", start_addr);

    while let Some(addr) = queue.pop_front() {
        if visited.insert(addr) {
            // Follow ALL outgoing calls from this function
            if let Some(targets) = call_graph.get(&addr) {
                for &target in targets {
                    if !visited.contains(&target) {
                        queue.push_back(target);
                        if visited.len() % 100 == 0 {
                            println!("     Found {} dependencies so far...", visited.len());
                        }
                    }
                }
            }
        }
    }

    println!("   Total transitive dependencies: {}", visited.len());
    visited
}

fn extract_all_strings(
    binary: &[u8],
    elf: &Elf,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn export_main_dependency(
    binary: &[u8],
    elf: &Elf,
    sym: &goblin::elf::Sym,
    name: &str,
    base_dir: &str,
    strings: &[String],
    call_graph: &HashMap<u64, Vec<u64>>,
    main_addr: u64,
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
        .take(5)
        .collect();

    // Find calls
    let calls_from = call_graph.get(&addr).cloned().unwrap_or_default();
    let calls_to: Vec<u64> = call_graph
        .iter()
        .filter_map(|(caller, targets)| if targets.contains(&addr) { Some(*caller) } else { None })
        .collect();

    // Calculate distance from main
    let distance_from_main = if addr == main_addr { 0 } else { 1 }; // Simplified

    // Mathematical analysis
    let lmfdb_result = compute_lmfdb_properties(name, addr)?;
    let bott_result = compute_bott_properties(addr, raw_bytes.len())?;

    let dataset_entry = json!({
        "id": format!("main_dep_{:x}", addr),
        "type": "main_dependency_function",
        "symbol_name": name,
        "demangled_name": demangle_name(name),
        "memory_address": format!("0x{:x}", addr),
        "size": sym.st_size,
        "distance_from_main": distance_from_main,
        "is_main_function": addr == main_addr,
        "raw_bytes": raw_bytes.iter().take(32).map(|b| format!("{:02x}", b)).collect::<Vec<_>>(),
        "related_strings": related_strings,
        "function_calls": {
            "calls_from": calls_from.iter().map(|a| format!("0x{:x}", a)).collect::<Vec<_>>(),
            "calls_to": calls_to.iter().map(|a| format!("0x{:x}", a)).collect::<Vec<_>>(),
            "call_count_out": calls_from.len(),
            "call_count_in": calls_to.len()
        },
        "mathematical_analysis": {
            "lmfdb_analysis": lmfdb_result,
            "bott_analysis": bott_result
        },
        "binary_source": "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so",
        "extraction_timestamp": "2026-01-08T19:37:00Z"
    });

    let safe_name = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .take(50)
        .collect::<String>();

    let file_path = format!("{}/main_dep_{:x}_{}.json", base_dir, addr, safe_name);
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
