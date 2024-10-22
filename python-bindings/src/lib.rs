use arrow::array::{ArrayData, Float64Array, Int64Array};
use arrow::datatypes::DataType;
use arrow::pyarrow::PyArrowType;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn double_arr(arr: PyArrowType<ArrayData>) -> PyResult<PyArrowType<ArrayData>> {
    let arr = arr.0; // get from the pyarrow type wrapper

    let doubled_data: ArrayData = match arr.data_type() {
        DataType::Int64 => {
            let int_arr: Int64Array = arr.into();
            int_arr
            .iter()
            .map(|v| v.map(|x| x * 2))
            .collect::<Int64Array>().into()
        }
        DataType::Float64 => {
            let float_arr: Float64Array = arr.into();
            float_arr
                .iter()
                .map(|v| v.map(|x| x * 2.0))
                .collect::<Float64Array>()
                .into()
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported array type. Expected Float64Array or Int64Array.",
            ))
        }
    };

    // Wrap the result in PyArrowType and return
    Ok(PyArrowType(doubled_data))
}


/// A Python module implemented in Rust.
#[pymodule]
fn dan(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double_arr, m)?)?;
    Ok(())
}
