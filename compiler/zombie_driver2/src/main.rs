#![feature(rustc_private)]
#![feature(str_as_str)]

mod span_wrapper;
mod ty_wrapper; 
mod hir_wrapper;
mod plugin_driver;
mod p2p_server;

use plugin_driver::{PluginDriver, CompilerEvent};

// Minimal rustc driver integration
use rustc_driver::Callbacks;
use rustc_interface::interface;

struct ZombieCallbacks {
    driver: PluginDriver,
}

impl ZombieCallbacks {
    fn new() -> Self {
        let mut driver = PluginDriver::new();
        // Load span wrapper .so at startup
        let _ = driver.load_plugin("span_wrapper", "target/release/libspan_wrapper.so");
        Self { driver }
    }
}

impl Callbacks for ZombieCallbacks {
    fn after_parsing<'tcx>(&mut self, _compiler: &interface::Compiler, _queries: &'tcx rustc_interface::Queries<'tcx>) -> rustc_driver::Compilation {
        // Stream compiler event to plugin driver
        let event = CompilerEvent {
            event_type: 1, // parsing
            data: std::ptr::null(),
            size: 0,
        };
        self.driver = std::mem::take(&mut self.driver).react(event);
        
        // Execute span wrapper plugin
        let _ = self.driver.execute_plugin("span_wrapper", "span_execute_c");
        
        rustc_driver::Compilation::Continue
    }
}

fn main() {
    let callbacks = ZombieCallbacks::new();
    
    // Run rustc with our plugin-reactive callbacks
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::RunCompiler::new(&args, &mut Box::new(callbacks)).run().unwrap();
}
