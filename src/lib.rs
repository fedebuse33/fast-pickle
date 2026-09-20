use pyo3::prelude::*;
mod decoder;
mod encoder;
mod errors;
mod python;
mod tags;
mod value;
use decoder::Decoder;
use encoder::Encoder;
use errors::DecodeError;
use value::Value;

pub fn encode(value: &Value) -> Vec<u8> {
    let mut encoder = Encoder::new();
    encoder.encode(value);
    encoder.into_bytes()
}
pub fn decode(data: &[u8]) -> Result<Value, DecodeError> {
    let mut decoder = Decoder::new(data);
    decoder.decode()
}

#[pymodule]
mod _core {
    use super::*;
    use pyo3::exceptions::PyValueError;
    use pyo3::types::PyBytes;

    #[pyfunction]
    fn dumps(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyBytes>> {
        let value = python::from_python(obj)?;
        let bytes = encode(&value);
        Ok(PyBytes::new(py, &bytes).into())
    }

    #[pyfunction]
    fn loads<'py>(py: Python<'py>, data: &Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyAny>> {
        let value =
            decode(data.as_bytes()).map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        python::to_python(py, &value)
    }
}

#[cfg(test)]
mod tests;
