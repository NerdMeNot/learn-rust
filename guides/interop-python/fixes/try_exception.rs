use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

pyo3::create_exception!(wordstats, EmptyText, PyValueError);

/// Count the words, or raise EmptyText when there are none.
#[pyfunction]
fn count_words(text: &str) -> PyResult<usize> {
    match text.split_whitespace().count() {
        0 => Err(EmptyText::new_err("no words in the text")),
        n => Ok(n),
    }
}

#[pymodule]
mod wordstats {
    #[pymodule_export]
    use super::{EmptyText, count_words};
}
