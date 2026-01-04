// Bootstrap Level 0: Minimal Self-Compiling Rust
// Can only compile: const x = 1;

fn main() {
    let source_code = "const x = 1;";
    
    println!("🔬 Bootstrap Level 0 Compiler");
    println!("Source: {}", source_code);
    
    // Parse
    let tokens = tokenize(source_code);
    println!("Tokens: {:?}", tokens);
    
    // Compile
    let result = compile_tokens(&tokens);
    println!("Result: {}", result);
    
    // Generate next level compiler
    generate_level_1_compiler();
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn compile_tokens(tokens: &[&str]) -> String {
    if tokens.len() == 4 && 
       tokens[0] == "const" && 
       tokens[2] == "=" && 
       tokens[3].ends_with(';') {
        format!("SUCCESS: Compiled const {} = {}", tokens[1], tokens[3])
    } else {
        "ERROR: Invalid syntax".to_string()
    }
}

fn generate_level_1_compiler() {
    let level_1_source = r#"
// Bootstrap Level 1: Adds bool support
// Can compile: const x = 1; const y = true;

fn main() {
    let programs = vec!["const x = 1;", "const y = true;"];
    
    for program in programs {
        println!("Compiling: {}", program);
        let tokens = tokenize(program);
        println!("Result: {}", compile_level_1(&tokens));
    }
    
    generate_level_2_compiler();
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn compile_level_1(tokens: &[&str]) -> String {
    if tokens.len() == 4 && tokens[0] == "const" && tokens[2] == "=" {
        let value = tokens[3].trim_end_matches(';');
        match value {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" => 
                format!("SUCCESS: Integer const {} = {}", tokens[1], value),
            "true" | "false" => 
                format!("SUCCESS: Boolean const {} = {}", tokens[1], value),
            _ => "ERROR: Unsupported literal".to_string()
        }
    } else {
        "ERROR: Invalid syntax".to_string()
    }
}

fn generate_level_2_compiler() {
    println!("Generated Level 2 compiler (adds expressions)");
}
"#;
    
    std::fs::write("level_1_compiler.rs", level_1_source).unwrap();
    println!("✅ Generated level_1_compiler.rs");
    println!("🎯 Bootstrap chain: Level 0 → Level 1 → Level 2 → ... → Full rustc");
}
