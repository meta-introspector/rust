fn main() {
    println!("Hello, world!");
    let x = Some(42);
    if let Some(value) = x {
        println!("Value: {}", value);
    }
}
