use goblin::elf::Elf;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 CALL GRAPH WITH COUNTS EXPORT");
    println!("=================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let base_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/call_graph";
    fs::create_dir_all(&base_dir)?;

    println!("📊 Building call graph with frequency counts...");
    let (call_graph, call_frequency) = build_call_graph_with_counts(&binary, &elf)?;

    println!("   Call relationships: {}", call_graph.len());
    println!("   Unique call targets: {}", call_frequency.len());

    // Export call graph
    let graph_data = json!({
        "type": "rustc_call_graph",
        "binary_source": binary_path,
        "extraction_timestamp": "2026-01-08T19:44:00Z",
        "statistics": {
            "total_call_sites": call_graph.len(),
            "unique_targets": call_frequency.len(),
            "total_calls": call_frequency.values().sum::<u32>()
        },
        "call_relationships": call_graph.iter().map(|(caller, targets)| {
            json!({
                "caller": format!("0x{:x}", caller),
                "targets": targets.iter().map(|t| format!("0x{:x}", t)).collect::<Vec<_>>()
            })
        }).take(10000).collect::<Vec<_>>(), // Limit for file size
        "call_frequency": call_frequency.iter().map(|(target, count)| {
            json!({
                "target": format!("0x{:x}", target),
                "call_count": count
            })
        }).collect::<Vec<_>>()
    });

    let graph_file = format!("{}/call_graph_with_counts.json", base_dir);
    fs::write(&graph_file, serde_json::to_string_pretty(&graph_data)?)?;

    // Export top called functions summary
    let mut sorted_calls: Vec<_> = call_frequency.iter().collect();
    sorted_calls.sort_by(|a, b| b.1.cmp(a.1));

    let top_summary = json!({
        "type": "top_called_functions_summary",
        "top_100": sorted_calls.iter().take(100).map(|(addr, count)| {
            // Try to find symbol name
            let symbol_name = elf.syms.iter()
                .find(|sym| sym.st_value == **addr)
                .and_then(|sym| elf.strtab.get_at(sym.st_name))
                .unwrap_or("unknown");

            json!({
                "address": format!("0x{:x}", addr),
                "call_count": count,
                "symbol_name": symbol_name,
                "demangled": demangle_name(symbol_name)
            })
        }).collect::<Vec<_>>()
    });

    let summary_file = format!("{}/top_called_functions.json", base_dir);
    fs::write(&summary_file, serde_json::to_string_pretty(&top_summary)?)?;

    println!("\n✅ CALL GRAPH EXPORT COMPLETE:");
    println!("   Call graph: {}", graph_file);
    println!("   Top functions: {}", summary_file);
    println!("   Total call sites: {}", call_graph.len());
    println!("   Unique targets: {}", call_frequency.len());
    println!("   Most called: 0x{:x} ({} calls)", sorted_calls[0].0, sorted_calls[0].1);

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
            // Direct call
            let rel_offset = i32::from_le_bytes([window[1], window[2], window[3], window[4]]);
            let call_addr = text_addr + i as u64 + 5;
            let target_addr = (call_addr as i64 + rel_offset as i64) as u64;

            // Add to call graph
            call_graph.entry(call_addr).or_insert_with(Vec::new).push(target_addr);

            // Count frequency
            *call_frequency.entry(target_addr).or_insert(0) += 1;
        }
    }

    Ok((call_graph, call_frequency))
}

fn demangle_name(name: &str) -> String {
    if name.starts_with("_ZN") {
        name.replace("_ZN", "").replace("E", "::").chars().take(100).collect()
    } else {
        name.to_string()
    }
}
