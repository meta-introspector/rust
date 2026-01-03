use std::collections::HashMap;

const MY_CONSTANT: i32 = 42;

#[derive(Debug)]
struct MyStruct {
    field: i32,
}

enum MyEnum {
    Variant1,
    Variant2(i32),
}

fn main() {
    // Constant usage
    let x = MY_CONSTANT;
    
    // Struct usage
    let s = MyStruct { field: x };
    println!("{:?}", s);
    
    // Enum usage
    let e = MyEnum::Variant1;
    match e {
        MyEnum::Variant1 => println!("Variant1"),
        MyEnum::Variant2(val) => println!("Variant2: {}", val),
    }
    
    // HashMap usage (from std)
    let mut map = HashMap::new();
    map.insert("key", "value");
    
    // Option usage
    let opt = Some(42);
    if let Some(value) = opt {
        println!("Value: {}", value);
    }
}
