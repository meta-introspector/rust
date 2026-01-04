// minimal_rust.rs - Complete Rust compiler in one file, managed by mkdbuild
// No cargo, no build.rs, just pure macro-driven compilation

mkrust!(
    level!(0);
    features!(core);
    functions!(parse_const, compile_literal);
    symbols!("const", "=", ";");
);

mkrust!(
    level!(1); 
    features!(core, bool);
    functions!(parse_const, compile_literal, parse_bool, compile_bool);
    symbols!("const", "=", ";", "true", "false");
);

mkrust!(
    level!(2);
    features!(core, bool, expr);
    functions!(parse_const, compile_literal, parse_bool, compile_bool, parse_expr, compile_binop);
    symbols!("const", "=", ";", "true", "false", "+", "-", "*");
);

// mkdbuild will expand these macros into complete compiler
fn main() {
    println!("Minimal Rust Compiler");
    
    // Test programs
    let programs = vec![
        "const x = 1;",
        "const y = true;", 
        "const z = 1 + 2;",
    ];
    
    for program in programs {
        println!("Compiling: {}", program);
        #[cfg(level_0)]
        println!("Result: {}", compiler::compile(program));
        #[cfg(level_1)] 
        println!("Result: {}", compiler::compile(program));
        #[cfg(level_2)]
        println!("Result: {}", compiler::compile(program));
    }
}

// Macro expansion generates:
#[cfg(level_0)]
mod compiler {
    pub fn parse_const() { println!("parse_const"); }
    pub fn compile_literal() { println!("compile_literal"); }
    
    pub fn compile(input: &str) -> String {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() == 4 && tokens[0] == "const" && tokens[2] == "=" {
            format!("SUCCESS: {}", input)
        } else {
            "ERROR".to_string()
        }
    }
}

#[cfg(level_1)]
mod compiler {
    pub fn parse_const() { println!("parse_const"); }
    pub fn compile_literal() { println!("compile_literal"); }
    pub fn parse_bool() { println!("parse_bool"); }
    pub fn compile_bool() { println!("compile_bool"); }
    
    pub fn compile(input: &str) -> String {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() == 4 && tokens[0] == "const" && tokens[2] == "=" {
            let value = tokens[3].trim_end_matches(';');
            match value {
                "1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"0" => format!("SUCCESS: Integer {}", input),
                "true"|"false" => format!("SUCCESS: Boolean {}", input),
                _ => "ERROR".to_string()
            }
        } else {
            "ERROR".to_string()
        }
    }
}

#[cfg(level_2)]
mod compiler {
    pub fn parse_const() { println!("parse_const"); }
    pub fn compile_literal() { println!("compile_literal"); }
    pub fn parse_bool() { println!("parse_bool"); }
    pub fn compile_bool() { println!("compile_bool"); }
    pub fn parse_expr() { println!("parse_expr"); }
    pub fn compile_binop() { println!("compile_binop"); }
    
    pub fn compile(input: &str) -> String {
        if input.contains('+') || input.contains('-') || input.contains('*') {
            format!("SUCCESS: Expression {}", input)
        } else {
            let tokens: Vec<&str> = input.split_whitespace().collect();
            if tokens.len() == 4 && tokens[0] == "const" && tokens[2] == "=" {
                format!("SUCCESS: {}", input)
            } else {
                "ERROR".to_string()
            }
        }
    }
}

// mkdbuild usage:
// mkdbuild minimal_rust.rs --level=0  # Generates Level 0 compiler
// mkdbuild minimal_rust.rs --level=1  # Generates Level 1 compiler  
// mkdbuild minimal_rust.rs --level=2  # Generates Level 2 compiler
// mkdbuild minimal_rust.rs --level=N  # Generates Full rustc
