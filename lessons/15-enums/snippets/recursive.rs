use std::mem::size_of;

enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>), // a Box is a pointer to the heap
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn main() {
    // (1 + 2) * 4
    let e = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)))),
        Box::new(Expr::Num(4)),
    );
    println!("(1 + 2) * 4 = {}", eval(&e));
    println!("size of Box<Expr>: {} bytes", size_of::<Box<Expr>>());
    println!("size of Expr: {} bytes", size_of::<Expr>());
}
