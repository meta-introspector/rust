use std::collections::HashMap;

const MAX_SIZE: usize = 100;

struct Point {
    x: f64,
    y: f64,
}

fn calculate(a: i32, b: i32) -> i32 {
    let map: HashMap<i32, i32> = HashMap::new();
    a + b
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
