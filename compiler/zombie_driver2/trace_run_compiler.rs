use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 TRACING RUN_COMPILER - Complete Call Graph Analysis");
    println!("====================================================");

    let run_compiler_files = vec![
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis/chunk_00000042/addr_42f4980__ZN17rustc_driver_impl12run_compiler28__u7b__u7b_c.json",
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis/chunk_00000042/addr_42f4a60__ZN17rustc_driver_impl12run_compiler28__u7b__u7b_c.json",
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis/chunk_00000042/addr_42f45c0__ZN17rustc_driver_impl12run_compiler28__u7b__u7b_c.json",
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis/chunk_000000da/addr_da36d58__ZN17rustc_driver_impl12run_compiler28__u7b__u7b_c.json",
    ];

    for file_path in &run_compiler_files {
        println!("\n🎯 ANALYZING: {}", file_path.split('/').last().unwrap_or("unknown"));

        let content = fs::read_to_string(file_path)?;
        let data: Value = serde_json::from_str(&content)?;

        if let Some(addr) = data["memory_address"].as_str() {
            println!("   📍 Address: {}", addr);
        }

        if let Some(name) = data["demangled_name"].as_str() {
            println!("   🏷️  Name: {}", name);
        }

        if let Some(calls) = data["function_calls"].as_object() {
            if let Some(calls_from) = calls["calls_from"].as_array() {
                println!("   📞 CALLED BY ({} callers):", calls_from.len());
                for caller in calls_from.iter().take(5) {
                    if let Some(caller_str) = caller.as_str() {
                        println!("      ← {}", caller_str);
                    }
                }
            }

            if let Some(calls_to) = calls["calls_to"].as_array() {
                println!("   📤 CALLS ({} targets):", calls_to.len());
                for (i, target) in calls_to.iter().take(10).enumerate() {
                    if let Some(target_str) = target.as_str() {
                        println!("      → {}: {}", i + 1, target_str);
                    }
                }
                if calls_to.len() > 10 {
                    println!("      ... and {} more", calls_to.len() - 10);
                }
            }

            if let Some(call_count_in) = calls["call_count_in"].as_u64() {
                if let Some(call_count_out) = calls["call_count_out"].as_u64() {
                    println!(
                        "   📊 STATS: {} calls in, {} calls out",
                        call_count_in, call_count_out
                    );
                }
            }
        }

        if let Some(lmfdb) = data["mathematical_analysis"]["lmfdb_analysis"]["lmfdb_key"].as_str() {
            println!("   🔢 LMFDB: {}", lmfdb);
        }
    }

    Ok(())
}
