// Example code for testing full graph advisor
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Clone usage - could be optimized
    let cloned_data = data.clone();
    let another_clone = cloned_data.clone();
    
    // Unwrap usage - could be safer
    let result = Some(42);
    let value = result.unwrap();
    
    // String conversion patterns
    let name = "hello".to_string();
    let greeting = format!("Hello {}", name);
    
    // Iterator patterns
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    
    // More unwrap usage
    let numbers = vec![1, 2, 3];
    let first = numbers.get(0).unwrap();
    
    println!("Value: {}, First: {}", value, first);
    println!("Doubled: {:?}", doubled);
}
