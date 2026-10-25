// Backs the "written / reads as" table in lesson.mdx: every form compiles and works.
#![allow(dead_code)]

struct Parser<'a> {
    rest: &'a str,
}

fn elided(x: &str) -> &str {
    x
}
fn named<'a>(x: &'a str) -> &'a str {
    x
}
fn named_mut<'a>(v: &'a mut Vec<i32>) -> &'a mut i32 {
    &mut v[0]
}
fn through_struct<'a>(p: &Parser<'a>) -> &'a str {
    p.rest
}
fn anonymous(p: &Parser<'_>) -> usize {
    p.rest.len()
}
fn literal() -> &'static str {
    "literal"
}
fn bounded<T: 'static>(_t: T) {}
fn holds<'a, T: 'a + std::fmt::Debug>(x: &'a T) -> String {
    format!("{x:?}")
}
fn outlives<'a, 'b: 'a>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

fn main() {
    assert_eq!(elided("a"), named("a"));
    let mut v = vec![1, 2];
    *named_mut(&mut v) += 10;
    assert_eq!(v, [11, 2]);
    let p = Parser { rest: "abc" };
    assert_eq!(through_struct(&p), "abc");
    assert_eq!(anonymous(&p), 3);
    assert_eq!(literal(), "literal");
    bounded(String::from("owned"));
    bounded("literal");
    assert_eq!(outlives("x", "y"), "x");
    assert_eq!(holds(&5), "5");
    println!("all forms compile and work");
}
