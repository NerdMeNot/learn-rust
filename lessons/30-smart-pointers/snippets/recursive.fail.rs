enum Expr {
    Num(i64),
    Add(Expr, Expr),
}

fn main() {
    let e = Expr::Add(Expr::Num(1), Expr::Num(2));
    let _ = e;
}
