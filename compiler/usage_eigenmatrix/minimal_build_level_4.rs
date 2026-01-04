// Minimal Build Configuration

// Target Level: 4
// Included Features: {"bool", "control_flow", "core", "expr", "functions"}

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
    parse_fn(); // From feature: functions
    compile_call(); // From feature: functions
    type_check(); // From feature: functions
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
fn parse_fn() { println!("Executing parse_fn"); }
fn compile_call() { println!("Executing compile_call"); }
fn type_check() { println!("Executing type_check"); }
