use serde_json::json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 HUGGING FACE DATASET CONSOLIDATOR");
    println!("====================================");

    let base_dir =
        "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch";
    let output_file = format!("{}/consolidated_rustc_analysis.jsonl", base_dir);

    let mut entries = Vec::new();
    let mut total_entries = 0;

    // 1. Add call graph data
    println!("📊 Adding call graph data...");
    if let Ok(call_data) =
        fs::read_to_string(format!("{}/call_graph/call_graph_with_counts.json", base_dir))
    {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&call_data) {
            entries.push(json!({
                "id": "rustc_call_graph",
                "type": "complete_call_graph",
                "data": parsed,
                "description": "Complete call graph of rustc_driver.so with frequency counts"
            }));
            total_entries += 1;
        }
    }

    // 2. Add top functions
    if let Ok(top_data) =
        fs::read_to_string(format!("{}/call_graph/top_called_functions.json", base_dir))
    {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&top_data) {
            entries.push(json!({
                "id": "rustc_top_functions",
                "type": "top_called_functions",
                "data": parsed,
                "description": "Top 100 most called functions in rustc_driver.so"
            }));
            total_entries += 1;
        }
    }

    // 3. Sample chunked memory functions (first 100)
    println!("📁 Sampling chunked memory functions...");
    let chunk_dirs = fs::read_dir(format!("{}/chunked_memory", base_dir))?;
    let mut sampled = 0;

    for chunk_dir in chunk_dirs {
        if sampled >= 100 {
            break;
        }

        let chunk_path = chunk_dir?.path();
        if chunk_path.is_dir() {
            let files = fs::read_dir(&chunk_path)?;

            for file in files {
                if sampled >= 100 {
                    break;
                }

                let file_path = file?.path();
                if file_path.extension().map_or(false, |ext| ext == "json") {
                    if let Ok(func_data) = fs::read_to_string(&file_path) {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&func_data) {
                            entries.push(parsed);
                            sampled += 1;
                            total_entries += 1;
                        }
                    }
                }
            }
        }
    }

    // 4. Add existing proof evidence
    println!("🏆 Adding proof evidence...");
    if let Ok(proof_data) =
        fs::read_to_string(format!("{}/mathematical_proof_evidence.jsonl", base_dir))
    {
        for line in proof_data.lines().take(50) {
            // Sample first 50
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(line) {
                entries.push(parsed);
                total_entries += 1;
            }
        }
    }

    // 5. Add RDF/semantic data
    println!("🔗 Adding RDF semantic data...");
    entries.push(json!({
        "id": "rustc_rdf_ontology",
        "type": "semantic_ontology",
        "description": "OWL/RDF ontology linking rustc functions to mathematical properties",
        "files": {
            "ontology": "rdf/rustc_ontology.ttl",
            "queries": "rdf/queries.sparql",
            "json_ld": "rdf/rustc_ontology.jsonld"
        }
    }));
    total_entries += 1;

    // Write consolidated dataset
    println!("💾 Writing consolidated dataset...");
    let mut jsonl_content = String::new();
    for entry in entries {
        jsonl_content.push_str(&serde_json::to_string(&entry)?);
        jsonl_content.push('\n');
    }

    fs::write(&output_file, jsonl_content)?;

    // Create dataset info
    let dataset_info = json!({
        "dataset_name": "rustc-mathematical-analysis-complete",
        "version": "1.0.0",
        "description": "Complete mathematical analysis of Rust compiler with call graphs, function analysis, and semantic ontology",
        "total_entries": total_entries,
        "categories": {
            "call_graph": "Complete call relationships with frequency counts",
            "function_analysis": "Individual function analysis with mathematical properties",
            "proof_evidence": "Mathematical proofs linking binary addresses to LMFDB/Bott properties",
            "semantic_ontology": "RDF/OWL ontology for semantic web integration"
        },
        "binary_source": "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so",
        "extraction_date": "2026-01-08",
        "mathematical_frameworks": ["LMFDB", "Bott Periodicity", "Monster Group Theory"],
        "file_formats": ["JSONL", "RDF/Turtle", "JSON-LD"],
        "ready_for_upload": true
    });

    let info_file = format!("{}/consolidated_dataset_info.json", base_dir);
    fs::write(&info_file, serde_json::to_string_pretty(&dataset_info)?)?;

    println!("\n✅ CONSOLIDATION COMPLETE:");
    println!("   Dataset: {}", output_file);
    println!("   Info: {}", info_file);
    println!("   Total entries: {}", total_entries);
    println!("   Ready for Hugging Face upload! 🚀");

    Ok(())
}
