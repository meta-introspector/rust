use crate::binary_entropy::{extract_function_entropy, fallback_random};

pub fn generate_float(arg: u64) -> f64 {
    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    
    extract_function_entropy(binary_path, arg)
        .unwrap_or_else(|| fallback_random(arg))
}

pub fn generate_bool(arg: u64) -> bool {
    generate_float(arg) > 0.5
}

pub fn generate_u8(arg: u64) -> u8 {
    (generate_float(arg) * 256.0) as u8
}
