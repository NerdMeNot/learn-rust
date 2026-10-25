use pyo3::prelude::*;

/// Length of a list, computed with the GIL released.
#[pyfunction]
fn slow_len(py: Python<'_>, items: Vec<String>) -> usize {
    py.detach(move || items.len())
}

#[pymodule]
mod wordstats {
    #[pymodule_export]
    use super::slow_len;
}
