// Self-hosting rustc using our built rustc_driver
#![feature(rustc_private)]

fn main() {
    // Set maximum diagnostics environment
    std::env::set_var("RUSTC_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    
    // Call the rustc_driver we built
    rustc_driver::main()
}
