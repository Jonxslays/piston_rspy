use pyo3::prelude::*;

use piston_rs::Client as Client_;

use super::ExecResponse;
use super::ExecResult;
use super::Executor;
use super::Runtime;

#[pyclass]
pub struct Client {
    inner: Client_,
}

#[pymethods]
impl Client {
    #[new]
    fn new() -> Self {
        Self {
            inner: Client_::new(),
        }
    }
}
