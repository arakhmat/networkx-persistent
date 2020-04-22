#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

extern crate pyrpds;

mod nodes_container;
mod python;

#[pymodule]
fn networkx_persistent(py: Python, m: &PyModule) -> PyResult<()> {
    python::nodes_container::py_binding(py, m)?;

    Ok(())
}
