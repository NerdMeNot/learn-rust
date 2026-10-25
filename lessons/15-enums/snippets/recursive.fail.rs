enum Expr {
    Num(i64),
    Add(Expr, Expr),
}

fn main() {
    let _ = Expr::Num(1);
}
