// Minimal Build Configuration

// Target Level: 1
// Included Features: {"bool", "core"}

fn main() {
    println!("Minimal Rust Compiler");
    parse_bool(); // From feature: bool
    compile_bool(); // From feature: bool
    parse_const(); // From feature: core
    compile_literal(); // From feature: core
}

fn parse_bool() { println!("Executing parse_bool"); }
fn compile_bool() { println!("Executing compile_bool"); }
fn parse_const() { println!("Executing parse_const"); }
fn compile_literal() { println!("Executing compile_literal"); }
