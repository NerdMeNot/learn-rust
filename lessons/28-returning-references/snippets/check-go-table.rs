struct Config {
    retries: u32,
}

fn new_config() -> Config {
    Config { retries: 3 } // `return &Config{...}` in Go
}

fn head(xs: &[i32]) -> &[i32] {
    &xs[..2]
}

fn head_mut(xs: &mut [i32]) -> &mut [i32] {
    &mut xs[..2]
}

fn before(s: &str, i: usize) -> &str {
    &s[..i]
}

fn fill(buf: &mut [u8]) -> usize {
    buf[0] = b'x';
    1
}

fn main() {
    assert_eq!(new_config().retries, 3);
    let mut xs = [1, 2, 3];
    assert_eq!(head(&xs), [1, 2]);
    head_mut(&mut xs)[0] = 9;
    assert_eq!(xs, [9, 2, 3]);
    assert_eq!(before("abc", 1), "a");
    let mut buf = [0u8; 4];
    assert_eq!(fill(&mut buf), 1);
}
