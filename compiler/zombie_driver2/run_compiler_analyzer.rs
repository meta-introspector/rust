use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 RUN_COMPILER ANALYZER - Finding the Real Entry Point");
    println!("=====================================================");

    let call_graph_file = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/call_graph/call_graph_with_counts.json";

    println!("📊 Loading call graph...");
    let file = File::open(call_graph_file)?;
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader)?;

    let mut run_compiler_funcs: Vec<(String, u32, u32)> = Vec::new();

    if let Some(functions) = data["functions"].as_array() {
        for func in functions {
            if let (Some(name), Some(incoming), Some(outgoing)) = (
                func["symbol_name"].as_str(),
                func["incoming_calls"].as_u64(),
                func["outgoing_calls"].as_u64(),
            ) {
                if name.contains("run_compiler") && !name.contains("closure") {
                    run_compiler_funcs.push((name.to_string(), incoming as u32, outgoing as u32));
                }
            }
        }
    }

    println!("🔍 Found {} run_compiler functions:", run_compiler_funcs.len());

    for (name, incoming, outgoing) in &run_compiler_funcs {
        println!("   {} calls in, {} calls out", incoming, outgoing);
        println!("      {}", name);

        if *incoming == 0 && *outgoing > 0 {
            println!("      🎯 POTENTIAL MAIN ENTRY POINT!");
        }
    }

    Ok(())
}
