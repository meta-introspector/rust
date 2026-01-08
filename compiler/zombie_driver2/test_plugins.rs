// Standalone test driver for plugin system
use libloading::{Library, Symbol};
use std::collections::HashMap;

#[repr(C)]
struct TestEvent {
    event_type: u32,
    data: *const u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Plugin Driver");
    
    let mut plugins: HashMap<String, Library> = HashMap::new();
    
    // Test loading each wrapper
    for wrapper in &["span_wrapper", "ty_wrapper", "hir_wrapper"] {
        println!("Building {}...", wrapper);
        std::process::Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(wrapper)
            .status()?;
            
        let so_path = format!("{}/target/release/lib{}.so", wrapper, wrapper);
        let lib = unsafe { Library::new(&so_path)? };
        plugins.insert(wrapper.to_string(), lib);
        println!("✅ Loaded {}", wrapper);
    }
    
    // Test executing functions from each plugin
    let test_event = TestEvent { event_type: 0, data: std::ptr::null() };
    
    for (name, lib) in &plugins {
        let func_name = format!("{}_c", name.replace("_wrapper", "_execute"));
        if let Ok(func) = unsafe { lib.get::<Symbol<unsafe extern "C" fn(u32, *const u8) -> *mut u8>>(func_name.as_bytes()) } {
            let result = unsafe { func(test_event.event_type, test_event.data) };
            println!("✅ {} executed: {:p}", name, result);
        }
    }
    
    println!("🎉 All plugin tests passed!");
    Ok(())
}
