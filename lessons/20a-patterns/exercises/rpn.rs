enum Token {
    Num(i64),
    Op(char),
}

/// Evaluate a reverse Polish expression: numbers go on a stack, and an operator
/// takes the top two (the older one is the left operand) and pushes the result.
/// None for: a missing operand, an operator other than + - * /, a division by
/// zero, or anything but exactly one value left at the end.
fn eval(tokens: &[Token]) -> Option<i64> {
    todo!("match each token; let ... else for the two operands; match stack[..] at the end")
}

fn main() {
    use Token::{Num, Op};
    assert_eq!(eval(&[Num(3), Num(4), Op('+')]), Some(7));
    assert_eq!(eval(&[Num(2), Num(3), Op('-')]), Some(-1));
    assert_eq!(eval(&[Num(8), Num(2), Op('/')]), Some(4));
    // (5 + ((1 + 2) * 4)) - 3
    let long = [
        Num(5),
        Num(1),
        Num(2),
        Op('+'),
        Num(4),
        Op('*'),
        Op('+'),
        Num(3),
        Op('-'),
    ];
    assert_eq!(eval(&long), Some(14));
    assert_eq!(eval(&[Num(7)]), Some(7));
    assert_eq!(eval(&[]), None);
    assert_eq!(eval(&[Num(1), Num(2)]), None);
    assert_eq!(eval(&[Op('+')]), None);
    assert_eq!(eval(&[Num(1), Op('+')]), None);
    assert_eq!(eval(&[Num(1), Num(0), Op('/')]), None);
    assert_eq!(eval(&[Num(1), Num(2), Op('^')]), None);
    println!("all checks passed");
}
