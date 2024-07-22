use pyo3::pyfunction;
use pyo3_polars::PyDataFrame;
use pyo3::prelude::*;

#[pyfunction]
fn empty_df() -> PyDataFrame {
    PyDataFrame(polars::frame::DataFrame::empty())
}

#[pymodule]
fn repro_bug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(empty_df, m)?)?;
    Ok(())
}
