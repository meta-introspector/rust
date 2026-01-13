mod p2p_lib_wrapper;
use p2p_lib_wrapper::{LibVerb, P2PLibWrapper};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 REAL P2P LIBRARY WRAPPER");
    println!("===========================");

    let mut wrapper = P2PLibWrapper::new();

    // Load the 2.8GB rustc_driver.so
    let lib_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";

    println!("🔌 Loading 2.8GB library...");
    wrapper.execute_verb(LibVerb::LoadLib(lib_path.to_string()))?;

    println!("📊 Getting library info...");
    wrapper.execute_verb(LibVerb::GetLibInfo)?;

    println!("📋 Listing symbols...");
    wrapper.execute_verb(LibVerb::ListSymbols)?;

    println!("🔍 Reading specific symbols...");
    let _ = wrapper.execute_verb(LibVerb::ReadSymbol("main".to_string()));
    let _ = wrapper.execute_verb(LibVerb::ReadSymbol("rustc_driver_main".to_string()));

    println!("💾 Saving all results...");
    wrapper.execute_verb(LibVerb::SaveResults("rustc_driver_analysis.json".to_string()))?;

    println!("✅ Real library analysis complete!");

    Ok(())
}
