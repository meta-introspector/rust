// Test file with common Rust errors for DWIM correction
fn main() {
    // Error 1: Missing parentheses on method call
    let vec = vec![1, 2, 3];
    let length = vec.len;  // Should be vec.len()
    
    // Error 2: Type mismatch - &str vs String
    let name: String = "hello";  // Should be "hello".to_string()
    
    // Error 3: Missing println! macro
    println("Hello world");  // Should be println!("Hello world")
    
    // Error 4: Field access on private field
    let point = Point { x: 1, y: 2 };
    let x_val = point.x;  // Might be private
    
    // Error 5: Borrow checker issue
    let mut data = vec![1, 2, 3];
    let first = &data[0];
    data.push(4);  // Cannot borrow as mutable while immutable borrow exists
    println!("{}", first);
    
    // Error 6: Pattern matching issue
    let option = Some(42);
    match option {
        Some(x) => println!("{}", x),
        // Missing None case
    }
    
    // Error 7: Method not found
    let text = "hello";
    let count = text.size();  // Should be text.len()
}

struct Point {
    x: i32,
    y: i32,
}
