#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
// src/lib.rs
use std::usize;

use pyo3::prelude::*;

// like this
// def sum_as_string(a:str, b:str) -> str:
//      return a+b
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Mount method to module
// #[pymodule]
// fn string_sum(py: Python, m: &PyModule) -> PyResult<()>{
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }

// like this
// def add(a:int, b:int) -> str:
//      return a+b
#[pyfunction]
fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok((a + b))
}

#[pymodule]
fn rust_math(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
