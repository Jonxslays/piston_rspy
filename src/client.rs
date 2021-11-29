use pyo3::prelude::*;
use pyo3::types::PyType;
use std::collections::HashMap;

use piston_rs::Client as Client_;

use super::ExecResponse;
use super::ExecResult;
use super::Executor;
use super::Runtime;

#[pyclass]
pub struct Client {
    inner: Client_,
    headers: HashMap<String, String>,
}

#[pymethods]
impl Client {
    #[new]
    fn new() -> Self {
        let inner = Client_::new();
        let headers = inner
            .get_headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
            .collect();

        Self { inner, headers }
    }

    #[staticmethod]
    fn with_key(key: String) -> Self {
        let inner = Client_::with_key(&key);
        let headers = inner
            .get_headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
            .collect();

        Self { inner, headers }
    }

    #[getter]
    fn url(&self) -> String {
        self.inner.get_url()
    }

    #[getter]
    fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    #[pyo3(text_signature = "($self) -> dict[str, str]")]
    fn get_headers(&self) -> HashMap<String, String> {
        self.headers()
    }
}
