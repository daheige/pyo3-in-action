// 引入pyo3包
use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn explode<'a>(s: &'a str, sep: &'a str) -> Vec<&'a str> {
    let v = s.split(sep).collect();
    v
}

#[pyfunction]
fn implode(v: Vec<String>, sep: &str) -> String {
    let s = v.join(sep);
    s
}

// 将函数注册到模块string_utils中
#[pymodule]
fn string_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(explode, m)?)?;
    m.add_function(wrap_pyfunction!(implode, m)?)?;
    Ok(())
}
