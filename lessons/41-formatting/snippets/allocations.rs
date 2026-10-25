use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt::{Arguments, Write as _};
use std::io::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

// Counts calls to the system allocator. (`unsafe impl` is boilerplate here.)
struct Counting;
static CALLS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        CALLS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTING: Counting = Counting;

fn count(work: impl FnOnce()) -> usize {
    let before = CALLS.load(Relaxed);
    work();
    CALLS.load(Relaxed) - before
}

// Writes the description out; no String is ever built.
fn pass_on(args: Arguments) {
    std::io::sink().write_fmt(args).unwrap();
}

fn main() {
    let n = 42;
    let mut empty = String::new();
    let mut room = String::with_capacity(64);
    println!(
        "format!(\"n = {{n}}\"):        {}",
        count(|| drop(format!("n = {n}")))
    );
    println!(
        "format!(\"{{n:>200}}\"):       {}",
        count(|| drop(format!("{n:>200}")))
    );
    println!(
        "write! into String::new(): {}",
        count(|| write!(empty, "n = {n}").unwrap())
    );
    println!(
        "write! with_capacity(64):  {}",
        count(|| write!(room, "n = {n}").unwrap())
    );
    println!(
        "pass format_args! on:      {}",
        count(|| pass_on(format_args!("n = {n}")))
    );
}
