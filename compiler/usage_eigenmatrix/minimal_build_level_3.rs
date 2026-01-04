// Minimal Build Configuration

// Target Level: 3
// Included Features: {"bool", "control_flow", "core", "expr"}

fn main() {
    println!("Minimal Rust Compiler");
    parse_bool(); // From feature: bool
    compile_bool(); // From feature: bool
    parse_if(); // From feature: control_flow
    parse_match(); // From feature: control_flow
    compile_branch(); // From feature: control_flow
    parse_const(); // From feature: core
    compile_literal(); // From feature: core
    parse_expr(); // From feature: expr
    compile_binop(); // From feature: expr
}

fn parse_bool() { println!("Executing parse_bool"); }
fn compile_bool() { println!("Executing compile_bool"); }
fn parse_if() { println!("Executing parse_if"); }
fn parse_match() { println!("Executing parse_match"); }
fn compile_branch() { println!("Executing compile_branch"); }
fn parse_const() { println!("Executing parse_const"); }
fn compile_literal() { println!("Executing compile_literal"); }
fn parse_expr() { println!("Executing parse_expr"); }
fn compile_binop() { println!("Executing compile_binop"); }
