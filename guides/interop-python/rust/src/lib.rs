//! Word counting written in Rust, called from Python.
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

/// Add two integers.
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Upper-case the text and add an exclamation mark.
#[pyfunction]
fn shout(text: &str) -> String {
    format!("{}!", text.to_uppercase())
}

/// Count the words in `text`, ignoring case. The dict is sorted by word.
#[pyfunction]
fn word_counts(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// Sum a list of floats.
#[pyfunction]
fn total(values: Vec<f64>) -> f64 {
    values.iter().sum()
}

/// Parse "3,4" into (3, 4). A bad input raises ValueError.
#[pyfunction]
fn parse_pair(text: &str) -> PyResult<(i64, i64)> {
    let (left, right) = text
        .split_once(',')
        .ok_or_else(|| PyValueError::new_err(format!("expected 'a,b', got {text:?}")))?;
    // `?` turns a ParseIntError into a Python ValueError.
    Ok((left.trim().parse()?, right.trim().parse()?))
}

/// A Rust panic, to see what Python gets.
#[pyfunction]
fn explode(text: &str) -> i32 {
    text.parse().unwrap()
}

/// A word counter that lives in Rust and is used like a Python object.
#[pyclass]
struct Counter {
    counts: BTreeMap<String, usize>,
}

#[pymethods]
impl Counter {
    #[new]
    fn new() -> Self {
        Counter { counts: BTreeMap::new() }
    }

    fn add(&mut self, word: &str) {
        *self.counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    /// Add every count from `other` into this counter.
    fn merge(&mut self, other: PyRef<'_, Counter>) {
        for (word, n) in &other.counts {
            *self.counts.entry(word.clone()).or_insert(0) += n;
        }
    }

    /// The `n` most frequent words, most frequent first, ties by word.
    fn top(&self, n: usize) -> Vec<(String, usize)> {
        let mut all: Vec<_> = self.counts.iter().map(|(w, c)| (w.clone(), *c)).collect();
        all.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        all.truncate(n);
        all
    }

    #[getter]
    fn distinct(&self) -> usize {
        self.counts.len()
    }

    fn __repr__(&self) -> String {
        format!("Counter({} distinct words)", self.counts.len())
    }
}

// A flag that one thread can wait for and another can set.
static FLAG: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());

fn wait_for_flag(timeout_ms: u64) -> bool {
    let (lock, signal) = &FLAG;
    let guard = lock.lock().unwrap();
    let (guard, _) = signal
        .wait_timeout_while(guard, Duration::from_millis(timeout_ms), |set| !*set)
        .unwrap();
    *guard
}

#[pyfunction]
fn reset_flag() {
    *FLAG.0.lock().unwrap() = false;
}

#[pyfunction]
fn set_flag() {
    *FLAG.0.lock().unwrap() = true;
    FLAG.1.notify_all();
}

/// Wait for the flag while still holding the GIL.
#[pyfunction]
fn wait_holding_gil(timeout_ms: u64) -> bool {
    wait_for_flag(timeout_ms)
}

/// Wait for the flag with the GIL released.
#[pyfunction]
fn wait_detached(py: Python<'_>, timeout_ms: u64) -> bool {
    py.detach(|| wait_for_flag(timeout_ms))
}

#[pymodule]
mod wordstats {
    #[pymodule_export]
    use super::{
        Counter, add, explode, parse_pair, reset_flag, set_flag, shout, total, wait_detached,
        wait_holding_gil, word_counts,
    };
}
