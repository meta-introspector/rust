// Broken Rust code for DWIM plugin testing
use std::collections::HashMap;

fn main() {
    // Error 1: Wrong method name
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let length = vec.len; // Missing parentheses
    
    // Error 2: Type mismatch
    let name: String = "hello"; // &str assigned to String
    
    // Error 3: Missing trait bound
    #[derive(Clone)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let p = Person { name: "Alice".to_string(), age: 30 };
    println!("{:?}", p); // Debug not derived
    
    // Error 4: Borrow checker issue
    let mut data = vec![1, 2, 3];
    let first = &data[0];
    data.push(4); // Cannot borrow as mutable while immutable borrow exists
    println!("{}", first);
    
    // Error 5: Wrong function call
    println("Hello world"); // Missing !
    
    // Error 6: Missing import
    let mut map = HashMap::new();
    map.insert("key", "value");
    
    // Error 7: Lifetime issue
    fn get_first(data: &Vec<i32>) -> &i32 {
        &data[0] // Lifetime not specified
    }
    
    let numbers = vec![1, 2, 3];
    let first_num = get_first(&numbers);
    println!("{}", first_num);
}
