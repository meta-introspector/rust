use syn;
use std::fs;

fn main() {
    let content = fs::read_to_string("submodules/rust/compiler/rustc_middle/src/lib.rs").unwrap();
    
    println!("First 500 chars:");
    println!("{}", &content[..500.min(content.len())]);
    
    match syn::parse_file(&content) {
        Ok(_) => println!("✅ Parse successful"),
        Err(e) => {
            println!("❌ Parse error: {}", e);
            println!("Error span: {:?}", e.span());
        }
    }
}
