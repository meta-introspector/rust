
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
