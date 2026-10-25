use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Counts allocations and frees, so the checks can tell "moved the nodes" from "built
// new ones" and "freed them" from "leaked them". You do not need to read it.
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static FREED: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        FREED.fetch_add(1, Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTING: Counting = Counting;

fn allocated() -> usize {
    ALLOCATED.load(Ordering::Relaxed)
}

fn freed() -> usize {
    FREED.load(Ordering::Relaxed)
}

struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

/// A stack of numbers: a chain of boxed nodes, newest first.
struct Stack {
    head: Option<Box<Node>>,
}

impl Stack {
    fn new() -> Stack {
        Stack { head: None }
    }

    /// Put `value` on top.
    fn push(&mut self, value: u32) {
        todo!("a new Node whose `next` is the old head (take it with `self.head.take()`)")
    }

    /// Remove and return the top, or None when empty.
    fn pop(&mut self) -> Option<u32> {
        todo!("take the head; the new head is its `next`")
    }

    /// The top, without removing it.
    fn peek(&self) -> Option<&u32> {
        todo!("as_ref() on the head, then look at the node's value")
    }

    /// Turn the stack upside down by relinking the nodes it already has.
    fn reverse(&mut self) {
        todo!("walk the chain with take(), pointing each node's `next` at the one before it")
    }
}

impl Drop for Stack {
    // Dropping a Box drops the node inside it, which drops the next Box, and so on:
    // one stack frame per node. Free the nodes in a loop instead.
    fn drop(&mut self) {}
}

fn main() {
    // push, peek and pop
    let (a_pop, f_pop) = (allocated(), freed());
    let mut s = Stack::new();
    assert_eq!(s.pop(), None);
    assert_eq!(s.peek(), None);
    for v in [1, 2, 3] {
        s.push(v);
    }
    assert_eq!(s.peek(), Some(&3));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    assert_eq!(
        allocated() - a_pop,
        freed() - f_pop,
        "pop must free the node it removes"
    );

    // reverse moves the nodes it has: no new allocation
    let mut s = Stack::new();
    for v in 1..=5 {
        s.push(v);
    }
    let before = allocated();
    s.reverse();
    assert_eq!(
        allocated(),
        before,
        "reverse must relink the nodes, not build new ones"
    );
    let popped: Vec<u32> = std::iter::from_fn(|| s.pop()).collect();
    assert_eq!(popped, [1, 2, 3, 4, 5]);

    // an empty stack and a one-node stack reverse to themselves
    let mut s = Stack::new();
    s.reverse();
    assert_eq!(s.pop(), None);
    s.push(9);
    s.reverse();
    assert_eq!(s.pop(), Some(9));

    // dropping a stack frees every node
    let (a0, f0) = (allocated(), freed());
    {
        let mut s = Stack::new();
        for v in 0..10 {
            s.push(v);
        }
    }
    assert_eq!(allocated() - a0, 10);
    assert_eq!(freed() - f0, 10, "every node must be freed");

    // and a long stack is freed without one stack frame per node
    {
        let mut s = Stack::new();
        for v in 0..1_000_000 {
            s.push(v);
        }
    }

    println!("all checks passed");
}
