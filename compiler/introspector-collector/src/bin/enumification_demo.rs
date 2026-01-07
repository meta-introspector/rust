use introspector_collector::{crates, nixpkgs, languages};
use introspector_collector::enumification_system::*;

fn main() {
    println!("🔢 ENUMIFICATION SYSTEM");
    println!("📦 Converting entire ecosystems into enum values");
    
    let mut engine = EnumificationEngine::new();
    engine.load_all_ecosystems();
    
    println!("\n🦀 Rust Ecosystem Enumification:");
    let rustc_crates = crates!(rustc);
    println!("  rustc compiler crates: {}", rustc_crates.len());
    for crate_enum in &rustc_crates {
        println!("    {:?}", crate_enum);
    }
    
    let std_crates = crates!(std);
    println!("  std library crates: {}", std_crates.len());
    
    let external_crates = crates!(external);
    println!("  external crates: {}", external_crates.len());
    
    println!("\n❄️  Nix Ecosystem Enumification:");
    let nix_core = nixpkgs!(core);
    println!("  core packages: {}", nix_core.len());
    
    let build_tools = nixpkgs!(build_tools);
    println!("  build tools: {}", build_tools.len());
    
    let system_packages = nixpkgs!(system);
    println!("  system packages: {}", system_packages.len());
    
    println!("\n🔧 Language Ecosystem Enumification:");
    let functional_langs = languages!(functional);
    println!("  functional languages: {:?}", functional_langs);
    
    let theorem_proving = languages!(theorem_proving);
    println!("  theorem proving: {:?}", theorem_proving);
    
    // Query examples
    println!("\n🔍 Ecosystem Queries:");
    
    // crates!(rustc).imports.names()
    let rustc_imports = rustc_crates.imports();
    println!("  crates!(rustc).imports(): {}", rustc_imports.len());
    for import in rustc_imports.iter().take(3) {
        println!("    {}", import);
    }
    
    let rustc_names = rustc_crates.names();
    println!("  crates!(rustc).names(): {:?}", rustc_names);
    
    // Query specific packages
    let query_results = engine.query("rust", "rustc");
    println!("  engine.query(\"rust\", \"rustc\"): {:?}", query_results);
    
    let imports = engine.imports("rustc");
    println!("  engine.imports(\"rustc\"): {:?}", imports);
    
    // Generate complete ecosystem enums
    println!("\n📝 Generated Ecosystem Enums:");
    
    let rust_enum = engine.generate_ecosystem_enum("rust");
    println!("Rust ecosystem enum (first 10 lines):");
    for line in rust_enum.lines().take(10) {
        println!("  {}", line);
    }
    
    let nix_enum = engine.generate_ecosystem_enum("nix");
    println!("\nNix ecosystem enum (first 10 lines):");
    for line in nix_enum.lines().take(10) {
        println!("  {}", line);
    }
    
    // Show total enumification
    println!("\n📊 Complete Enumification Summary:");
    println!("  🦀 Rust crates enumified: {}", engine.rust_ecosystem.len());
    println!("  ❄️  Nix packages enumified: {}", engine.nix_ecosystem.len());
    println!("  🔧 System packages enumified: {}", engine.system_ecosystem.len());
    println!("  🧠 Languages enumified: {}", engine.language_ecosystem.len());
    
    let total_enum_variants = engine.rust_ecosystem.len() + 
                             engine.nix_ecosystem.len() + 
                             engine.system_ecosystem.len() + 
                             engine.language_ecosystem.len();
    
    println!("  📦 Total enum variants: {}", total_enum_variants);
    
    println!("\n✨ Enumification Complete!");
    println!("🎯 Every package, crate, and repo is now an enum variant");
    println!("🔍 Query with: crates!(rustc).imports.names()");
    println!("📊 Build dependency graphs from enum relationships");
    println!("🌐 Universal package management through enum system");
    
    // Save enumified ecosystems
    std::fs::write("src/generated/rust_ecosystem_enum.rs", rust_enum)
        .expect("Failed to write Rust ecosystem enum");
    
    std::fs::write("src/generated/nix_ecosystem_enum.rs", nix_enum)
        .expect("Failed to write Nix ecosystem enum");
    
    println!("💾 Enumified ecosystems saved to src/generated/");
}
