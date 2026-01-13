use goblin::elf::Elf;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌳 TRUE MAIN FUNCTIONS - ENTRY POINTS WITH NO CALLERS");
    println!("====================================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/true_main_rollup";
    fs::create_dir_all(&base_dir)?;

    println!("📊 Building complete call graph...");
    let (call_graph, call_frequency) = build_call_graph_with_counts(&binary, &elf)?;
    let lmfdb_map = compute_all_lmfdb_keys(&elf);

    println!("🔍 Finding true main functions (entry points with no callers)...");
    let true_mains = find_true_main_functions(&call_graph, &elf);
    println!("   Found {} true main functions", true_mains.len());

    println!("📈 Rolling up ALL dependencies from true mains...");
    let rollup_data = rollup_from_true_mains(&true_mains, &call_graph, &lmfdb_map, &call_frequency);

    println!("💾 Exporting true main rollup analysis...");

    let rollup_export = json!({
        "type": "true_main_functions_complete_rollup",
        "description": "Complete rollup of all LMFDB frequencies and Gogobglabulab power from true entry point functions that have no callers",
        "true_main_functions": rollup_data.iter().map(|(addr, data)| {
            json!({
                "address": format!("0x{:x}", addr),
                "function_name": data.name.clone(),
                "demangled_name": demangle_name(&data.name),
                "is_entry_point": true,
                "total_dominated_functions": data.total_dominated_functions,
                "own_lmfdb_key": data.own_lmfdb_key.clone(),
                "complete_lmfdb_rollup": data.complete_lmfdb_rollup,
                "total_gogobglabulab_power": data.total_gogobglabulab_power,
                "max_call_depth": data.max_call_depth,
                "mathematical_dominance_signature": data.mathematical_dominance_signature.clone(),
                "top_dominated_lmfdb_keys": data.top_dominated_lmfdb_keys.clone(),
                "call_tree_statistics": {
                    "unique_lmfdb_keys": data.complete_lmfdb_rollup.len(),
                    "most_frequent_lmfdb": data.top_dominated_lmfdb_keys.first().cloned(),
                    "dominance_breadth": data.total_dominated_functions,
                    "dominance_depth": data.max_call_depth
                }
            })
        }).collect::<Vec<_>>(),
        "global_statistics": {
            "total_true_mains": rollup_data.len(),
            "total_functions_dominated": rollup_data.values().map(|d| d.total_dominated_functions).sum::<u32>(),
            "total_gogobglabulab_power": rollup_data.values().map(|d| d.total_gogobglabulab_power).sum::<f64>(),
            "deepest_call_tree": rollup_data.values().map(|d| d.max_call_depth).max().unwrap_or(0),
            "most_dominant_main": rollup_data.iter()
                .max_by_key(|(_, data)| data.total_dominated_functions)
                .map(|(addr, data)| json!({
                    "address": format!("0x{:x}", addr),
                    "name": data.name.clone(),
                    "dominated_functions": data.total_dominated_functions,
                    "power": data.total_gogobglabulab_power
                }))
        },
        "extraction_timestamp": "2026-01-08T19:52:00Z"
    });

    let rollup_file = format!("{}/true_main_complete_rollup.json", base_dir);
    fs::write(&rollup_file, serde_json::to_string_pretty(&rollup_export)?)?;

    // Show top true mains by dominance
    let mut sorted_mains: Vec<_> = rollup_data.iter().collect();
    sorted_mains.sort_by_key(|(_, data)| std::cmp::Reverse(data.total_dominated_functions));

    println!("\n🏆 TOP 10 MOST DOMINANT TRUE MAIN FUNCTIONS:");
    for (i, (addr, data)) in sorted_mains.iter().take(10).enumerate() {
        println!(
            "   #{}: 0x{:x} - {} dominated, depth {}, {:.2} power",
            i + 1,
            addr,
            data.total_dominated_functions,
            data.max_call_depth,
            data.total_gogobglabulab_power
        );
        println!("       {}", data.name.chars().take(60).collect::<String>());
        println!(
            "       Top LMFDB: {:?}",
            data.top_dominated_lmfdb_keys.iter().take(3).collect::<Vec<_>>()
        );
    }

    println!("\n✅ TRUE MAIN ROLLUP COMPLETE:");
    println!("   True main functions: {}", rollup_data.len());
    println!(
        "   Total dominated functions: {}",
        rollup_data.values().map(|d| d.total_dominated_functions).sum::<u32>()
    );
    println!(
        "   Deepest call tree: {}",
        rollup_data.values().map(|d| d.max_call_depth).max().unwrap_or(0)
    );
    println!("   Export file: {}", rollup_file);

    Ok(())
}

#[derive(Debug, Clone)]
struct TrueMainRollup {
    name: String,
    total_dominated_functions: u32,
    own_lmfdb_key: String,
    complete_lmfdb_rollup: HashMap<String, u32>,
    total_gogobglabulab_power: f64,
    max_call_depth: u32,
    mathematical_dominance_signature: String,
    top_dominated_lmfdb_keys: Vec<(String, u32)>,
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

fn find_true_main_functions(call_graph: &HashMap<u64, Vec<u64>>, elf: &Elf) -> Vec<(u64, String)> {
    // Find all functions that are called (targets)
    let mut called_functions = HashSet::new();
    for targets in call_graph.values() {
        for &target in targets {
            called_functions.insert(target);
        }
    }

    // Find functions that make calls but are never called themselves (true entry points)
    let mut true_mains = Vec::new();

    for (&caller, targets) in call_graph {
        if !targets.is_empty() && !called_functions.contains(&caller) {
            // This function calls others but is never called - it's a true main
            if let Some(sym) = elf.syms.iter().find(|s| s.st_value == caller) {
                if let Some(name) = elf.strtab.get_at(sym.st_name) {
                    if !name.is_empty() {
                        true_mains.push((caller, name.to_string()));
                    }
                }
            }
        }
    }

    // Also include entry point and main functions
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if name == "main" || name == "_start" || name.contains("main") && sym.st_size > 100 {
                true_mains.push((sym.st_value, name.to_string()));
            }
        }
    }

    // Remove duplicates and sort by address
    true_mains.sort_by_key(|(addr, _)| *addr);
    true_mains.dedup_by_key(|(addr, _)| *addr);

    true_mains
}

fn rollup_from_true_mains(
    true_mains: &[(u64, String)],
    call_graph: &HashMap<u64, Vec<u64>>,
    lmfdb_map: &HashMap<u64, String>,
    _call_frequency: &HashMap<u64, u32>,
) -> HashMap<u64, TrueMainRollup> {
    let mut rollup_data = HashMap::new();

    for (main_addr, main_name) in true_mains {
        // Collect ALL functions dominated by this true main
        let dominated_functions = collect_all_dominated_functions(*main_addr, call_graph);
        let own_lmfdb_key = lmfdb_map.get(main_addr).cloned().unwrap_or_default();

        // Roll up LMFDB frequencies from ALL dominated functions
        let mut complete_lmfdb_rollup = HashMap::new();
        let mut total_gogobglabulab_power = 0.0;

        for &dom_addr in &dominated_functions {
            if let Some(lmfdb_key) = lmfdb_map.get(&dom_addr) {
                *complete_lmfdb_rollup.entry(lmfdb_key.clone()).or_insert(0) += 1;
                total_gogobglabulab_power += compute_gogobglabulab_power_single(lmfdb_key);
            }
        }

        // Get top dominated LMFDB keys
        let mut top_dominated: Vec<_> = complete_lmfdb_rollup.iter().collect();
        top_dominated.sort_by(|a, b| b.1.cmp(a.1));
        let top_dominated_lmfdb_keys: Vec<(String, u32)> =
            top_dominated.into_iter().take(10).map(|(k, v)| (k.clone(), *v)).collect();

        // Calculate max call depth
        let max_call_depth = calculate_max_depth(*main_addr, call_graph, 50);

        // Create mathematical dominance signature
        let mathematical_dominance_signature = format!(
            "MAIN[{}] dominates {} functions with {} unique LMFDB keys",
            own_lmfdb_key,
            dominated_functions.len(),
            complete_lmfdb_rollup.len()
        );

        let rollup = TrueMainRollup {
            name: main_name.clone(),
            total_dominated_functions: dominated_functions.len() as u32,
            own_lmfdb_key,
            complete_lmfdb_rollup,
            total_gogobglabulab_power,
            max_call_depth,
            mathematical_dominance_signature,
            top_dominated_lmfdb_keys,
        };

        rollup_data.insert(*main_addr, rollup);
    }

    rollup_data
}

fn collect_all_dominated_functions(
    start_addr: u64,
    call_graph: &HashMap<u64, Vec<u64>>,
) -> Vec<u64> {
    let mut dominated = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = vec![start_addr];

    while let Some(addr) = stack.pop() {
        if visited.insert(addr) {
            dominated.push(addr);

            if let Some(targets) = call_graph.get(&addr) {
                for &target in targets {
                    if !visited.contains(&target) {
                        stack.push(target);
                    }
                }
            }
        }
    }

    dominated
}

fn calculate_max_depth(
    start_addr: u64,
    call_graph: &HashMap<u64, Vec<u64>>,
    max_depth: u32,
) -> u32 {
    let mut visited = HashSet::new();
    let mut max_found = 0;

    fn dfs(
        addr: u64,
        depth: u32,
        call_graph: &HashMap<u64, Vec<u64>>,
        visited: &mut HashSet<u64>,
        max_depth: u32,
        max_found: &mut u32,
    ) {
        if depth >= max_depth || !visited.insert(addr) {
            return;
        }

        *max_found = (*max_found).max(depth);

        if let Some(targets) = call_graph.get(&addr) {
            for &target in targets {
                dfs(target, depth + 1, call_graph, visited, max_depth, max_found);
            }
        }
    }

    dfs(start_addr, 0, call_graph, &mut visited, max_depth, &mut max_found);
    max_found
}

fn compute_gogobglabulab_power_single(lmfdb_key: &str) -> f64 {
    let parts: Vec<&str> = lmfdb_key.split('.').collect();
    if parts.len() >= 2 {
        if let (Ok(level), Ok(weight)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            return (level.powf(weight) / 37.0).ln_1p().abs();
        }
    }
    0.0
}

fn demangle_name(name: &str) -> String {
    if name.starts_with("_ZN") {
        name.replace("_ZN", "").replace("E", "::").chars().take(100).collect()
    } else {
        name.to_string()
    }
}
