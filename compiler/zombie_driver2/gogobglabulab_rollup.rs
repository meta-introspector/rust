use goblin::elf::Elf;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 GOGOBGLABULAB ROLLUP TO MAIN FUNCTIONS");
    println!("=========================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rollup_analysis";
    fs::create_dir_all(&base_dir)?;

    println!("📊 Building call graph and LMFDB mappings...");
    let (call_graph, call_frequency) = build_call_graph_with_counts(&binary, &elf)?;
    let lmfdb_map = compute_all_lmfdb_keys(&elf);

    println!("🔍 Finding main functions (high call frequency)...");
    let main_functions = find_main_functions(&call_frequency, &elf, 50); // Top functions with 50+ calls
    println!("   Found {} main functions", main_functions.len());

    println!("📈 Rolling up LMFDB frequencies to main functions...");
    let rollup_data =
        rollup_to_main_functions(&main_functions, &call_graph, &lmfdb_map, &call_frequency);

    println!("💾 Exporting rollup analysis...");

    let rollup_export = json!({
        "type": "gogobglabulab_main_function_rollup",
        "description": "LMFDB frequencies and Gogobglabulab power rolled up from all called functions into main functions",
        "main_functions": rollup_data.iter().map(|(addr, data)| {
            json!({
                "address": format!("0x{:x}", addr),
                "function_name": data.name.clone(),
                "demangled_name": demangle_name(&data.name),
                "direct_calls": data.direct_calls,
                "total_dependencies": data.total_dependencies,
                "own_lmfdb_key": data.own_lmfdb_key.clone(),
                "rolled_up_lmfdb_frequencies": data.rolled_up_lmfdb_frequencies,
                "total_gogobglabulab_power": data.total_gogobglabulab_power,
                "dependency_tree_depth": data.dependency_tree_depth,
                "mathematical_signature": data.mathematical_signature.clone(),
                "top_dependency_lmfdb_keys": data.top_dependency_lmfdb_keys.clone()
            })
        }).collect::<Vec<_>>(),
        "statistics": {
            "total_main_functions": rollup_data.len(),
            "total_dependencies_analyzed": rollup_data.values().map(|d| d.total_dependencies).sum::<u32>(),
            "total_gogobglabulab_power": rollup_data.values().map(|d| d.total_gogobglabulab_power).sum::<f64>(),
            "most_powerful_main": rollup_data.iter()
                .max_by(|a, b| a.1.total_gogobglabulab_power.partial_cmp(&b.1.total_gogobglabulab_power).unwrap())
                .map(|(addr, data)| json!({
                    "address": format!("0x{:x}", addr),
                    "name": data.name.clone(),
                    "power": data.total_gogobglabulab_power
                }))
        },
        "extraction_timestamp": "2026-01-08T19:50:00Z"
    });

    let rollup_file = format!("{}/main_function_rollup.json", base_dir);
    fs::write(&rollup_file, serde_json::to_string_pretty(&rollup_export)?)?;

    // Show top 10 most powerful main functions
    let mut sorted_mains: Vec<_> = rollup_data.iter().collect();
    sorted_mains.sort_by(|a, b| {
        b.1.total_gogobglabulab_power.partial_cmp(&a.1.total_gogobglabulab_power).unwrap()
    });

    println!("\n🏆 TOP 10 MOST POWERFUL MAIN FUNCTIONS:");
    for (i, (addr, data)) in sorted_mains.iter().take(10).enumerate() {
        println!(
            "   #{}: 0x{:x} - {} deps, {:.2} power",
            i + 1,
            addr,
            data.total_dependencies,
            data.total_gogobglabulab_power
        );
        println!("       {}", data.name.chars().take(60).collect::<String>());
        println!(
            "       Top LMFDB: {:?}",
            data.top_dependency_lmfdb_keys.iter().take(3).collect::<Vec<_>>()
        );
    }

    println!("\n✅ ROLLUP COMPLETE:");
    println!("   Main functions analyzed: {}", rollup_data.len());
    println!(
        "   Total dependencies: {}",
        rollup_data.values().map(|d| d.total_dependencies).sum::<u32>()
    );
    println!("   Export file: {}", rollup_file);

    Ok(())
}

#[derive(Debug, Clone)]
struct MainFunctionRollup {
    name: String,
    direct_calls: u32,
    total_dependencies: u32,
    own_lmfdb_key: String,
    rolled_up_lmfdb_frequencies: HashMap<String, u32>,
    total_gogobglabulab_power: f64,
    dependency_tree_depth: u32,
    mathematical_signature: String,
    top_dependency_lmfdb_keys: Vec<(String, u32)>,
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

fn find_main_functions(
    call_frequency: &HashMap<u64, u32>,
    elf: &Elf,
    min_calls: u32,
) -> Vec<(u64, String)> {
    let mut main_functions = Vec::new();

    for (&addr, &calls) in call_frequency {
        if calls >= min_calls {
            // Find the symbol name for this address
            if let Some(sym) = elf.syms.iter().find(|s| s.st_value == addr) {
                if let Some(name) = elf.strtab.get_at(sym.st_name) {
                    if !name.is_empty() {
                        main_functions.push((addr, name.to_string()));
                    }
                }
            }
        }
    }

    // Sort by call frequency (descending)
    main_functions.sort_by(|a, b| {
        let calls_a = call_frequency.get(&a.0).unwrap_or(&0);
        let calls_b = call_frequency.get(&b.0).unwrap_or(&0);
        calls_b.cmp(calls_a)
    });

    main_functions.truncate(100); // Top 100 main functions
    main_functions
}

fn rollup_to_main_functions(
    main_functions: &[(u64, String)],
    call_graph: &HashMap<u64, Vec<u64>>,
    lmfdb_map: &HashMap<u64, String>,
    call_frequency: &HashMap<u64, u32>,
) -> HashMap<u64, MainFunctionRollup> {
    let mut rollup_data = HashMap::new();

    for (main_addr, main_name) in main_functions {
        let dependencies = collect_all_dependencies(*main_addr, call_graph, 20); // Max depth 20
        let own_lmfdb_key = lmfdb_map.get(main_addr).cloned().unwrap_or_default();

        // Roll up LMFDB frequencies from all dependencies
        let mut rolled_up_lmfdb_frequencies = HashMap::new();
        let mut total_gogobglabulab_power = 0.0;

        for &dep_addr in &dependencies {
            if let Some(lmfdb_key) = lmfdb_map.get(&dep_addr) {
                *rolled_up_lmfdb_frequencies.entry(lmfdb_key.clone()).or_insert(0) += 1;

                // Add Gogobglabulab power
                total_gogobglabulab_power += compute_gogobglabulab_power_single(lmfdb_key);
            }
        }

        // Get top dependency LMFDB keys
        let mut top_deps: Vec<_> = rolled_up_lmfdb_frequencies.iter().collect();
        top_deps.sort_by(|a, b| b.1.cmp(a.1));
        let top_dependency_lmfdb_keys: Vec<(String, u32)> =
            top_deps.into_iter().take(5).map(|(k, v)| (k.clone(), *v)).collect();

        // Create mathematical signature
        let mathematical_signature = format!(
            "{} -> [{}]",
            own_lmfdb_key,
            top_dependency_lmfdb_keys
                .iter()
                .map(|(k, c)| format!("{}({})", k, c))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let rollup = MainFunctionRollup {
            name: main_name.clone(),
            direct_calls: call_frequency.get(main_addr).copied().unwrap_or(0),
            total_dependencies: dependencies.len() as u32,
            own_lmfdb_key,
            rolled_up_lmfdb_frequencies,
            total_gogobglabulab_power,
            dependency_tree_depth: calculate_max_depth(*main_addr, call_graph, 20),
            mathematical_signature,
            top_dependency_lmfdb_keys,
        };

        rollup_data.insert(*main_addr, rollup);
    }

    rollup_data
}

fn collect_all_dependencies(
    start_addr: u64,
    call_graph: &HashMap<u64, Vec<u64>>,
    max_depth: u32,
) -> Vec<u64> {
    let mut dependencies = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![(start_addr, 0)];

    while let Some((addr, depth)) = stack.pop() {
        if depth < max_depth && visited.insert(addr) {
            dependencies.push(addr);

            if let Some(targets) = call_graph.get(&addr) {
                for &target in targets {
                    if !visited.contains(&target) {
                        stack.push((target, depth + 1));
                    }
                }
            }
        }
    }

    dependencies
}

fn calculate_max_depth(
    start_addr: u64,
    call_graph: &HashMap<u64, Vec<u64>>,
    max_depth: u32,
) -> u32 {
    let mut visited = std::collections::HashSet::new();
    let mut max_found = 0;

    fn dfs(
        addr: u64,
        depth: u32,
        call_graph: &HashMap<u64, Vec<u64>>,
        visited: &mut std::collections::HashSet<u64>,
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
