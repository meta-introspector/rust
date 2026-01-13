use goblin::elf::Elf;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌳 TOP INDIRECT USERS - ZERO-CALL FUNCTIONS BY TRANSITIVE CALLS");
    println!("================================================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/top_indirect_users";
    fs::create_dir_all(&base_dir)?;

    println!("📊 Building call graph...");
    let (call_graph, call_frequency) = build_call_graph_with_counts(&binary, &elf)?;
    let lmfdb_map = compute_all_lmfdb_keys(&elf);

    println!("🔍 Finding zero-call functions...");
    let zero_call_functions = find_zero_call_functions(&call_frequency, &elf);
    println!("   Found {} zero-call functions", zero_call_functions.len());

    println!("📈 Ranking by transitive call count...");
    let mut ranked_functions = Vec::new();

    for (addr, name) in zero_call_functions.iter().take(1000) {
        // Limit for performance
        let transitive_calls = count_transitive_calls(*addr, &call_graph);
        let own_lmfdb = lmfdb_map.get(addr).cloned().unwrap_or_default();

        ranked_functions.push((*addr, name.clone(), transitive_calls, own_lmfdb));

        if ranked_functions.len() % 100 == 0 {
            println!("   Analyzed {} functions...", ranked_functions.len());
        }
    }

    // Sort by transitive call count (descending)
    ranked_functions.sort_by_key(|(_, _, count, _)| std::cmp::Reverse(*count));

    println!("💾 Exporting top indirect users...");

    let top_indirect = json!({
        "type": "top_indirect_users_analysis",
        "description": "Zero-call functions ranked by how many functions they transitively call",
        "top_indirect_users": ranked_functions.iter().take(100).enumerate().map(|(rank, (addr, name, count, lmfdb))| {
            json!({
                "rank": rank + 1,
                "address": format!("0x{:x}", addr),
                "function_name": name,
                "demangled_name": demangle_name(name),
                "transitive_call_count": count,
                "lmfdb_key": lmfdb,
                "dominance_score": *count as f64 / ranked_functions.iter().map(|(_, _, c, _)| *c).max().unwrap_or(1) as f64
            })
        }).collect::<Vec<_>>(),
        "statistics": {
            "total_analyzed": ranked_functions.len(),
            "max_transitive_calls": ranked_functions.iter().map(|(_, _, c, _)| *c).max().unwrap_or(0),
            "avg_transitive_calls": ranked_functions.iter().map(|(_, _, c, _)| *c as f64).sum::<f64>() / ranked_functions.len() as f64,
            "top_10_total_calls": ranked_functions.iter().take(10).map(|(_, _, c, _)| *c as u64).sum::<u64>()
        },
        "extraction_timestamp": "2026-01-08T19:56:00Z"
    });

    let export_file = format!("{}/top_indirect_users.json", base_dir);
    fs::write(&export_file, serde_json::to_string_pretty(&top_indirect)?)?;

    println!("\n🏆 TOP 20 INDIRECT USERS (Zero-call functions by transitive calls):");
    for (rank, (addr, name, count, lmfdb)) in ranked_functions.iter().take(20).enumerate() {
        println!("   #{}: 0x{:x} - {} transitive calls", rank + 1, addr, count);
        println!("       {} [{}]", name.chars().take(70).collect::<String>(), lmfdb);
        println!("       Demangled: {}", demangle_name(name).chars().take(60).collect::<String>());
        println!();
    }

    println!("✅ TOP INDIRECT USERS ANALYSIS COMPLETE:");
    println!("   Functions analyzed: {}", ranked_functions.len());
    println!(
        "   Top indirect user: {} transitive calls",
        ranked_functions.first().map(|(_, _, c, _)| *c).unwrap_or(0)
    );
    println!("   Export file: {}", export_file);

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

fn find_zero_call_functions(call_frequency: &HashMap<u64, u32>, elf: &Elf) -> Vec<(u64, String)> {
    let mut zero_call_functions = Vec::new();

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 && !name.is_empty() {
                let call_count = call_frequency.get(&sym.st_value).copied().unwrap_or(0);
                if call_count == 0 {
                    zero_call_functions.push((sym.st_value, name.to_string()));
                }
            }
        }
    }

    zero_call_functions
}

fn count_transitive_calls(start_addr: u64, call_graph: &HashMap<u64, Vec<u64>>) -> usize {
    let mut visited = HashSet::new();
    let mut stack = vec![start_addr];
    let mut count: usize = 0;

    while let Some(addr) = stack.pop() {
        if visited.insert(addr) {
            count += 1;

            if let Some(targets) = call_graph.get(&addr) {
                for &target in targets {
                    if !visited.contains(&target) {
                        stack.push(target);
                    }
                }
            }
        }

        // Prevent infinite loops in large call graphs
        if count > 10000 {
            break;
        }
    }

    count.saturating_sub(1) // Don't count the starting function itself
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
