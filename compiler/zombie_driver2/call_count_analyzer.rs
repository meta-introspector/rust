use goblin::elf::Elf;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 CALL COUNT 0 OR 1 ANALYSIS");
    println!("==============================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/call_count_analysis";
    fs::create_dir_all(&base_dir)?;

    println!("📊 Building call frequency map...");
    let (call_graph, call_frequency) = build_call_graph_with_counts(&binary, &elf)?;
    let lmfdb_map = compute_all_lmfdb_keys(&elf);

    println!("🔍 Analyzing call counts...");
    let mut zero_calls = Vec::new();
    let mut one_call = Vec::new();
    let mut all_functions = Vec::new();

    // Collect all functions with their call counts
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 && !name.is_empty() {
                let call_count = call_frequency.get(&sym.st_value).copied().unwrap_or(0);
                let lmfdb_key = lmfdb_map.get(&sym.st_value).cloned().unwrap_or_default();

                let func_data = json!({
                    "address": format!("0x{:x}", sym.st_value),
                    "name": name,
                    "demangled": demangle_name(name),
                    "call_count": call_count,
                    "lmfdb_key": lmfdb_key,
                    "size": sym.st_size,
                    "makes_calls": call_graph.contains_key(&sym.st_value)
                });

                match call_count {
                    0 => zero_calls.push(func_data.clone()),
                    1 => one_call.push(func_data.clone()),
                    _ => {}
                }

                all_functions.push((call_count, func_data));
            }
        }
    }

    // Sort by call count
    all_functions.sort_by_key(|(count, _)| *count);

    println!("📈 Call count distribution:");
    println!("   Functions with 0 calls: {}", zero_calls.len());
    println!("   Functions with 1 call: {}", one_call.len());
    println!("   Total functions: {}", all_functions.len());

    // Export analysis
    let analysis = json!({
        "type": "call_count_analysis",
        "description": "Analysis of functions by call frequency, focusing on entry points (0 calls) and rarely called functions (1 call)",
        "statistics": {
            "zero_calls": zero_calls.len(),
            "one_call": one_call.len(),
            "total_functions": all_functions.len(),
            "zero_call_percentage": (zero_calls.len() as f64 / all_functions.len() as f64) * 100.0,
            "one_call_percentage": (one_call.len() as f64 / all_functions.len() as f64) * 100.0
        },
        "functions_with_zero_calls": zero_calls,
        "functions_with_one_call": one_call,
        "call_count_distribution": {
            "0": zero_calls.len(),
            "1": one_call.len(),
            "2+": all_functions.len() - zero_calls.len() - one_call.len()
        },
        "extraction_timestamp": "2026-01-08T19:54:00Z"
    });

    let analysis_file = format!("{}/call_count_0_or_1_analysis.json", base_dir);
    fs::write(&analysis_file, serde_json::to_string_pretty(&analysis)?)?;

    // Show samples
    println!("\n🎯 SAMPLE FUNCTIONS WITH 0 CALLS (Entry Points):");
    for (i, func) in zero_calls.iter().take(10).enumerate() {
        println!(
            "   #{}: {} - {}",
            i + 1,
            func["address"].as_str().unwrap(),
            func["name"].as_str().unwrap().chars().take(60).collect::<String>()
        );
    }

    println!("\n🎯 SAMPLE FUNCTIONS WITH 1 CALL (Rarely Used):");
    for (i, func) in one_call.iter().take(10).enumerate() {
        println!(
            "   #{}: {} - {}",
            i + 1,
            func["address"].as_str().unwrap(),
            func["name"].as_str().unwrap().chars().take(60).collect::<String>()
        );
    }

    println!("\n✅ CALL COUNT ANALYSIS COMPLETE:");
    println!(
        "   Zero calls: {} ({:.1}%)",
        zero_calls.len(),
        (zero_calls.len() as f64 / all_functions.len() as f64) * 100.0
    );
    println!(
        "   One call: {} ({:.1}%)",
        one_call.len(),
        (one_call.len() as f64 / all_functions.len() as f64) * 100.0
    );
    println!("   Export file: {}", analysis_file);

    Ok(())
}

fn build_call_graph_with_counts(
    binary: &[u8],
    elf: &Elf,
) -> Result<(HashMap<u64, Vec<u64>>, HashMap<u64, u32>), Box<dyn std::error::Error>> {
    let mut call_graph = HashMap::new();
    let mut call_frequency = HashMap::new();

    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .ok_or("Text section not found")?;

    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_addr = text_section.sh_addr;

    if text_start + text_size > binary.len() {
        return Ok((call_graph, call_frequency));
    }

    let text_data = &binary[text_start..text_start + text_size];

    for (i, window) in text_data.windows(5).enumerate() {
        if window[0] == 0xe8 {
            let rel_offset = i32::from_le_bytes([window[1], window[2], window[3], window[4]]);
            let call_addr = text_addr + i as u64 + 5;
            let target_addr = (call_addr as i64 + rel_offset as i64) as u64;

            call_graph.entry(call_addr).or_insert_with(Vec::new).push(target_addr);
            *call_frequency.entry(target_addr).or_insert(0) += 1;
        }
    }

    Ok((call_graph, call_frequency))
}

fn compute_all_lmfdb_keys(elf: &Elf) -> HashMap<u64, String> {
    let mut lmfdb_map = HashMap::new();

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 && !name.is_empty() {
                let lmfdb_key = compute_lmfdb_key(name, sym.st_value);
                lmfdb_map.insert(sym.st_value, lmfdb_key);
            }
        }
    }

    lmfdb_map
}

fn compute_lmfdb_key(name: &str, address: u64) -> String {
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

    format!("{}.{}.{}.{}", level, weight, character, orbit)
}

fn demangle_name(name: &str) -> String {
    if name.starts_with("_ZN") {
        name.replace("_ZN", "").replace("E", "::").chars().take(100).collect()
    } else {
        name.to_string()
    }
}
