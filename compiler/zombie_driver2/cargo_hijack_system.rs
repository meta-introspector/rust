// 🧟 CARGO HIJACK: Trick old system to give us build order
use libloading::Library;
use std::env;
use std::ffi::CString;
use std::fs;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 CARGO HIJACK: Tricking old system for build order");

    // 1. Build our rustc shim
    build_rustc_shim()?;

    // 2. Set environment to hijack cargo
    setup_hijack_environment()?;

    // 3. Run cargo as subprocess - it will call our shim
    let build_order = run_cargo_subprocess()?;

    // 4. Now we have the build order - use our driver
    execute_with_zombie_driver(&build_order)?;

    println!("✅ Successfully hijacked cargo build order!");
    Ok(())
}

fn build_rustc_shim() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Building rustc shim...");

    let shim_code = r#"
use std::env;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Log the build order
    let args: Vec<String> = env::args().collect();
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("build_order.log")
        .unwrap();
    writeln!(log, "{}", args.join(" ")).unwrap();
    
    // Call back to our main process
    Command::new("./zombie_callback")
        .args(&args[1..])
        .status()
        .unwrap();
}
"#;

    fs::write("rustc_shim.rs", shim_code)?;

    Command::new("rustc").args(&["rustc_shim.rs", "-o", "rustc_shim"]).status()?;

    println!("✅ Rustc shim built");
    Ok(())
}

fn setup_hijack_environment() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Setting up hijack environment...");

    // Set RUSTC to our shim
    env::set_var("RUSTC", "./rustc_shim");

    // Set other cargo/rustc environment variables
    env::set_var("CARGO_BUILD_RUSTC", "./rustc_shim");
    env::set_var("RUSTC_WRAPPER", "./rustc_shim");

    // Create callback binary that uses our driver
    let callback_code = r#"
use libloading::Library;
use std::env;
use std::ffi::CString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = unsafe { Library::new("rustc_driver_self.so")? };
    let rustc_main = unsafe { driver.get::<unsafe extern "C" fn(i32, *const *const i8) -> i32>(b"rustc_driver_main")? };
    
    let args: Vec<String> = env::args().collect();
    let c_args: Vec<CString> = args.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
    let c_ptrs: Vec<*const i8> = c_args.iter().map(|s| s.as_ptr()).collect();
    
    let result = unsafe { rustc_main(c_ptrs.len() as i32, c_ptrs.as_ptr()) };
    std::process::exit(result);
}
"#;

    fs::write("zombie_callback.rs", callback_code)?;
    Command::new("rustc").args(&["zombie_callback.rs", "-o", "zombie_callback"]).status()?;

    println!("✅ Hijack environment ready");
    Ok(())
}

fn run_cargo_subprocess() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("🚀 Running cargo subprocess to capture build order...");

    // Clear previous build order log
    let _ = fs::remove_file("build_order.log");

    // Run cargo build - it will call our shim for each crate
    let output = Command::new("cargo")
        .args(&["build", "--verbose"])
        .current_dir("rust_source/")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    println!("Cargo output: {}", String::from_utf8_lossy(&output.stdout));

    // Read the build order from our shim's log
    let build_order: Vec<String> =
        fs::read_to_string("build_order.log")?.lines().map(|s| s.to_string()).collect();

    println!("📋 Captured build order: {} steps", build_order.len());
    Ok(build_order)
}

fn execute_with_zombie_driver(build_order: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 Executing build order with zombie driver...");

    let driver = unsafe { Library::new("rustc_driver_self.so")? };
    let rustc_main = unsafe {
        driver.get::<unsafe extern "C" fn(i32, *const *const i8) -> i32>(b"rustc_driver_main")?
    };

    for (i, build_step) in build_order.iter().enumerate() {
        println!("🔄 Step {}: {}", i + 1, build_step);

        let args: Vec<&str> = build_step.split_whitespace().collect();
        let c_args: Vec<CString> = args.iter().map(|s| CString::new(*s).unwrap()).collect();
        let c_ptrs: Vec<*const i8> = c_args.iter().map(|s| s.as_ptr()).collect();

        let result = unsafe { rustc_main(c_ptrs.len() as i32, c_ptrs.as_ptr()) };

        if result != 0 {
            return Err(format!("Build step {} failed with code {}", i + 1, result).into());
        }
    }

    println!("✅ All build steps completed with zombie driver");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shim_creation() {
        assert!(build_rustc_shim().is_ok());
        assert!(std::path::Path::new("rustc_shim").exists());
    }

    #[test]
    fn test_environment_setup() {
        assert!(setup_hijack_environment().is_ok());
        assert!(std::path::Path::new("zombie_callback").exists());
    }
}
