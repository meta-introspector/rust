use lib_zombie::{TopologicalCompilationAnalyzer, RustPeriodicTable};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Rust Compiler Periodic Table Generator");
    println!("=========================================");
    
    // Create topological analyzer
    let analyzer = TopologicalCompilationAnalyzer::new();
    
    // Create and populate periodic table
    let mut periodic_table = RustPeriodicTable::new();
    periodic_table.classify_rustc_modules(&analyzer);
    
    // Display the table
    periodic_table.print_periodic_table();
    
    // Export to JSON
    periodic_table.export_table("rust_periodic_table.json")?;
    
    println!("\n✅ Rust Compiler Periodic Table complete!");
    println!("📊 Each rustc module classified by:");
    println!("   - 10-fold periodicity (compilation phases)");
    println!("   - 18 topological groups (functional similarity)");
    println!("   - Morse index (critical point complexity)");
    println!("   - K-theory rank (stable invariants)");
    println!("   - Bott periodicity class (geometric classification)");
    
    Ok(())
}
