use nokhwa::{native_api_backend, query};
use pyo3::{prelude::*, Python};
use pythonize::pythonize;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct CameraInfo {
    name: String,
    index: String,
}

/// Returns a list of cameras.
#[pyfunction]
fn camera_list() -> Py<PyAny> {
    let backend = native_api_backend().unwrap();
    let cameras = query(backend).unwrap();
    let infos = cameras
        .iter()
        .map(|camera| CameraInfo {
            name: camera.human_name(),
            index: camera.index().as_string(),
        })
        .collect::<Vec<_>>();

    Python::with_gil(|py| -> Py<PyAny> { pythonize(py, &infos).unwrap() })
}

#[pymodule]
fn _opencvutil(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(camera_list, m)?)?;
    Ok(())
}
