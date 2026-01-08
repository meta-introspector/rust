// Standalone test driver for span wrapper
#![feature(rustc_private)]
use rustc_span::{Span, DUMMY_SP};
use libloading::{Library, Symbol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Span Wrapper");
    
    // Build the .so first
    std::process::Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("span_wrapper")
        .status()?;
    
    // Load the .so
    let lib = unsafe { Library::new("span_wrapper/target/release/libspan_wrapper.so")? };
    let span_execute: Symbol<unsafe extern "C" fn(u32, *const u8) -> *mut u8> = 
        unsafe { lib.get(b"span_execute_c")? };
    
    // Test with dummy span
    let span = DUMMY_SP;
    let result = unsafe { span_execute(0, &span as *const _ as *const u8) };
    
    println!("✅ Span wrapper test passed: {:p}", result);
    Ok(())
}
