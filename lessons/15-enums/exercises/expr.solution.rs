enum Expr {
    Num(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

use Expr::*;

fn num(n: i64) -> Box<Expr> {
    Box::new(Num(n))
}

/// Num(5) is 5. Neg(x) is minus x. Add and Mul work as you expect.
fn eval(e: &Expr) -> i64 {
    match e {
        Num(n) => *n,
        Neg(x) => -eval(x),
        Add(a, b) => eval(a) + eval(b),
        Mul(a, b) => eval(a) * eval(b),
    }
}

/// Fully bracketed text: Num(5) -> "5", Neg(x) -> "-x",
/// Add(a, b) -> "(a + b)", Mul(a, b) -> "(a * b)".
fn show(e: &Expr) -> String {
    match e {
        Num(n) => n.to_string(),
        Neg(x) => format!("-{}", show(x)),
        Add(a, b) => format!("({} + {})", show(a), show(b)),
        Mul(a, b) => format!("({} * {})", show(a), show(b)),
    }
}

fn main() {
    // (1 + 2) * 4
    let e = Mul(Box::new(Add(num(1), num(2))), num(4));
    assert_eq!(eval(&e), 12);
    assert_eq!(show(&e), "((1 + 2) * 4)");
    // a lone number, and a negative one
    assert_eq!(eval(&Num(5)), 5);
    assert_eq!(show(&Num(-5)), "-5");
    // Neg of Neg, with a negative number inside
    let n = Neg(Box::new(Neg(num(-3))));
    assert_eq!(eval(&n), -3);
    assert_eq!(show(&n), "---3");
    // deep on the left: ((((0 + 1) + 2) + 3) + 4)
    let mut deep = Num(0);
    for i in 1..=4 {
        deep = Add(Box::new(deep), num(i));
    }
    assert_eq!(eval(&deep), 10);
    assert_eq!(show(&deep), "((((0 + 1) + 2) + 3) + 4)");
    println!("all checks passed");
}
