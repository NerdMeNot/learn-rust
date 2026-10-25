mod words;

use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Passes every request on to the system allocator, and counts two kinds:
/// new blocks (`alloc`) and blocks resized in place or moved (`realloc`).
struct Counting;

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Runs `build`, and returns what it built with the number of `alloc` and
/// `realloc` calls it made.
fn measure<T>(build: impl FnOnce() -> T) -> (T, usize, usize) {
    let allocs = ALLOCS.load(Ordering::Relaxed);
    let reallocs = REALLOCS.load(Ordering::Relaxed);
    let built = build();
    (
        built,
        ALLOCS.load(Ordering::Relaxed) - allocs,
        REALLOCS.load(Ordering::Relaxed) - reallocs,
    )
}

fn sorted<K: ToString>(counts: &HashMap<K, usize>) -> Vec<(String, usize)> {
    let mut all: Vec<(String, usize)> = counts.iter().map(|(k, n)| (k.to_string(), *n)).collect();
    all.sort();
    all
}

fn main() {
    // 1,000 copies of the sample, built before anything is counted.
    let text = fs::read_to_string("sample.txt").unwrap().repeat(1000);
    let lower = text.to_lowercase();
    let _warm_up: HashMap<String, usize> = HashMap::new();

    // A: what count_words does. A new lowercase String for every word.
    let (a, a_allocs, a_reallocs) = measure(|| {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for word in words::words(&text) {
            *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
        counts
    });

    // B: lowercase the whole text once, and keep &str keys that point into it.
    let lower_once = measure(|| text.to_lowercase());
    let (b, b_allocs, b_reallocs) = measure(|| {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for word in words::words(&lower_once.0) {
            *counts.entry(word).or_insert(0) += 1;
        }
        counts
    });
    let (b_allocs, b_reallocs) = (b_allocs + lower_once.1, b_reallocs + lower_once.2);

    // C: owned keys, cloned for every word, even when the key is already in the map.
    let (c, c_allocs, c_reallocs) = measure(|| {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for word in words::words(&lower) {
            *counts.entry(word.to_string()).or_insert(0) += 1;
        }
        counts
    });

    // D: owned keys, cloned only the first time a word is seen.
    let (d, d_allocs, d_reallocs) = measure(|| {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for word in words::words(&lower) {
            match counts.get_mut(word) {
                Some(n) => *n += 1,
                None => {
                    counts.insert(word.to_string(), 1);
                }
            }
        }
        counts
    });

    println!(
        "words: {}, distinct: {}",
        words::words(&text).len(),
        a.len()
    );
    println!(
        "same counts in all four: {}",
        sorted(&a) == sorted(&b) && sorted(&a) == sorted(&c) && sorted(&a) == sorted(&d)
    );
    println!();
    println!("                                       alloc calls  realloc calls");
    let rows = [
        ("A  entry(word.to_lowercase())", a_allocs, a_reallocs),
        ("B  one lowercase copy, &str keys", b_allocs, b_reallocs),
        ("C  entry(word.to_string())", c_allocs, c_reallocs),
        ("D  get_mut, insert(word.to_string())", d_allocs, d_reallocs),
    ];
    for (name, allocs, reallocs) in rows {
        println!("{name:<38} {allocs:>11}  {reallocs:>13}");
    }
}
