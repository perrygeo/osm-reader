use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn rust_func(_pbf_path: String) -> usize {
    15
}

/// A Python module implemented in Rust.
#[pymodule]
fn _pyo3_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(rust_func))?;

    Ok(())
}
