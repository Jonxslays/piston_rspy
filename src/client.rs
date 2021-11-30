use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObjectProtocol;
use std::collections::HashMap;

use piston_rs::Client as Client_;

use super::ExecResponse;
use super::Executor;
use super::Runtime;

#[pyclass]
#[pyo3(text_signature = "() -> Client")]
#[derive(Clone)]
pub struct Client {
    inner: Client_,
    headers: HashMap<String, String>,
}

#[pyproto]
impl PyObjectProtocol for Client {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Client {{ url: \"{}\", headers: {:?} }}",
            self.url(),
            self.headers,
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
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
    #[pyo3(text_signature = "(key: str) -> Client")]
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

    #[pyo3(text_signature = "($self) -> str")]
    fn get_url(&self) -> String {
        self.url()
    }

    #[getter]
    fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    #[pyo3(text_signature = "($self) -> dict[str, str]")]
    fn get_headers(&self) -> HashMap<String, String> {
        self.headers()
    }

    #[pyo3(text_signature = "($self) -> list[Runtime]")]
    fn fetch_runtimes<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.inner.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            match client.fetch_runtimes().await {
                Ok(runtimes) => Ok(Python::with_gil(|py| {
                    PyList::new(
                        py,
                        runtimes
                            .iter()
                            .map(|r| Runtime::from_runtime(r.to_owned()).into_py(py)),
                    )
                    .into()
                })),
                Err(e) => Err(Python::with_gil(|_| {
                    super::FailedRequest::new_err(format!("{:?}", e))
                })),
            }
        })
    }

    #[pyo3(text_signature = "($self, executor: Executor, /) -> ExecResponse | str")]
    fn execute<'a>(&self, py: Python<'a>, executor: &Executor) -> PyResult<&'a PyAny> {
        let client = self.inner.clone();
        let exec = executor.convert();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            match client.execute(&exec).await {
                Ok(response) => Ok(Python::with_gil(|py| {
                    ExecResponse::from_response(response).into_py(py)
                })),
                Err(e) => Err(Python::with_gil(|_| {
                    super::FailedRequest::new_err(format!("{:?}", e))
                })),
            }
        })
    }
}
