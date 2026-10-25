enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Neg(a) => -eval(a),
    }
}

fn main() {
    // 1 + -(2 + 3)
    let e = Expr::Add(
        Box::new(Expr::Num(1)),
        Box::new(Expr::Neg(Box::new(Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Num(3)),
        )))),
    );
    println!("eval: {}", eval(&e));
    println!("size_of Box<Expr>: {}", size_of::<Box<Expr>>());
    println!("size_of Expr: {}", size_of::<Expr>());
}
