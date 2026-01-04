use split_decls_genesis::macro_wrappers::get_use_matrix;
use split_decls_genesis::{include_rust_compiler, mkmod, mkuse};

fn main() {
    println!("🔄 Including processed file to trigger mkuse! registrations...");
    
    // Create a module context and include the processed file
    mod test_include {
        use super::*;
        const MODULE_NAME: &str = "rustc_driver_impl";
        
        // Use the new include macro - much cleaner!
        include_rust_compiler!("driver_impl");
    }
    
    println!("📊 USE_MATRIX contents:");
    let matrix = get_use_matrix();
    
    if matrix.is_empty() {
        println!("❌ No mkuse! calls registered yet");
    } else {
        for (module, uses) in &matrix {
            println!("📦 Module: {}", module);
            for use_stmt in uses {
                println!("  └─ {}", use_stmt);
            }
            println!();
        }
    }
    
    println!("✅ Total modules with uses: {}", matrix.len());
}
