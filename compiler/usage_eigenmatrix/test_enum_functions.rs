// Test file for Monster Group Enhanced Compiler Driver

#[derive(Debug, Clone)]
enum TestEnum {
    First,
    Second,
    Third,
}

impl TestEnum {
    fn to_string(&self) -> String {
        match self {
            TestEnum::First => "first_value".to_string(),
            TestEnum::Second => "second_value".to_string(),
            TestEnum::Third => "third_value".to_string(),
        }
    }
    
    fn get_description(&self) -> &str {
        match self {
            TestEnum::First => "This is the first option",
            TestEnum::Second => "This is the second option", 
            TestEnum::Third => "This is the third option",
        }
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn name(&self) -> String {
        match self {
            Color::Red => "red".to_string(),
            Color::Green => "green".to_string(),
            Color::Blue => "blue".to_string(),
        }
    }
}

fn main() {
    let test = TestEnum::First;
    println!("{}", test.to_string());
    println!("{}", test.get_description());
    
    let color = Color::Red;
    println!("{}", color.name());
}
