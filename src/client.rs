use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObjectProtocol;
use std::collections::HashMap;

use piston_rs::Client as Client_;

use super::ExecResponse;
use super::Executor;
use super::Runtime;

/// A client used to send requests to Piston.
///
/// ### Returns:
/// - `Client`: The new client.
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "() -> Client")]
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

    /// Creates a new client, with an api key.
    ///
    /// ### Args:
    ///
    /// - key `str`:
    /// The api key to use.
    ///
    /// ### Returns:
    ///
    /// - `Client`: The new client.
    #[staticmethod]
    #[pyo3(text_signature = "(key: str, /) -> Client")]
    fn with_key(key: String) -> Self {
        let inner = Client_::with_key(&key);
        let headers = inner
            .get_headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
            .collect();

        Self { inner, headers }
    }

    /// `str`: The base url for the Piston v2 api.
    #[getter]
    fn url(&self) -> String {
        self.inner.get_url()
    }

    /// The base url for the Piston v2 api.
    /// **NOTE**: The url is immutable.
    ///
    /// ### Returns:
    ///
    /// - `str`: The url.
    #[pyo3(text_signature = "(self) -> str")]
    fn get_url(&self) -> String {
        self.url()
    }

    /// `dict[str, str]`: The headers being sent with requests.
    #[getter]
    fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// The headers being sent with requests.
    /// **NOTE**: The headers are immutable.
    ///
    /// ### Returns:
    ///
    /// - `dict[str, str]`: The headers.
    #[pyo3(text_signature = "(self) -> dict[str, str]")]
    fn get_headers(&self) -> HashMap<String, String> {
        self.headers()
    }

    /// **CORO**: Fetches the runtimes from Piston. This is an http request.
    ///
    /// ### Returns:
    ///
    /// - `list[Runtime]`: The available Piston runtimes.
    ///
    /// ### Raises:
    ///
    /// - `FailedRequest`: If the request to Piston failed.
    #[pyo3(text_signature = "(self) -> list[Runtime]")]
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

    /// **CORO**: Executes code using a given executor. This is an http request.
    ///
    /// ### Args:
    ///
    /// - executor `Executor`:
    /// The executor to use for the request.
    ///
    /// ### Returns:
    ///
    /// - `ExecResponse`: The response from Piston.
    ///
    /// ### Raises:
    ///
    /// - `FailedRequest`: If the request to Piston failed.
    #[pyo3(text_signature = "(self, executor: Executor, /) -> ExecResponse")]
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
