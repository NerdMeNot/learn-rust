fn call_twice(f: impl Fn()) {
    f();
    f();
}

fn call_twice_mut(mut f: impl FnMut()) {
    f();
    f();
}

fn call_once(f: impl FnOnce()) {
    f();
}

fn main() {
    let name = String::from("ferris");
    let mut hits = 0;

    // only reads `name`: fits all three
    call_twice(|| println!("hello, {name}"));
    call_twice_mut(|| println!("hello, {name}"));
    call_once(|| println!("hello, {name}"));

    // changes `hits`: fits FnMut and FnOnce, not Fn
    call_twice_mut(|| hits += 1);
    println!("hits: {hits}");
    call_once(|| hits += 10);
    println!("hits: {hits}");

    // gives `name` away: fits only FnOnce
    call_once(|| drop(name));
}
