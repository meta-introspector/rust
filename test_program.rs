fn main() {
    println!("Hello from test Rust program!");
    let vec = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", vec);
    
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("HashMap: {:?}", map);
}
