// Three functions that call a closure they are given. Every parameter below is
// `FnOnce`, because `FnOnce` accepts every closure and so the starter compiles.
// Write the body of each function, read the compiler's complaint, and change the trait
// to the most permissive one that works: `FnOnce` if the function calls it once,
// `FnMut` if it calls it several times, `Fn` if it calls it through a shared reference.

// Calls `job` once and returns what it gives back.
fn run_once(job: impl FnOnce() -> String) -> String {
    todo!("call job once and return its result")
}

// Calls `tick` `times` times.
fn repeat(times: u32, tick: impl FnOnce()) {
    todo!("call tick `times` times")
}

struct Button<F> {
    on_click: F,
}

impl<F: FnOnce() -> i32> Button<F> {
    // Takes `&self`, so it can be called any number of times on a shared button.
    fn click(&self) -> i32 {
        todo!("call on_click through &self")
    }
}

fn main() {
    // This closure hands `name` over, so it can only run once.
    let name = String::from("ferris");
    assert_eq!(run_once(move || name), "ferris");

    // This one changes `total`, so it runs many times but needs `mut` access.
    let mut total = 0;
    repeat(3, || total += 2);
    assert_eq!(total, 6);
    repeat(0, || total += 100);
    assert_eq!(total, 6);

    // This one only reads `limit`, so a shared button can call it again and again.
    let limit = 10;
    let button = Button {
        on_click: || limit * 2,
    };
    assert_eq!(button.click(), 20);
    assert_eq!(button.click(), 20);
    println!("all checks passed");
}
