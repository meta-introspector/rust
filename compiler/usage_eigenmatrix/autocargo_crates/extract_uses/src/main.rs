#![feature(rustc_private)]

use split_decls_genesis::*;

macro_rules! rustroot {
    () => {
        "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust"
    };
}

macro_rules! processedroot {
    () => {
        "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-genesis/submodules"
    };
}

macro_rules! include_rustc {
    ($name:ident, $path:literal) => {
        mod $name {
            use super::*;
            include!(concat!(processedroot!(), "/rust/compiler/", $path, "/src/lib.rs"));
        }
    };
}

fn main() {
    println!("🔄 Testing mkuse macro with rustc files...");
    
    include_rustc!(driver_impl_module, "rustc_driver_impl");
    include_rustc!(driver_module, "rustc_driver");
    include_rustc!(session_module, "rustc_session");
    
    println!("✅ Compilation test complete");
}
