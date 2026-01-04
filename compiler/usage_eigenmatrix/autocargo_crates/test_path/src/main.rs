fn main() {
    println!("🔄 Testing macro path resolution...");
    
    macro_rules! rustroot {
        () => {
            "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust"
        };
    }
    
    macro_rules! mkuse {
        ($use_stmt:path) => {
            compile_error!(concat!("USE|", module_path!(), "|", stringify!($use_stmt)));
        };
    }
    
    macro_rules! include_rustc {
        ($name:ident, $path:literal) => {
            println!("Testing path: {}/compiler/{}/src/lib.rs", rustroot!(), $path);
            
            // Check if file exists
            let file_path = format!("{}/compiler/{}/src/lib.rs", rustroot!(), $path);
            if std::path::Path::new(&file_path).exists() {
                println!("✅ File exists: {}", file_path);
            } else {
                println!("❌ File not found: {}", file_path);
            }
        };
    }
    
    include_rustc!(driver_impl_module, "rustc_driver_impl");
    include_rustc!(driver_module, "rustc_driver");
    include_rustc!(session_module, "rustc_session");
    
    println!("\n🔍 Testing mkuse macro...");
    mkuse!(std::collections::HashMap);
    mkuse!(rustc_session::Session);
    mkuse!(rustc_driver_impl::main);
}
