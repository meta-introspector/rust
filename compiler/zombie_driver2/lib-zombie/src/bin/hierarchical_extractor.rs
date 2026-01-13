use lib_zombie::HierarchicalSignatureExtractor;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <rust_file_or_directory>", args[0]);
        return Ok(());
    }

    println!("🔬 5-Level Hierarchical Signature Extractor");
    println!("==========================================");
    println!("Levels: 0.Byte → 1.Syn → 2.Span → 3.HIR → 4.Ty");

    let mut extractor = HierarchicalSignatureExtractor::new();
    let target = &args[1];

    if std::path::Path::new(target).is_dir() {
        // Process all .rs files in directory
        for entry in std::fs::read_dir(target)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(path_str) = path.to_str() {
                    println!("\n📄 Processing: {}", path_str);
                    extractor.extract_all_levels(path_str)?;

                    // Show final signature
                    if let Some(final_sig) = extractor.build_final_signature(path_str) {
                        println!("🎯 Final signature length: {} features", final_sig.len());
                    }
                }
            }
        }
    } else {
        // Process single file
        println!("\n📄 Processing: {}", target);
        extractor.extract_all_levels(target)?;

        if let Some(final_sig) = extractor.build_final_signature(target) {
            println!("🎯 Final signature: {} features", final_sig.len());
            println!("   First 10 features: {:?}", &final_sig[..final_sig.len().min(10)]);
        }
    }

    // Export all signatures
    extractor.export_signatures("hierarchical_signatures.json")?;

    println!("\n✅ Hierarchical signature extraction complete!");
    println!("📊 Each file analyzed through 5 compilation levels:");
    println!("   Level 0: Byte/text frequency analysis");
    println!("   Level 1: Syntax token analysis");
    println!("   Level 2: Span/location analysis");
    println!("   Level 3: HIR structural analysis");
    println!("   Level 4: Type system analysis");
    println!("🔗 Level transitions recorded for compilation flow analysis");

    Ok(())
}
