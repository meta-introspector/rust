mod rustc_enum_generator;

use rustc_enum_generator::RustcEnumGenerator;

fn main() {
    println!("Generating enum-to-string macros for all rustc enums...");
    
    let mut generator = RustcEnumGenerator::new();
    generator.load_rustc_enums();
    
    // Generate all macros
    let macros = generator.generate_all_macros();
    
    // Write to file
    std::fs::write("src/generated/rustc_enum_macros.rs", &macros)
        .expect("Failed to write macro file");
    
    println!("Generated macros written to src/generated/rustc_enum_macros.rs");
    
    // Generate orbit analysis
    let analysis = generator.generate_orbit_analysis();
    std::fs::write("src/generated/rustc_orbit_analysis.rs", &analysis)
        .expect("Failed to write analysis file");
    
    println!("Orbit analysis written to src/generated/rustc_orbit_analysis.rs");
    
    println!("Done!");
}
