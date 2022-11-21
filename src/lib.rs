use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyBytes;

#[pyfunction]
fn get(_py: Python, url: String) -> PyResult<&PyDict> {
    let res = reqwest::blocking::get(url).unwrap();

    let d: &PyDict = PyDict::new(_py);
    d.set_item("status_code", res.status().as_u16())?;
    d.set_item("content", PyBytes::new(_py, &(res.bytes().unwrap().to_vec())[..]))?;    
    return Ok(d);
}

/// A Python module implemented in Rust.
#[pymodule]
fn ruquest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}
