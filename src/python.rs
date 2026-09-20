use crate::value::{MAX_DEPTH, Value};
use pyo3::exceptions::{PyOverflowError, PyRecursionError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PySet, PyTuple};

pub fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    from_python_with_depth(obj, 0)
}
fn from_python_with_depth(obj: &Bound<'_, PyAny>, depth: usize) -> PyResult<Value> {
    if depth > MAX_DEPTH {
        return Err(PyRecursionError::new_err(
            "maximum nesting depth exceeded (circular reference?)",
        ));
    }
    if obj.is_none() {
        return Ok(Value::None);
    }

    if obj.is_instance_of::<PyBool>() {
        return Ok(Value::Bool(obj.extract::<bool>()?));
    }

    if obj.is_instance_of::<PyInt>() {
        return match obj.extract::<i64>() {
            Ok(value) => Ok(Value::Int(value)),
            Err(_) => Err(PyOverflowError::new_err(
                "integer out of range: fastpickle supports 64-bit signed integers only",
            )),
        };
    }

    if obj.is_instance_of::<PyFloat>() {
        return Ok(Value::Float(obj.extract::<f64>()?));
    }

    if let Ok(value) = obj.extract::<String>() {
        return Ok(Value::String(value));
    }

    if let Ok(value) = obj.cast::<PyBytes>() {
        return Ok(Value::Bytes(value.as_bytes().to_vec()));
    }

    if let Ok(list) = obj.cast::<PyList>() {
        let mut values = Vec::with_capacity(list.len());

        for item in list.iter() {
            values.push(from_python_with_depth(&item, depth + 1)?);
        }

        return Ok(Value::List(values));
    }

    if let Ok(tuple) = obj.cast::<PyTuple>() {
        let mut values = Vec::with_capacity(tuple.len());

        for item in tuple.iter() {
            values.push(from_python_with_depth(&item, depth + 1)?);
        }

        return Ok(Value::Tuple(values));
    }

    if let Ok(set) = obj.cast::<PySet>() {
        let mut values = Vec::with_capacity(set.len());

        for item in set.iter() {
            values.push(from_python_with_depth(&item, depth + 1)?);
        }

        return Ok(Value::Set(values));
    }

    if let Ok(dict) = obj.cast::<PyDict>() {
        let mut values = Vec::with_capacity(dict.len());

        for (key, value) in dict.iter() {
            values.push((
                from_python_with_depth(&key, depth + 1)?,
                from_python_with_depth(&value, depth + 1)?,
            ));
        }

        return Ok(Value::Dict(values));
    }

    if let Ok(dict_attr) = obj.getattr("__dict__") {
        if let Ok(state_dict) = dict_attr.cast::<PyDict>() {
            let class_name: String = obj
                .getattr("__class__")?
                .getattr("__qualname__")?
                .extract()?;

            let mut state = Vec::with_capacity(state_dict.len());
            for (key, value) in state_dict.iter() {
                state.push((
                    from_python_with_depth(&key, depth + 1)?,
                    from_python_with_depth(&value, depth + 1)?,
                ));
            }

            return Ok(Value::Object { class_name, state });
        }
    }

    let type_name = obj.get_type().fully_qualified_name()?;

    Err(PyTypeError::new_err(format!(
        "Unsupported type: {type_name}"
    )))
}

pub fn to_python<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
    match value {
        Value::None => Ok(py.None().into_bound(py).into_any()),
        Value::Bool(b) => Ok((*b).into_pyobject(py)?.to_owned().into_any()),
        Value::Int(i) => Ok(i.into_pyobject(py)?.into_any()),
        Value::Float(f) => Ok(f.into_pyobject(py)?.into_any()),
        Value::String(s) => Ok(s.into_pyobject(py)?.into_any()),
        Value::Bytes(b) => Ok(PyBytes::new(py, b).into_any()),
        Value::List(values) => {
            let list = PyList::empty(py);
            for value in values {
                list.append(to_python(py, value)?)?;
            }
            Ok(list.into_any())
        }
        Value::Tuple(values) => {
            let items = values
                .iter()
                .map(|value| to_python(py, value))
                .collect::<PyResult<Vec<_>>>()?;
            Ok(PyTuple::new(py, items)?.into_any())
        }
        Value::Set(values) => {
            let set = PySet::empty(py).unwrap();
            for value in values {
                set.add(to_python(py, value)?)?;
            }
            Ok(set.into_any())
        }
        Value::Dict(values) => {
            let dict = PyDict::new(py);
            for (key, value) in values {
                dict.set_item(to_python(py, key)?, to_python(py, value)?)?;
            }
            Ok(dict.into_any())
        }
        Value::Object { class_name, state } => {
            let dict = PyDict::new(py);
            for (key, value) in state {
                dict.set_item(to_python(py, key)?, to_python(py, value)?)?;
            }
            let tuple = PyTuple::new(
                py,
                [class_name.into_pyobject(py)?.into_any(), dict.into_any()],
            )?;
            Ok(tuple.into_any())
        }
    }
}
