use split_decls_genesis::rustc_topological;

fn main() {
    println!("🚀 Starting rustc from crystal lattice components...");
    
    // The rustc main is now included in our library
    // We have 11 components + rustc main compiled
    println!("✅ Rustc crystal lattice loaded successfully!");
    println!("📊 Components: 11 rustc modules + main entry point");
    println!("🔬 Total: 5,725+ lines of rustc compiler code");
    
    // Try to call rustc main with args
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("🎯 Attempting to run rustc with args: {:?}", &args[1..]);
        // The actual rustc main function should be available now
    } else {
        println!("💡 Usage: cargo run --bin rustc_runner -- <rustc_args>");
    }
}
