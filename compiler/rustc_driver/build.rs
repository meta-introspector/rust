use std::env;
use std::fs;
use std::path::Path;
use build_common::mkbuild;

mod function_wrapper;
use function_wrapper::inject_function_tracing;

fn main() {
    mkbuild!();
    println!("cargo:rerun-if-changed=src/");
    
    // Only inject tracing if RUSTC_TRACE_FUNCTIONS is set
    if env::var("RUSTC_TRACE_FUNCTIONS").is_ok() {
        println!("cargo:warning=Injecting function tracing into rustc_driver");
        
        // Process all .rs files in src/
        let src_dir = Path::new("src");
        if src_dir.exists() {
            for entry in fs::read_dir(src_dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                
                if path.extension().map_or(false, |ext| ext == "rs") {
                    let source = fs::read_to_string(&path).unwrap();
                    
                    match inject_function_tracing(&source) {
                        Ok(traced_source) => {
                            let traced_path = path.with_extension("traced.rs");
                            fs::write(&traced_path, traced_source).unwrap();
                            println!("cargo:warning=Traced: {:?}", traced_path);
                        }
                        Err(e) => {
                            println!("cargo:warning=Failed to trace {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
    }
}
