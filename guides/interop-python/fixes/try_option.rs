use pyo3::prelude::*;

/// The longest word, or None for an empty list.
#[pyfunction]
fn longest(words: Vec<String>) -> Option<String> {
    words.into_iter().max_by_key(|w| w.chars().count())
}

#[pymodule]
mod wordstats {
    #[pymodule_export]
    use super::longest;
}
