use libloading::{Library, Symbol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Span Wrapper .so");
    
    let lib = unsafe { Library::new("../target/release/libspan_wrapper.so")? };
    let span_execute: Symbol<unsafe extern "C" fn(u32, *const u8) -> *mut u8> = 
        unsafe { lib.get(b"span_execute_c")? };
    
    let result = unsafe { span_execute(0, std::ptr::null()) };
    
    println!("✅ Span wrapper executed: {:p}", result);
    Ok(())
}
