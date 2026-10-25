// Three functions the page compiles to assembly. `#[inline(never)]` keeps each one as a
// function of its own, so there is something to look at.

#[inline(never)]
pub fn sum_const() -> u64 {
    (0..1000u64).sum()
}

#[inline(never)]
pub fn sum_to(n: u64) -> u64 {
    (0..n).sum()
}

#[inline(never)]
pub fn sum_loop(n: u64) -> u64 {
    let mut total = 0;
    for i in 0..n {
        total += i;
    }
    total
}
