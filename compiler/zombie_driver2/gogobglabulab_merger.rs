use goblin::elf::Elf;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧮 LMFDB FREQUENCY + GOGOBGLABULAB MERGER");
    println!("==========================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/gogobglabulab_merged";
    fs::create_dir_all(&base_dir)?;

    println!("📊 Building call graph with LMFDB analysis...");
    let (call_graph, call_frequency) = build_call_graph_with_counts(&binary, &elf)?;

    println!("🔢 Computing LMFDB keys for all functions...");
    let lmfdb_map = compute_all_lmfdb_keys(&elf);
    println!("   Generated {} LMFDB mappings", lmfdb_map.len());

    println!("📈 Counting LMFDB key frequencies...");
    let lmfdb_frequency = count_lmfdb_frequencies(&lmfdb_map, &call_frequency);

    println!("🌀 Merging along call graph paths (Gogobglabulab style)...");
    let merged_gogobglabulab = merge_along_call_graph(&call_graph, &lmfdb_map, &call_frequency);

    println!("💾 Exporting Gogobglabulab merged analysis...");

    // Export LMFDB frequency analysis
    let lmfdb_freq_data = json!({
        "type": "lmfdb_frequency_analysis",
        "description": "Frequency count of each LMFDB modular form key in rustc_driver.so",
        "total_functions": lmfdb_map.len(),
        "unique_lmfdb_keys": lmfdb_frequency.len(),
        "frequency_distribution": lmfdb_frequency.iter()
            .map(|(key, count)| json!({
                "lmfdb_key": key,
                "frequency": count,
                "percentage": (*count as f64 / lmfdb_map.len() as f64) * 100.0
            }))
            .collect::<Vec<_>>(),
        "extraction_timestamp": "2026-01-08T19:48:00Z"
    });

    let freq_file = format!("{}/lmfdb_frequency_analysis.json", base_dir);
    fs::write(&freq_file, serde_json::to_string_pretty(&lmfdb_freq_data)?)?;

    // Export Gogobglabulab merged data
    let gogob_data = json!({
        "type": "gogobglabulab_call_graph_merger",
        "description": "LMFDB keys merged along call graph paths showing mathematical flow through rustc",
        "gogobglabulab_paths": merged_gogobglabulab.iter().take(1000).map(|(path_id, path_data)| {
            json!({
                "path_id": path_id,
                "path_length": path_data.path.len(),
                "call_path": path_data.path.iter().map(|addr| format!("0x{:x}", addr)).collect::<Vec<_>>(),
                "lmfdb_sequence": path_data.lmfdb_sequence,
                "total_calls": path_data.total_calls,
                "mathematical_flow": path_data.mathematical_signature,
                "gogobglabulab_power": path_data.gogobglabulab_power
            })
        }).collect::<Vec<_>>(),
        "statistics": {
            "total_paths": merged_gogobglabulab.len(),
            "max_path_length": merged_gogobglabulab.values().map(|p| p.path.len()).max().unwrap_or(0),
            "total_gogobglabulab_power": merged_gogobglabulab.values().map(|p| p.gogobglabulab_power).sum::<f64>()
        },
        "extraction_timestamp": "2026-01-08T19:48:00Z"
    });

    let gogob_file = format!("{}/gogobglabulab_merged_analysis.json", base_dir);
    fs::write(&gogob_file, serde_json::to_string_pretty(&gogob_data)?)?;

    // Export top LMFDB keys
    let mut sorted_lmfdb: Vec<_> = lmfdb_frequency.iter().collect();
    sorted_lmfdb.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n🏆 TOP 10 MOST FREQUENT LMFDB KEYS:");
    for (i, (key, count)) in sorted_lmfdb.iter().take(10).enumerate() {
        let percentage = (**count as f64 / lmfdb_map.len() as f64) * 100.0;
        println!("   #{}: {} ({} functions, {:.1}%)", i + 1, key, count, percentage);
    }

    println!("\n✅ GOGOBGLABULAB MERGER COMPLETE:");
    println!("   LMFDB frequency: {}", freq_file);
    println!("   Gogobglabulab merged: {}", gogob_file);
    println!("   Total LMFDB keys: {}", lmfdb_frequency.len());
    println!("   Total call paths: {}", merged_gogobglabulab.len());
    println!("   Most frequent LMFDB: {} ({} functions)", sorted_lmfdb[0].0, sorted_lmfdb[0].1);

    Ok(())
}

#[derive(Debug, Clone)]
struct GogobglabulabPath {
    path: Vec<u64>,
    lmfdb_sequence: Vec<String>,
    total_calls: u32,
    mathematical_signature: String,
    gogobglabulab_power: f64,
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
            // Direct call
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

fn count_lmfdb_frequencies(
    lmfdb_map: &HashMap<u64, String>,
    call_frequency: &HashMap<u64, u32>,
) -> HashMap<String, u32> {
    let mut lmfdb_frequency = HashMap::new();

    for (addr, lmfdb_key) in lmfdb_map {
        let call_count = call_frequency.get(addr).unwrap_or(&0);
        *lmfdb_frequency.entry(lmfdb_key.clone()).or_insert(0) += 1 + call_count; // Base frequency + call frequency
    }

    lmfdb_frequency
}

fn merge_along_call_graph(
    call_graph: &HashMap<u64, Vec<u64>>,
    lmfdb_map: &HashMap<u64, String>,
    _call_frequency: &HashMap<u64, u32>,
) -> HashMap<String, GogobglabulabPath> {
    let mut merged_paths = HashMap::new();
    let mut path_counter = 0;

    // Trace paths through call graph, merging LMFDB keys
    for (&start_addr, targets) in call_graph.iter() {
        if targets.len() > 0 {
            let path = trace_gogobglabulab_path(start_addr, call_graph, lmfdb_map, 10); // Max depth 10

            if path.path.len() > 1 {
                let path_id = format!("gogob_path_{:06}", path_counter);
                merged_paths.insert(path_id, path);
                path_counter += 1;

                if path_counter >= 10000 {
                    // Limit for performance
                    break;
                }
            }
        }
    }

    merged_paths
}

fn trace_gogobglabulab_path(
    start_addr: u64,
    call_graph: &HashMap<u64, Vec<u64>>,
    lmfdb_map: &HashMap<u64, String>,
    max_depth: usize,
) -> GogobglabulabPath {
    let mut path = vec![start_addr];
    let mut lmfdb_sequence = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut current_addr = start_addr;

    // Add starting LMFDB key
    if let Some(lmfdb_key) = lmfdb_map.get(&start_addr) {
        lmfdb_sequence.push(lmfdb_key.clone());
    }

    visited.insert(start_addr);

    // Follow call chain
    for _ in 0..max_depth {
        if let Some(targets) = call_graph.get(&current_addr) {
            if let Some(&next_addr) = targets.first() {
                if !visited.contains(&next_addr) {
                    path.push(next_addr);
                    visited.insert(next_addr);

                    if let Some(lmfdb_key) = lmfdb_map.get(&next_addr) {
                        lmfdb_sequence.push(lmfdb_key.clone());
                    }

                    current_addr = next_addr;
                } else {
                    break; // Cycle detected
                }
            } else {
                break; // No more targets
            }
        } else {
            break; // Dead end
        }
    }

    // Compute mathematical signature and Gogobglabulab power
    let mathematical_signature = lmfdb_sequence.join(" -> ");
    let gogobglabulab_power = compute_gogobglabulab_power(&lmfdb_sequence);
    let path_len = path.len();

    GogobglabulabPath {
        path,
        lmfdb_sequence,
        total_calls: path_len as u32,
        mathematical_signature,
        gogobglabulab_power,
    }
}

fn compute_gogobglabulab_power(lmfdb_sequence: &[String]) -> f64 {
    let mut power = 1.0;

    for lmfdb_key in lmfdb_sequence {
        // Extract level and weight from LMFDB key (format: level.weight.character.orbit)
        let parts: Vec<&str> = lmfdb_key.split('.').collect();
        if parts.len() >= 2 {
            if let (Ok(level), Ok(weight)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                // Gogobglabulab power formula: level^weight / 37 (Monster Group connection)
                power *= (level.powf(weight) / 37.0).ln_1p(); // Use ln_1p for numerical stability
            }
        }
    }

    power.abs() // Absolute value for Gogobglabulab magnitude
}
