use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt::Write as _;
use std::hint::black_box;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

// Counts calls to the system allocator. (`unsafe impl` is boilerplate here: the
// Unsafe lesson explains why it is needed.)
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
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        CALLS.fetch_add(1, Relaxed); // growing a buffer in place or by copying: one call
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static COUNTING: Counting = Counting;

// Runs `work` and returns how many times it asked the allocator for memory.
fn count<T>(work: impl FnOnce() -> T) -> usize {
    let before = CALLS.load(Relaxed);
    black_box(work());
    CALLS.load(Relaxed) - before
}

fn main() {
    let line = String::from("alpha beta gamma delta epsilon zeta eta theta");

    // Owned pieces versus borrowed pieces of the same line.
    println!("split into Vec<String>: {}", count(|| line.split(' ').map(String::from).collect::<Vec<_>>()));
    println!("split into Vec<&str>:   {}", count(|| line.split(' ').collect::<Vec<_>>()));

    // Cloning: a String copies its text; an Rc only bumps a counter.
    let shared = Rc::new(line.clone());
    println!("String::clone:          {}", count(|| line.clone()));
    println!("Rc::clone:              {}", count(|| Rc::clone(&shared)));

    // Growing a Vec one push at a time, or asking for the room first.
    println!("1000 pushes, no hint:   {}", count(|| {
        let mut v = Vec::new();
        for i in 0..1000u32 {
            v.push(i);
        }
        v
    }));
    println!("1000 pushes, capacity:  {}", count(|| {
        let mut v = Vec::with_capacity(1000);
        for i in 0..1000u32 {
            v.push(i);
        }
        v
    }));

    // Building 100 lines of text: a new String for each, or one buffer written into.
    println!("format! x100:           {}", count(|| (0..100).map(|i| format!("item {i}")).collect::<Vec<_>>()));
    println!("write! into one String: {}", count(|| {
        let mut out = String::new();
        for i in 0..100 {
            writeln!(out, "item {i}").unwrap();
        }
        out
    }));
}
