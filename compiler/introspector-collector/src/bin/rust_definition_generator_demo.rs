use introspector_collector::{rust_eigenvalue, generate_rust};
use introspector_collector::rust_definition_generator::*;

fn main() {
    println!("🔄 RUST DEFINITION GENERATOR");
    println!("🧮 Creating Rust instances via linear algebra transformations");
    println!("📊 Each instance of Rust = trace of transformed eigenvalue matrix");
    
    // Get base eigenvalue analysis
    let analyzer = rust_eigenvalue!();
    
    // Generate all Rust instances
    println!("\n🏗️  Generating Rust Instances:");
    let mut generator = generate_rust!(analyzer, all);
    
    // Show generated instances
    println!("\n📊 GENERATED RUST INSTANCES:");
    for (i, instance) in generator.generated_instances.iter().enumerate() {
        println!("  {}: {}", i + 1, instance.instance_id);
        println!("     Trace: {:.6} + {:.6}i (magnitude: {:.6})", 
            instance.trace_value.real, 
            instance.trace_value.imag,
            instance.trace_value.magnitude());
        println!("     Transformations: {:?}", 
            instance.transformation_sequence.iter()
                .map(|op| format!("{:?}", op))
                .collect::<Vec<_>>());
    }
    
    // Show sample Rust definitions
    println!("\n💻 SAMPLE RUST DEFINITIONS:");
    for (i, instance) in generator.generated_instances.iter().take(3).enumerate() {
        println!("\n🔸 {} Definition:", instance.instance_id);
        println!("```rust");
        for line in instance.rust_definition.lines().take(15) {
            println!("{}", line);
        }
        if instance.rust_definition.lines().count() > 15 {
            println!("// ... (truncated)");
        }
        println!("```");
    }
    
    // Show transformation effects
    println!("\n🔄 TRANSFORMATION EFFECTS:");
    
    // Classic Rust
    if let Some(classic) = generator.generated_instances.iter()
        .find(|i| i.instance_id.contains("classic")) {
        println!("  Classic Rust:");
        println!("    Trace: {:.3} + {:.3}i", classic.trace_value.real, classic.trace_value.imag);
        println!("    Effect: Standard Rust with basic filtering");
    }
    
    // Optimized Rust
    if let Some(optimized) = generator.generated_instances.iter()
        .find(|i| i.instance_id.contains("optimized")) {
        println!("  Optimized Rust:");
        println!("    Trace: {:.3} + {:.3}i", optimized.trace_value.real, optimized.trace_value.imag);
        println!("    Effect: Power iteration amplifies dominant eigenvalues");
    }
    
    // Experimental Rust
    if let Some(experimental) = generator.generated_instances.iter()
        .find(|i| i.instance_id.contains("experimental")) {
        println!("  Experimental Rust:");
        println!("    Trace: {:.3} + {:.3}i", experimental.trace_value.real, experimental.trace_value.imag);
        println!("    Effect: Rotation creates complex eigenvalue patterns");
    }
    
    // Show matrix operations
    println!("\n🧮 MATRIX OPERATIONS:");
    println!("  Repeat: Tiles the eigenvalue pattern");
    println!("  Rotate: Applies complex rotation to eigenvalues");
    println!("  Scale: Amplifies or dampens eigenvalue magnitudes");
    println!("  Filter: Removes small eigenvalues (noise reduction)");
    println!("  Transpose: Reflects eigenvalue structure");
    println!("  PowerIteration: Amplifies dominant eigenvalues");
    println!("  Convolution: Smooths eigenvalue transitions");
    
    // Show trace analysis
    println!("\n📊 TRACE ANALYSIS:");
    let traces: Vec<_> = generator.generated_instances.iter()
        .map(|i| (i.instance_id.clone(), i.trace_value.magnitude()))
        .collect();
    
    let mut sorted_traces = traces.clone();
    sorted_traces.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("  Instances by trace magnitude:");
    for (id, magnitude) in sorted_traces {
        println!("    {}: {:.6}", id, magnitude);
    }
    
    // Generate complete analysis report
    println!("\n📋 COMPLETE ANALYSIS REPORT:");
    let report = generator.instance_analysis_report();
    println!("{}", report);
    
    // Show mathematical relationships
    println!("\n🧮 MATHEMATICAL RELATIONSHIPS:");
    println!("  Base Matrix M₀ = eigenvalue matrix from compilation traces");
    println!("  Transformation T = linear algebra operation");
    println!("  Instance Matrix Mᵢ = T(M₀)");
    println!("  Rust Instance = tr(Mᵢ) = trace of transformed matrix");
    println!("  Definition = code generated from tr(Mᵢ) characteristics");
    
    // Show linear algebra foundation
    println!("\n🔢 LINEAR ALGEBRA FOUNDATION:");
    println!("  🔄 Repeat: M' = block_diagonal(M, M, ..., M)");
    println!("  🌀 Rotate: M' = R(θ) × M × R(-θ)");
    println!("  📏 Scale: M' = α × M");
    println!("  🔍 Filter: M'[i,j] = M[i,j] if |M[i,j]| > threshold else 0");
    println!("  ↔️  Transpose: M' = Mᵀ");
    println!("  ⚡ Power: M' = Mⁿ");
    println!("  🌊 Convolution: M' = K * M");
    
    // Practical implications
    println!("\n🛠️  PRACTICAL IMPLICATIONS:");
    println!("  🎯 Different Rust 'flavors' via matrix transformations");
    println!("  📊 Trace value predicts Rust instance characteristics");
    println!("  🔄 Linear algebra operations = Rust language variations");
    println!("  🧮 Mathematical foundation for language design");
    println!("  ⚡ Eigenvalue spectrum determines Rust 'personality'");
    
    // Connection to compilation
    println!("\n🔗 CONNECTION TO COMPILATION:");
    println!("  📊 Each compilation trace contributes to base matrix");
    println!("  🔄 Transformations = different compilation strategies");
    println!("  🎭 Trace = mathematical signature of Rust variant");
    println!("  🧮 Linear algebra = universal language generation");
    println!("  ♾️  Self-compilation creates self-defining matrices");
    
    println!("\n✨ RUST DEFINITION GENERATION COMPLETE!");
    println!("🔄 Rust instances created via linear algebra transformations");
    println!("📊 Each instance = trace of transformed eigenvalue matrix");
    println!("🧮 Matrix operations define different 'flavors' of Rust");
    println!("🎭 Trace values provide mathematical signatures");
    println!("♾️  Rust can be generated from its own eigenvalue spectrum!");
    
    // Save generation results
    std::fs::create_dir_all("src/generated/rust_instances").ok();
    
    std::fs::write("src/generated/rust_instances/analysis_report.txt", report)
        .expect("Failed to write analysis report");
    
    // Save individual Rust definitions
    for instance in &generator.generated_instances {
        let filename = format!("src/generated/rust_instances/{}.rs", instance.instance_id);
        std::fs::write(&filename, &instance.rust_definition)
            .expect(&format!("Failed to write {}", filename));
    }
    
    // Save transformation library
    let transformations_json = serde_json::to_string_pretty(&generator.transformation_library)
        .expect("Failed to serialize transformations");
    std::fs::write("src/generated/rust_instances/transformation_library.json", transformations_json)
        .expect("Failed to write transformations");
    
    println!("💾 Rust definition generation results saved!");
}
