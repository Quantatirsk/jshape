use ::jshape as core;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn analyze_json_impl(input: &str, show_examples: bool) -> Result<String, String> {
    core::analyze_json(input, show_examples)
}

#[pyfunction(signature = (input, show_examples = true))]
fn analyze_json(input: &str, show_examples: bool) -> PyResult<String> {
    analyze_json_impl(input, show_examples).map_err(PyValueError::new_err)
}

#[pymodule]
fn jshape(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_json, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::analyze_json_impl;

    #[test]
    fn analyzes_json_from_python_binding() {
        let output = analyze_json_impl(r#"{"user":{"name":"Ada"}}"#, true).unwrap();
        assert!(output.contains("\"user\""));
        assert!(output.contains("\"name\""));
    }
}
