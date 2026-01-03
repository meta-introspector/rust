include!("witness_macros.rs");

fn main() {
    witness!(symbol: "main", from: "user_code");
    witness!(crate: "std", version: "1.0");
    
    println!("Hello, witnessed world!");
    
    witness!(resolve: "println!" => "std::io::_print");
}
