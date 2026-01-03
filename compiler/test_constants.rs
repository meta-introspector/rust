const MAX_SIZE: usize = 1024;
static THRESHOLD: f64 = 3.14159;

struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() {
    let point = Point { x: 10.0, y: 20.0 };
    let status = Status::Active;
    println!("Point: ({}, {}), Status: {:?}", point.x, point.y, status);
}
