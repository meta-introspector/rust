fn main() {
    println!("Hello, world!");
    let x = 42;
    match x {
        42 => println!("Found it!"),
        _ => println!("Not found"),
    }
}
