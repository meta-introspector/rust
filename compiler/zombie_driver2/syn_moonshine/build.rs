use std::process::Command;
use std::env;
use std::fs;
use syn::{parse_quote, ItemFn, Stmt};
use quote::quote;

fn main() {
    println!("🧟 ZOMBIE COMPILER BUILD: Self-Instrumenting");
    println!("============================================");
    
    // Phase 1: Instrument ourselves during build
    instrument_source_code().expect("Failed to instrument source");
    
    // Phase 2: Capture rustc_driver.so location
    let rustc_driver_path = capture_rustc_driver().expect("Failed to capture rustc driver");
    println!("📦 Captured rustc_driver.so: {}", rustc_driver_path);
    
    // Phase 3: Generate ping-pong macros
    generate_pingpong_macros().expect("Failed to generate ping-pong macros");
    
    // Phase 4: Setup zombie compiler hooks
    setup_zombie_hooks(&rustc_driver_path).expect("Failed to setup zombie hooks");
    
    println!("✅ Build instrumentation complete - ready for zombie compilation");
}

fn instrument_source_code() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Instrumenting source code with compile-time hooks...");
    
    let source_files = ["src/main.rs", "src/self_analysis.rs"];
    
    for file_path in &source_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            let mut file = syn::parse_file(&content)?;
            
            // Inject instrumentation into every function
            for item in &mut file.items {
                if let syn::Item::Fn(func) = item {
                    instrument_function(func);
                }
            }
            
            // Write back instrumented code
            let instrumented = quote! { #file }.to_string();
            fs::write(format!("{}.instrumented", file_path), instrumented)?;
        }
    }
    
    Ok(())
}

fn instrument_function(func: &mut ItemFn) {
    let func_name = &func.sig.ident;
    
    // Inject compile-time → runtime ping
    let ping_stmt: Stmt = parse_quote! {
        compile_time_ping!(#func_name);
    };
    
    // Inject runtime → compile-time pong  
    let pong_stmt: Stmt = parse_quote! {
        runtime_pong!(#func_name);
    };
    
    // Insert at beginning and end of function
    func.block.stmts.insert(0, ping_stmt);
    func.block.stmts.push(pong_stmt);
}

fn capture_rustc_driver() -> Result<String, Box<dyn std::error::Error>> {
    println!("🎯 Capturing rustc_driver.so for zombie compilation...");
    
    // Use our known 2.8GB rustc_driver.so location
    let driver_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    
    if std::path::Path::new(driver_path).exists() {
        println!("✅ Found 2.8GB rustc_driver.so: {}", driver_path);
        return Ok(driver_path.to_string());
    }
    
    // Fallback to system rustc
    let output = Command::new("rustc")
        .args(&["--print", "sysroot"])
        .output()?;
    
    let sysroot = String::from_utf8(output.stdout)?.trim().to_string();
    let fallback_path = format!("{}/lib/librustc_driver.so", sysroot);
    
    if std::path::Path::new(&fallback_path).exists() {
        println!("✅ Using system rustc_driver.so: {}", fallback_path);
        Ok(fallback_path)
    } else {
        Err("No rustc_driver.so found".into())
    }
}
    
    // Copy it to our control
    let zombie_driver_path = "target/zombie_rustc_driver.so";
    fs::copy(&driver_path, zombie_driver_path)?;
    
    // Generate the zombie wrapper
    let zombie_code = quote! {
        use libloading::{Library, Symbol};
        use std::ffi::CString;
        
        pub struct ZombieCompiler {
            driver: Library,
            compile_fn: Symbol<'static, unsafe extern "C" fn(*const i8) -> i32>,
        }
        
        impl ZombieCompiler {
            pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
                let driver = unsafe { Library::new("target/zombie_rustc_driver.so")? };
                let compile_fn = unsafe { driver.get(b"rustc_main")? };
                Ok(Self { driver, compile_fn })
            }
            
            pub fn zombie_compile(&self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
                let c_source = CString::new(source)?;
                let result = unsafe { (self.compile_fn)(c_source.as_ptr()) };
                if result == 0 { Ok(()) } else { Err("Zombie compilation failed".into()) }
            }
        }
    };
    
    fs::write("src/zombie_compiler.rs", zombie_code.to_string())?;
    
    Ok(zombie_driver_path.to_string())
}

fn generate_pingpong_macros() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏓 Generating compile-time ↔ runtime ping-pong macros...");
    
    let pingpong_macros = quote! {
        use std::sync::mpsc::{channel, Sender, Receiver};
        use std::thread;
        use std::time::Instant;
        
        static mut COMPILE_RUNTIME_CHANNEL: Option<(Sender<PingPongMessage>, Receiver<PingPongMessage>)> = None;
        static mut RUNTIME_COMPILE_CHANNEL: Option<(Sender<PingPongMessage>, Receiver<PingPongMessage>)> = None;
        
        #[derive(Debug, Clone)]
        pub struct PingPongMessage {
            pub function_name: String,
            pub timestamp: Instant,
            pub phase: Phase,
            pub data: Vec<u8>,
        }
        
        #[derive(Debug, Clone)]
        pub enum Phase {
            CompileTime,
            Runtime,
            Shifting,
        }
        
        // Macro for compile-time → runtime ping
        #[macro_export]
        macro_rules! compile_time_ping {
            ($func_name:ident) => {
                {
                    let msg = PingPongMessage {
                        function_name: stringify!($func_name).to_string(),
                        timestamp: Instant::now(),
                        phase: Phase::CompileTime,
                        data: vec![], // Could include AST data
                    };
                    
                    unsafe {
                        if let Some((sender, _)) = &COMPILE_RUNTIME_CHANNEL {
                            let _ = sender.send(msg);
                        }
                    }
                    
                    println!("🏓 PING: {} (compile-time)", stringify!($func_name));
                }
            };
        }
        
        // Macro for runtime → compile-time pong
        #[macro_export]
        macro_rules! runtime_pong {
            ($func_name:ident) => {
                {
                    let msg = PingPongMessage {
                        function_name: stringify!($func_name).to_string(),
                        timestamp: Instant::now(),
                        phase: Phase::Runtime,
                        data: vec![], // Could include binary data
                    };
                    
                    unsafe {
                        if let Some((sender, _)) = &RUNTIME_COMPILE_CHANNEL {
                            let _ = sender.send(msg);
                        }
                    }
                    
                    println!("🏓 PONG: {} (runtime)", stringify!($func_name));
                }
            };
        }
        
        // Initialize ping-pong channels
        pub fn init_pingpong() {
            unsafe {
                COMPILE_RUNTIME_CHANNEL = Some(channel());
                RUNTIME_COMPILE_CHANNEL = Some(channel());
            }
            
            // Start ping-pong listener thread
            thread::spawn(|| {
                loop {
                    unsafe {
                        if let Some((_, receiver)) = &COMPILE_RUNTIME_CHANNEL {
                            if let Ok(msg) = receiver.try_recv() {
                                handle_ping_message(msg);
                            }
                        }
                        
                        if let Some((_, receiver)) = &RUNTIME_COMPILE_CHANNEL {
                            if let Ok(msg) = receiver.try_recv() {
                                handle_pong_message(msg);
                            }
                        }
                    }
                    
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            });
        }
        
        fn handle_ping_message(msg: PingPongMessage) {
            println!("🎯 Received PING from {}: {:?}", msg.function_name, msg.phase);
            // Could trigger zombie compilation here
        }
        
        fn handle_pong_message(msg: PingPongMessage) {
            println!("🎯 Received PONG from {}: {:?}", msg.function_name, msg.phase);
            // Could trigger runtime analysis here
        }
    };
    
    fs::write("src/pingpong.rs", pingpong_macros.to_string())?;
    
    Ok(())
}

fn setup_zombie_hooks(driver_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 Setting up zombie compiler hooks...");
    
    let zombie_main = quote! {
        mod zombie_compiler;
        mod pingpong;
        
        use zombie_compiler::ZombieCompiler;
        use pingpong::{init_pingpong, PingPongMessage, Phase};
        use std::fs;
        
        fn main() -> Result<(), Box<dyn std::error::Error>> {
            println!("🧟 ZOMBIE COMPILER: Using captured rustc_driver.so");
            println!("=================================================");
            
            // Initialize ping-pong system
            init_pingpong();
            
            // Create zombie compiler
            let zombie = ZombieCompiler::new()?;
            
            // Compile ourselves using the zombie compiler!
            let our_source = fs::read_to_string("src/main.rs")?;
            
            println!("🔄 Zombie compiling ourselves...");
            zombie.zombie_compile(&our_source)?;
            
            println!("✅ Successfully compiled ourselves with zombie rustc!");
            
            // Now we can compile anything
            let test_code = r#"
                fn main() {
                    println!("Hello from zombie-compiled code!");
                }
            "#;
            
            println!("🧪 Testing zombie compilation on sample code...");
            zombie.zombie_compile(test_code)?;
            
            println!("🧟 ZOMBIE COMPILER ACTIVE - Ready to compile anything!");
            
            Ok(())
        }
    };
    
    fs::write("src/zombie_main.rs", zombie_main.to_string())?;
    
    // Update Cargo.toml to include new dependencies
    let cargo_toml_addition = r#"
[dependencies]
libloading = "0.8"
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"

[[bin]]
name = "zombie_compiler"
path = "src/zombie_main.rs"
"#;
    
    fs::write("Cargo_zombie.toml", cargo_toml_addition)?;
    
    println!("📝 Generated zombie compiler configuration");
    println!("   Run with: cargo build --manifest-path Cargo_zombie.toml");
    
    Ok(())
}
