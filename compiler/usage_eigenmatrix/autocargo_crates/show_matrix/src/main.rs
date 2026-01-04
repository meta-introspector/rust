use split_decls_genesis::macro_wrappers::get_use_matrix;

fn main() {
    println!("📊 Current USE_MATRIX contents:");
    let matrix = get_use_matrix();
    
    if matrix.is_empty() {
        println!("❌ No mkuse! calls registered yet - need to include processed files");
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
