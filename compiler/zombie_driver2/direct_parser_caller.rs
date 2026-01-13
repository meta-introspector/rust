use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_void};

// Direct function loading from rustc_driver.so
extern "C" {
    fn dlopen(filename: *const c_char, flag: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> i32;
    fn dlerror() -> *mut c_char;
}

const RTLD_LAZY: i32 = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 DIRECT RUSTC PARSER FUNCTION CALLER");
    println!("=====================================");

    // Load the 2.8GB rustc_driver.so into our process
    let rustc_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let rustc_path_c = CString::new(rustc_path)?;

    println!("📦 Loading rustc_driver.so (2.8GB)...");
    let handle = unsafe { dlopen(rustc_path_c.as_ptr(), RTLD_LAZY) };

    if handle.is_null() {
        let error = unsafe { CStr::from_ptr(dlerror()) };
        return Err(format!("Failed to load rustc_driver.so: {:?}", error).into());
    }

    println!("✅ rustc_driver.so loaded successfully!");

    // Load parser function symbols from our target list
    let target_json = fs::read_to_string("parser_target_functions.json")?;
    let parser_functions: Vec<String> = parse_simple_json_array(&target_json)?;

    let mut loaded_functions = HashMap::new();

    for func_name in &parser_functions {
        let func_name_c = CString::new(func_name.clone())?;
        let func_ptr = unsafe { dlsym(handle, func_name_c.as_ptr()) };

        if !func_ptr.is_null() {
            loaded_functions.insert(func_name.to_string(), func_ptr);
            println!("✅ Found function: {}", func_name);
        } else {
            println!("❌ Function not found: {}", func_name);
        }
    }

    println!("📊 Loaded {} parser functions", loaded_functions.len());

    // Try to call the functions we found
    for (func_name, func_ptr) in &loaded_functions {
        println!("🎯 Attempting to call: {}", func_name);

        // For now, just print the function address
        println!("   Function address: {:p}", func_ptr);

        // TODO: Set up proper function signature and call
        // This requires knowing the exact function signature
        // We can extract this from the Rust source or debug symbols
    }

    // Cleanup
    unsafe {
        dlclose(handle);
    }

    println!("✅ Direct function loading complete!");
    println!("📋 Next: Define function signatures and call parser functions directly");

    Ok(())
}

fn parse_simple_json_array(json: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let lines: Vec<&str> = json.lines().collect();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('"') && trimmed.ends_with('"') || trimmed.ends_with("\",") {
            let mut func_name = trimmed.to_string();
            // Remove quotes and comma
            func_name = func_name.replace('"', "").replace(',', "");
            if !func_name.is_empty() {
                result.push(func_name);
            }
        }
    }

    Ok(result)
}
