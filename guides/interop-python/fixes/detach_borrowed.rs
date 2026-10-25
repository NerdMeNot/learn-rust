use pyo3::prelude::*;
use pyo3::types::PyList;

/// Length of a list, computed with the GIL released. This does not compile.
#[pyfunction]
fn slow_len(py: Python<'_>, items: Bound<'_, PyList>) -> usize {
    py.detach(|| items.len())
}

#[pymodule]
mod wordstats {
    #[pymodule_export]
    use super::slow_len;
}
