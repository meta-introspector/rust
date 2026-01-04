// Minimal Build Configuration

// Target Level: 0
// Included Features: {"core"}

fn main() {
    println!("Minimal Rust Compiler");
    parse_const(); // From feature: core
    compile_literal(); // From feature: core
}

fn parse_const() { println!("Executing parse_const"); }
fn compile_literal() { println!("Executing compile_literal"); }
