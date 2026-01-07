enum TestEnum {
    Variant1,
    Variant2(i32),
    Variant3 { field: String },
}

enum SimpleEnum {
    A,
    B,
    C,
}

// Recursive expression enum
enum Expr {
    Literal(i32),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Call { name: String, args: Vec<Expr> },
}

fn main() {
    let x = TestEnum::Variant1;
    let y = SimpleEnum::A;
    
    // Use the recursive enum
    let expr = Expr::Add(
        Box::new(Expr::Literal(42)),
        Box::new(Expr::Variable("x".to_string()))
    );
    
    let complex_expr = Expr::Call {
        name: "func".to_string(),
        args: vec![
            Expr::Literal(1),
            Expr::Multiply(
                Box::new(Expr::Variable("a".to_string())),
                Box::new(Expr::Literal(2))
            )
        ]
    };
}
