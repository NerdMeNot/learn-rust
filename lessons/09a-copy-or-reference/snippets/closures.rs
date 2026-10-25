fn main() {
    let mut shared = 0;
    let mut bump = || shared += 1; // borrows `shared`
    bump();
    bump();
    println!("closure without move: shared = {shared}");

    let mut owned = 0;
    let mut bump_own = move || {
        owned += 1; // changes the closure's own copy
        owned
    };
    bump_own();
    let inside = bump_own();
    println!("closure with move:    owned = {owned}, the closure's copy = {inside}");
}
