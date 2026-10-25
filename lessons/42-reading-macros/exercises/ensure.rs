use std::cell::Cell;

// ensure!(condition) returns early from the surrounding function with an Err that names the
// condition, as written, when it is false. A function cannot do that: only a macro can
// return from its caller, and only a macro can see the condition as text.
macro_rules! ensure {
    ($cond:expr) => {
        todo!("if the condition is false, return Err(String) that says which condition failed")
    };
}

fn check(age: i32, name: &str) -> Result<(), String> {
    ensure!(age >= 0);
    ensure!(!name.is_empty());
    ensure!(age + 1 < 150);
    Ok(())
}

fn side_effect(calls: &Cell<i32>) -> Result<(), String> {
    ensure!({
        calls.set(calls.get() + 1);
        true
    });
    Ok(())
}

fn main() {
    assert_eq!(check(30, "Ada"), Ok(()));
    assert_eq!(check(-1, "Ada"), Err("ensure failed: age >= 0".to_string()));
    assert_eq!(
        check(30, ""),
        Err("ensure failed: !name.is_empty()".to_string())
    );
    assert_eq!(
        check(200, "Ada"),
        Err("ensure failed: age + 1 < 150".to_string())
    );
    // The first failing check ends the function: the later ones never run.
    assert_eq!(check(-1, ""), Err("ensure failed: age >= 0".to_string()));

    let calls = Cell::new(0);
    assert_eq!(side_effect(&calls), Ok(()));
    assert_eq!(calls.get(), 1, "the condition must be evaluated once");
    println!("all checks passed");
}
