use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use std::thread;

/// Numbers in, number out. `+` panics on overflow in a debug build.
#[napi]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// The same job, with the failure as a value.
#[napi]
pub fn add_checked(a: i32, b: i32) -> Result<i32> {
    a.checked_add(b)
        .ok_or_else(|| Error::new(Status::GenericFailure, "i32 overflow"))
}

/// The same `+`, with the panic caught at the boundary and thrown as a JavaScript `Error`.
#[napi(catch_unwind)]
pub fn add_caught(a: i32, b: i32) -> i32 {
    a + b
}

/// A struct that becomes a plain JavaScript object, copied field by field.
#[napi(object)]
pub struct Stats {
    pub count: u32,
    pub mean: f64,
    pub largest: Option<i32>,
}

/// The JavaScript array is copied into a `Vec`.
#[napi]
pub fn stats(values: Vec<i32>) -> Stats {
    let sum: i64 = values.iter().map(|&v| i64::from(v)).sum();
    Stats {
        count: values.len() as u32,
        mean: if values.is_empty() { 0.0 } else { sum as f64 / values.len() as f64 },
        largest: values.iter().copied().max(),
    }
}

/// An `Err` becomes a thrown JavaScript `Error`.
#[napi]
pub fn parse_port(text: String) -> Result<u16> {
    text.trim()
        .parse::<u16>()
        .map_err(|e| Error::new(Status::InvalidArg, format!("not a port: {e}")))
}

/// A Rust struct that JavaScript holds as a class instance.
#[napi]
pub struct Counter {
    count: u32,
}

#[napi]
impl Counter {
    #[napi(constructor)]
    pub fn new(start: u32) -> Self {
        Counter { count: start }
    }

    #[napi]
    pub fn increment(&mut self) -> u32 {
        self.count += 1;
        self.count
    }

    #[napi(getter)]
    pub fn count(&self) -> u32 {
        self.count
    }
}

/// A `Buffer` argument is JavaScript's own memory: this changes the caller's bytes.
#[napi]
pub fn xor_in_place(mut data: Buffer, key: u8) {
    for byte in data.iter_mut() {
        *byte ^= key;
    }
}

/// A `Vec<u8>` argument is a copy: the caller's array is untouched.
#[napi]
pub fn xor_copy(mut data: Vec<u8>, key: u8) -> Vec<u8> {
    for byte in data.iter_mut() {
        *byte ^= key;
    }
    data
}

/// An `async fn` returns a Promise; the body runs on napi's Tokio runtime.
#[napi]
pub async fn count_words(text: String) -> u32 {
    text.split_whitespace().count() as u32
}

/// Which thread am I on? Rust's `ThreadId`, as text.
#[napi]
pub fn thread_id() -> String {
    format!("{:?}", thread::current().id())
}

#[napi]
pub async fn thread_id_async() -> String {
    format!("{:?}", thread::current().id())
}

/// A callback that a Rust thread may call. `false` means the callback takes
/// the value alone, with no leading error argument.
#[napi]
pub fn count_up(limit: u32, on_tick: ThreadsafeFunction<u32, (), u32, Status, false>) {
    thread::spawn(move || {
        for i in 1..=limit {
            on_tick.call(i, ThreadsafeFunctionCallMode::Blocking);
        }
    });
}
