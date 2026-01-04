// Test enum for demonstrating usage collection
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Named { name: String, hex: String },
}

#[derive(Debug, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() {
    // Enum variant construction
    let red = Color::Red;
    let blue = Color::Blue;
    let custom = Color::Rgb(255, 128, 0);
    let named = Color::Named { 
        name: "Purple".to_string(), 
        hex: "#800080".to_string() 
    };
    
    let status = Status::Active;
    
    // Pattern matching
    match &red {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
        Color::Rgb(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
        Color::Named { name, hex } => println!("Named color: {} ({})", name, hex),
    }
    
    // Comparisons
    if red == Color::Red {
        println!("Red comparison works");
    }
    
    if status == Status::Active {
        println!("Status is active");
    }
    
    // Method calls on enums
    println!("Debug: {:?}", red);
    println!("Debug: {:?}", status);
    
    // More pattern matching with different variants
    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Rgb(100, 200, 50),
        Color::Named { name: "Cyan".to_string(), hex: "#00FFFF".to_string() }
    ];
    
    for color in colors {
        match color {
            Color::Red | Color::Green | Color::Blue => println!("Primary color"),
            Color::Rgb(_, _, _) => println!("RGB color"),
            Color::Named { .. } => println!("Named color"),
        }
    }
}
